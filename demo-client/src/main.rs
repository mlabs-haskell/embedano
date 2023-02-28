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
use node_client::NodeClient;

mod node_client;
mod tx_tools;


// todo balancing
// todo abstraction for embedded device client
fn main() {

    let minAda = 2_000_000;
    let fee = 150000;

    let node_client = node_client::CliNodeClient::new(
        "/home/mike/dev/mlabs/embedano-project/plutip-made-keys/pool-1/node.socket".to_string(),
        "--mainnet".to_string(),
    );
    let user_address = //slip-14 address
        Address::from_bech32("addr1vxq0nckg3ekgzuqg7w5p9mvgnd9ym28qh5grlph8xd2z92su77c6m").unwrap();
    
    // current local test setup creates wallet with 1 UTXO only with known Value
    // so it safe to use TransactionInputs for now w/o balancing
    // for single run from scratch
    let (inputs, ins_value) = node_client.query_utxos(&user_address).unwrap();
    print!("{:?}", inputs);

    //setting outputs
    let recevier = TransactionOutput::new(
        &Address::from_bech32("addr1vx4aur6jt8h6etqgez9a3j23a2khk9wcnz32fqhshgah79swzdsp9")
            .unwrap(),
        &lovalace(minAda),
    );

    let change = TransactionOutput::new(
        &Address::from_bech32("addr1vxq0nckg3ekgzuqg7w5p9mvgnd9ym28qh5grlph8xd2z92su77c6m")
            .unwrap(),
        &lovalace(ins_value - minAda - fee),
    );

    let mut outputs = TransactionOutputs::new();
    outputs.add(&recevier);
    outputs.add(&change);

    let fee: Coin = coin(fee);

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

    // building transaction: unsigned
    let unsigned_tx = Transaction::new(&tx_body, &TransactionWitnessSet::new(), None);

    let tx_id = tx_tools::get_tx_id(&unsigned_tx);

    // building transaction: SIGNED
    let (pub_key, signature) = sign_with_slip14(&tx_id);
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
