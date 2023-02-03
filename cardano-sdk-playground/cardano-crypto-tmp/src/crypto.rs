use alloc::{
    format,
    string::{String, ToString},
    vec::Vec,
};
use core::str::FromStr;

use crate::chain_crypto as crypto;
use crate::impl_mockchain as chain;
use chain::key;
use crypto::bech32::Bech32 as _;

use bech32::ToBase32;
use cryptoxide::blake2b::Blake2b;
use rand_core::{CryptoRng, RngCore};

use super::*;

pub(crate) fn blake2b224(data: &[u8]) -> [u8; 28] {
    let mut out = [0; 28];
    Blake2b::blake2b(&mut out, data, &[]);
    out
}

#[allow(dead_code)]
pub(crate) fn blake2b256(data: &[u8]) -> [u8; 32] {
    let mut out = [0; 32];
    Blake2b::blake2b(&mut out, data, &[]);
    out
}

// All key structs were taken from js-chain-libs:
// https://github.com/Emurgo/js-chain-libs

pub struct Bip32PrivateKey(crypto::SecretKey<crypto::Ed25519Bip32>);

impl Bip32PrivateKey {
    /// derive this private key with the given index.
    ///
    /// # Security considerations
    ///
    /// * hard derivation index cannot be soft derived with the public key
    ///
    /// # Hard derivation vs Soft derivation
    ///
    /// If you pass an index below 0x80000000 then it is a soft derivation.
    /// The advantage of soft derivation is that it is possible to derive the
    /// public key too. I.e. derivation the private key with a soft derivation
    /// index and then retrieving the associated public key is equivalent to
    /// deriving the public key associated to the parent private key.
    ///
    /// Hard derivation index does not allow public key derivation.
    ///
    /// This is why deriving the private key should not fail while deriving
    /// the public key may fail (if the derivation index is invalid).
    ///
    pub fn derive(&self, index: u32) -> Bip32PrivateKey {
        Bip32PrivateKey(crypto::derive::derive_sk_ed25519(&self.0, index))
    }

    /// 128-byte xprv a key format in Cardano that some software still uses or requires
    /// the traditional 96-byte xprv is simply encoded as
    /// prv | chaincode
    /// however, because some software may not know how to compute a public key from a private key,
    /// the 128-byte inlines the public key in the following format
    /// prv | pub | chaincode
    /// so be careful if you see the term "xprv" as it could refer to either one
    /// our library does not require the pub (instead we compute the pub key when needed)
    pub fn from_128_xprv(bytes: &[u8]) -> Result<Bip32PrivateKey, JsError> {
        let mut buf = [0; 96];
        buf[0..64].clone_from_slice(&bytes[0..64]);
        buf[64..96].clone_from_slice(&bytes[96..128]);

        Bip32PrivateKey::from_bytes(&buf)
    }
    /// see from_128_xprv
    pub fn to_128_xprv(&self) -> Vec<u8> {
        let prv_key = self.to_raw_key().as_bytes();
        let pub_key = self.to_public().to_raw_key().as_bytes();
        let cc = self.chaincode();

        let mut buf = [0; 128];
        buf[0..64].clone_from_slice(&prv_key);
        buf[64..96].clone_from_slice(&pub_key);
        buf[96..128].clone_from_slice(&cc);
        buf.to_vec()
    }

    pub fn generate_ed25519_bip32<T: RngCore + CryptoRng>(rng: T) -> Bip32PrivateKey {
        Bip32PrivateKey(crypto::SecretKey::<crypto::Ed25519Bip32>::generate(rng))
    }

    pub fn to_raw_key(&self) -> PrivateKey {
        PrivateKey(key::EitherEd25519SecretKey::Extended(
            crypto::derive::to_raw_sk(&self.0),
        ))
    }

