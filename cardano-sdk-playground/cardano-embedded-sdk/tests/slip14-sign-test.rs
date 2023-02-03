use cardano_embedded_sdk::sdktypes::{XPrvKey, XPubKey};
use cardano_embedded_sdk::util::slip14;

#[test]
fn test_tx_id_signing() {
    let (account_prv_key, account_pub_key) = slip14::make_address_keys();

    // known transaction id for slip-14 keys according to slip14-data/README.md
    let tx_id = "bb1eb401cd03b0cd8caa08997df0a2ab226772c4d3a08adfb5a60ba34de12dfb";
    // known signature for slip-14 keys according to slip14-data/README.md
    let reference_signature = "e6766adf71231ec80faddbe12dcea623fd6bc31982cdbc69e90fb8c4dd937d4cdc87c2d3287a1c62be928a4ec01b970099410301adba27ca20fee0c08f68e50a";

    let sign_payload = hex::decode(tx_id).unwrap();
    let signature = account_prv_key.sign(&sign_payload);

    assert_eq!(reference_signature, signature.to_hex());
    assert!(
        account_pub_key.verify(&sign_payload, &signature),
        "Public key was not able to verify signature of corresponding private key"
    )
}
