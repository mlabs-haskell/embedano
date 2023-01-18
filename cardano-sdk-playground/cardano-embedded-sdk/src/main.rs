use cardano_embedded_sdk::hdwallet::Signature;
use cardano_embedded_sdk::{bip::bip39, hdwallet, tx, wallet::bip44};

use cardano_serialization_lib::crypto::Bip32PrivateKey;

fn main() {
    let mnemonics = bip39::Mnemonics::from_string(
        &bip39::dictionary::ENGLISH,
        "all all all all all all all all all all all all",
    )
    .unwrap();
    let seed = bip39::Seed::from_mnemonic_string(&mnemonics.to_string(&bip39::dictionary::ENGLISH), b"");
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

    let entropy = bip39::Entropy::from_mnemonics(&mnemonics).unwrap();

    let mut xprv = Bip32PrivateKey::from_bip39_entropy(entropy.as_ref(), b"");
    println!("xprv: {}", xprv.to_hex());

    for derivation_index in &[1852, 1852, 0, 0, 0] {
        xprv = xprv.derive(*derivation_index);
    }
    println!("derived xprv: {}", xprv.to_hex());

    let xprv = xprv.to_raw_key();
    let signature = xprv.sign(tx_id);
    println!("signature: {:#?}", signature);
}

// 78fe04891cbda885b3ee9b7a60bb5991c3209b07f16324c2d68cb9c7c328ed512a18cdf9b5c0fa98e7d620ae9d851a58aca7e4e0ab46f607c03e78498b345b1b 80def65319d69eb65c59d6a67b18b27f03c9c005f5499f75bdb8ac5ba4b5104b7c0b5c44c1ddb9049bfcaf4ec5d73236392321c69979bbcff1f7c1b6d74c9c5a

// 78fe04891cbda885b3ee9b7a60bb5991c3209b07f16324c2d68cb9c7c328ed512a18cdf9b5c0fa98e7d620ae9d851a58aca7e4e0ab46f607c03e78498b345b1b 7c0b5c44c1ddb9049bfcaf4ec5d73236392321c69979bbcff1f7c1b6d74c9c5a

// 78fe04891cbda885b3ee9b7a60bb5991c3209b07f16324c2d68cb9c7c328ed512a18cdf9b5c0fa98e7d620ae9d851a58aca7e4e0ab46f607c03e78498b345b1b
// 908124d9e64c75db36bd672d6774fade03d10fdc86ae0db50a492a12b218a85026d3f7d99f331f117f4ce87ab47ecaa119d496f46a2e54620216cdd9a35de9e474a9043eeb77bdd53aa6fc3a0e31462270316fa04b8c19114c8798706cd02ac8
// d8b65600823a95fbf290528054bc94ee45ce52b95582a6585b92022414dc1745bf2d4d8e4ed26ff24ee677bd0230475460c154fe6360ae4d94cf94a7725e4d04