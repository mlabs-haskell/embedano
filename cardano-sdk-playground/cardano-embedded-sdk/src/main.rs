use cardano_embedded_sdk::hdwallet::Signature;
use cardano_embedded_sdk::{bip::bip39, hdwallet, tx, wallet::bip44};

use cardano_serialization_lib::crypto::Bip32PrivateKey;

fn main() {
    let mnemonics_phrase = bip39::MnemonicString::new(
        &bip39::dictionary::ENGLISH,
        "all all all all all all all all all all all all".to_string(),
    )
    .unwrap();
    let seed = bip39::Seed::from_mnemonic_string(&mnemonics_phrase, b"");
    let mut xprv = hdwallet::XPrv::generate_from_bip39(&seed);
    println!("xprv: {:#?}", xprv);

    for derivation_index in &[1852, 1852, 0, 0, 0] {
        xprv = xprv.derive(hdwallet::DerivationScheme::V2, *derivation_index);
    }
    println!("derived xprv: {:#?}", xprv);

    let xpub = xprv.public();
    println!("derived xpub: {:#?}", xpub);

    let tx_id = b"bb1eb401cd03b0cd8caa08997df0a2ab226772c4d3a08adfb5a60ba34de12dfb";
    let signature: Signature<Vec<u8>> = xprv.sign(tx_id);
    println!("signature: {:#?}", signature);

    let mut xprv = Bip32PrivateKey::from_bip39_entropy(seed.to_bytes(), b"");
    println!("xprv: {}", xprv.to_hex());

    for derivation_index in &[1852, 1852, 0, 0, 0] {
        xprv = xprv.derive(*derivation_index);
    }
    println!("derived xprv: {}", xprv.to_hex());

    let xprv = xprv.to_raw_key();
    let signature = xprv.sign(tx_id);
    println!("signature: {:#?}", signature);
}

// 78fe04891cbda885b3ee9b7a60bb5991c3209b07f16324c2d68cb9c7c328ed512a18cdf9b5c0fa98e7d620ae9d851a58aca7e4e0ab46f607c03e78498b345b1b80def65319d69eb65c59d6a67b18b27f03c9c005f5499f75bdb8ac5ba4b5104b7c0b5c44c1ddb9049bfcaf4ec5d73236392321c69979bbcff1f7c1b6d74c9c5a
// 78fe04891cbda885b3ee9b7a60bb5991c3209b07f16324c2d68cb9c7c328ed512a18cdf9b5c0fa98e7d620ae9d851a58aca7e4e0ab46f607c03e78498b345b1b
