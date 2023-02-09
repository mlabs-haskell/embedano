use cardano_embedded_sdk::api as embedano;
use cardano_embedded_sdk::bip::bip39::{dictionary, Entropy, Mnemonics};
use cardano_embedded_sdk::types::{harden, TxId, XPrvKey};
use derivation_path::DerivationPath;

fn main() {
    // Preparations: define some mnemonic and make entropy.
    // (in real setup entropy or seed will be loaded from device memory)
    let mnemonics = "aim wool into nose tell ball arm expand design push elevator multiply glove lonely minimum";
    let mnemonics = Mnemonics::from_string(&dictionary::ENGLISH, mnemonics).unwrap();

    let password = b"embedano";
    let entropy = Entropy::from_mnemonics(&mnemonics).unwrap();

    // Make derivation path for account 0 and address 0 according to CIP-1852
    let path: DerivationPath = "m/1852'/1815'/0'/0/0".parse().unwrap();

    // Try to parse transaction id and sign it
    let tx_id =
        TxId::from_hex("bb1eb401cd03b0cd8caa08997df0a2ab226772c4d3a08adfb5a60ba34de12dfb").unwrap();
    let signature = embedano::sign_tx_id(&tx_id, &entropy, password, &path);

    // Derive key pair using same path ant try to verify signature from `sign_tx_id`
    let (prv_key, pub_key) = embedano::derive_key_pair(&entropy, password, &path);
    assert!(pub_key.verify(tx_id.to_bytes(), &signature));

    // Check if public key can be derived from given entropy by signing nonce
    let nonce = b"some nonce";
    // Define what key type of public key we pass to prove function.
    // This will affect what address derivation will be used and how many indexes will be checked.
    // As earlier we used derivation path "m/1852'/1815'/0'/0/0" to make keys,
    // `pub_key` corresponds to address 0 of account 0 so as `key_type`.
    let key_type = embedano::KeyType::AddressKey {
        account_gap: 5,
        address_gap: 5,
    };
    let proof_sig =
        embedano::prove_ownership(nonce, &pub_key, &entropy, password, key_type).unwrap();
    // If we got Some(signature), then we can verify proof with public key we tested.
    assert!(pub_key.verify(nonce, &proof_sig));

    // Function above defined on top of types in `types.rs` that can provide more fine grinded control.
    // E.g.:

    // Create root private key from entropy
    let root_key = XPrvKey::from_entropy(&entropy, password);
    println!("Root key: {}", root_key.to_hex());

    // Derive private key for same path that was used in `derive_key_pair` above
    let prv_key = root_key
        .derive(harden(1852))
        .derive(harden(1815))
        .derive(harden(0))
        .derive(0)
        .derive(0);
    println!("Private key: {}", prv_key.to_hex());

    // Derive corresponding public key
    let pub_key = prv_key.to_public();
    println!("Public key: {}", pub_key.to_hex());

    // Sign and verify using derived keys
    let some_data = b"some data";
    let signature = prv_key.sign(some_data);
    println!("Verify: {}", pub_key.verify(some_data, &signature))
}
