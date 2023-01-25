use cardano_embedded_sdk::hdwallet::Signature;
use cardano_embedded_sdk::{bip::bip39, hdwallet, tx, wallet::bip44};
use derivation_path::{DerivationPath, ChildIndex};
use cardano_serialization_lib::crypto;

fn main() {
    let tx_id = b"bb1eb401cd03b0cd8caa08997df0a2ab226772c4d3a08adfb5a60ba34de12dfb";

    let mnemonics = bip39::Mnemonics::from_string(
        &bip39::dictionary::ENGLISH,
        "all all all all all all all all all all all all",
    )
    .unwrap();
//    let seed = bip39::Seed::from_mnemonic_string(&mnemonics.to_string(&bip39::dictionary::ENGLISH), b"");
//    let mut xprv = hdwallet::XPrv::generate_from_bip39(&seed);
//    println!("xprv: {:#?}", xprv);
//
//    for derivation_index in &[1852, 1815, 0, 0, 0] {
//        xprv = xprv.derive(hdwallet::DerivationScheme::V2, *derivation_index);
//    }
//    println!("derived xprv: {:#?}", xprv);
//
//    let xpub = xprv.public();
//    println!("derived xpub: {:#?}", xpub);
//
//    let signature: Signature<Vec<u8>> = xprv.sign(tx_id);
//    println!("signature: {:#?}", signature);

    let data = hex::decode("bb1eb401cd03b0cd8caa08997df0a2ab226772c4d3a08adfb5a60ba34de12dfb").unwrap();
    println!("data len: {}\n", data.len());


    let entropy = bip39::Entropy::from_mnemonics(&mnemonics).unwrap();
    let mut xprv = crypto::Bip32PrivateKey::from_bip39_entropy(entropy.as_ref(), b"");
    println!("root xprv: {}", xprv.to_hex());

    let path: DerivationPath = "m/1852'/1815'/0'/0/0".parse().unwrap();
    println!("path: {:#?}", path);

    for index in path.into_iter() {
        let i = match index {
            &ChildIndex::Hardened(i) => i + 0x80000000,
            &ChildIndex::Normal(i) => i,
        };
        xprv = xprv.derive(i);
        println!("derived {:x} xprv: {}\n", i, xprv.to_hex());
    }

    let xprv = xprv.to_raw_key();
    let xpub = xprv.to_public();
    println!("raw xprv bech32: {}\n", xprv.to_bech32());
    let signature = xprv.sign(&data);
    println!("signature: {:#?}", signature);
    println!("verify: {}", xpub.verify(&data, &signature));

    let signature = crypto::Ed25519Signature::from_hex("e6766adf71231ec80faddbe12dcea623fd6bc31982cdbc69e90fb8c4dd937d4cdc87c2d3287a1c62be928a4ec01b970099410301adba27ca20fee0c08f68e50a").unwrap();
    println!("verify: {}", xpub.verify(&data, &signature));
}
