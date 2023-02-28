use cardano_embedded_sdk::api as embedano;
use cardano_embedded_sdk::bip::bip39::{dictionary, Entropy, Mnemonics};
use cardano_embedded_sdk::crypto as sdk_crypto;
use cardano_embedded_sdk::types::{TxId, XPrvKey, XPubKey};
use derivation_path::DerivationPath;
use rand::Rng;

pub fn sign_with_address_0(
    tx_id: &TxId,
    mnemonics: &str,
) -> (XPubKey, sdk_crypto::Ed25519Signature) {
    let mnemonics = Mnemonics::from_string(&dictionary::ENGLISH, mnemonics).unwrap();
    let entropy = Entropy::from_mnemonics(&mnemonics).unwrap();
    let password = b""; // todo: pass as argument
    let path: DerivationPath = "m/1852'/1815'/0'/0/0".parse().unwrap();
    let (_, pub_key) = embedano::derive_key_pair(&entropy, password, &path);
    let signature = embedano::sign_tx_id(tx_id, &entropy, password, &path);
    (pub_key, signature)
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
pub fn get_signed_sensor_data(mnemonics: &str) -> (u64, Vec<u8>) {
  let mnemonics = Mnemonics::from_string(&dictionary::ENGLISH, mnemonics).unwrap();
  let entropy = Entropy::from_mnemonics(&mnemonics).unwrap();
  let password = b""; // todo: pass as argument
  let path: DerivationPath = "m/1852'/1815'/0'/0/0".parse().unwrap();

  let mut rng = rand::thread_rng();
  let sensor_data: u64 = rng.gen();
  let sensor_data_b = sensor_data.to_ne_bytes();
  let signed_data = embedano::sign_data(&sensor_data_b, &entropy, password, &path);
  (sensor_data, signed_data.to_bytes())
}
