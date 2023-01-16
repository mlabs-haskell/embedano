//! Cardano Basic types and manipulation functions
//!
//! Features:
//!
//! * Address generation and parsing
//! * Block types and parsing
//! * HDWallet (ED25519-BIP32)
//! * BIP39 codec (Including dictionaries: English, Japanese, French, Spanish, Chinese)
//! * BIP44 wallet addressing scheme
//! * Paperwallet V1
//! * Transaction creation, parsing, signing
//! * Fee calculation
//! * Redeem Key
//! * Wallet abstraction
//!
#![no_std]
#![feature(error_in_core)]
#![cfg_attr(feature = "with-bench", feature(test))]

extern crate alloc;

// #[cfg(feature = "generic-serialization")]
#[macro_use]
extern crate serde_derive;
// #[cfg(feature = "generic-serialization")]
extern crate serde;

#[cfg(test)]
extern crate serde_json;
#[cfg(test)]
#[macro_use]
extern crate lazy_static;

#[cfg(test)]
#[cfg(feature = "with-bench")]
extern crate test;

#[cfg(test)]
#[macro_use]
extern crate quickcheck;

#[cfg(test)]
extern crate rand;

extern crate cryptoxide;
#[macro_use]
extern crate cbor_event;

#[cfg(test)]
extern crate base64;

pub mod address;
pub mod coin;
pub mod config;
mod crc32;
pub mod hash;
pub mod hdpayload;
pub mod hdwallet;
pub mod redeem;
pub mod tx;
pub mod txutils;
pub mod util;

pub mod bip;
pub mod wallet;

pub mod merkle;
pub mod tags;
pub mod vss;