    pub fn to_public(&self) -> Bip32PublicKey {
        Bip32PublicKey(self.0.to_public().into())
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Bip32PrivateKey, JsError> {
        crypto::SecretKey::<crypto::Ed25519Bip32>::from_binary(bytes)
            .map_err(|e| JsError::from_str(&format!("{}", e)))
            .map(Bip32PrivateKey)
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        self.0.as_ref().to_vec()
    }

    pub fn from_bech32(bech32_str: &str) -> Result<Bip32PrivateKey, JsError> {
        crypto::SecretKey::try_from_bech32_str(&bech32_str)
            .map(Bip32PrivateKey)
            .map_err(|_| JsError::from_str("Invalid secret key"))
    }

    pub fn to_bech32(&self) -> String {
        self.0.to_bech32_str()
    }

    pub fn from_bip39_entropy(entropy: &[u8], password: &[u8]) -> Bip32PrivateKey {
        Bip32PrivateKey(crypto::derive::from_bip39_entropy(&entropy, &password))
    }

    pub fn chaincode(&self) -> Vec<u8> {
        const ED25519_PRIVATE_KEY_LENGTH: usize = 64;
        const XPRV_SIZE: usize = 96;
        self.0.as_ref()[ED25519_PRIVATE_KEY_LENGTH..XPRV_SIZE].to_vec()
    }

    pub fn to_hex(&self) -> String {
        hex::encode(self.as_bytes())
    }

    pub fn from_hex(hex_str: &str) -> Result<Bip32PrivateKey, JsError> {
        match hex::decode(hex_str) {
            Ok(data) => Ok(Self::from_bytes(data.as_ref())?),
            Err(e) => Err(JsError::from_str(&e.to_string())),
        }
    }
}

pub struct Bip32PublicKey(crypto::PublicKey<crypto::Ed25519Bip32>);

impl Bip32PublicKey {
    /// derive this public key with the given index.
    ///
    /// # Errors
    ///
    /// If the index is not a soft derivation index (< 0x80000000) then
    /// calling this method will fail.
    ///
    /// # Security considerations
    ///
    /// * hard derivation index cannot be soft derived with the public key
    ///
    /// # Hard derivation vs Soft derivation
    ///
    /// If you pass an index below 0x80000000 then it is a soft derivation.
    /// The advantage of soft derivation is that it is possible to derive the
    /// public key too. I.e. derivation the private key with a soft derivation
    /// index and then retrieving the associated public key is equivalent to
    /// deriving the public key associated to the parent private key.
    ///
    /// Hard derivation index does not allow public key derivation.
    ///
    /// This is why deriving the private key should not fail while deriving
    /// the public key may fail (if the derivation index is invalid).
    ///
    pub fn derive(&self, index: u32) -> Result<Bip32PublicKey, JsError> {
        crypto::derive::derive_pk_ed25519(&self.0, index)
            .map(Bip32PublicKey)
            .map_err(|e| JsError::from_str(&format! {"{:?}", e}))
    }

    pub fn to_raw_key(&self) -> PublicKey {
        PublicKey(crypto::derive::to_raw_pk(&self.0))
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Bip32PublicKey, JsError> {
        crypto::PublicKey::<crypto::Ed25519Bip32>::from_binary(bytes)
            .map_err(|e| JsError::from_str(&format!("{}", e)))
            .map(Bip32PublicKey)
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        self.0.as_ref().to_vec()
    }

    pub fn from_bech32(bech32_str: &str) -> Result<Bip32PublicKey, JsError> {
        crypto::PublicKey::try_from_bech32_str(&bech32_str)
            .map(Bip32PublicKey)
            .map_err(|e| JsError::from_str(&format!("{}", e)))
    }

    pub fn to_bech32(&self) -> String {
        self.0.to_bech32_str()
    }

    pub fn chaincode(&self) -> Vec<u8> {
        const ED25519_PUBLIC_KEY_LENGTH: usize = 32;
        const XPUB_SIZE: usize = 64;
        self.0.as_ref()[ED25519_PUBLIC_KEY_LENGTH..XPUB_SIZE].to_vec()
    }

    pub fn to_hex(&self) -> String {
        hex::encode(self.as_bytes())
    }

    pub fn from_hex(hex_str: &str) -> Result<Bip32PublicKey, JsError> {
        match hex::decode(hex_str) {
            Ok(data) => Ok(Self::from_bytes(data.as_ref())?),
            Err(e) => Err(JsError::from_str(&e.to_string())),
        }
    }
}

pub struct PrivateKey(key::EitherEd25519SecretKey);

impl From<key::EitherEd25519SecretKey> for PrivateKey {
    fn from(secret_key: key::EitherEd25519SecretKey) -> PrivateKey {
        PrivateKey(secret_key)
    }
}

impl PrivateKey {
    pub fn to_public(&self) -> PublicKey {
        self.0.to_public().into()
    }

    pub fn generate_ed25519<T: RngCore + CryptoRng>(rng: T) -> PrivateKey {
        PrivateKey(key::EitherEd25519SecretKey::Normal(crypto::SecretKey::<
            crypto::Ed25519,
        >::generate(rng)))
    }

    pub fn generate_ed25519extended<T: RngCore + CryptoRng>(rng: T) -> PrivateKey {
        PrivateKey(key::EitherEd25519SecretKey::Extended(crypto::SecretKey::<
            crypto::Ed25519Extended,
        >::generate(
            rng
        )))
    }

