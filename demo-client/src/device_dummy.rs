use cardano_embedded_sdk::api as embedano;
use cardano_embedded_sdk::bip::bip39::{dictionary, Entropy, Mnemonics};
use cardano_embedded_sdk::crypto as sdk_crypto;
use cardano_embedded_sdk::types::{TxId, XPubKey};
use derivation_path::DerivationPath;
use rand::Rng;

pub struct DeviceData {
    pub sensor_readings: u64,
    pub signed_readings: Vec<u8>,
}

pub fn sign_tx_id(
    tx_id: &TxId,
    mnemonics: &str,
    password: &str,
    derivation_path: &DerivationPath,
) -> sdk_crypto::Ed25519Signature {
    let mnemonics = Mnemonics::from_string(&dictionary::ENGLISH, mnemonics).unwrap();
    let entropy = Entropy::from_mnemonics(&mnemonics).unwrap();
    let password = password.as_bytes(); // todo: pass as argument
                                        // let path: DerivationPath = "m/1852'/1815'/0'/0/0".parse().unwrap();
    let signature = embedano::sign_tx_id(tx_id, &entropy, password, derivation_path);
    signature
}

pub fn get_pub_key(mnemonics: &str, password: &str, derivation_path: &DerivationPath) -> XPubKey {
    let mnemonics = Mnemonics::from_string(&dictionary::ENGLISH, mnemonics).unwrap();
    let entropy = Entropy::from_mnemonics(&mnemonics).unwrap();
    let (_, pub_key) = embedano::derive_key_pair(&entropy, password.as_bytes(), &derivation_path);
    pub_key
}

// returns random number as sensor data and bytes of signature
pub fn get_signed_sensor_data(
    mnemonics: &str,
    password: &str,
    derivation_path: &DerivationPath,
) -> DeviceData {
    let mnemonics = Mnemonics::from_string(&dictionary::ENGLISH, mnemonics).unwrap();
    let entropy = Entropy::from_mnemonics(&mnemonics).unwrap();

    let mut rng = rand::thread_rng();
    let sensor_data: u64 = rng.gen();
    let sensor_data_b = sensor_data.to_ne_bytes();
    let signed_data = embedano::sign_data(
        &sensor_data_b,
        &entropy,
        password.as_bytes(),
        &derivation_path,
    );
    DeviceData {
        sensor_readings: sensor_data,
        signed_readings: signed_data.to_bytes(),
    }
}
