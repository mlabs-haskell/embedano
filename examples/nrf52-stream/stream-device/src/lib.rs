#![no_std]

extern crate alloc;
use alloc::fmt::format;
use alloc::{format, string::String, vec::Vec};
use core::prelude::rust_2021::derive;
use cortex_m_semihosting::hprintln;
use mock_hasher::MockHahser;

use cardano_embedded_sdk::bip::bip39::Entropy;
use cardano_embedded_sdk::crypto::Ed25519Signature;
use cardano_embedded_sdk::types::{TxId, TxIdParseError};
use cardano_embedded_sdk::{api as embedano, tx_stream, types::TransactionInput};
use derivation_path::DerivationPath;

use minicbor::{Decode, Encode};
pub mod mock_hasher;
use nrf52840_hal::gpio::{Input, Pin, PullUp};

use panic_halt as _;

use embedded_hal::digital::v2::InputPin;

pub enum Data {
    Head(Vec<u8>),
    Body(Vec<u8>, usize),
}

pub enum State {
    Read(Data),
    Write(Data),
    Exec(In),
}

/// Incoming messages that device receives from host.
/// Serialized to CBOR.
#[derive(Clone, Debug, Encode, Decode)]
pub enum In {
    #[n(0)]
    Init(#[n(0)] String),
    #[n(1)]
    Sign(#[n(0)] Vec<u8>, #[n(1)] Vec<u8>, #[n(2)] String),
    #[n(2)]
    Verify(
        #[n(0)] Vec<u8>,
        #[n(1)] Vec<u8>,
        #[n(2)] Vec<u8>,
        #[n(3)] String,
    ),
    #[n(3)]
    Temp(#[n(0)] Vec<u8>, #[n(1)] u64, #[n(2)] String),
    #[n(4)]
    PubKey(#[n(0)] Vec<u8>, #[n(1)] String),
    #[n(5)]
    Stream(#[n(0)] tx_stream::TxStream),
}

/// Outgoing messages that device sends to host.
/// Serialized to CBOR.
#[derive(Clone, Debug, Encode, Decode)]
pub enum Out {
    #[n(0)]
    Init,
    #[n(1)]
    Sign(#[n(0)] Vec<u8>),
    #[n(2)]
    Verify(#[n(0)] bool),
    #[n(3)]
    Error(#[n(0)] String),
    #[n(4)]
    Length(#[n(0)] u64),
    #[n(5)]
    Read(#[n(0)] u64),
    #[n(6)]
    Temp(#[n(0)] i32, #[n(1)] Vec<u8>),
    #[n(7)]
    PubKey(#[n(0)] String),
    #[n(8)]
    StreamResponse(#[n(0)] String),
}

/// Helper function to perform signing on the device
pub fn sign(tx_id: &[u8], entropy: &Entropy, password: &[u8], path: &str) -> Out {
    match (TxId::from_bytes(tx_id), path.parse::<DerivationPath>()) {
        (Ok(tx_id), Ok(path)) => {
            let signature = embedano::sign_tx_id(&tx_id, entropy, password, &path);
            Out::Sign(signature.to_bytes())
        }
        (Err(e), _) => Out::Error(format!("Decode tx_id failed: {e:?}")),
        (_, Err(e)) => Out::Error(format!("Decode path failed: {e}")),
    }
}

/// Helper function to obtain public key on the device
pub fn get_pub_key(entropy: &Entropy, password: &[u8], path: &str) -> Out {
    match path.parse::<DerivationPath>() {
        Ok(path) => {
            let (_, pub_key) = embedano::derive_key_pair(entropy, password, &path);
            Out::PubKey(pub_key.to_hex())
        }
        Err(e) => Out::Error(format!("Decode path failed: {e}")),
    }
}

/// Helper function to verify transaction ID on he device
pub fn verify(
    tx_id: &[u8],
    signature: Vec<u8>,
    entropy: &Entropy,
    password: &[u8],
    path: &str,
) -> Out {
    match (
        TxId::from_bytes(tx_id),
        Ed25519Signature::from_bytes(signature),
        path.parse::<DerivationPath>(),
    ) {
        (Ok(tx_id), Ok(signature), Ok(path)) => {
            let (_prv_key, pub_key) = embedano::derive_key_pair(&entropy, password, &path);
            Out::Verify(pub_key.verify(tx_id.to_bytes(), &signature))
        }
        (Err(e), _, _) => Out::Error(format!("Decode tx_id failed: {e:?}")),
        (_, Err(e), _) => Out::Error(format!("Decode signature failed: {e}")),
        (_, _, Err(e)) => Out::Error(format!("Decode path failed: {e}")),
    }
}

pub fn process_stream_item(
    stream: tx_stream::TxStream,
    hasher: &mut MockHahser,
    entropy: &Entropy,
    confirm_button: &Pin<Input<PullUp>>,
    reject_button: &Pin<Input<PullUp>>,
) -> Out {
    match stream {
        tx_stream::TxStream::Entry(tx_entry) => {
            let out = process_tx_entry(&tx_entry, &confirm_button, &reject_button);
            if let Out::StreamResponse(_) = out {
                // If entry was confirmed, add it to rolling hash
                hasher.add_entry(&tx_entry)
            } else {
                // If entry was rejected, reset rolling hash
                hasher.reset()
            }
            out
        }
        tx_stream::TxStream::Done(password, path) => {
            let tx_id = hasher.final_tx_id();
            let tx_id_hex = hex::encode(tx_id.clone());
            prompt_confirmation(format!("Confirm and sign transaction id:\n{}", tx_id_hex));
            match wait_confirmation("TxId".into(), confirm_button, reject_button) {
                err @ Out::Error(_) => err,
                Out::StreamResponse(s) if s == "TxId confirmed" => {
                    hprintln!("Firmware: signing transaction id {}", tx_id_hex);
                    sign(&tx_id, entropy, &password, &path)
                }
                other => Out::Error(format!(
                    "Unexpected Output for TxId confirmation: {:?}",
                    other
                )),
            }
            // hprintln!("Final tx id: {}", res);
            // Out::StreamResponse(format!("Tx id: {}", res))
        }
    }
}

fn process_tx_entry(
    entry: &tx_stream::TxEntry,
    confirm_button: &Pin<Input<PullUp>>,
    reject_button: &Pin<Input<PullUp>>,
) -> Out {
    match entry {
        tx_stream::TxEntry::TxInput(hash, index) => match parse_input(hash, index) {
            Ok(tx_in) => {
                prompt_confirmation(format!(
                    "Confirm Tx input {}",
                    tx_in.transaction_id.to_hex()
                ));
                wait_confirmation("TxIn".into(), confirm_button, reject_button)
            }
            Err(err) => Out::Error(format!("Failed to parse streamed input: {:?}", err)),
        },
        tx_stream::TxEntry::Fee(fee) => {
            prompt_confirmation(format!("Confirm fee {}", fee));
            wait_confirmation("Fee".into(), confirm_button, reject_button)
        }
    }
}

fn prompt_confirmation(confirmation_message: String) {
    hprintln!(
        "\n----- Display -----------------------
        \n {}
        \nConfirm - button-1 | Reject - button-2
        \n--------------------------------------",
        confirmation_message
    );
}

fn wait_confirmation(
    of_what: String,
    confirm_button: &Pin<Input<PullUp>>,
    reject_button: &Pin<Input<PullUp>>,
) -> Out {
    let mut confirm_out;
    loop {
        if confirm_button.is_low().unwrap() {
            confirm_out = Out::StreamResponse(format!("{} confirmed", of_what));
            hprintln!("Firmware: Confirmed");
            break;
        }
        if reject_button.is_low().unwrap() {
            confirm_out = Out::Error(format!("{} rejected by the user", of_what));
            hprintln!("Firmware: Rejected");
            break;
        }
    }
    confirm_out
}

pub fn parse_input(hash: &Vec<u8>, index: &u32) -> Result<TransactionInput, TxIdParseError> {
    let tx_hash = TxId::from_bytes(&hash[..])?;
    Ok(TransactionInput {
        transaction_id: tx_hash,
        index: *index,
    })
}
