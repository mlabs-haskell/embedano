#![no_std]
#![feature(error_in_core)]

extern crate alloc;

extern crate hex;

pub mod chain_crypto;
pub mod crypto;
pub mod error;
pub mod impl_mockchain;
pub mod typed_bytes;

#[macro_use]
mod serialization_macros;

use error::*;
