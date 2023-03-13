use std::{
    thread,
    time::{self, Duration},
};

use cardano_embedded_sdk::{crypto::Ed25519Signature, types::XPubKey};
use cardano_serialization_lib::{
    address::{Address, EnterpriseAddress, StakeCredential},
    crypto::Ed25519KeyHash,
};

use cardano_embedded_sdk::crypto as sdk_crypto;

use clap::{command, Parser};
use derivation_path::DerivationPath;
use device_dummy::DeviceDummy;
use node_client::{Network, NodeClient};

use crate::{serialization::{In, Out}, types::DeviceData};

mod device_dummy;
mod node_client;
mod serialization;
mod transport;
mod tx_build;
mod tx_envelope;
mod types;

#[derive(Parser, Debug)]
#[command(about, long_about = None)]
struct Args {
    /// Mnemonics for HD wallet
    #[arg(long)]
    mnemonics: String,
    /// HD wallet password
    #[arg(long)]
    password: String,
    /// Address of script that will store sensor data (address should correspond to network type!)
    #[arg(long)]
    /// Derivation path for keys
    derivation_path: String,
    #[arg(long)]
    script_address: String,
    /// Network type
    #[arg(long)]
    network: Network,
    /// Path to node socket
    #[arg(long)]
    node_socket: String,
}

fn main() {
    let args = Args::parse();

    let mnemonics = args.mnemonics.as_str();
    let password = args.password;
    let password_b = password.as_bytes();
    let network = args.network;

    let derivation_path: DerivationPath = args
        .derivation_path
        .parse()
        .expect("Should parse derivation path");

    // address of always succeeds script
    let script_address =
        &Address::from_bech32(args.script_address.as_str()).expect("Should parse script address");

    let node_client = node_client::CliNodeClient::new(args.node_socket, args.network);

    let ports = serialport::available_ports().expect("No ports found!");
    println!("ports:");
    for p in ports {
        println!("{}", p.port_name);
    }

    let mut port = serialport::new("/dev/ttyACM0", 115_200)
        .timeout(Duration::from_millis(100000))
        .open()
        .expect("Failed to open port");

    println!("sending init");
    transport::send(&mut port, In::Init(mnemonics.to_string()));
    println!("receiving init");
    println!("received init {:#?}", transport::recieve(&mut port));

    println!("sending temp");
    transport::send(
        &mut port,
        In::Temp(password_b.to_vec(), derivation_path.to_string()),
    );
    println!("receiving temp");
    let temp_data = transport::recieve(&mut port);
    let device_data = match temp_data {
        Ok(Some(Out::Temp(sensor_readings, signed_readings))) => DeviceData {sensor_readings, signed_readings},
        x => panic!("Failed to get temperature data. Got: {:?}", x),
    };
    println!("Receiveddevice data {:?}", device_data);

    // FXIME: should be able to get pub key from device, not Dummy

    println!("sending pub key request");
    let pub_key_request = In::PubKey(
        password_b.to_vec(),
        derivation_path.to_string(),
    );
    transport::send(&mut port, pub_key_request);
    println!("receiving pub key");
    let result = transport::recieve(&mut port);
    let pub_key_hex = match result {
        Ok(Some(Out::PubKey(key_hex))) => key_hex,
        x => panic!("Could not get hex of public key. Device returned: {:?}", x),
    };
    let pub_key = match XPubKey::from_hex(&pub_key_hex) {
        Ok(pk) => pk,
        Err(err) => panic!("Could not parse pub key from hex: {:?}", err),
    };


    let device_wallet_address = EnterpriseAddress::new(
        translate_network(network),
        &StakeCredential::from_keyhash(
            &Ed25519KeyHash::from_hex(pub_key.hash_hex().as_str())
                .expect("Should be able to parse public key hash from hex"),
        ),
    )
    .to_address();


    let (inputs, ins_total_value) = node_client
        .query_inputs(&device_wallet_address)
        .expect("Should return inputs from user address. Is node running and available?");


    // make unsigned Tx (with empty witness set) to get id
    println!("Making unsigned Tx");
    let unsigned_tx = tx_build::make_unsigned_tx(
        &device_wallet_address,
        &script_address,
        device_data,
        &inputs,
        ins_total_value, //for balancing
    );

    let tx_id = node_client.get_tx_id(&unsigned_tx);

    println!("Tx ID: {}", tx_id.to_hex());

    println!("sending sign");
    let tx_id_sign_reques = In::Sign(
        tx_id.to_bytes().to_vec(),
        password_b.to_vec(),
        derivation_path.to_string(),
    );
    transport::send(&mut port, tx_id_sign_reques);
    println!("receiving sign");
    let result = transport::recieve(&mut port);

    let tx_signature = match result {
        Ok(Some(Out::Sign(signature))) => {
            signature
        },
        x => panic!("Could not get Tx signature. Device returned: {:?}", x),
    };
    // println!("received sign {:#?}", &result);

    // let signature = device.sign_tx_id(&tx_id, password, derivation_path);

    let signed_tx = tx_build::make_signed_tx(&unsigned_tx, &pub_key, tx_signature);

    let submit_result = node_client.submit_tx(&signed_tx);
    println!("Submission result: {:?}", submit_result)


    // REST OF STUFF
    // // for _ in 0..5 {
    // submit_data_to_blockchain(
    //     &node_client,
    //     &device,
    //     args.network,
    //     &script_address,
    //     args.password.as_str(),
    //     &derivation_path,
    // );
    // thread::sleep(time::Duration::from_secs(2))
    // // }
}

fn submit_data_to_blockchain(
    node_client: &impl NodeClient,
    device: &DeviceDummy,
    network: Network,
    script_address: &Address,
    password: &str,
    derivation_path: &DerivationPath,
) {
    let pub_key = device.get_pub_key(password, derivation_path);
    // build users wallet address from public key
    let user_wallet_address = EnterpriseAddress::new(
        translate_network(network),
        &StakeCredential::from_keyhash(
            &Ed25519KeyHash::from_hex(pub_key.hash_hex().as_str())
                .expect("Should be able to parse public key hash from hex"),
        ),
    )
    .to_address();

    // todo: throw error if inputs empty
    let (inputs, ins_total_value) = node_client
        .query_inputs(&user_wallet_address)
        .expect("Should return inputs from user address. Is node running and available?");

    let device_data = device.get_signed_sensor_data(password, derivation_path);

    // make unsigned Tx (with empty witness set) to get id
    let unsigned_tx = tx_build::make_unsigned_tx(
        &user_wallet_address,
        &script_address,
        device_data,
        &inputs,
        ins_total_value, //for balancing
    );

    let tx_id = node_client.get_tx_id(&unsigned_tx);

    println!("Tx ID: {}", tx_id.to_hex());

    let signature = device.sign_tx_id(&tx_id, password, derivation_path);

    let signed_tx = tx_build::make_signed_tx(&unsigned_tx, &pub_key, signature.to_bytes());

    let submit_result = node_client.submit_tx(&signed_tx);
    println!("Submission result: {:?}", submit_result)
}

fn translate_network(net: Network) -> u8 {
    match net {
        Network::Mainnet => 1,
        Network::Preprod => 0,
    }
}
