use cardano_embedded_sdk::crypto::Ed25519Signature;
use cardano_serialization_lib::{
    address::{Address, EnterpriseAddress, StakeCredential},
    crypto::Ed25519KeyHash,
};

use clap::{command, Parser};
use derivation_path::DerivationPath;
use node_client::{Network, NodeClient};

use crate::device::Device;

mod device;
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
    #[arg(long)]
    device_port: String,
}

fn main() {
    let args = Args::parse();

    let mnemonics = args.mnemonics;
    let password = args.password;
    let network = args.network;
    let device_port = args.device_port;

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

    build_and_stream_tx(
        mnemonics,
        &password,
        &derivation_path,
        script_address,
        &node_client,
        network,
        &device_port,
    )
}

/// Performs following steps:
/// - Initializes device with mnemonic
/// - Gets mock temperature data to build example transaction
/// - Requests public key from device for account 0 address 0
/// - Requests UTXOs from the address dedicated to account 0 address 0
/// - Builds and balances example transaction using UTXOs from account 0 address 0: sensor readings added to Datum
/// - Partially streams Tx to device (only as an example of streaming)
fn build_and_stream_tx(
    mnemonics: String,
    password: &String,
    derivation_path: &DerivationPath,
    script_address: &Address,
    node_client: &node_client::CliNodeClient,
    network: Network,
    device_port: &String,
) {
    println!("Initializing device");
    let mut device = Device::new(device_port);
    device.init(mnemonics);

    println!("Building transaction");
    let temp_data = device.query_mock_sensor_data(password, derivation_path);
    let pub_key = device.get_public_key(password, derivation_path);
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
    let unsigned_tx = tx_build::make_unsigned_tx(
        &device_wallet_address,
        &script_address,
        temp_data,
        &inputs,
        ins_total_value, //for balancing
    );

    println!("Streaming unsigned transaction to the device - 1");
    let stream_result = device.stream_tx(&unsigned_tx, password, derivation_path);
    match stream_result {
        Ok(signature) => {
            println!("Transaction was streamed to device successfully!");
            let sig = Ed25519Signature::from_bytes(signature).unwrap();
            println!("Signature: {:?}", sig);
        }
        Err(msg) => println!("Transaction stream failed: {}", msg),
    }

    println!("Streaming unsigned transaction to the device - 2");
    let stream_result = device.stream_tx(&unsigned_tx, password, derivation_path);
    match stream_result {
        Ok(signature) => {
            println!("Transaction was streamed to device successfully!");
            let sig = Ed25519Signature::from_bytes(signature).unwrap();
            println!("Signature: {:?}", sig);
        }
        Err(msg) => println!("Transaction stream failed: {}", msg),
    }

    println!("Streaming unsigned transaction to the device - 3");
    let stream_result = device.stream_tx(&unsigned_tx, password, derivation_path);
    match stream_result {
        Ok(signature) => {
            println!("Transaction was streamed to device successfully!");
            let sig = Ed25519Signature::from_bytes(signature).unwrap();
            println!("Signature: {:?}", sig);
        }
        Err(msg) => println!("Transaction stream failed: {}", msg),
    }
}

fn translate_network(net: Network) -> u8 {
    match net {
        Network::Mainnet => 1,
        Network::Preprod => 0,
    }
}