    /// Get private key from its bech32 representation
    /// ```javascript
    /// PrivateKey.from_bech32(&#39;ed25519_sk1ahfetf02qwwg4dkq7mgp4a25lx5vh9920cr5wnxmpzz9906qvm8qwvlts0&#39;);
    /// ```
    /// For an extended 25519 key
    /// ```javascript
    /// PrivateKey.from_bech32(&#39;ed25519e_sk1gqwl4szuwwh6d0yk3nsqcc6xxc3fpvjlevgwvt60df59v8zd8f8prazt8ln3lmz096ux3xvhhvm3ca9wj2yctdh3pnw0szrma07rt5gl748fp&#39;);
    /// ```
    pub fn from_bech32(bech32_str: &str) -> Result<PrivateKey, JsError> {
        crypto::SecretKey::try_from_bech32_str(&bech32_str)
            .map(key::EitherEd25519SecretKey::Extended)
            .or_else(|_| {
                crypto::SecretKey::try_from_bech32_str(&bech32_str)
                    .map(key::EitherEd25519SecretKey::Normal)
            })
            .map(PrivateKey)
            .map_err(|_| JsError::from_str("Invalid secret key"))
    }

    pub fn to_bech32(&self) -> String {
        match self.0 {
            key::EitherEd25519SecretKey::Normal(ref secret) => secret.to_bech32_str(),
            key::EitherEd25519SecretKey::Extended(ref secret) => secret.to_bech32_str(),
        }
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        match self.0 {
            key::EitherEd25519SecretKey::Normal(ref secret) => secret.as_ref().to_vec(),
            key::EitherEd25519SecretKey::Extended(ref secret) => secret.as_ref().to_vec(),
        }
    }

    pub fn from_extended_bytes(bytes: &[u8]) -> Result<PrivateKey, JsError> {
        crypto::SecretKey::from_binary(bytes)
            .map(key::EitherEd25519SecretKey::Extended)
            .map(PrivateKey)
            .map_err(|_| JsError::from_str("Invalid extended secret key"))
    }

    pub fn from_normal_bytes(bytes: &[u8]) -> Result<PrivateKey, JsError> {
        crypto::SecretKey::from_binary(bytes)
            .map(key::EitherEd25519SecretKey::Normal)
            .map(PrivateKey)
            .map_err(|_| JsError::from_str("Invalid normal secret key"))
    }

    pub fn sign(&self, message: &[u8]) -> Ed25519Signature {
        Ed25519Signature(self.0.sign(&message.to_vec()))
    }

    pub fn to_hex(&self) -> String {
        hex::encode(self.as_bytes())
    }

    pub fn from_hex(hex_str: &str) -> Result<PrivateKey, JsError> {
        let data: Vec<u8> = match hex::decode(hex_str) {
            Ok(d) => d,
            Err(e) => return Err(JsError::from_str(&e.to_string())),
        };
        let data_slice: &[u8] = data.as_slice();
        crypto::SecretKey::from_binary(data_slice)
            .map(key::EitherEd25519SecretKey::Normal)
            .or_else(|_| {
                crypto::SecretKey::from_binary(data_slice)
                    .map(key::EitherEd25519SecretKey::Extended)
            })
            .map(PrivateKey)
            .map_err(|_| JsError::from_str("Invalid secret key"))
    }
}

/// ED25519 key used as public key
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PublicKey(crypto::PublicKey<crypto::Ed25519>);

impl From<crypto::PublicKey<crypto::Ed25519>> for PublicKey {
    fn from(key: crypto::PublicKey<crypto::Ed25519>) -> PublicKey {
        PublicKey(key)
    }
}

impl PublicKey {
    /// Get public key from its bech32 representation
    /// Example:
    /// ```javascript
    /// const pkey = PublicKey.from_bech32(&#39;ed25519_pk1dgaagyh470y66p899txcl3r0jaeaxu6yd7z2dxyk55qcycdml8gszkxze2&#39;);
    /// ```
    pub fn from_bech32(bech32_str: &str) -> Result<PublicKey, JsError> {
        crypto::PublicKey::try_from_bech32_str(&bech32_str)
            .map(PublicKey)
            .map_err(|_| JsError::from_str("Malformed public key"))
    }

