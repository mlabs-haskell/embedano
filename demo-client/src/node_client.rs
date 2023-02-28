use std::{collections::HashMap, fmt::Display, process::Command};

use cardano_serialization_lib::{
    address::Address, crypto::TransactionHash, Transaction, TransactionInput, TransactionInputs,
};
use serde_json::Value;

#[derive(Debug)]
pub struct NodeClientError {
    message: String,
}

use crate::tx_tools;

pub trait NodeClient {
    fn query_utxos(&self, address: &Address) -> Result<(TransactionInputs, u64), NodeClientError>;

    fn submit_tx(&self, tx: &Transaction) -> Result<String, NodeClientError>;
}

pub struct CliNodeClient {
    socket_path: String,
    network: String, //todo: will need better design
}

impl CliNodeClient {
    pub fn new(socket_path: String, network: String) -> Self {
        Self {
            socket_path,
            network,
        }
    }
}

impl NodeClient for CliNodeClient {
    fn query_utxos(&self, address: &Address) -> Result<(TransactionInputs, u64), NodeClientError> {
        // run cardano-cli to get utxos
        let addr = address.to_bech32(None).map_err(to_err)?;
        let result = Command::new("cardano-cli")
            .env("CARDANO_NODE_SOCKET_PATH", &self.socket_path)
            .args([
                "query",
                "utxo",
                self.network.as_str(),
                "--address",
                addr.as_str(),
                "--out-file=/dev/stdout",
            ])
            .output()
            .map_err(to_err)?;

        //todo: throw error if stderr not empty
        // parse cardano-cli response to inputs
        let inputs = String::from_utf8_lossy(&result.stdout);
        // print!("{:?}", result);
        let inputs: HashMap<String, Value> = serde_json::from_str(&inputs).map_err(to_err)?;

        let total_inputs_value = get_total_value(&inputs);

        let mut tx_inputs = TransactionInputs::new();
        for key in inputs.keys() {
            let res = key.split_once("#").unwrap(); //todo: error handling
            let res = TransactionInput::new(
                &TransactionHash::from_hex(res.0).unwrap(), //todo: error handling
                res.1.parse::<u32>().unwrap(),              //todo: error handling
            );
            tx_inputs.add(&res)
        }
        Ok((tx_inputs,total_inputs_value))
    }

    fn submit_tx(&self, tx: &Transaction) -> Result<String, NodeClientError> {
        let tx_file = "./demo-client/to_submit.tx";
        tx_tools::write_as_envelope(tx_file, tx);

        let result = Command::new("cardano-cli")
            .env("CARDANO_NODE_SOCKET_PATH", &self.socket_path)
            .args([
                "transaction",
                "submit",
                self.network.as_str(),
                "--tx-file",
                tx_file,
            ])
            .output()
            .map_err(to_err)?;
        println!("CMD SUBM res: {:?}", result); //todo: throw error if stderr not empty
        let result = String::from_utf8_lossy(&result.stdout);
        Ok(result.to_string())
    }
}

fn to_err<T: Display>(e: T) -> NodeClientError {
    NodeClientError {
        message: e.to_string(),
    }
}

fn get_total_value(inputs: &HashMap<String, Value>) -> u64 {
    let mut total_lovelace = 0;
    for v in inputs.values() {
        let input_lovelace = v["value"]["lovelace"].as_u64().expect("Should be number");
        total_lovelace = total_lovelace + input_lovelace;
        // print!("VAL {:?}", );
    }
    total_lovelace
}
