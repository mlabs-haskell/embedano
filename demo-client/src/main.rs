use cardano_embedded_sdk::api as embedano;
use cardano_embedded_sdk::bip::bip39::{dictionary, Entropy, Mnemonics};
use cardano_embedded_sdk::crypto as sdk_crypto;
use cardano_embedded_sdk::types::{TxId, XPrvKey, XPubKey};

use cardano_serialization_lib::crypto::{
    Ed25519Signature, PublicKey, Vkey, Vkeywitness, Vkeywitnesses,
};
use cardano_serialization_lib::{
    address::Address,
    crypto::{Ed25519KeyHash, TransactionHash},
    utils::{BigNum, Coin, Value},
    NetworkId, RequiredSigners, Transaction, TransactionBody, TransactionInput, TransactionInputs,
    TransactionOutput, TransactionOutputs, TransactionWitnessSet,
};

use derivation_path::DerivationPath;
use serde::{Deserialize, Serialize};
use serde_json;
use std::fs::File;
use std::io::Write;

fn main() {
    let input_hash = "fb03abe73ddca76bc2f4a4fd18fde3b8e7844d7d1e3049042b4ed0875e7a6e04";
    let input_hash = TransactionHash::from_hex(input_hash).unwrap();
    let input_ix = 1;

    // setting inputs
    let mut inputs = TransactionInputs::new();
    inputs.add(&TransactionInput::new(&input_hash, input_ix));
    println!("Inputs:\n{:?}", inputs);

    //setting outputs
    let recevier = TransactionOutput::new(
        &Address::from_bech32("addr1vx4aur6jt8h6etqgez9a3j23a2khk9wcnz32fqhshgah79swzdsp9")
            .unwrap(),
        &lovalace(111000000),
    );

    let change = TransactionOutput::new(
        &Address::from_bech32("addr1vxq0nckg3ekgzuqg7w5p9mvgnd9ym28qh5grlph8xd2z92su77c6m")
            .unwrap(),
        &lovalace(888853600),
    );

    let mut outputs = TransactionOutputs::new();
    outputs.add(&recevier);
    outputs.add(&change);

    let fee: Coin = coin(146400);

    //making body
    let mut tx_body = TransactionBody::new_tx_body(&inputs, &outputs, &fee);
    let mut required_signers = RequiredSigners::new();
    required_signers.add(
        &Ed25519KeyHash::from_hex("80f9e2c88e6c817008f3a812ed889b4a4da8e0bd103f86e7335422aa")
            .unwrap(),
    );
    tx_body.set_required_signers(&required_signers);
    tx_body.set_network_id(&NetworkId::mainnet());

    //building transaction
    let tx_id =
        TxId::from_hex("1c1823f5c839bda816f2b887fa5d87b20194988ad5852146ceb814dceada96a6").unwrap();
    let (pub_key, signature) = sign_with_slip14(&tx_id);

    // building transaction: unsigned
    let tx = Transaction::new(&tx_body, &TransactionWitnessSet::new(), None);
    println!("{}", tx.to_json().unwrap());
    let envelope = Envelope::from_tx(&tx);
    write_envelope("./demo-client/demo_empty_wit.tx", &envelope);

    let p_key = PublicKey::from_hex(&pub_key.raw_key_hex()).unwrap();
    let v_key = Vkey::new(&p_key);
    let sig = Ed25519Signature::from_bytes(signature.to_bytes()).unwrap();
    let witness = Vkeywitness::new(&v_key, &sig);

    let mut keys_ws = Vkeywitnesses::new();
    keys_ws.add(&witness);
    let mut wit_set = TransactionWitnessSet::new();
    wit_set.set_vkeys(&keys_ws);
    let tx = Transaction::new(&tx_body, &wit_set, None);
    println!("{}", tx.to_json().unwrap());
    let envelope = Envelope::from_tx(&tx);
    write_envelope("./demo-client/demo.tx", &envelope);
}

fn coin(amt: u32) -> Coin {
    BigNum::from_str(&amt.to_string()).unwrap()
}

fn lovalace(amt: u32) -> Value {
    Value::new(&coin(amt))
}

fn sign_with_slip14(tx_id: &TxId) -> (XPubKey, sdk_crypto::Ed25519Signature) {
    let mnemonics = Mnemonics::from_string(
        &dictionary::ENGLISH,
        "all all all all all all all all all all all all",
    )
    .unwrap();
    let entropy = Entropy::from_mnemonics(&mnemonics).unwrap();
    let password = b"";
    let path: DerivationPath = "m/1852'/1815'/0'/0/0".parse().unwrap();
    let (_, pub_key) = embedano::derive_key_pair(&entropy, password, &path);
    let signature = embedano::sign_tx_id(tx_id, &entropy, password, &path);
    (pub_key, signature)
}

#[derive(Serialize, Deserialize)]
struct Envelope {
    #[serde(rename = "type")]
    env_type: String,
    description: String,
    #[serde(rename = "cborHex")]
    cbor_hex: String,
}

impl Envelope {
    fn from_tx(tx: &Transaction) -> Envelope {
        Envelope {
            env_type: String::from("Tx BabbageEra"),
            description: String::from(""),
            cbor_hex: tx.to_hex(),
        }
    }
}

fn write_envelope(name: &str, envelope: &Envelope) -> () {
    let json = serde_json::to_string(&envelope).expect("Unable to serialise envelope");
    let mut f = File::create(name).expect("Unable to create file {name}");
    f.write_all(json.as_bytes()).expect("Unable to write data");
}