    pub fn to_bech32(&self) -> String {
        self.0.to_bech32_str()
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        self.0.as_ref().to_vec()
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<PublicKey, JsError> {
        crypto::PublicKey::from_binary(bytes)
            .map_err(|e| JsError::from_str(&format!("{}", e)))
            .map(PublicKey)
    }

    pub fn verify(&self, data: &[u8], signature: &Ed25519Signature) -> bool {
        signature.0.verify_slice(&self.0, data) == crypto::Verification::Success
    }

    pub fn hash(&self) -> Ed25519KeyHash {
        Ed25519KeyHash::from(blake2b224(self.as_bytes().as_ref()))
    }

    pub fn to_hex(&self) -> String {
        hex::encode(self.as_bytes())
    }

    pub fn from_hex(hex_str: &str) -> Result<PublicKey, JsError> {
        match hex::decode(hex_str) {
            Ok(data) => Ok(Self::from_bytes(data.as_ref())?),
            Err(e) => Err(JsError::from_str(&e.to_string())),
        }
    }
}

macro_rules! impl_signature {
    ($name:ident, $signee_type:ty, $verifier_type:ty) => {
        #[derive(Clone, Debug, Eq, PartialEq)]
        pub struct $name(crypto::Signature<$signee_type, $verifier_type>);

        impl $name {
            pub fn to_bytes(&self) -> Vec<u8> {
                self.0.as_ref().to_vec()
            }

            pub fn to_bech32(&self) -> String {
                self.0.to_bech32_str()
            }

            pub fn to_hex(&self) -> String {
                hex::encode(&self.0.as_ref())
            }

            pub fn from_bech32(bech32_str: &str) -> Result<$name, JsError> {
                crypto::Signature::try_from_bech32_str(&bech32_str)
                    .map($name)
                    .map_err(|e| JsError::from_str(&format!("{}", e)))
            }

            pub fn from_hex(input: &str) -> Result<$name, JsError> {
                crypto::Signature::from_str(input)
                    .map_err(|e| JsError::from_str(&format!("{:?}", e)))
                    .map($name)
            }
        }

        from_bytes!($name, bytes, {
            crypto::Signature::from_binary(bytes.as_ref())
                .map_err(|e| {
                    DeserializeError::new(stringify!($name), DeserializeFailure::SignatureError(e))
                })
                .map($name)
        });
    };
}

impl_signature!(Ed25519Signature, Vec<u8>, crypto::Ed25519);

macro_rules! impl_hash_type {
    ($name:ident, $byte_count:expr) => {
        #[derive(Debug, Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
        pub struct $name(pub(crate) [u8; $byte_count]);

        // TODO: fix
        // hash types are the only types in this library to not expect the entire CBOR structure.
        // There is no CBOR binary tag here just the raw hash bytes.
        // from_bytes!($name, bytes, {
        //     use core::convert::TryInto;
        //     match bytes.len() {
        //         $byte_count => Ok($name(bytes[..$byte_count].try_into().unwrap())),
        //         other_len => {
        //             let cbor_error = cbor_event::Error::WrongLen(
        //                 $byte_count,
        //                 cbor_event::Len::Len(other_len as u64),
        //                 "hash length",
        //             );
        //             Err(DeserializeError::new(
        //                 stringify!($name),
        //                 DeserializeFailure::CBOR(cbor_error),
        //             ))
        //         }
        //     }
        // });

        impl $name {
            // hash types are the only types in this library to not give the entire CBOR structure.
            // There is no CBOR binary tag here just the raw hash bytes.
            pub fn to_bytes(&self) -> Vec<u8> {
                self.0.to_vec()
            }

            pub fn to_bech32(&self, prefix: &str) -> Result<String, JsError> {
                bech32::encode(&prefix, self.to_bytes().to_base32())
                    .map_err(|e| JsError::from_str(&format! {"{:?}", e}))
            }

            // TODO: fix
            // pub fn from_bech32(bech_str: &str) -> Result<$name, JsError> {
            //     let (_hrp, u5data) =
            //         bech32::decode(bech_str).map_err(|e| JsError::from_str(&e.to_string()))?;
            //     let data: Vec<u8> = bech32::FromBase32::from_base32(&u5data).unwrap();
            //     Ok(Self::from_bytes(data)?)
            // }

            pub fn to_hex(&self) -> String {
                hex::encode(&self.0)
            }

            // TODO: fix
            // pub fn from_hex(hex: &str) -> Result<$name, JsError> {
            //     let bytes = hex::decode(hex)
            //         .map_err(|e| JsError::from_str(&format!("hex decode failed: {}", e)))?;
            //     Self::from_bytes(bytes).map_err(|e| JsError::from_str(&format!("{:?}", e)))
            // }
        }

        // associated consts are not supported in wasm_bindgen
        impl $name {
            pub const BYTE_COUNT: usize = $byte_count;
        }

        // can't expose [T; N] to wasm for new() but it's useful internally so we implement From trait
        impl From<[u8; $byte_count]> for $name {
            fn from(bytes: [u8; $byte_count]) -> Self {
                Self(bytes)
            }
        }
    };
}

impl_hash_type!(Ed25519KeyHash, 28);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nonce_identity() {
        let orig = Nonce::new_identity();
        let deser = Nonce::deserialize(&mut Deserializer::from(std::io::Cursor::new(
            orig.to_bytes(),
        )))
        .unwrap();
        assert_eq!(orig.to_bytes(), deser.to_bytes());
    }

