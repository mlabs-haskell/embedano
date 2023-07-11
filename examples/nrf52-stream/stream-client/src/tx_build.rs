use cardano_embedded_sdk::types::XPubKey;
use cardano_serialization_lib::{
    address::Address,
    crypto::{Ed25519Signature, PublicKey, Vkey, Vkeywitness, Vkeywitnesses},
    plutus::{PlutusData, PlutusList},
    utils::{BigInt, BigNum, Coin, Value},
    Transaction, TransactionBody, TransactionInputs, TransactionOutput, TransactionOutputs,
    TransactionWitnessSet,
};

use crate::device::DeviceData;

// some constants for balancing
const FEE: u64 = 200000;
const MIN_ADA: u64 = 2_000_000;

pub fn make_unsigned_tx(
    from_address: &Address,
    to_address: &Address,
    device_data: DeviceData,
    inputs: &TransactionInputs,
    ins_value: u64,
) -> Transaction {
    let mut receiver = TransactionOutput::new(to_address, &lovelace(MIN_ADA));

    let mut to_send_data = PlutusList::new();
    // adding temperature data
    let sensor_data = format!("\"{}\"", device_data.sensor_readings);
    let sensor_data = &BigInt::from_json(sensor_data.as_str()).unwrap();
    to_send_data.add(&PlutusData::new_integer(sensor_data));
    // adding time data
    let time_data = format!("\"{}\"", device_data.measure_time);
    let time_data = &BigInt::from_json(time_data.as_str()).unwrap();
    to_send_data.add(&PlutusData::new_integer(time_data));
    // adding signed bytes
    let signed_data = PlutusData::new_bytes(device_data.signed_readings);
    to_send_data.add(&signed_data);

    receiver.set_plutus_data(&PlutusData::new_list(&to_send_data));

    let change = TransactionOutput::new(&from_address, &lovelace(ins_value - MIN_ADA - FEE));

    let mut outputs = TransactionOutputs::new();
    outputs.add(&receiver);
    outputs.add(&change);

    let tx_fee: Coin = coin(FEE);

    let tx_body = TransactionBody::new_tx_body(&inputs, &outputs, &tx_fee);
    Transaction::new(&tx_body, &TransactionWitnessSet::new(), None)
}

pub fn _make_signed_tx(
    unsigned_tx: &Transaction,
    signer_pub_key: &XPubKey,
    signature: Vec<u8>,
) -> Transaction {
    let p_key = PublicKey::from_hex(&signer_pub_key.raw_key_hex()).unwrap();
    let v_key = Vkey::new(&p_key);
    let sig = Ed25519Signature::from_bytes(signature).unwrap();
    let witness = Vkeywitness::new(&v_key, &sig);

    let mut keys_ws = Vkeywitnesses::new();
    keys_ws.add(&witness);
    let mut wit_set = TransactionWitnessSet::new();
    wit_set.set_vkeys(&keys_ws);
    Transaction::new(&unsigned_tx.body(), &wit_set, None)
}

fn coin(amt: u64) -> Coin {
    BigNum::from_str(&amt.to_string()).unwrap()
}

fn lovelace(amt: u64) -> Value {
    Value::new(&coin(amt))
}
