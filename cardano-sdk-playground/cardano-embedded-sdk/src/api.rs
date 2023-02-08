//! Embedano SDK API
//! SDK API functions aim to be as easy to use as possible.
//! Main ingredients required:
//! - entropy (seed) for HD wallet, that usually stored in the memory of the embedded device
//! - password provided by the user
//! - derivation path of required keys (as all operations using keys)
//!
//! Functions of API are built on top of the more low-level functions
//! that can be found in `types.rs`.
//!
//! Examples can be found in docs for each function.

use crate::{
    bip::bip39::Entropy,
    crypto::Ed25519Signature,
    types::{TxId, XPrvKey, XPubKey},
};

use derivation_path::{ChildIndex, DerivationPath};

const EXTERNAL_CHAIN_CODE: u32 = 0;

/// Derive extended private key from entropy (seed) for specified derivation path.
/// # Example
/// ```
/// use cardano_embedded_sdk::bip::bip39::{dictionary, Entropy, Mnemonics};
/// use cardano_embedded_sdk::types::XPrvKey;
/// use cardano_embedded_sdk::api as embedano;
/// use derivation_path::{DerivationPath};
///
/// let mnemonics = Mnemonics::from_string(&dictionary::ENGLISH,"all all all all all all all all all all all all",).unwrap();
/// let entropy = Entropy::from_mnemonics(&mnemonics).unwrap();
/// let password = b"embedano";
/// let path: DerivationPath = "m/1852'/1815'/0'/0/0".parse().unwrap();
///
/// let private_key: XPrvKey  = embedano::derive_key(&entropy, password, &path);
/// ```
pub fn derive_key(entropy: &Entropy, password: &[u8], path: &DerivationPath) -> XPrvKey {
    let mut key = XPrvKey::from_entropy(entropy, password);
    for index in path.into_iter().map(adjust_hardened) {
        key = key.derive(index);
    }
    key
}

/// Extension of `derive_key` that also returns public key.
/// # Example
/// ```
/// use cardano_embedded_sdk::bip::bip39::{dictionary, Entropy, Mnemonics};
/// use derivation_path::{ChildIndex, DerivationPath};
/// use cardano_embedded_sdk::api as embedano;
///
///
/// let mnemonics = Mnemonics::from_string(&dictionary::ENGLISH,"all all all all all all all all all all all all",).unwrap();
/// let entropy = Entropy::from_mnemonics(&mnemonics).unwrap();
/// let password = b"embedano";
/// let path: DerivationPath = "m/1852'/1815'/0'/0/0".parse().unwrap();
/// let (pub_key, prv_key) = embedano::derive_key_pair(&entropy, password, &path);
/// ```
pub fn derive_key_pair(
    entropy: &Entropy,
    password: &[u8],
    path: &DerivationPath,
) -> (XPrvKey, XPubKey) {
    let private = derive_key(entropy, password, path);
    let public = private.to_public();
    (private, public)
}

/// Sign transaction id with private key derived for provided path.
/// Transaction id (`TxId`) - is hash of transaction body.
/// # Example
/// ```
/// use cardano_embedded_sdk::bip::bip39::{dictionary, Entropy, Mnemonics};
/// use cardano_embedded_sdk::types::{TxId, XPrvKey};
/// use cardano_embedded_sdk::api as embedano;
/// use derivation_path::{DerivationPath};
///
/// let mnemonics = Mnemonics::from_string(&dictionary::ENGLISH,"all all all all all all all all all all all all",).unwrap();
/// let entropy = Entropy::from_mnemonics(&mnemonics).unwrap();
/// let password = b"embedano";
/// let path: DerivationPath = "m/1852'/1815'/0'/0/0".parse().unwrap();
///
/// let tx_id = TxId::from_hex("bb1eb401cd03b0cd8caa08997df0a2ab226772c4d3a08adfb5a60ba34de12dfb").unwrap();
/// let signature = embedano::sign_tx_id(&tx_id, &entropy, password, &path);
/// ```
pub fn sign_tx_id(
    tx_id: &TxId,
    entropy: &Entropy,
    password: &[u8],
    path: &DerivationPath,
) -> Ed25519Signature {
    sign_data(tx_id.to_bytes(), entropy, password, path)
}

/// Sign binary data with private key derived for provided path.
/// # Example
/// ```
/// use cardano_embedded_sdk::bip::bip39::{dictionary, Entropy, Mnemonics};
/// use cardano_embedded_sdk::types::{TxId, XPrvKey};
/// use cardano_embedded_sdk::api as embedano;
/// use derivation_path::{DerivationPath};
///
/// let mnemonics = Mnemonics::from_string(&dictionary::ENGLISH,"all all all all all all all all all all all all",).unwrap();
/// let entropy = Entropy::from_mnemonics(&mnemonics).unwrap();
/// let password = b"embedano";
/// let path: DerivationPath = "m/1852'/1815'/0'/0/0".parse().unwrap();
///
/// let signature = embedano::sign_data(b"some bytes", &entropy, password, &path);
/// ```
pub fn sign_data(
    data: &[u8],
    entropy: &Entropy,
    password: &[u8],
    path: &DerivationPath,
) -> Ed25519Signature {
    derive_key(entropy, password, path).sign(data)
}

