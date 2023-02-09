use cardano_embedded_sdk::api as embedano;
use cardano_embedded_sdk::types::{TxId};
use cardano_embedded_sdk::util::slip14;
use derivation_path::DerivationPath;

#[test]
fn test_tx_id_signing() {
    let entropy = slip14::make_entropy();
    let password = b"";
    let path: DerivationPath = "m/1852'/1815'/0'/0/0".parse().unwrap();
    let pub_key = embedano::derive_key(&entropy, password, &path).to_public();

    // Known transaction id for slip-14 keys according to slip14-data/README.md
    let tx_id = "bb1eb401cd03b0cd8caa08997df0a2ab226772c4d3a08adfb5a60ba34de12dfb";
    let tx_id = TxId::from_hex(tx_id).unwrap();
    
    // Known signature for slip-14 keys according to slip14-data/README.md
    let reference_signature = "e6766adf71231ec80faddbe12dcea623fd6bc31982cdbc69e90fb8c4dd937d4cdc87c2d3287a1c62be928a4ec01b970099410301adba27ca20fee0c08f68e50a";

    let signature = embedano::sign_tx_id(&tx_id, &entropy, password, &path);

    assert_eq!(reference_signature, signature.to_hex());
    assert!(
        pub_key.verify(tx_id.to_bytes(), &signature),
        "Public key was not able to verify signature of corresponding private key"
    )
}
