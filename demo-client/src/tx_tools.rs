use std::fs::File;
use std::io::Write;
use std::process::Command;

use cardano_serialization_lib::Transaction;
use serde::{Deserialize, Serialize};

use cardano_embedded_sdk::types::TxId;

// cardano-cli transaction txid --tx-file tx_csl.signed

#[derive(Serialize, Deserialize)]
pub struct Envelope {
    #[serde(rename = "type")]
    env_type: String,
    description: String,
    #[serde(rename = "cborHex")]
    cbor_hex: String,
}

impl Envelope {
    pub fn from_tx(tx: &Transaction) -> Envelope {
        Envelope {
            env_type: String::from("Tx BabbageEra"),
            description: String::from(""),
            cbor_hex: tx.to_hex(),
        }
    }
}

pub fn write_envelope(path: &str, envelope: &Envelope) -> () {
    let json = serde_json::to_string(&envelope).expect("Unable to serialise envelope");
    let mut f = File::create(path).expect("Unable to create file {path}");
    f.write_all(json.as_bytes()).expect("Unable to write data");
}

pub fn write_as_envelope(path: &str, tx: &Transaction) {
    write_envelope(path, &Envelope::from_tx(tx))
}

pub fn get_tx_id(tx: &Transaction) -> TxId {
    let tmp_tx_path = "./demo-client/empty_wit_for_id.tx";
    write_as_envelope(tmp_tx_path, tx);

    let result = Command::new("cardano-cli")
        .args(["transaction", "txid", "--tx-file", tmp_tx_path])
        .output()
        .expect("failed to execute process");
    let result = String::from_utf8_lossy(&result.stdout);
    let result = result.strip_suffix("\n").unwrap();
    TxId::from_hex(result).unwrap()
}