/// Prove public key ownership (account or address level) by signing nonce. See also `KeyType` docs.
///
/// If function returns `Some(signature)`, caller should be able to verify signature
/// with public key provided as function argument.
/// To control how deep to search for the corresponding private key `KeyType`
/// parameter should be specified.
/// # Example
/// ```
/// use cardano_embedded_sdk::bip::bip39::{dictionary, Entropy, Mnemonics};
/// use cardano_embedded_sdk::types::{TxId, XPrvKey};
/// use cardano_embedded_sdk::api as embedano;
/// use derivation_path::{DerivationPath};
///
/// let mnemonics = "aim wool into nose tell ball arm expand design push elevator multiply glove lonely minimum";
/// let mnemonics = Mnemonics::from_string(&dictionary::ENGLISH, mnemonics).unwrap();
/// let entropy = Entropy::from_mnemonics(&mnemonics).unwrap();
/// let password = b"embedano";
/// let nonce = "some nonce".as_bytes();
/// let path: DerivationPath = "m/1852'/1815'/0'/0/0".parse().unwrap();
///
/// let (_, pub_key) = embedano::derive_key_pair(&entropy, password, &path);
/// let key_type = embedano::KeyType::AddressKey { account_gap: 5, address_gap: 5};
/// let proof = embedano::prove_ownership(&nonce, &pub_key, &entropy, password,key_type).unwrap();
/// assert!(&pub_key.verify(nonce, &proof))
/// ```
pub fn prove_ownership(
    nonce: &[u8],
    payment_key: &XPubKey,
    entropy: &Entropy,
    password: &[u8],
    key_type: KeyType,
) -> Option<Ed25519Signature> {
    let root_key = XPrvKey::from_entropy(entropy, password);
    find_matching_private_key(payment_key, &root_key, key_type).map(|key| key.sign(nonce))
}

// Helper function for `prove_ownership`
fn find_matching_private_key(
    pub_key: &XPubKey,
    root_key: &XPrvKey,
    key_type: KeyType,
) -> Option<XPrvKey> {
    let level_2_key = root_key.derive(harden(1852)).derive(harden(1815));
    let account_keys = |gap: u32| (0..=gap).map(|i| level_2_key.derive(harden(i)));
    match key_type {
        KeyType::AccountKey { account_gap } => {
            for account_key in account_keys(account_gap) {
                if account_key.is_pair_of(pub_key) {
                    return Some(account_key);
                }
            }
            None
        }

        KeyType::AddressKey {
            account_gap,
            address_gap,
        } => {
            for account_key in account_keys(account_gap) {
                for address_index in 0..=address_gap {
                    let addr_key = account_key
                        // todo: account discovery goes only thorough external chain
                        // but maybe we should support proofs for staking keys as well with code `2`
                        .derive(EXTERNAL_CHAIN_CODE)
                        .derive(address_index);
                    if addr_key.is_pair_of(pub_key) {
                        return Some(addr_key);
                    }
                }
            }
            None
        }
    }
}

/// Allows to specify gaps for account and address indexes for `prove_ownership` function.
///
/// `prove_ownership` uses algorithm similar to account discovery (BIP-0044) to find
/// corresponding private key and sign nonce to prove key ownership.
/// `KeyType` allows to limit number of account and address indexes to iterate
/// through while deriving private key.
/// Usually account discovery uses indexes from 0 to 20 (inclusive).
#[derive(Clone)]
pub enum KeyType {
    /// Use together with `prove_ownership` to limit the number of account indexes to search.
    AccountKey { account_gap: u32 },
    /// Use together with `prove_ownership` to limit the number of account and address
    /// indexes to search.
    AddressKey { account_gap: u32, address_gap: u32 },
}

/// Harden derivation index.
pub fn harden(i: u32) -> u32 {
    i + 0x80000000
}

fn adjust_hardened(index: &ChildIndex) -> u32 {
    match *index {
        ChildIndex::Hardened(i) => harden(i),
        ChildIndex::Normal(i) => i,
    }
}

// todo: more tests
#[cfg(test)]
mod tests {
    use crate::{
        api::KeyType::*,
        bip::bip39::{dictionary, Mnemonics},
        util::slip14,
    };

    use super::*;

    // helper function for test cases
    fn check_ownership(
        pub_key: &XPubKey,
        entropy: &Entropy,
        password: &[u8],
        nonce: &[u8],
        key_type: KeyType,
    ) -> Option<bool> {
        prove_ownership(nonce, pub_key, entropy, password, key_type)
            .map(|s| pub_key.verify(nonce, &s))
    }

    #[test]
    fn test_proof_account_ownership() {
        let entropy = slip14::make_entropy();
        let nonce = "some test nonce".as_bytes();
        let path: DerivationPath = "m/1852'/1815'/4'".parse().unwrap();
        let (_, x_pub) = slip14::make_keys_for(&path);
        let key_type = AccountKey { account_gap: 20 };

        let check = check_ownership(&x_pub, &entropy, b"", &nonce, key_type);
        assert_eq!(check, Some(true))
    }

