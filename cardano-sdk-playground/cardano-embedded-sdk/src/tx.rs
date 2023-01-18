//! Transaction types
//!
//! `TxoPointer` : Input
//! `TxOut` : Output
//! `Tx` : Input + Output
//! `TxInWitness`: Witness providing for TxoPointer (e.g. cryptographic signature)
//! `TxAux` : Signed Tx (Tx + Witness)
//!
use alloc::vec::Vec;
use core::fmt;

use crate::{
    address::{AddrType, Attributes, ExtendedAddr, SpendingData},
    config::ProtocolMagic,
    hash::Blake2b256,
    hdwallet::{Signature, XPrv, XPub, SIGNATURE_SIZE, XPUB_SIZE},
    redeem,
    tags::SigningTag,
};

// Transaction IDs are either a hash of the CBOR serialisation of a
// given Tx, or a hash of a redeem address.
pub type TxId = Blake2b256;

pub fn redeem_pubkey_to_txid(
    pubkey: &redeem::PublicKey,
    protocol_magic: ProtocolMagic,
) -> (TxId, ExtendedAddr) {
    let address = ExtendedAddr::new(
        AddrType::ATRedeem,
        SpendingData::RedeemASD(*pubkey),
        Attributes::new_bootstrap_era(None, protocol_magic.into()),
    );
    // TODO: cbor
    // let txid = Blake2b256::new(&cbor!(&address).unwrap());
    // (txid, address)
    todo!()
}


// type TODO = u8;
// type ValidatorScript = TODO;
// type RedeemerScript = TODO;

/// Provide a witness to a specific transaction, generally by revealing
/// all the hidden information from the tx and cryptographic signatures.
///
/// Witnesses are of types:
/// * PkWitness: a simple witness for a PubKeyASD type, which is composed
///              of the revealed XPub associated with the address and
///              the associated signature of the tx.
/// * ScriptWitness: a witness for ScriptASD.
/// * RedeemWitness: a witness for RedeemASD type, similar to PkWitness
///                  but for normal Public Key.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TxInWitness {
    /// signature of the `Tx` with the associated `XPub`
    /// the `XPub` is the public key set in the AddrSpendingData
    PkWitness(XPub, Signature<Tx>),
    // TODO: misha: we probably wont need anything but PkWitness
    // ScriptWitness(ValidatorScript, RedeemerScript),
    // RedeemWitness(redeem::PublicKey, redeem::Signature),
}
impl fmt::Display for TxInWitness {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl TxInWitness {
    /// this is used to create a fake signature useful for fee evaluation
    pub fn fake() -> Self {
        let fakesig = Signature::from_bytes([0u8; SIGNATURE_SIZE]);
        TxInWitness::PkWitness(XPub::from_bytes([0u8; XPUB_SIZE]), fakesig)
    }

    /// create a TxInWitness from a given private key `XPrv` for the given transaction id `TxId`.
    #[deprecated(note = "use new_extended_pk method instead")]
    pub fn new(protocol_magic: ProtocolMagic, key: &XPrv, txid: &TxId) -> Self {
        Self::new_extended_pk(protocol_magic, key, txid)
    }

    /// create a TxInWitness from a given private key `XPrv` for the given transaction id `TxId`.
    pub fn new_extended_pk(protocol_magic: ProtocolMagic, key: &XPrv, txid: &TxId) -> Self {
        let vec = Self::prepare_byte_to_sign(protocol_magic, SigningTag::Tx, txid);
        TxInWitness::PkWitness(key.public(), key.sign(&vec))
    }

    // TODO: misha: we probably wont need anything but PkWitness
    /// create a TxInWitness from a given Redeem key
    // pub fn new_redeem_pk(
    //     protocol_magic: ProtocolMagic,
    //     key: &redeem::PrivateKey,
    //     txid: &TxId,
    // ) -> Self {
    //     let vec = Self::prepare_byte_to_sign(protocol_magic, SigningTag::RedeemTx, txid);
    //     TxInWitness::RedeemWitness(key.public(), key.sign(&vec))
    // }

