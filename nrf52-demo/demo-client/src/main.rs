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
use node_client::{Network, NodeClient};

use crate::{
    device::Device,
    serialization::{In, Out},
};

mod device;
mod node_client;
mod serialization;
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

    let mnemonics = args.mnemonics;
    let password = args.password;
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

    
    println!("Creating and Initializing device");
    let mut device = Device::new("/dev/ttyACM0");
    device.init(mnemonics);
    
    println!("Requesting temperature data");
    let temp_data = device.query_sensor_data(&password, &derivation_path);
    println!("Received sensor data: {:?}", temp_data.sensor_readings);


    println!("Building transaction");
    // Receive public key from device for given derivation path
    let pub_key = device.get_public_key(&password, &derivation_path);

    // Make address from received public key
    // This address will be used to receive UTXOs for balancing and send back change
    let device_wallet_address = EnterpriseAddress::new(
        translate_network(network),
        &StakeCredential::from_keyhash(
            &Ed25519KeyHash::from_hex(pub_key.hash_hex().as_str())
                .expect("Should be able to parse public key hash from hex"),
        ),
    )
    .to_address();

    // Get UTXOs from device address for balancing
    let (inputs, ins_total_value) = node_client
        .query_inputs(&device_wallet_address)
        .expect("Should return inputs from user address. Is node running and available?");

    // Build balanced unsigned transaction
    // Transaction will have output for script address
    // with inlined datum which holds temperature data from the device
    println!("Making unsigned Tx");
    let unsigned_tx = tx_build::make_unsigned_tx(
        &device_wallet_address,
        &script_address,
        temp_data,
        &inputs,
        ins_total_value, //for balancing
    );

    let tx_id = node_client.get_tx_id(&unsigned_tx);

    println!("Transaction built and balanced.\n Transaction ID: {}", tx_id.to_hex());
    
    
    println!("Signing transaction ID: {}", tx_id.to_hex());
    let tx_signature = device.sign_transaction_id(&tx_id, &password, &derivation_path);
    
    println!("Adding signature to transaction");
    let signed_tx = tx_build::make_signed_tx(&unsigned_tx, &pub_key, tx_signature);
    
    println!("Submitting signed transaction");
    let submit_result = node_client.submit_tx(&signed_tx);
    println!("Submission result: {:?}", submit_result)
}

fn translate_network(net: Network) -> u8 {
    match net {
        Network::Mainnet => 1,
        Network::Preprod => 0,
    }
}
