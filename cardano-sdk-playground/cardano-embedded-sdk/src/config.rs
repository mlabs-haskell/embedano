//! Blockchain network specific config (ProtocolMagic)
//!
//! there are some settings that need to be set in order to guarantee
//! operability with the appropriate network or different option.
//!
use core::fmt;

/// this is the protocol magic number
///
/// it is meant to be used on some places in order to guarantee
/// incompatibility between forks, test network and the main-net.
///
/// # Default
///
/// The default value is set to the mainnet
///
/// ```
/// use cardano::config::{ProtocolMagic};
///
/// assert_eq!(ProtocolMagic::default(), ProtocolMagic::new(0x2D964A09));
/// ```
///
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
#[repr(C)]
pub struct ProtocolMagic(u32); // FIXME: should be i32
impl ProtocolMagic {
    #[deprecated]
    pub fn new(val: u32) -> Self {
        ProtocolMagic(val)
    }
}
impl fmt::Display for ProtocolMagic {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl ::core::ops::Deref for ProtocolMagic {
    type Target = u32;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<u32> for ProtocolMagic {
    fn from(v: u32) -> Self {
        ProtocolMagic(v)
    }
}
impl Default for ProtocolMagic {
    fn default() -> Self {
        ProtocolMagic::from(764824073)
    }
}
// TODO: cbor
// impl cbor_event::se::Serialize for ProtocolMagic {
//     fn serialize<'se, W: Write>(
//         &self,
//         serializer: &'se mut Serializer<W>,
//     ) -> cbor_event::Result<&'se mut Serializer<W>> {
//         serializer.write_unsigned_integer(self.0 as u64)
//     }
// }
// impl cbor_event::Deserialize for ProtocolMagic {
//     fn deserialize<R: BufRead>(reader: &mut Deserializer<R>) -> cbor_event::Result<Self> {
//         let v = reader.unsigned_integer()? as u32;
//         Ok(ProtocolMagic::from(v))
//     }
// }

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum NetworkMagic {
    NoMagic,
    Magic(u32), // FIXME: should by i32
}

impl From<ProtocolMagic> for NetworkMagic {
    fn from(pm: ProtocolMagic) -> Self {
        NetworkMagic::from(*pm)
    }
}

impl From<u32> for NetworkMagic {
    fn from(pm: u32) -> Self {
        // FIXME: is there a better way to determine whether to emit
        // NetworkMagic? There is a requiresNetworkMagic field in
        // lib/configuration.yaml, but not in the genesis data.
        if pm == *ProtocolMagic::default() || pm == 633343913 {
            NetworkMagic::NoMagic
        } else {
            NetworkMagic::Magic(pm)
        }
    }
}

/// Configuration for the wallet-crypto
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Config {
    pub protocol_magic: ProtocolMagic,
}
impl Config {
    pub fn new(protocol_magic: ProtocolMagic) -> Self {
        Config {
            protocol_magic: protocol_magic,
        }
    }
}
impl Default for Config {
    fn default() -> Self {
        Config::new(ProtocolMagic::default())
    }
}