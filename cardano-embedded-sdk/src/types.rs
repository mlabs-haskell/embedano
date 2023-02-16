use crate::{
    bip::bip39::Entropy,
    crypto::{Bip32PrivateKey, Bip32PublicKey, Ed25519Signature},
};

use alloc::{
    format,
    string::{String, ToString},
    vec::Vec,
};

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct TxIdParseError {
    message: String,
}

pub struct TxId([u8; 32]);

impl TxId {
    pub fn to_bytes(&self) -> &[u8] {
        &self.0[..]
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<TxId, TxIdParseError> {
        match bytes.try_into() {
            Ok(bs) => Ok(TxId(bs)),
            Err(_) => {
                let error = format!(
                    "TxId length should be 32 bytes, but data length is {}",
                    bytes.len()
                );
                Err(TxIdParseError { message: error })
            }
        }
    }

    pub fn from_hex(hex_str: &str) -> Result<TxId, TxIdParseError> {
        let tx_id = hex::decode(hex_str).map_err(|_| TxIdParseError {
            message: "Failed to decode hex".to_string(),
        })?;

        Self::from_bytes(&tx_id[..])
    }
}

pub struct XPrvKey(Bip32PrivateKey);

impl XPrvKey {
    pub fn from_entropy(entropy: &Entropy, password: &[u8]) -> Self {
        XPrvKey(Bip32PrivateKey::from_bip39_entropy(entropy, password))
    }

    pub fn to_hex(&self) -> String {
        let XPrvKey(key) = self;
        hex::encode(key.as_bytes())
    }

    pub fn derive(&self, index: u32) -> Self {
        XPrvKey(self.0.derive(index))
    }

    pub fn to_public(&self) -> XPubKey {
        let XPrvKey(key) = self;
        XPubKey(key.to_public())
    }

    pub fn sign(&self, message: &[u8]) -> Ed25519Signature {
        self.0.to_raw_key().sign(message)
    }

    pub fn is_pair_of(&self, pub_key: &XPubKey) -> bool {
        self.to_public().as_bytes() == pub_key.as_bytes()
    }
}

pub struct XPubKey(Bip32PublicKey);

// TODO: add `from_` methods (`from_hex`, `from_bytes`, etc)
impl XPubKey {
    pub fn to_hex(&self) -> String {
       self.0.to_hex()
    }

    /// Get hex of key without chain code
    pub fn raw_key_hex(&self) -> String {
        self.0.to_raw_key().to_hex()
    }

    pub fn verify(&self, data: &[u8], signature: &Ed25519Signature) -> bool {
        self.0.to_raw_key().verify(data, signature)
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        self.0.as_bytes()
    }
}

/// Harden derivation index.
pub fn harden(i: u32) -> u32 {
    i + 0x80000000
}

#[cfg(test)]
mod tests {
    use crate::util::slip14;

    #[test]
    fn test_pair_check() {
        let (account_prv_key, account_pub_key) = slip14::make_address_keys();
        assert!(account_prv_key.is_pair_of(&account_pub_key))
    }
}
