use derivation_path::DerivationPath;

use crate::{
    bip::bip39::{self, Entropy},
    sdkwallet::{XPrvKey, XPubKey},
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

pub fn make_keys() -> (XPrvKey, XPubKey) {
    let path: DerivationPath = "m/1852'/1815'/0'/0/0".parse().unwrap();
    make_keys_for(path)
}

pub fn make_keys_for(path: DerivationPath) -> (XPrvKey, XPubKey) {
    let mnemonics =
        bip39::Mnemonics::from_string(&bip39::dictionary::ENGLISH, SLIP14_MNEMONICS).unwrap();
    let entropy = bip39::Entropy::from_mnemonics(&mnemonics).unwrap();

    let root_key = XPrvKey::from_entropy(&entropy, b"");
    let account_prv_key = XPrvKey::derive_for_path(root_key, path);
    let account_pub_key = account_prv_key.to_public();
    (account_prv_key, account_pub_key)
}