    fn prepare_byte_to_sign(
        protocol_magic: ProtocolMagic,
        sign_tag: SigningTag,
        txid: &TxId,
    ) -> Vec<u8> {
        // let mut se = Serializer::new_vec();
        // se.write_unsigned_integer(sign_tag as u64)
        //     .expect("write the sign tag")
        //     .serialize(&protocol_magic)
        //     .expect("serialize protocol magic")
        //     .serialize(txid)
        //     .expect("serialize Tx's Id");
        // se.finalize()
        todo!()
    }

    /// verify a given extended address is associated to the witness.
    ///
    pub fn verify_address(&self, address: &ExtendedAddr) -> bool {
        match self {
            &TxInWitness::PkWitness(ref pk, _) => {
                let sd = SpendingData::PubKeyASD(pk.clone());
                let ea = ExtendedAddr::new(address.addr_type, sd, address.attributes.clone());

                &ea == address
            }
            // TODO: misha: we probably wont need anything but PkWitness
            // &TxInWitness::ScriptWitness(_, _) => unimplemented!(),
            // &TxInWitness::RedeemWitness(ref pk, _) => {
            //     let sd = SpendingData::RedeemASD(pk.clone());
            //     let ea = ExtendedAddr::new(address.addr_type, sd, address.attributes.clone());

            //     &ea == address
            // }
        }
    }

    fn get_sign_tag(&self) -> SigningTag {
        match self {
            &TxInWitness::PkWitness(_, _) => SigningTag::Tx,
            // &TxInWitness::ScriptWitness(_, _) => unimplemented!(),
            // &TxInWitness::RedeemWitness(_, _) => SigningTag::RedeemTx,
        }
    }

    
}
// TODO: cbor
// impl cbor_event::se::Serialize for TxInWitness {
//     fn serialize<'se, W: Write>(
//         &self,
//         serializer: &'se mut Serializer<W>,
//     ) -> cbor_event::Result<&'se mut Serializer<W>> {
//         serializer.write_array(cbor_event::Len::Len(2))?;
//         let inner_serializer = match self {
//             &TxInWitness::PkWitness(ref xpub, ref signature) => {
//                 serializer.write_unsigned_integer(0)?;
//                 let mut se = Serializer::new_vec();
//                 se.write_array(cbor_event::Len::Len(2))?
//                     .serialize(xpub)?
//                     .serialize(signature)?;
//                 se
//             }
//             &TxInWitness::ScriptWitness(_, _) => unimplemented!(),
//             &TxInWitness::RedeemWitness(ref pk, ref signature) => {
//                 serializer.write_unsigned_integer(2)?;
//                 let mut se = Serializer::new_vec();
//                 se.write_array(cbor_event::Len::Len(2))?
//                     .serialize(pk)?
//                     .serialize(signature)?;
//                 se
//             }
//         };
//         serializer
//             .write_tag(24)?
//             .write_bytes(&inner_serializer.finalize())
//     }
// }
// impl cbor_event::de::Deserialize for TxInWitness {
//     fn deserialize<R: BufRead>(raw: &mut Deserializer<R>) -> cbor_event::Result<Self> {
//         raw.tuple(2, "TxInWitness")?;
//         let sum_type_idx = raw.unsigned_integer()?;
//         match sum_type_idx {
//             0 => {
//                 let tag = raw.tag()?;
//                 if tag != 24 {
//                     return Err(cbor_event::Error::CustomError(format!(
//                         "Invalid Tag: {} but expected 24",
//                         tag
//                     )));
//                 }
//                 let bytes = raw.bytes()?;
//                 let mut raw = Deserializer::from(std::io::Cursor::new(bytes));
//                 raw.tuple(2, "TxInWitness::PkWitness")?;
//                 let pk = cbor_event::de::Deserialize::deserialize(&mut raw)?;
//                 let sig = cbor_event::de::Deserialize::deserialize(&mut raw)?;
//                 Ok(TxInWitness::PkWitness(pk, sig))
//             }
//             2 => {
//                 let tag = raw.tag()?;
//                 if tag != 24 {
//                     return Err(cbor_event::Error::CustomError(format!(
//                         "Invalid Tag: {} but expected 24",
//                         tag
//                     )));
//                 }
//                 let bytes = raw.bytes()?;
//                 let mut raw = Deserializer::from(std::io::Cursor::new(bytes));
//                 raw.tuple(2, "TxInWitness::PkRedeemWitness")?;
//                 let pk = cbor_event::de::Deserialize::deserialize(&mut raw)?;
//                 let sig = cbor_event::de::Deserialize::deserialize(&mut raw)?;
//                 Ok(TxInWitness::RedeemWitness(pk, sig))
//             }
//             _ => Err(cbor_event::Error::CustomError(format!(
//                 "Unsupported TxInWitness: {}",
//                 sum_type_idx
//             ))),
//         }
//     }
// }

/// Structure used for addressing a specific output of a transaction
/// built from a TxId (hash of the tx) and the offset in the outputs of this
/// transaction.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct TxoPointer {
    pub id: TxId,
    pub index: u32,
}

