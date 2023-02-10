//! This module built on top of `api.rs` and contains helper functions
//! to generate keys according to SLIP-14
//! for mnemonics "all all all all all all all all all all all all".
//!
//! Used in tests.
use derivation_path::DerivationPath;

use crate::{
    api,
    bip::bip39::{self, Entropy},
    types::{XPrvKey, XPubKey},
};

const SLIP14_MNEMONICS: &str = "all all all all all all all all all all all all";

pub fn make_entropy() -> Entropy {
    let mnemonics =
        bip39::Mnemonics::from_string(&bip39::dictionary::ENGLISH, SLIP14_MNEMONICS).unwrap();
    bip39::Entropy::from_mnemonics(&mnemonics).unwrap()
}

pub fn make_root_key() -> XPrvKey {
    let mnemonics =
        bip39::Mnemonics::from_string(&bip39::dictionary::ENGLISH, SLIP14_MNEMONICS).unwrap();
    let entropy = bip39::Entropy::from_mnemonics(&mnemonics).unwrap();

    XPrvKey::from_entropy(&entropy, b"")
}

pub fn make_address_keys() -> (XPrvKey, XPubKey) {
    let path: DerivationPath = "m/1852'/1815'/0'/0/0".parse().unwrap();
    make_keys_for(&path)
}

pub fn make_keys_for(path: &DerivationPath) -> (XPrvKey, XPubKey) {
    let mnemonics =
        bip39::Mnemonics::from_string(&bip39::dictionary::ENGLISH, SLIP14_MNEMONICS).unwrap();
    let entropy = bip39::Entropy::from_mnemonics(&mnemonics).unwrap();

    api::derive_key_pair(&entropy, b"", path)
}
