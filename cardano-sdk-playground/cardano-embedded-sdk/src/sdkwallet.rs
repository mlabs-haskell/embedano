use cardano_serialization_lib::crypto::{self, Bip32PrivateKey, Bip32PublicKey, Ed25519Signature};
use derivation_path::{ChildIndex, DerivationPath};

use crate::bip::bip39::Entropy;

pub struct XPrvKey(Bip32PrivateKey);

impl XPrvKey {
    pub fn from_entropy(entropy: &Entropy, password: &[u8]) -> Self {
        XPrvKey(Bip32PrivateKey::from_bip39_entropy(entropy, &password))
    }

    pub fn to_hex(&self) -> String {
        let XPrvKey(key) = self;
        hex::encode(key.as_bytes())
    }

    pub fn derive(&self, index: u32) -> Self {
        let XPrvKey(key) = self;
        XPrvKey(key.derive(index))
    }

    pub fn derive_for_path(xprv: XPrvKey, path: DerivationPath) -> Self {
        let mut derived = xprv;
        for index in path.into_iter().map(|ix| adjust_hardened(ix)) {
            derived = derived.derive(index);
        }
        derived
    }

    pub fn to_public(&self) -> XPubKey {
        let XPrvKey(key) = self;
        XPubKey(key.to_public())
    }

    pub fn sign(&self, message: &[u8]) -> Ed25519Signature {
        let XPrvKey(key) = self;
        key.to_raw_key().sign(&message.to_vec())
    }

    pub fn is_pair_of(&self, pub_key: &XPubKey) -> bool {
        self.to_public().as_bytes() == pub_key.as_bytes()
    }
}

pub struct XPubKey(Bip32PublicKey);

impl XPubKey {
    pub fn to_hex(&self) -> String {
        let XPubKey(key) = self;
        hex::encode(key.as_bytes())
    }

    pub fn verify(&self, data: &[u8], signature: &Ed25519Signature) -> bool {
        let XPubKey(key) = self;
        key.to_raw_key().verify(data, signature)
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let XPubKey(key) = self;
        key.as_bytes()
    }
}

fn adjust_hardened(index: &ChildIndex) -> u32 {
    match index {
        &ChildIndex::Hardened(i) => harden(i),
        &ChildIndex::Normal(i) => i,
    }
}

fn harden(i: u32) -> u32 {
    i + 0x80000000
}

pub fn check_ownership(
    payment_key: &XPubKey,
    entropy: &Entropy,
    password: &[u8],
    account_gap: u32,
    address_gap: u32,
) -> bool {
    let root_key = XPrvKey::from_entropy(&entropy, password);
    let level_2_key = root_key.derive(harden(1852)).derive(harden(1815));

    for account_index in 0..account_gap {
        let account_key = level_2_key.derive(harden(account_index));
        for address_index in 0..address_gap {
            let addr_key = account_key.derive(0).derive(address_index);
            if addr_key.is_pair_of(payment_key) {
                return true;
            }
        }
    }

    return false;
}

pub fn proof_ownership(
    nonce: &[u8],
    payment_key: &XPubKey,
    entropy: &Entropy,
    password: &[u8],
    account_gap: u32,
    address_gap: u32,
) -> Option<crypto::Ed25519Signature> {
    let root_key = XPrvKey::from_entropy(&entropy, password);
    let level_2_key = root_key.derive(harden(1852)).derive(harden(1815));

    for account_index in 0..account_gap {
        let account_key = level_2_key.derive(harden(account_index));
        for address_index in 0..address_gap {
            let addr_key = account_key.derive(0).derive(address_index);
            if addr_key.is_pair_of(payment_key) {
                return Some(addr_key.sign(nonce));
            }
        }
    }

    return None;
}

#[cfg(test)]
mod tests {
    use crate::{
        bip::bip39::{dictionary, Mnemonics},
        util::slip14,
    };

    use super::*;

    #[test]
    fn test_pair_check() {
        // TODO: property test?
        let (account_prv_key, account_pub_key) = slip14::make_address_keys();
        assert!(account_prv_key.is_pair_of(&account_pub_key))
    }

    #[test]
    fn test_ownership() {
        let entropy = slip14::make_entropy();
        let (_, account_pub_key) = slip14::make_address_keys();
        assert!(check_ownership(&account_pub_key, &entropy, b"", 20, 20));
    }

    #[test]
    #[should_panic]
    fn test_ownership_wrong_seed() {
        let (_, account_pub_key) = slip14::make_address_keys();

        // different seen/entropy
        let other_mnemonics = "aim wool into nose tell ball arm expand design push elevator multiply glove lonely minimum";
        let other_mnemonics =
            Mnemonics::from_string(&dictionary::ENGLISH, other_mnemonics).unwrap();
        let other_entropy = Entropy::from_mnemonics(&other_mnemonics).unwrap();

        assert!(check_ownership(
            &account_pub_key,
            &other_entropy,
            b"",
            20,
            20
        ));
    }

    #[test]
    #[should_panic]
    fn test_ownership_out_of_account_gap() {
        // out of account gap range
        let entropy = slip14::make_entropy();
        let path: DerivationPath = "m/1852'/1815'/0'/0/21".parse().unwrap();
        let (_, diff_address_key) = slip14::make_keys_for(path);
        assert!(check_ownership(&diff_address_key, &entropy, b"", 20, 20));
    }

    #[test]
    #[should_panic]
    fn test_ownership_out_of_address_gap() {
        let entropy = slip14::make_entropy();
        let path: DerivationPath = "m/1852'/1815'/21'/0/20".parse().unwrap();
        let (_, diff_address_key) = slip14::make_keys_for(path);
        assert!(check_ownership(&diff_address_key, &entropy, b"", 20, 20));
    }

    #[test]
    fn test_proof_of_ownership() {
        let entropy = slip14::make_entropy();
        let (_, address_key) = slip14::make_address_keys();
        let nonce = "some test nonce".as_bytes();
        let proof_signature = proof_ownership(nonce, &address_key, &entropy, b"", 20, 20);
        match proof_signature {
            Some(sig) => assert!(
                address_key.verify(nonce, &sig),
                "Payment key could not verify signature"
            ),
            None => panic!("Proof function should return signature, but it didn't"),
        }
    }

    #[test]
    fn test_proof_of_ownership_wrong_seed() {
        // non-slip14 entropy
        let other_mnemonics = "aim wool into nose tell ball arm expand design push elevator multiply glove lonely minimum";
        let other_mnemonics =
            Mnemonics::from_string(&dictionary::ENGLISH, other_mnemonics).unwrap();
        let other_entropy = Entropy::from_mnemonics(&other_mnemonics).unwrap();

        let (_, address_key) = slip14::make_address_keys();
        let nonce = "some test nonce".as_bytes();
        let proof_signature = proof_ownership(nonce, &address_key, &other_entropy, b"", 20, 20);
        assert!(
            proof_signature.is_none(),
            "Key was found for wrong seed and nonce signed - this should not happen"
        )
    }
}
