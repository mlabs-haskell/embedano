use cardano_serialization_lib::crypto::{Bip32PrivateKey, Bip32PublicKey, Ed25519Signature};
use derivation_path::{ChildIndex, DerivationPath};

pub struct XPrvKey(Bip32PrivateKey);

impl XPrvKey {
    pub fn from_entropy(entropy: &[u8], password: &[u8]) -> Self {
        XPrvKey(Bip32PrivateKey::from_bip39_entropy(&entropy, &password))
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
        for index in path.into_iter() {
            let i = match index {
                &ChildIndex::Hardened(i) => i + 0x80000000,
                &ChildIndex::Normal(i) => i,
            };
            derived = derived.derive(i);
            // println!("derived {:x} xprv: {}\n", i, derived.to_hex());
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
}
