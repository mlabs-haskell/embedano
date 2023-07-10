use alloc::{string::String, vec::Vec};
use minicbor::{Decode, Encode};

#[derive(Clone, Debug, Encode, Decode)]
pub enum TxEntry {
    #[n(0)]
    TxInput(#[n(0)] Vec<u8>, #[n(1)] u32),
    #[n(1)]
    Fee(#[n(0)] u64),
}

#[derive(Clone, Debug, Encode, Decode)]
pub enum TxStream {
    #[n(0)]
    Entry(#[n(0)] TxEntry),
    #[n(1)]
    Done(#[n(0)] Vec<u8>, #[n(1)] String), // password and key path
}
