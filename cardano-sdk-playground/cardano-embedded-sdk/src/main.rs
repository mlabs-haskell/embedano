use cardano_embedded_sdk::bip::bip39;
use cardano_embedded_sdk::sdkwallet::XPrvKey;
use cardano_serialization_lib::crypto;
use derivation_path::DerivationPath;

fn main() {
    let mnemonics = bip39::Mnemonics::from_string(
        &bip39::dictionary::ENGLISH,
        "all all all all all all all all all all all all",
    )
    .unwrap();

    let entropy = bip39::Entropy::from_mnemonics(&mnemonics).unwrap();
    let root_key = XPrvKey::from_entropy(&entropy, b"");
    println!("root xprv: {}", root_key.to_hex());

    let path: DerivationPath = "m/1852'/1815'/0'/0/0".parse().unwrap();
    println!("path: {:#?}", path);

    let path2: DerivationPath = "m".parse().unwrap();
    println!("path2: {:#?}", path2);

    let acc_0_xprv = XPrvKey::derive_for_path(root_key, path);
    println!("Account 0 xprv key: {}", acc_0_xprv.to_hex());
    // println!("root xprv: {}", root_key.to_hex());

    let acc_0_xpub = acc_0_xprv.to_public();
    println!("Account 0 xpub key: {}", acc_0_xpub.to_hex());

    let tx_id = "bb1eb401cd03b0cd8caa08997df0a2ab226772c4d3a08adfb5a60ba34de12dfb";
    let data = hex::decode(tx_id).unwrap();
    // println!("data len: {}\n", data.len());

    let signature = acc_0_xprv.sign(&data);
    println!("signature: {:#?}", signature);

    println!("verify: {}", acc_0_xpub.verify(&data, &signature));

    let signature = crypto::Ed25519Signature::from_hex("e6766adf71231ec80faddbe12dcea623fd6bc31982cdbc69e90fb8c4dd937d4cdc87c2d3287a1c62be928a4ec01b970099410301adba27ca20fee0c08f68e50a").unwrap();
    println!(
        "verify sig from hex: {}",
        acc_0_xpub.verify(&data, &signature)
    );
}
