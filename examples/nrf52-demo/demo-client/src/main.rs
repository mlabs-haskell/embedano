use std::collections::HashMap;

use cardano_embedded_sdk::crypto::Ed25519Signature;
use cardano_serialization_lib::{
    address::{Address, EnterpriseAddress, StakeCredential},
    crypto::Ed25519KeyHash,
    plutus::{PlutusData, PlutusDatumSchema},
};

use clap::{command, Parser, ValueEnum};
use derivation_path::DerivationPath;
use node_client::{Network, NodeClient};
use serde_json::{from_str, Value};

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
    mode: Mode,
    #[arg(long)]
    device_port: String,
}

#[derive(ValueEnum, Debug, Clone, Copy)]
pub enum Mode {
    Submit,
    Verify,
}

fn main() {
    let args = Args::parse();

    let mode = args.mode;
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

    match mode {
        Mode::Submit => submit_device_data(
            mnemonics,
            &password,
            &derivation_path,
            script_address,
            &node_client,
            network,
            &device_port,
        ),
        Mode::Verify => verify(
            mnemonics,
            &password,
            &derivation_path,
            script_address,
            &node_client,
            &device_port,
        ),
    }
}

/// Performs following steps:
/// - Initializes device with mnemonic
/// - Requests sensor data from device (temperature)
/// - Requests public key from device for account 0 address 0
/// - Requests UTXOs from the address dedicated to account 0 address 0
/// - Builds and balances transaction using UTXOs from account 0 address 0: sensor readings added to Datum
/// - Signs transaction ID using device
/// - Adds witness with signature to transaction and submits it
fn submit_device_data(
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

    println!("Requesting temperature data");
    let temp_data = device.query_sensor_data(password, derivation_path);
    println!("Received sensor data: {:?}", temp_data.sensor_readings);

    println!("Building transaction");
    println!("Requesting public key from device device");
    // Receive public key from device for given derivation path
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
    println!("Making unsigned Tx");
    let unsigned_tx = tx_build::make_unsigned_tx(
        &device_wallet_address,
        &script_address,
        temp_data,
        &inputs,
        ins_total_value, //for balancing
    );

    let tx_id = node_client.get_tx_id(&unsigned_tx);

    println!(
        "Transaction built and balanced.\n -> transaction ID: {}",
        tx_id.to_hex()
    );

    println!("Signing transaction ID: {}", tx_id.to_hex());
    let tx_signature = device.sign_transaction_id(&tx_id, &password, &derivation_path);

    println!("Adding signature to transaction");
    let signed_tx = tx_build::make_signed_tx(&unsigned_tx, &pub_key, tx_signature);

    println!("Submitting signed transaction");
    let submit_result = node_client.submit_tx(&signed_tx);
    println!("Submission result: {:?}", submit_result)
}

/// Performs check of data posted to chain:
/// - Initializes device with mnemonics
/// - Request public key from device for account 0 address 0
/// - Queries UTXOs with sensor readings from script address
/// - Using public key from device verifies that bytes of temperature and time from datum
///   correspond to signed data from the same datum
///   (i.e. device can derive same private and public keys, that were used to send data to chain)
fn verify(
    mnemonics: String,
    password: &String,
    derivation_path: &DerivationPath,
    script_address: &Address,
    node_client: &node_client::CliNodeClient,
    device_port: &String,
) {
    println!("Initializing device");
    let mut device = Device::new(device_port);
    device.init(mnemonics);

    println!("Requesting public key from device device");
    let pub_key = device.get_public_key(password, derivation_path);
    println!("Public key hash: {}", pub_key.hash_hex());

    let utxo_map = node_client.query_raw_inputs(script_address).unwrap();
    let utxo_map: HashMap<String, Value> =
        serde_json::from_str(&utxo_map).expect("Couldn't parse UTXOs map");

    // Verify data in UTXOs
    for (k, v) in utxo_map.iter() {
        let datum = &v["inlineDatum"].to_string();
        let datum = PlutusData::from_json(datum, PlutusDatumSchema::DetailedSchema).unwrap();
        let datum = datum.as_list().unwrap();

        let temperature = datum.get(0).as_integer().unwrap();
        let temperature = from_str::<i32>(temperature.to_str().as_str()).unwrap();
        println!("\nVerifying data\nUTXO ID: {}", k);
        println!("Temperature: {:?}", temperature);

        let time = datum.get(1).as_integer().unwrap();
        let time = from_str::<u64>(time.to_str().as_str()).unwrap();
        println!("Time: {:?}", time);

        let sig = datum.get(2).as_bytes().unwrap();
        let sig = Ed25519Signature::from_bytes(sig).unwrap();
        println!("Signed data:  {:?}", sig.to_hex());

        let data_to_verify = chain_data_bytes(temperature, time);

        let ver = pub_key.verify(&data_to_verify, &sig);
        println!("Verification passed: {:?}", ver);
    }
}

fn translate_network(net: Network) -> u8 {
    match net {
        Network::Mainnet => 1,
        Network::Preprod => 0,
    }
}

/// Device performs same operation on data before signing it.
/// This function performs same manipulation.
/// See `chain_data_bytes` in `main.rs` of device package.
fn chain_data_bytes(a: i32, b: u64) -> Vec<u8> {
    a.to_be_bytes()
        .into_iter()
        .chain(b.to_be_bytes().into_iter())
        .collect::<Vec<u8>>()
}
