use std::{thread, time};

use cardano_serialization_lib::{
    address::{Address, EnterpriseAddress, StakeCredential},
    crypto::Ed25519KeyHash,
};

use clap::{command, Parser};
use derivation_path::DerivationPath;
use device_dummy::DeviceDummy;
use node_client::{Network, NodeClient};

mod device_dummy;
mod node_client;
mod tx_build;
mod tx_envelope;

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

    let derivation_path: DerivationPath = args
        .derivation_path
        .parse()
        .expect("Should parse derivation path");

    // address of always succeeds script
    let script_address =
        &Address::from_bech32(args.script_address.as_str()).expect("Should parse script address");

    let node_client = node_client::CliNodeClient::new(args.node_socket, args.network);

    let device = device_dummy::DeviceDummy::init(args.mnemonics.as_str());

    // for _ in 0..5 {
    submit_data_to_blockchain(
        &node_client,
        &device,
        args.network,
        &script_address,
        args.password.as_str(),
        &derivation_path,
    );
    thread::sleep(time::Duration::from_secs(2))
    // }
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
        &pub_key,
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