    #[test]
    fn test_proof_address_ownership() {
        let entropy = slip14::make_entropy();
        let nonce = "some test nonce".as_bytes();
        let path: DerivationPath = "m/1852'/1815'/4'/0/2".parse().unwrap();
        let (_, x_pub) = slip14::make_keys_for(&path);
        let key_type = AddressKey {
            account_gap: 20,
            address_gap: 20,
        };

        let check = check_ownership(&x_pub, &entropy, b"", &nonce, key_type);
        assert_eq!(check, Some(true))
    }

    #[test]
    fn test_account_out_of_gap_limit() {
        let entropy = slip14::make_entropy();
        let nonce = "some test nonce".as_bytes();
        let path: DerivationPath = "m/1852'/1815'/21'".parse().unwrap();
        let (_, x_pub) = slip14::make_keys_for(&path);
        let key_type = AccountKey { account_gap: 20 };
        let check = check_ownership(&x_pub, &entropy, b"", &nonce, key_type);
        assert_eq!(check, None)
    }

    #[test]
    fn test_address_out_of_gap_limit() {
        let entropy = slip14::make_entropy();
        let nonce = "some test nonce".as_bytes();

        let path: DerivationPath = "m/1852'/1815'/5'/0/5".parse().unwrap();
        let (_, x_pub) = slip14::make_keys_for(&path);

        // check out of the gap on account level
        let key_type = AddressKey {
            account_gap: 4,
            address_gap: 6,
        };
        let check1 = check_ownership(&x_pub, &entropy, b"", &nonce, key_type);

        // check out of the gap on address level
        let key_type = AddressKey {
            account_gap: 6,
            address_gap: 4,
        };
        let check2 = check_ownership(&x_pub, &entropy, b"", &nonce, key_type);

        assert_eq!((None, None), (check1, check2))
    }

    #[test]
    fn test_ownership_wrong_seed() {
        // non-slip14 entropy
        let other_mnemonics = "aim wool into nose tell ball arm expand design push elevator multiply glove lonely minimum";
        let other_mnemonics =
            Mnemonics::from_string(&dictionary::ENGLISH, other_mnemonics).unwrap();
        let other_entropy = Entropy::from_mnemonics(&other_mnemonics).unwrap();

        let (_, x_pub) = slip14::make_address_keys();
        let nonce = "some test nonce".as_bytes();

        // check out of the gap on account level
        let key_type = AddressKey {
            account_gap: 5,
            address_gap: 5,
        };
        let check1 = check_ownership(&x_pub, &other_entropy, b"", &nonce, key_type);

        // check out of the gap on address level
        let key_type = AddressKey {
            account_gap: 20,
            address_gap: 20,
        };
        let check2 = check_ownership(&x_pub, &other_entropy, b"", &nonce, key_type);

        assert_eq!((None, None), (check1, check2))
    }

    #[test]
    fn test_sign_tx_id() {
        let mnemonics = "aim wool into nose tell ball arm expand design push elevator multiply glove lonely minimum";
        let mnemonics = Mnemonics::from_string(&dictionary::ENGLISH, mnemonics).unwrap();
        let entropy = Entropy::from_mnemonics(&mnemonics).unwrap();
        let path: DerivationPath = "m/1852'/1815'/5'/0/5".parse().unwrap();
        let tx_id =
            TxId::from_hex("bb1eb401cd03b0cd8caa08997df0a2ab226772c4d3a08adfb5a60ba34de12dfb")
                .unwrap();
        let password = b"embedano";

        let signature = sign_tx_id(&tx_id, &entropy, password, &path);

        let (_, pub_key) = derive_key_pair(&entropy, password, &path);
        assert!(pub_key.verify(tx_id.to_bytes(), &signature))
    }

    #[test]
    fn test_key_exploration() {
        let mnemonics = "aim wool into nose tell ball arm expand design push elevator multiply glove lonely minimum";
        let mnemonics = Mnemonics::from_string(&dictionary::ENGLISH, mnemonics).unwrap();
        let entropy = Entropy::from_mnemonics(&mnemonics).unwrap();
        let password = b"embedano";
        let nonce = "some nonce".as_bytes();

        // check account level exploration consistency
        let path: DerivationPath = "m/1852'/1815'/5'".parse().unwrap();
        let (_, pub_key) = derive_key_pair(&entropy, password, &path);
        let key_type = AccountKey { account_gap: 5 };
        let check1 = check_ownership(&pub_key, &entropy, password, nonce, key_type);

        // check address level exploration consistency
        let path: DerivationPath = "m/1852'/1815'/5'/0/5".parse().unwrap();
        let (_, pub_key) = derive_key_pair(&entropy, password, &path);
        let key_type = AddressKey {
            account_gap: 5,
            address_gap: 5,
        };
        let check2 = check_ownership(&pub_key, &entropy, password, nonce, key_type);
        assert_eq!((Some(true), Some(true)), (check1, check2))
    }
}
