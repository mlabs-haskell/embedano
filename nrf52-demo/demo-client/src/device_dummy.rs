use cardano_embedded_sdk::api as embedano;
use cardano_embedded_sdk::bip::bip39::{dictionary, Entropy, Mnemonics};
use cardano_embedded_sdk::crypto as sdk_crypto;
use cardano_embedded_sdk::types::{TxId, XPubKey};
use derivation_path::DerivationPath;
use rand::Rng;

use crate::types::DeviceData;


pub struct DeviceDummy {
    entropy: Entropy,
}

impl DeviceDummy {
    pub fn init(mnemonics: &str) -> Self {
        let mnemonics = Mnemonics::from_string(&dictionary::ENGLISH, mnemonics)
            .expect("Cant parse mnemonics, initialization filed.");

        let entropy =
            Entropy::from_mnemonics(&mnemonics).expect("Cant parse entropy, initialization filed.");
        DeviceDummy { entropy: entropy }
    }

    pub fn sign_tx_id(
        &self,
        tx_id: &TxId,
        password: &str,
        derivation_path: &DerivationPath,
    ) -> sdk_crypto::Ed25519Signature {
        let password = password.as_bytes(); // todo: pass as argument
                                            // let path: DerivationPath = "m/1852'/1815'/0'/0/0".parse().unwrap();
        let signature = embedano::sign_tx_id(tx_id, &self.entropy, password, derivation_path);
        signature
    }

    pub fn get_pub_key(&self, password: &str, derivation_path: &DerivationPath) -> XPubKey {
        let (_, pub_key) =
            embedano::derive_key_pair(&self.entropy, password.as_bytes(), &derivation_path);
        pub_key
    }

    // returns random number as sensor data and bytes of signature
    pub fn get_signed_sensor_data(
        &self,
        password: &str,
        derivation_path: &DerivationPath,
    ) -> DeviceData {
        let mut rng = rand::thread_rng();
        let sensor_data: i32 = rng.gen();
        let sensor_data_b = sensor_data.to_ne_bytes();
        let signed_data = embedano::sign_data(
            &sensor_data_b,
            &self.entropy,
            password.as_bytes(),
            &derivation_path,
        );
        DeviceData {
            sensor_readings: sensor_data,
            signed_readings: signed_data.to_bytes(),
        }
    }
}

