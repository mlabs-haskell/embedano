use std::{thread, time};

use cardano_serialization_lib::{address::Address, NetworkId};

use clap::{command, Parser};
use derivation_path::DerivationPath;
use device_dummy::DeviceDummy;
use node_client::NodeClient;

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
    #[arg(long)]
    /// Address that corresponds to mnemonics (used to provide inputs)
    wallet_address: String,
    /// address of script that will store sensor data
    #[arg(long)]
    script_address: String,
    #[arg(long)]
    /// Derivation path for keys (should correspond to wallet_address atm)
    derivation_path: String,
    /// Network id (0 for mainnet)
    #[arg(long)]
    network_id: u32,
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
    //slip-14 address
    let user_wallet_address = Address::from_bech32(args.wallet_address.as_str())
        .expect("Should parse user wallet address");

    // mainnet address of always succeeds script
    let script_address =
        &Address::from_bech32(args.script_address.as_str()).expect("Should parse script address");

    let node_client = node_client::CliNodeClient::new(args.node_socket, args.network_id);

    let device = device_dummy::DeviceDummy::init(args.mnemonics.as_str());

    for _ in 0..5 {
        submit_data_to_blockchain(
            &node_client,
            &device,
            &user_wallet_address,
            &script_address,
            args.password.as_str(),
            &derivation_path,
        );
        thread::sleep(time::Duration::from_secs(2))
    }
}

fn submit_data_to_blockchain(
    node_client: &impl NodeClient,
    device: &DeviceDummy,
    user_wallet_address: &Address,
    script_address: &Address,
    password: &str,
    derivation_path: &DerivationPath,
) {
    let (inputs, ins_total_value) = node_client
        .query_inputs(user_wallet_address)
        .expect("Should return inputs from user address. Is node running and available?");

    let device_data = device.get_signed_sensor_data(password, derivation_path);

    let pub_key = device.get_pub_key(password, derivation_path);

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

    let signature = device.sign_tx_id(&tx_id, password, derivation_path);

    let signed_tx = tx_build::make_signed_tx(&unsigned_tx, &pub_key, signature.to_bytes());

    let submit_result = node_client.submit_tx(&signed_tx);
    println!("Submission result: {:?}", submit_result)
}
