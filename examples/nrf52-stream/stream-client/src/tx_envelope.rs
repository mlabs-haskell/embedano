use std::fs::File;
use std::io::Write;

use cardano_serialization_lib::Transaction;
use serde::{Deserialize, Serialize};

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