/// old haskell name for TxoPointer
#[deprecated]
pub type TxIn = TxoPointer;

impl fmt::Display for TxoPointer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}@{}", self.id, self.index)
    }
}
impl TxoPointer {
    pub fn new(id: TxId, index: u32) -> Self {
        TxoPointer {
            id: id,
            index: index,
        }
    }
}
// TODO: cbor
// impl cbor_event::se::Serialize for TxoPointer {
//     fn serialize<'se, W: Write>(
//         &self,
//         serializer: &'se mut Serializer<W>,
//     ) -> cbor_event::Result<&'se mut Serializer<W>> {
//         serializer
//             .write_array(cbor_event::Len::Len(2))?
//             .write_unsigned_integer(0)?
//             .write_tag(24)?
//             .write_bytes(&cbor!(&(&self.id, &self.index))?)
//     }
// }
// impl cbor_event::de::Deserialize for TxoPointer {
//     fn deserialize<R: BufRead>(raw: &mut Deserializer<R>) -> cbor_event::Result<Self> {
//         raw.tuple(2, "TxoPointer")?;
//         let sum_type_idx = raw.unsigned_integer()?;
//         if sum_type_idx != 0 {
//             return Err(cbor_event::Error::CustomError(format!(
//                 "Unsupported TxoPointer: {}",
//                 sum_type_idx
//             )));
//         }
//         let tag = raw.tag()?;
//         if tag != 24 {
//             return Err(cbor_event::Error::CustomError(format!(
//                 "Invalid Tag: {} but expected 24",
//                 tag
//             )));
//         }
//         let bytes = raw.bytes()?;
//         let mut raw = Deserializer::from(std::io::Cursor::new(bytes));
//         raw.tuple(2, "TxoPointer")?;
//         let id = cbor_event::de::Deserialize::deserialize(&mut raw)?;
//         let idx = raw.unsigned_integer()?;
//         Ok(TxoPointer::new(id, idx as u32))
//     }
// }

/// A Transaction containing tx inputs and tx outputs.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Tx {
}

impl fmt::Display for Tx {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        todo!()
    }
}

/// A transaction witness is a vector of input witnesses
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TxWitness(Vec<TxInWitness>);

