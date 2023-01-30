//! Redeem keys
//!
//! The Redeem was a one off to bootstrap the initial funds of the blockchain.
//! You should not need to create new redeem keys unless you are starting
//! a new hardfork of the main network.
//!
//! On the **mainnet** you can use the redeem keys to claim redeem addresses.
//!

use core::{cmp, fmt, result};

use crate::util::hex;

use cryptoxide::ed25519;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum Error {
    InvalidPublicKeySize(usize),
    InvalidPrivateKeySize(usize),
    InvalidSignatureSize(usize),
    HexadecimalError(hex::Error),
}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Error::InvalidPublicKeySize(sz) => write!(
                f,
                "invalid PublicKey size, expected {} but received {} bytes.",
                PUBLICKEY_SIZE, sz
            ),
            &Error::InvalidPrivateKeySize(sz) => write!(
                f,
                "invalid PrivateKey size, expected {} but received {} bytes.",
                PRIVATEKEY_SIZE, sz
            ),
            &Error::InvalidSignatureSize(sz) => write!(
                f,
                "invalid Signature size, expected {} but received {} bytes.",
                SIGNATURE_SIZE, sz
            ),
            &Error::HexadecimalError(_) => write!(f, "Invalid hexadecimal"),
        }
    }
}
impl From<hex::Error> for Error {
    fn from(e: hex::Error) -> Error {
        Error::HexadecimalError(e)
    }
}
impl ::core::error::Error for Error {
    fn cause(&self) -> Option<&dyn (::core::error::Error)> {
        match self {
            Error::HexadecimalError(ref err) => Some(err),
            _ => None,
        }
    }
}

pub type Result<T> = result::Result<T, Error>;

pub const PUBLICKEY_SIZE: usize = 32;

#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Hash)]
pub struct PublicKey([u8; PUBLICKEY_SIZE]);
impl PublicKey {
    pub fn from_bytes(bytes: [u8; PUBLICKEY_SIZE]) -> Self {
        PublicKey(bytes)
    }

    pub fn from_slice(bytes: &[u8]) -> Result<Self> {
        if bytes.len() != PUBLICKEY_SIZE {
            return Err(Error::InvalidPublicKeySize(bytes.len()));
        }
        let mut buf = [0; PUBLICKEY_SIZE];
        buf[0..PUBLICKEY_SIZE].clone_from_slice(bytes);
        Ok(Self::from_bytes(buf))
    }

    pub fn from_hex(hex: &str) -> Result<Self> {
        let bytes = hex::decode(hex)?;
        Self::from_slice(&bytes)
    }

    pub fn verify(&self, signature: &Signature, bytes: &[u8]) -> bool {
        ed25519::verify(bytes, &self.0, signature.as_ref())
    }
}
impl AsRef<[u8]> for PublicKey {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}
impl fmt::Debug for PublicKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", hex::encode(self.as_ref()))
    }
}
impl fmt::Display for PublicKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", hex::encode(self.as_ref()))
    }
}

pub const PRIVATEKEY_SIZE: usize = 32;

#[derive(Clone)]
pub struct PrivateKey([u8; PRIVATEKEY_SIZE]);
impl fmt::Debug for PrivateKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", hex::encode(self.as_ref()))
    }
}
impl fmt::Display for PrivateKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", hex::encode(self.as_ref()))
    }
}
impl AsRef<[u8]> for PrivateKey {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}
impl PrivateKey {
    /// takes the given raw bytes and perform some modifications to normalize
    /// it properly to a Private Key.
    ///
    pub fn from_bytes(bytes: [u8; PRIVATEKEY_SIZE]) -> Self {
        PrivateKey(bytes)
    }

    pub fn from_slice(bytes: &[u8]) -> Result<Self> {
        if bytes.len() != PRIVATEKEY_SIZE {
            return Err(Error::InvalidPrivateKeySize(bytes.len()));
        }
        let mut buf = [0; PRIVATEKEY_SIZE];
        buf[0..PRIVATEKEY_SIZE].clone_from_slice(bytes);
        Ok(Self::from_bytes(buf))
    }

    pub fn from_hex(hex: &str) -> Result<Self> {
        let bytes = hex::decode(hex)?;
        Self::from_slice(&bytes)
    }

    pub fn generate(seed: &[u8]) -> Result<Self> {
        Self::from_slice(seed)
    }

    pub fn public(&self) -> PublicKey {
        let (_, pk) = ed25519::keypair(&self.0);
        PublicKey::from_bytes(pk)
    }

    pub fn sign(&self, bytes: &[u8]) -> Signature {
        let (sk, _) = ed25519::keypair(&self.0);
        Signature::from_bytes(ed25519::signature(bytes, &sk))
    }
}

pub const SIGNATURE_SIZE: usize = 64;

pub struct Signature([u8; SIGNATURE_SIZE]);
impl Signature {
    pub fn from_bytes(bytes: [u8; SIGNATURE_SIZE]) -> Self {
        Signature(bytes)
    }

    pub fn from_slice(bytes: &[u8]) -> Result<Self> {
        if bytes.len() != SIGNATURE_SIZE {
            return Err(Error::InvalidSignatureSize(bytes.len()));
        }
        let mut buf = [0; SIGNATURE_SIZE];
        buf[0..SIGNATURE_SIZE].clone_from_slice(bytes);
        Ok(Self::from_bytes(buf))
    }

