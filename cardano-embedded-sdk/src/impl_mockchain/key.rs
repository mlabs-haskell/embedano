//! Module provides cryptographic utilities and types related to
//! the user keys.
//!
use crate::chain_crypto as crypto;
use crate::chain_crypto::SecretKey;
use rand_core::{CryptoRng, RngCore};

#[derive(Clone)]
pub enum EitherEd25519SecretKey {
    Extended(crypto::SecretKey<crypto::Ed25519Extended>),
    Normal(crypto::SecretKey<crypto::Ed25519>),
}

impl EitherEd25519SecretKey {
    pub fn generate<R: RngCore + CryptoRng>(rng: R) -> Self {
        EitherEd25519SecretKey::Extended(SecretKey::generate(rng))
    }

    pub fn to_public(&self) -> crypto::PublicKey<crypto::Ed25519> {
        match self {
            EitherEd25519SecretKey::Extended(sk) => sk.to_public(),
            EitherEd25519SecretKey::Normal(sk) => sk.to_public(),
        }
    }

    pub fn sign<T: AsRef<[u8]>>(&self, dat: &T) -> crypto::Signature<T, crypto::Ed25519> {
        match self {
            EitherEd25519SecretKey::Extended(sk) => sk.sign(dat),
            EitherEd25519SecretKey::Normal(sk) => sk.sign(dat),
        }
    }

    pub fn sign_slice<T: ?Sized>(&self, dat: &[u8]) -> crypto::Signature<T, crypto::Ed25519> {
        match self {
            EitherEd25519SecretKey::Extended(sk) => sk.sign_slice(dat),
            EitherEd25519SecretKey::Normal(sk) => sk.sign_slice(dat),
        }
    }
}

pub type Ed25519Signature<T> = crypto::Signature<T, crypto::Ed25519>;