impl TxWitness {
    pub fn new() -> Self {
        TxWitness(Vec::new())
    }
}
impl From<Vec<TxInWitness>> for TxWitness {
    fn from(v: Vec<TxInWitness>) -> Self {
        TxWitness(v)
    }
}
impl ::core::iter::FromIterator<TxInWitness> for TxWitness {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = TxInWitness>,
    {
        TxWitness(Vec::from_iter(iter))
    }
}
impl ::core::ops::Deref for TxWitness {
    type Target = Vec<TxInWitness>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ::core::ops::DerefMut for TxWitness {
    fn deref_mut(&mut self) -> &mut Vec<TxInWitness> {
        &mut self.0
    }
}
// TODO: cbor
// impl cbor_event::de::Deserialize for TxWitness {
//     fn deserialize<R: BufRead>(raw: &mut Deserializer<R>) -> cbor_event::Result<Self> {
//         Ok(TxWitness(cbor_event::de::Deserialize::deserialize(raw)?))
//     }
// }
//
// impl cbor_event::se::Serialize for TxWitness {
//     fn serialize<'se, W: Write>(
//         &self,
//         serializer: &'se mut Serializer<W>,
//     ) -> cbor_event::Result<&'se mut Serializer<W>> {
//         txwitness_serialize(&self.0, serializer)
//     }
// }
//
// pub fn txwitness_serialize<'se, W>(
//     in_witnesses: &Vec<TxInWitness>,
//     serializer: &'se mut Serializer<W>,
// ) -> cbor_event::Result<&'se mut Serializer<W>>
// where
//     W: Write,
// {
//     cbor_event::se::serialize_fixed_array(in_witnesses.iter(), serializer)
// }

/// A transaction witness is a vector of input witnesses
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TxWitnesses {
    pub in_witnesses: Vec<TxWitness>,
}

impl TxWitnesses {
    pub fn new(in_witnesses: Vec<TxWitness>) -> Self {
        TxWitnesses {
            in_witnesses: in_witnesses,
        }
    }
}

impl ::core::ops::Deref for TxWitnesses {
    type Target = Vec<TxWitness>;
    fn deref(&self) -> &Self::Target {
        &self.in_witnesses
    }
}
// TODO: cbor
// impl cbor_event::se::Serialize for TxWitnesses {
//     fn serialize<'se, W: Write>(
//         &self,
//         serializer: &'se mut Serializer<W>,
//     ) -> cbor_event::Result<&'se mut Serializer<W>> {
//         cbor_event::se::serialize_indefinite_array(self.iter(), serializer)
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;
    use address;
    use cbor_event::{self, de::Deserializer};
    use config::NetworkMagic;
    use hdpayload;
    use hdwallet;

    const SEED: [u8; hdwallet::SEED_SIZE] = [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0,
    ];

    const HDPAYLOAD: &'static [u8] = &[1, 2, 3, 4, 5];

