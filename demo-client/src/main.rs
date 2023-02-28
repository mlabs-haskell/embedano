use cardano_serialization_lib::crypto::{
    Ed25519Signature, PublicKey, Vkey, Vkeywitness, Vkeywitnesses,
};
use cardano_serialization_lib::plutus::{PlutusData, PlutusList};
use cardano_serialization_lib::utils::BigInt;
use cardano_serialization_lib::{
    address::Address,
    crypto::{Ed25519KeyHash},
    utils::{BigNum, Coin, Value},
    NetworkId, RequiredSigners, Transaction, TransactionBody,
    TransactionOutput, TransactionOutputs, TransactionWitnessSet,
};


use node_client::NodeClient;

mod device_dummy;
mod node_client;
mod tx_envelope;

fn main() {
    let minAda = 2_000_000;
    let fee = 150000;
    let user_mnemonics = "all all all all all all all all all all all all";

    //slip-14 address
    let user_wallet_address =
        Address::from_bech32("addr1vxq0nckg3ekgzuqg7w5p9mvgnd9ym28qh5grlph8xd2z92su77c6m")
            .expect("Should parse user wallet address");

    // mainnet address of always succeeds script
    let script_address =
        &Address::from_bech32("addr1w9nlxv2xv9a9ucvnvzqakwepzl9ltx7jzgm53av2e9ncv4slcd85z")
            .expect("Should parse script address");

    let node_client = node_client::CliNodeClient::new(
        "/home/mike/dev/mlabs/embedano-project/plutip-made-keys/pool-1/node.socket".to_string(),
        "--mainnet".to_string(),
    );

    let acc_0_pub_key = device_dummy::get_addr_0_pub_key(user_mnemonics);

    let (inputs, ins_value) = node_client
        .query_utxos(&user_wallet_address)
        .expect("Should return inputs from user address");
    print!("{:?}", inputs);

    //setting outputs
    let (sensor_data, signed_bytes) = device_dummy::get_signed_sensor_data(user_mnemonics);
    let mut receiver = TransactionOutput::new(script_address, &lovalace(minAda));

    let mut device_data = PlutusList::new();
    // adding raw data
    let sensor_data = format!("\"{}\"", sensor_data);
    let sensor_data = &BigInt::from_json(sensor_data.as_str()).unwrap();
    device_data.add(&PlutusData::new_integer(sensor_data));
    // adding signed bytes
    let signed_data = PlutusData::new_bytes(signed_bytes);
    device_data.add(&signed_data);


    receiver.set_plutus_data(&PlutusData::new_list(&device_data));

    let change = TransactionOutput::new(&user_wallet_address, &lovalace(ins_value - minAda - fee));

    let mut outputs = TransactionOutputs::new();
    outputs.add(&receiver);
    outputs.add(&change);

    let fee: Coin = coin(fee);

    //making body
    let mut tx_body = TransactionBody::new_tx_body(&inputs, &outputs, &fee);
    let mut required_signers = RequiredSigners::new();
    required_signers.add(
        &Ed25519KeyHash::from_hex(acc_0_pub_key.hash_hex().as_str())
            .expect("Should be able to parse public key hash from hex"),
    );
    tx_body.set_required_signers(&required_signers);
    tx_body.set_network_id(&NetworkId::mainnet());

    //building transaction

    // building transaction: unsigned
    let unsigned_tx = Transaction::new(&tx_body, &TransactionWitnessSet::new(), None);

    let tx_id = node_client.get_tx_id(&unsigned_tx);

    // building transaction: SIGNED
    let (pub_key, signature) = device_dummy::sign_with_address_0(&tx_id, user_mnemonics);
    let p_key = PublicKey::from_hex(&pub_key.raw_key_hex()).unwrap();
    let v_key = Vkey::new(&p_key);
    let sig = Ed25519Signature::from_bytes(signature.to_bytes()).unwrap();
    let witness = Vkeywitness::new(&v_key, &sig);

    let mut keys_ws = Vkeywitnesses::new();
    keys_ws.add(&witness);
    let mut wit_set = TransactionWitnessSet::new();
    wit_set.set_vkeys(&keys_ws);
    let signed_tx = Transaction::new(&tx_body, &wit_set, None);

    let submit_result = node_client.submit_tx(&signed_tx);
    println!("Submission result: {:?}", submit_result)
}

fn coin(amt: u64) -> Coin {
    BigNum::from_str(&amt.to_string()).unwrap()
}

fn lovalace(amt: u64) -> Value {
    Value::new(&coin(amt))
}
