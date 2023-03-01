use std::{thread, time};

use cardano_serialization_lib::{address::Address, NetworkId};

use derivation_path::DerivationPath;
use device_dummy::DeviceDummy;
use node_client::NodeClient;

mod device_dummy;
mod node_client;
mod tx_build;
mod tx_envelope;

fn main() {
    let user_mnemonics = "all all all all all all all all all all all all";
    let password = "";
    // address of key for account 0 address 0, should be aligned with mnemonics
    // ideally, we could build address from PubKey derived from mnemonics and network_id
    let user_wallet_address = "addr1vxq0nckg3ekgzuqg7w5p9mvgnd9ym28qh5grlph8xd2z92su77c6m";
    // address of script that will hold sensor readings
    let script_address = "addr1w9nlxv2xv9a9ucvnvzqakwepzl9ltx7jzgm53av2e9ncv4slcd85z";

    let derivation_path = "m/1852'/1815'/0'/0/0";
    let network_id = "--mainnet";
    let node_socket = "/home/mike/dev/mlabs/embedano-project/plutip-made-keys/pool-1/node.socket";

    let derivation_path: DerivationPath = derivation_path
        .parse()
        .expect("Should parse derivation path");
    //slip-14 address
    let user_wallet_address =
        Address::from_bech32(user_wallet_address).expect("Should parse user wallet address");

    // mainnet address of always succeeds script
    let script_address =
        &Address::from_bech32(script_address).expect("Should parse script address");

    let node_client =
        node_client::CliNodeClient::new(node_socket.to_string(), network_id.to_string());

    let device = device_dummy::DeviceDummy::init(user_mnemonics);

    for _ in 0..5 {
        submit_data_to_blockchain(
            &node_client,
            &device,
            &user_wallet_address,
            &script_address,
            password,
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
        ins_total_value,       //for balancing
        &NetworkId::mainnet(), //todo: detect network id
        &pub_key,
    );

    let tx_id = node_client.get_tx_id(&unsigned_tx);

    let signature = device.sign_tx_id(&tx_id, password, derivation_path);

    let signed_tx = tx_build::make_signed_tx(&unsigned_tx, &pub_key, signature.to_bytes());

    let submit_result = node_client.submit_tx(&signed_tx);
    println!("Submission result: {:?}", submit_result)
}