    // CBOR encoded TxOut
    const TX_OUT: &'static [u8] = &[
        0x82, 0x82, 0xd8, 0x18, 0x58, 0x29, 0x83, 0x58, 0x1c, 0x83, 0xee, 0xa1, 0xb5, 0xec, 0x8e,
        0x80, 0x26, 0x65, 0x81, 0x46, 0x4a, 0xee, 0x0e, 0x2d, 0x6a, 0x45, 0xfd, 0x6d, 0x7b, 0x9e,
        0x1a, 0x98, 0x3a, 0x50, 0x48, 0xcd, 0x15, 0xa1, 0x01, 0x46, 0x45, 0x01, 0x02, 0x03, 0x04,
        0x05, 0x00, 0x1a, 0x9d, 0x45, 0x88, 0x4a, 0x18, 0x2a,
    ];
    const TX_IN: &'static [u8] = &[
        0x82, 0x00, 0xd8, 0x18, 0x58, 0x26, 0x82, 0x58, 0x20, 0xaa, 0xd7, 0x8a, 0x13, 0xb5, 0x0a,
        0x01, 0x4a, 0x24, 0x63, 0x3c, 0x7d, 0x44, 0xfd, 0x8f, 0x8d, 0x18, 0xf6, 0x7b, 0xbb, 0x3f,
        0xa9, 0xcb, 0xce, 0xdf, 0x83, 0x4a, 0xc8, 0x99, 0x75, 0x9d, 0xcd, 0x19, 0x02, 0x9a,
    ];

    const TX: &'static [u8] = &[
        0x83, 0x9f, 0x82, 0x00, 0xd8, 0x18, 0x58, 0x26, 0x82, 0x58, 0x20, 0xaa, 0xd7, 0x8a, 0x13,
        0xb5, 0x0a, 0x01, 0x4a, 0x24, 0x63, 0x3c, 0x7d, 0x44, 0xfd, 0x8f, 0x8d, 0x18, 0xf6, 0x7b,
        0xbb, 0x3f, 0xa9, 0xcb, 0xce, 0xdf, 0x83, 0x4a, 0xc8, 0x99, 0x75, 0x9d, 0xcd, 0x19, 0x02,
        0x9a, 0xff, 0x9f, 0x82, 0x82, 0xd8, 0x18, 0x58, 0x29, 0x83, 0x58, 0x1c, 0x83, 0xee, 0xa1,
        0xb5, 0xec, 0x8e, 0x80, 0x26, 0x65, 0x81, 0x46, 0x4a, 0xee, 0x0e, 0x2d, 0x6a, 0x45, 0xfd,
        0x6d, 0x7b, 0x9e, 0x1a, 0x98, 0x3a, 0x50, 0x48, 0xcd, 0x15, 0xa1, 0x01, 0x46, 0x45, 0x01,
        0x02, 0x03, 0x04, 0x05, 0x00, 0x1a, 0x9d, 0x45, 0x88, 0x4a, 0x18, 0x2a, 0xff, 0xa0,
    ];

    const TX_IN_WITNESS: &'static [u8] = &[
        0x82, 0x00, 0xd8, 0x18, 0x58, 0x85, 0x82, 0x58, 0x40, 0x1c, 0x0c, 0x3a, 0xe1, 0x82, 0x5e,
        0x90, 0xb6, 0xdd, 0xda, 0x3f, 0x40, 0xa1, 0x22, 0xc0, 0x07, 0xe1, 0x00, 0x8e, 0x83, 0xb2,
        0xe1, 0x02, 0xc1, 0x42, 0xba, 0xef, 0xb7, 0x21, 0xd7, 0x2c, 0x1a, 0x5d, 0x36, 0x61, 0xde,
        0xb9, 0x06, 0x4f, 0x2d, 0x0e, 0x03, 0xfe, 0x85, 0xd6, 0x80, 0x70, 0xb2, 0xfe, 0x33, 0xb4,
        0x91, 0x60, 0x59, 0x65, 0x8e, 0x28, 0xac, 0x7f, 0x7f, 0x91, 0xca, 0x4b, 0x12, 0x58, 0x40,
        0x9d, 0x6d, 0x91, 0x1e, 0x58, 0x8d, 0xd4, 0xfb, 0x77, 0xcb, 0x80, 0xc2, 0xc6, 0xad, 0xbc,
        0x2b, 0x94, 0x2b, 0xce, 0xa5, 0xd8, 0xa0, 0x39, 0x22, 0x0d, 0xdc, 0xd2, 0x35, 0xcb, 0x75,
        0x86, 0x2c, 0x0c, 0x95, 0xf6, 0x2b, 0xa1, 0x11, 0xe5, 0x7d, 0x7c, 0x1a, 0x22, 0x1c, 0xf5,
        0x13, 0x3e, 0x44, 0x12, 0x88, 0x32, 0xc1, 0x49, 0x35, 0x4d, 0x1e, 0x57, 0xb6, 0x80, 0xfe,
        0x57, 0x2d, 0x76, 0x0c,
    ];

    const TX_AUX: &'static [u8] = &[
        0x82, 0x83, 0x9f, 0x82, 0x00, 0xd8, 0x18, 0x58, 0x26, 0x82, 0x58, 0x20, 0xaa, 0xd7, 0x8a,
        0x13, 0xb5, 0x0a, 0x01, 0x4a, 0x24, 0x63, 0x3c, 0x7d, 0x44, 0xfd, 0x8f, 0x8d, 0x18, 0xf6,
        0x7b, 0xbb, 0x3f, 0xa9, 0xcb, 0xce, 0xdf, 0x83, 0x4a, 0xc8, 0x99, 0x75, 0x9d, 0xcd, 0x19,
        0x02, 0x9a, 0xff, 0x9f, 0x82, 0x82, 0xd8, 0x18, 0x58, 0x29, 0x83, 0x58, 0x1c, 0x83, 0xee,
        0xa1, 0xb5, 0xec, 0x8e, 0x80, 0x26, 0x65, 0x81, 0x46, 0x4a, 0xee, 0x0e, 0x2d, 0x6a, 0x45,
        0xfd, 0x6d, 0x7b, 0x9e, 0x1a, 0x98, 0x3a, 0x50, 0x48, 0xcd, 0x15, 0xa1, 0x01, 0x46, 0x45,
        0x01, 0x02, 0x03, 0x04, 0x05, 0x00, 0x1a, 0x9d, 0x45, 0x88, 0x4a, 0x18, 0x2a, 0xff, 0xa0,
        0x81, 0x82, 0x00, 0xd8, 0x18, 0x58, 0x85, 0x82, 0x58, 0x40, 0x1c, 0x0c, 0x3a, 0xe1, 0x82,
        0x5e, 0x90, 0xb6, 0xdd, 0xda, 0x3f, 0x40, 0xa1, 0x22, 0xc0, 0x07, 0xe1, 0x00, 0x8e, 0x83,
        0xb2, 0xe1, 0x02, 0xc1, 0x42, 0xba, 0xef, 0xb7, 0x21, 0xd7, 0x2c, 0x1a, 0x5d, 0x36, 0x61,
        0xde, 0xb9, 0x06, 0x4f, 0x2d, 0x0e, 0x03, 0xfe, 0x85, 0xd6, 0x80, 0x70, 0xb2, 0xfe, 0x33,
        0xb4, 0x91, 0x60, 0x59, 0x65, 0x8e, 0x28, 0xac, 0x7f, 0x7f, 0x91, 0xca, 0x4b, 0x12, 0x58,
        0x40, 0x9d, 0x6d, 0x91, 0x1e, 0x58, 0x8d, 0xd4, 0xfb, 0x77, 0xcb, 0x80, 0xc2, 0xc6, 0xad,
        0xbc, 0x2b, 0x94, 0x2b, 0xce, 0xa5, 0xd8, 0xa0, 0x39, 0x22, 0x0d, 0xdc, 0xd2, 0x35, 0xcb,
        0x75, 0x86, 0x2c, 0x0c, 0x95, 0xf6, 0x2b, 0xa1, 0x11, 0xe5, 0x7d, 0x7c, 0x1a, 0x22, 0x1c,
        0xf5, 0x13, 0x3e, 0x44, 0x12, 0x88, 0x32, 0xc1, 0x49, 0x35, 0x4d, 0x1e, 0x57, 0xb6, 0x80,
        0xfe, 0x57, 0x2d, 0x76, 0x0c,
    ];

    #[test]
    fn txin_decode() {
        let mut raw = Deserializer::from(std::io::Cursor::new(TX_IN));
        let txo: TxoPointer = cbor_event::de::Deserialize::deserialize(&mut raw).unwrap();

        assert!(txo.index == 666);
    }

    #[test]
    fn txin_encode_decode() {
        let txid = TxId::new(&[0; 32]);
        assert!(cbor_event::test_encode_decode(&TxoPointer::new(txid, 666)).unwrap());
    }

    #[test]
    fn txinwitness_decode() {
        let protocol_magic = ProtocolMagic::default();
        let mut raw = Deserializer::from(std::io::Cursor::new(TX));
        let tx: Tx = raw.deserialize().expect("to decode a `Tx`");
        let mut raw = Deserializer::from(std::io::Cursor::new(TX_IN_WITNESS));
        let txinwitness: TxInWitness = raw.deserialize().expect("TxInWitness");

        let seed = hdwallet::Seed::from_bytes(SEED);
        let sk = hdwallet::XPrv::generate_from_seed(&seed);

        assert_eq!(
            txinwitness,
            TxInWitness::new_extended_pk(protocol_magic, &sk, &tx.id())
        );
    }

    #[test]
    fn txinwitness_encode_decode() {
        let protocol_magic = ProtocolMagic::default();
        let mut raw = Deserializer::from(std::io::Cursor::new(TX));
        let tx: Tx = raw.deserialize().expect("to decode a `Tx`");

        let seed = hdwallet::Seed::from_bytes(SEED);
        let sk = hdwallet::XPrv::generate_from_seed(&seed);

        let txinwitness = TxInWitness::new_extended_pk(protocol_magic, &sk, &tx.id());

        assert!(cbor_event::test_encode_decode(&txinwitness).expect("encode/decode TxInWitness"));
    }

    #[test]
    fn txinwitness_sign_verify() {
        let protocol_magic = ProtocolMagic::default();
        // create wallet's keys
        let seed = hdwallet::Seed::from_bytes(SEED);
        let sk = hdwallet::XPrv::generate_from_seed(&seed);
        let pk = sk.public();

        // create an Address
        let hdap = hdpayload::HDAddressPayload::from_bytes(HDPAYLOAD);
        let addr_type = address::AddrType::ATPubKey;
        let sd = address::SpendingData::PubKeyASD(pk.clone());
        let attrs = address::Attributes::new_single_key(&pk, Some(hdap), protocol_magic.into());
        let ea = address::ExtendedAddr::new(addr_type, sd, attrs);

        // create a transaction
        let txid = TxId::new(&[0; 32]);
        let txo = TxoPointer::new(txid, 666);
        let value = Coin::new(42).unwrap();
        let txout = TxOut::new(ea.clone(), value);
        let mut tx = Tx::new();
        tx.add_input(txo);
        tx.add_output(txout);

        // here we pretend that `ea` is the address we find from the found we want
        // to take. In the testing case, it is not important that it is also the
        // txout of this given transation

        // create a TxInWitness (i.e. sign the given transaction)
        let txinwitness = TxInWitness::new_extended_pk(protocol_magic, &sk, &tx.id());

        // check the address is the correct one
        assert!(txinwitness.verify_address(&ea));
        assert!(txinwitness.verify_tx(protocol_magic, &tx));
        assert!(txinwitness.verify(protocol_magic, &ea, &tx));
    }

    #[test]
    fn txaux_decode() {
        let mut raw = Deserializer::from(std::io::Cursor::new(TX_AUX));
        let _txaux: TxAux = raw.deserialize().expect("to decode a TxAux");
        let mut raw = Deserializer::from(std::io::Cursor::new(TX_AUX));
        let _txaux: TxAux = cbor_event::de::Deserialize::deserialize(&mut raw).unwrap();
    }

    #[test]
    fn txaux_encode_decode() {
        let mut raw = Deserializer::from(std::io::Cursor::new(TX));
        let tx: Tx = raw.deserialize().expect("to decode a `Tx`");
        let mut raw = Deserializer::from(std::io::Cursor::new(TX_IN_WITNESS));
        let txinwitness: TxInWitness = raw.deserialize().expect("to decode a `TxInWitness`");

        let txaux = TxAux::new(tx, TxWitness::from(vec![txinwitness]));

        assert!(cbor_event::test_encode_decode(&txaux).expect("encode/decode TxAux"));
    }
}

