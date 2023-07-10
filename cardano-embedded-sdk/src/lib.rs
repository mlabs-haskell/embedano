#![feature(error_in_core)]
#![feature(test)]
#![no_std]

#[macro_use]
extern crate cfg_if;

#[cfg(test)]
extern crate test;

#[cfg(test)]
extern crate quickcheck;
#[cfg(test)]
#[macro_use(quickcheck)]
extern crate quickcheck_macros;

extern crate alloc;
extern crate hex;

pub mod api;
pub mod types;

pub mod bip;
pub mod chain_crypto;
pub mod crypto;
pub mod error;
pub mod impl_mockchain;
pub mod typed_bytes;

pub mod util;
pub mod tx_stream;

#[macro_use]
mod serialization_macros;

use error::*;
