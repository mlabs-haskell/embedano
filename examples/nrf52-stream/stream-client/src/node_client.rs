use std::{collections::HashMap, fmt::Display, process::Command};

use cardano_embedded_sdk::types::TxId;
use cardano_serialization_lib::{
    address::Address, crypto::TransactionHash, Transaction, TransactionInput, TransactionInputs,
};
use clap::ValueEnum;
use serde_json::Value;

#[derive(Debug)]
pub struct NodeClientError {
    message: String,
}

use crate::tx_envelope::{self, write_as_envelope};

pub trait NodeClient {
    fn query_raw_inputs(&self, address: &Address) -> Result<String, NodeClientError>;
    fn query_inputs(&self, address: &Address) -> Result<(TransactionInputs, u64), NodeClientError>;

    fn submit_tx(&self, tx: &Transaction) -> Result<String, NodeClientError>;

    fn get_tx_id(&self, tx: &Transaction) -> TxId;
}

#[derive(ValueEnum, Debug, Clone, Copy)]
pub enum Network {
    Mainnet,
    Preprod,
}

pub struct CliNodeClient {
    socket_path: String,
    network: Network,
}

impl CliNodeClient {
    pub fn new(socket_path: String, network: Network) -> Self {
        Self {
            socket_path,
            network,
        }
    }
}

impl NodeClient for CliNodeClient {
    fn query_raw_inputs(&self, address: &Address) -> Result<String, NodeClientError> {
        // run cardano-cli to get utxos
        let addr = address.to_bech32(None).map_err(to_err)?;

        let args: Vec<&str> = vec![
            "query",
            "utxo",
            "--address",
            addr.as_str(),
            "--out-file=/dev/stdout",
            translate_network(self.network),
        ];

        //grabs utxos as JSON (as cardano-cli writes them as JSON with --out-file)
        let result = Command::new("cardano-cli")
            .env("CARDANO_NODE_SOCKET_PATH", &self.socket_path)
            .args(args)
            .output()
            .map_err(to_err)?;

        //todo: throw error if stderr not empty
        // parse cardano-cli response to inputs
        let inputs = String::from_utf8_lossy(&result.stdout).into_owned();
        Ok(inputs)
    }

    fn query_inputs(&self, address: &Address) -> Result<(TransactionInputs, u64), NodeClientError> {
        // run cardano-cli to get utxos

        let inputs = self.query_raw_inputs(address)?;
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
        Ok((tx_inputs, total_inputs_value))
    }

    fn submit_tx(&self, tx: &Transaction) -> Result<String, NodeClientError> {
        let tx_file = "./to_submit.tx";
        tx_envelope::write_as_envelope(tx_file, tx);
        let args: Vec<&str> = vec![
            "transaction",
            "submit",
            "--tx-file",
            tx_file,
            translate_network(self.network),
        ];
        let result = Command::new("cardano-cli")
            .env("CARDANO_NODE_SOCKET_PATH", &self.socket_path)
            .args(args)
            .output()
            .map_err(to_err)?; //todo: throw error if stderr not empty
                               // panic!("RESULT: {:?}", result);
        let result = String::from_utf8_lossy(&result.stdout);
        Ok(result.to_string())
    }

    fn get_tx_id(&self, tx: &Transaction) -> TxId {
        let tmp_tx_path = "./empty_wit_for_id.tx";
        write_as_envelope(tmp_tx_path, tx);

        let result = Command::new("cardano-cli")
            .args(["transaction", "txid", "--tx-file", tmp_tx_path])
            .output()
            .expect("Should get Tx id with cardano-cli");
        let result = String::from_utf8_lossy(&result.stdout);
        let result = result.strip_suffix("\n").unwrap();
        TxId::from_hex(result).expect("Should parse Tx id from hex")
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
    }
    total_lovelace
}

fn translate_network(net: Network) -> &'static str {
    match net {
        Network::Mainnet => "--mainnet",
        Network::Preprod => "--testnet-magic=1",
    }
}
