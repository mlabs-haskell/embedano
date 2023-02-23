use core::time::Duration;

use cardano_embedded_sdk::api as embedano;
use cardano_embedded_sdk::bip::bip39::{dictionary, Entropy, Mnemonics};
use cardano_embedded_sdk::crypto::Ed25519Signature;
use cardano_embedded_sdk::types::TxId;
use derivation_path::DerivationPath;

fn main() {
    let mnemonics = "aim wool into nose tell ball arm expand design push elevator multiply glove lonely minimum";
    let mnemonics = Mnemonics::from_string(&dictionary::ENGLISH, mnemonics).unwrap();
    let password = b"embedano";
    let entropy = Entropy::from_mnemonics(&mnemonics).unwrap();
    let path: DerivationPath = "m/1852'/1815'/0'/0/0".parse().unwrap();
    let (_prv_key, pub_key) = embedano::derive_key_pair(&entropy, password, &path);

    let tx_id =
        TxId::from_hex("bb1eb401cd03b0cd8caa08997df0a2ab226772c4d3a08adfb5a60ba34de12dfb").unwrap();

    let ports = serialport::available_ports().expect("No ports found!");
    for p in ports {
        println!("{}", p.port_name);
    }

    let mut port = serialport::new("/dev/tty.usbmodemTEST1", 115_200)
        .timeout(Duration::from_millis(100000))
        .open()
        .expect("Failed to open port");

    port.write(tx_id.to_bytes()).expect("Write failed!");

    let mut buf: Vec<u8> = vec![0; 64];
    port.read(buf.as_mut_slice()).expect("Found no data!");

    let signature = Ed25519Signature::from_bytes(buf).expect("Decode signature failed!");

    println!(
        "Signature: {}, verified: {}",
        signature.to_hex(),
        pub_key.verify(tx_id.to_bytes(), &signature)
    );
}
