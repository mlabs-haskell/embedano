use cardano_serialization_lib::crypto::{
    Ed25519Signature, PublicKey, Vkey, Vkeywitness, Vkeywitnesses,
};
use cardano_serialization_lib::plutus::{PlutusData, PlutusList};
use cardano_serialization_lib::utils::BigInt;
use cardano_serialization_lib::{
    address::Address,
    crypto::Ed25519KeyHash,
    utils::{BigNum, Coin, Value},
    NetworkId, RequiredSigners, Transaction, TransactionBody, TransactionOutput,
    TransactionOutputs, TransactionWitnessSet,
};

use node_client::NodeClient;

mod device_dummy;
mod node_client;
mod tx_build;
mod tx_envelope;

fn main() {
    let user_mnemonics = "all all all all all all all all all all all all";
    // address of account 0 address 0, should be aligned wiht mnemonics
    // ideally, we could build address from PubKey derived from mnemonics and network_id
    let user_wallet_address = "addr1vxq0nckg3ekgzuqg7w5p9mvgnd9ym28qh5grlph8xd2z92su77c6m";
    // address of script that will hold sensor readings
    let script_address = "addr1w9nlxv2xv9a9ucvnvzqakwepzl9ltx7jzgm53av2e9ncv4slcd85z";

    let network_id = "--mainnet";
    let node_socket = "/home/mike/dev/mlabs/embedano-project/plutip-made-keys/pool-1/node.socket";

    //slip-14 address
    let user_wallet_address =
        Address::from_bech32(user_wallet_address).expect("Should parse user wallet address");

    // mainnet address of always succeeds script
    let script_address =
        &Address::from_bech32(script_address).expect("Should parse script address");

    let node_client =
        node_client::CliNodeClient::new(node_socket.to_string(), network_id.to_string());

    let acc_0_pub_key = device_dummy::get_addr_0_pub_key(user_mnemonics);

    let (inputs, ins_value) = node_client
        .query_inputs(&user_wallet_address)
        .expect("Should return inputs from user address");
    print!("{:?}", inputs);

    //setting outputs
    let device_data = device_dummy::get_signed_sensor_data(user_mnemonics);

    let unsigned_tx = tx_build::make_unsigned_tx(
        &user_wallet_address,
        &script_address,
        device_data,
        &inputs,
        ins_value,
        &NetworkId::mainnet(), //todo: detect network id
        &acc_0_pub_key,
    );

    let tx_id = node_client.get_tx_id(&unsigned_tx);

    // building transaction: SIGNED
    let signature = device_dummy::sign_with_address_0(&tx_id, user_mnemonics);
    let signed_tx = tx_build::make_signed_tx(&unsigned_tx, &acc_0_pub_key, signature.to_bytes());

    let submit_result = node_client.submit_tx(&signed_tx);
    println!("Submission result: {:?}", submit_result)
}