#[cfg(feature = "with-bench")]
#[cfg(test)]
mod bench {
    use super::*;
    use cbor_event::de::RawCbor;
    use test;

    const TX_AUX: &'static [u8] = &[
        0x82, 0x83, 0x9f, 0x82, 0x00, 0xd8, 0x18, 0x58, 0x26, 0x82, 0x58, 0x20, 0xaa, 0xd7, 0x8a,
        0x13, 0xb5, 0x0a, 0x01, 0x4a, 0x24, 0x63, 0x3c, 0x7d, 0x44, 0xfd, 0x8f, 0x8d, 0x18, 0xf6,
        0x7b, 0xbb, 0x3f, 0xa9, 0xcb, 0xce, 0xdf, 0x83, 0x4a, 0xc8, 0x99, 0x75, 0x9d, 0xcd, 0x19,
        0x02, 0x9a, 0xff, 0x9f, 0x82, 0x82, 0xd8, 0x18, 0x58, 0x29, 0x83, 0x58, 0x1c, 0x83, 0xee,
        0xa1, 0xb5, 0xec, 0x8e, 0x80, 0x26, 0x65, 0x81, 0x46, 0x4a, 0xee, 0x0e, 0x2d, 0x6a, 0x45,
        0xfd, 0x6d, 0x7b, 0x9e, 0x1a, 0x98, 0x3a, 0x50, 0x48, 0xcd, 0x15, 0xa1, 0x01, 0x46, 0x45,
        0x01, 0x02, 0x03, 0x04, 0x05, 0x00, 0x1a, 0x9d, 0x45, 0x88, 0x4a, 0x18, 0x2a, 0xff, 0xa0,
        0x81, 0x82, 0x00, 0xd8, 0x18, 0x58, 0x85, 0x82, 0x58, 0x40, 0x1c, 0x0c, 0x3a, 0xe1, 0x82,
        0x5e, 0x90, 0xb6, 0xdd, 0xda, 0x3f, 0x40, 0xa1, 0x22, 0xc0, 0x07, 0xe1, 0x00, 0x8e, 0x83,
        0xb2, 0xe1, 0x02, 0xc1, 0x42, 0xba, 0xef, 0xb7, 0x21, 0xd7, 0x2c, 0x1a, 0x5d, 0x36, 0x61,
        0xde, 0xb9, 0x06, 0x4f, 0x2d, 0x0e, 0x03, 0xfe, 0x85, 0xd6, 0x80, 0x70, 0xb2, 0xfe, 0x33,
        0xb4, 0x91, 0x60, 0x59, 0x65, 0x8e, 0x28, 0xac, 0x7f, 0x7f, 0x91, 0xca, 0x4b, 0x12, 0x58,
        0x40, 0x9d, 0x6d, 0x91, 0x1e, 0x58, 0x8d, 0xd4, 0xfb, 0x77, 0xcb, 0x80, 0xc2, 0xc6, 0xad,
        0xbc, 0x2b, 0x94, 0x2b, 0xce, 0xa5, 0xd8, 0xa0, 0x39, 0x22, 0x0d, 0xdc, 0xd2, 0x35, 0xcb,
        0x75, 0x86, 0x2c, 0x0c, 0x95, 0xf6, 0x2b, 0xa1, 0x11, 0xe5, 0x7d, 0x7c, 0x1a, 0x22, 0x1c,
        0xf5, 0x13, 0x3e, 0x44, 0x12, 0x88, 0x32, 0xc1, 0x49, 0x35, 0x4d, 0x1e, 0x57, 0xb6, 0x80,
        0xfe, 0x57, 0x2d, 0x76, 0x0c,
    ];

    #[bench]
    fn encode_txaux_cbor_raw(b: &mut test::Bencher) {
        let mut raw = cbor_event::de::RawCbor::from(TX_AUX);
        let txaux: TxAux = cbor_event::de::Deserialize::deserialize(&mut raw).unwrap();
        b.iter(|| {
            let _ = cbor!(txaux).unwrap();
        })
    }
    #[bench]
    fn decode_txaux_cbor_raw(b: &mut test::Bencher) {
        b.iter(|| {
            let _: TxAux = RawCbor::from(TX_AUX).deserialize().unwrap();
        })
    }
}
