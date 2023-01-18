use cardano_embedded_sdk::hdwallet::Signature;
use cardano_embedded_sdk::{bip::bip39, hdwallet, tx, wallet::bip44};

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
}