    #[test]
    fn nonce_hash() {
        let orig = Nonce::new_from_hash(vec![
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,
            24, 25, 26, 27, 28, 29, 30, 31,
        ])
        .unwrap();
        let deser = Nonce::deserialize(&mut Deserializer::from(std::io::Cursor::new(
            orig.to_bytes(),
        )))
        .unwrap();
        assert_eq!(orig.to_bytes(), deser.to_bytes());
    }

    #[test]
    fn xprv_128_test() {
        // art forum devote street sure rather head chuckle guard poverty release quote oak craft enemy
        let entropy = [
            0x0c, 0xcb, 0x74, 0xf3, 0x6b, 0x7d, 0xa1, 0x64, 0x9a, 0x81, 0x44, 0x67, 0x55, 0x22,
            0xd4, 0xd8, 0x09, 0x7c, 0x64, 0x12,
        ];
        let root_key = Bip32PrivateKey::from_bip39_entropy(&entropy, &[]);

        assert_eq!(hex::encode(&root_key.as_bytes()), "b8f2bece9bdfe2b0282f5bad705562ac996efb6af96b648f4445ec44f47ad95c10e3d72f26ed075422a36ed8585c745a0e1150bcceba2357d058636991f38a3791e248de509c070d812ab2fda57860ac876bc489192c1ef4ce253c197ee219a4");
        let xprv_128 = root_key.to_128_xprv();
        // test the 128 xprv is the right format
        assert_eq!(hex::encode(&xprv_128), "b8f2bece9bdfe2b0282f5bad705562ac996efb6af96b648f4445ec44f47ad95c10e3d72f26ed075422a36ed8585c745a0e1150bcceba2357d058636991f38a37cf76399a210de8720e9fa894e45e41e29ab525e30bc402801c076250d1585bcd91e248de509c070d812ab2fda57860ac876bc489192c1ef4ce253c197ee219a4");
        let root_key_copy = Bip32PrivateKey::from_128_xprv(&xprv_128).unwrap();

        // test converting to and back is equivalent to the identity function
        assert_eq!(root_key.to_bech32(), root_key_copy.to_bech32());
    }

    #[test]
    fn chaincode_gen() {
        // art forum devote street sure rather head chuckle guard poverty release quote oak craft enemy
        let entropy = [
            0x0c, 0xcb, 0x74, 0xf3, 0x6b, 0x7d, 0xa1, 0x64, 0x9a, 0x81, 0x44, 0x67, 0x55, 0x22,
            0xd4, 0xd8, 0x09, 0x7c, 0x64, 0x12,
        ];
        let root_key = Bip32PrivateKey::from_bip39_entropy(&entropy, &[]);

        let prv_chaincode = root_key.chaincode();
        assert_eq!(
            hex::encode(&prv_chaincode),
            "91e248de509c070d812ab2fda57860ac876bc489192c1ef4ce253c197ee219a4"
        );

        let pub_chaincode = root_key.to_public().chaincode();
        assert_eq!(
            hex::encode(&pub_chaincode),
            "91e248de509c070d812ab2fda57860ac876bc489192c1ef4ce253c197ee219a4"
        );
    }

    #[test]
    fn private_key_from_bech32() {
        let pk = PrivateKey::generate_ed25519().unwrap();
        let pk_ext = PrivateKey::generate_ed25519extended().unwrap();

        assert_eq!(
            PrivateKey::from_bech32(&pk.to_bech32()).unwrap().as_bytes(),
            pk.as_bytes(),
        );
        assert_eq!(
            PrivateKey::from_bech32(&pk_ext.to_bech32())
                .unwrap()
                .as_bytes(),
            pk_ext.as_bytes(),
        );

        let er = PrivateKey::from_bech32("qwe");
        assert!(er.is_err());
    }
}
