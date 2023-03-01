use cardano_embedded_sdk::api as embedano;
use cardano_embedded_sdk::bip::bip39::{dictionary, Entropy, Mnemonics};
use cardano_embedded_sdk::crypto as sdk_crypto;
use cardano_embedded_sdk::types::{TxId, XPrvKey, XPubKey};
use derivation_path::DerivationPath;
use rand::Rng;

pub struct DeviceData {
    pub sensor_readings: u64,
    pub signed_readings: Vec<u8>,
}

pub fn sign_with_address_0(
    tx_id: &TxId,
    mnemonics: &str,
) -> sdk_crypto::Ed25519Signature {
    let mnemonics = Mnemonics::from_string(&dictionary::ENGLISH, mnemonics).unwrap();
    let entropy = Entropy::from_mnemonics(&mnemonics).unwrap();
    let password = b""; // todo: pass as argument
    let path: DerivationPath = "m/1852'/1815'/0'/0/0".parse().unwrap();
    let signature = embedano::sign_tx_id(tx_id, &entropy, password, &path);
    signature
}

pub fn get_addr_0_pub_key(mnemonics: &str) -> XPubKey {
    let mnemonics = Mnemonics::from_string(&dictionary::ENGLISH, mnemonics).unwrap();
    let entropy = Entropy::from_mnemonics(&mnemonics).unwrap();
    let password = b""; // todo: pass as argument
    let path: DerivationPath = "m/1852'/1815'/0'/0/0".parse().unwrap();
    let (_, pub_key) = embedano::derive_key_pair(&entropy, password, &path);
    pub_key
}

// returns random number as sensor data and bytes of signature
pub fn get_signed_sensor_data(mnemonics: &str) -> DeviceData {
    let mnemonics = Mnemonics::from_string(&dictionary::ENGLISH, mnemonics).unwrap();
    let entropy = Entropy::from_mnemonics(&mnemonics).unwrap();
    let password = b""; // todo: pass as argument
    let path: DerivationPath = "m/1852'/1815'/0'/0/0".parse().unwrap();

    let mut rng = rand::thread_rng();
    let sensor_data: u64 = rng.gen();
    let sensor_data_b = sensor_data.to_ne_bytes();
    let signed_data = embedano::sign_data(&sensor_data_b, &entropy, password, &path);
    DeviceData {
        sensor_readings: sensor_data,
        signed_readings: signed_data.to_bytes(),
    }
}