    pub fn from_hex(hex: &str) -> Result<Self> {
        let bytes = hex::decode(hex)?;
        Self::from_slice(&bytes)
    }
}
impl Clone for Signature {
    fn clone(&self) -> Self {
        let mut bytes = [0; SIGNATURE_SIZE];
        bytes.copy_from_slice(&self.0);
        Signature(bytes)
    }
}
impl fmt::Debug for Signature {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", hex::encode(self.as_ref()))
    }
}
impl fmt::Display for Signature {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", hex::encode(self.as_ref()))
    }
}
impl AsRef<[u8]> for Signature {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}
impl PartialEq for Signature {
    fn eq(&self, other: &Self) -> bool {
        PartialEq::eq(&self.0[..], &other.0[..])
    }
}
impl Eq for Signature {}
impl PartialOrd for Signature {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        PartialOrd::partial_cmp(&self.0[..], &other.0[..])
    }
}
impl Ord for Signature {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        Ord::cmp(&self.0[..], &other.0[..])
    }
}

// ---------------------------------------------------------------------------
//                                      CBOR
// ---------------------------------------------------------------------------
// TODO: cbor
// impl cbor_event::se::Serialize for PublicKey {
//     fn serialize<'se, W: Write>(
//         &self,
//         serializer: &'se mut Serializer<W>,
//     ) -> cbor_event::Result<&'se mut Serializer<W>> {
//         serializer.write_bytes(self.0.as_ref())
//     }
// }
// impl cbor_event::de::Deserialize for PublicKey {
//     fn deserialize<R: BufRead>(reader: &mut Deserializer<R>) -> cbor_event::Result<Self> {
//         match PublicKey::from_slice(&reader.bytes()?) {
//             Ok(digest) => Ok(digest),
//             Err(Error::InvalidPublicKeySize(sz)) => {
//                 Err(cbor_event::Error::NotEnough(sz, PUBLICKEY_SIZE))
//             }
//             Err(err) => Err(cbor_event::Error::CustomError(format!(
//                 "unexpected error: {:?}",
//                 err
//             ))),
//         }
//     }
// }
//
// impl cbor_event::se::Serialize for PrivateKey {
//     fn serialize<'se, W: Write>(
//         &self,
//         serializer: &'se mut Serializer<W>,
//     ) -> cbor_event::Result<&'se mut Serializer<W>> {
//         serializer.write_bytes(self.0.as_ref())
//     }
// }
// impl cbor_event::de::Deserialize for PrivateKey {
//     fn deserialize<R: BufRead>(reader: &mut Deserializer<R>) -> cbor_event::Result<Self> {
//         match PrivateKey::from_slice(&reader.bytes()?) {
//             Ok(digest) => Ok(digest),
//             Err(Error::InvalidPrivateKeySize(sz)) => {
//                 Err(cbor_event::Error::NotEnough(sz, PRIVATEKEY_SIZE))
//             }
//             Err(err) => Err(cbor_event::Error::CustomError(format!(
//                 "unexpected error: {:?}",
//                 err
//             ))),
//         }
//     }
// }
//
// impl cbor_event::se::Serialize for Signature {
//     fn serialize<'se, W: Write>(
//         &self,
//         serializer: &'se mut Serializer<W>,
//     ) -> cbor_event::Result<&'se mut Serializer<W>> {
//         serializer.write_bytes(self.0.as_ref())
//     }
// }
// impl cbor_event::de::Deserialize for Signature {
//     fn deserialize<R: BufRead>(reader: &mut Deserializer<R>) -> cbor_event::Result<Self> {
//         match Signature::from_slice(&reader.bytes()?) {
//             Ok(digest) => Ok(digest),
//             Err(Error::InvalidSignatureSize(sz)) => {
//                 Err(cbor_event::Error::NotEnough(sz, SIGNATURE_SIZE))
//             }
//             Err(err) => Err(cbor_event::Error::CustomError(format!(
//                 "unexpected error: {:?}",
//                 err
//             ))),
//         }
//     }
// }


// FIXME: commented to be able to run our tests
// #[cfg(test)]
// mod tests {
//     use super::*;

//     use quickcheck::{Arbitrary, Gen};

//     impl Arbitrary for PublicKey {
//         fn arbitrary<G: Gen>(g: &mut G) -> Self {
//             PrivateKey::arbitrary(g).public()
//         }
//     }
//     impl Arbitrary for PrivateKey {
//         fn arbitrary<G: Gen>(g: &mut G) -> Self {
//             let mut seed = [0u8; PRIVATEKEY_SIZE];
//             for byte in seed.iter_mut() {
//                 *byte = u8::arbitrary(g);
//             }
//             PrivateKey::from_bytes(seed)
//         }
//     }

//     quickcheck! {
//         fn redeem_signature(stuff: (PrivateKey, Vec<u8>)) -> bool {
//             let (private_key, data) = stuff;
//             let public_key = private_key.public();
//             let signature = private_key.sign(&data);
//             public_key.verify(&signature, &data)
//         }
//     }
// }
