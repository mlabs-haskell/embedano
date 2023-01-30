use cardano_serialization_lib::crypto::{Bip32PrivateKey, Bip32PublicKey, Ed25519Signature};
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
        &ChildIndex::Hardened(i) => i + 0x80000000,
        &ChildIndex::Normal(i) => i,
    }
}

// pub fn proof_ownership(entropy: &Entropy, password: &[u8], nonce)

#[cfg(test)]
mod tests {
    use crate::util::slip14;

    use super::*;

    #[test]
    fn test_pair_check() {
        let (account_prv_key, account_pub_key) = slip14::make_keys();
        assert!(account_prv_key.is_pair_of(&account_pub_key))
    }
}
