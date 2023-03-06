#![no_std]

extern crate alloc;
use alloc::{format, string::String, vec::Vec};
use core::prelude::rust_2021::derive;

use cardano_embedded_sdk::api as embedano;
use cardano_embedded_sdk::bip::bip39::Entropy;
use cardano_embedded_sdk::crypto::Ed25519Signature;
use cardano_embedded_sdk::types::TxId;
use derivation_path::DerivationPath;

use minicbor::{Decode, Encode};

pub enum Data {
    Head(Vec<u8>),
    Body(Vec<u8>, usize),
}

pub enum State {
    Read(Data),
    Write(Data),
    Exec(In),
}

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
}

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
}

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
