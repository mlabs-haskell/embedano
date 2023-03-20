#![feature(alloc_error_handler)]
#![no_std]
#![no_main]

// pick a panicking behavior
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
                     // use panic_abort as _; // requires nightly
                     // use panic_itm as _; // logs messages over ITM; requires ITM support
                     // use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

// use cortex_m::asm;
use alloc_cortex_m::CortexMHeap;
use core::alloc::Layout;
use cortex_m_rt::entry;
use cortex_m_semihosting::{debug, hprintln};

use cardano_embedded_sdk::api as embedano;
use cardano_embedded_sdk::bip::bip39::{dictionary, Entropy, Mnemonics};
use cardano_embedded_sdk::types::{harden, TxId, XPrvKey};
use derivation_path::DerivationPath;

#[global_allocator]
static ALLOCATOR: CortexMHeap = CortexMHeap::empty();

const HEAP_SIZE: usize = 1024; // in bytes

#[entry]
fn main() -> ! {
    hprintln!("Test: Generating keys").unwrap();

    // from example/allocator.rs
    unsafe { ALLOCATOR.init(cortex_m_rt::heap_start() as usize, HEAP_SIZE) }

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
    let (_prv_key, pub_key) = embedano::derive_key_pair(&entropy, password, &path);
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
    hprintln!("Root key: {}", root_key.to_hex()).unwrap();

    // Derive private key for same path that was used in `derive_key_pair` above
    let prv_key = root_key
        .derive(harden(1852))
        .derive(harden(1815))
        .derive(harden(0))
        .derive(0)
        .derive(0);
    hprintln!("Private key: {}", prv_key.to_hex()).unwrap();

    // Derive corresponding public key
    let pub_key = prv_key.to_public();
    hprintln!("Public key: {}", pub_key.to_hex()).unwrap();

    // Sign and verify using derived keys
    let some_data = b"some data";
    let signature = prv_key.sign(some_data);
    hprintln!("Verify: {}", pub_key.verify(some_data, &signature)).unwrap();

    debug::exit(debug::EXIT_SUCCESS);

    loop {}
}

#[alloc_error_handler]
fn alloc_error(_layout: Layout) -> ! {
    hprintln!("ALLOC ERROR").unwrap();
    debug::exit(debug::EXIT_FAILURE);
    //let f: [u32] = todo!();

    loop {}
}
