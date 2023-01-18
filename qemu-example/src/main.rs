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
use bip32_ed25519::{Xprv, ED25519_EXPANDED_SECRET_KEY_SIZE};
use core::alloc::Layout;
use cortex_m_rt::entry;
use cortex_m_semihosting::{debug, hprintln};

#[global_allocator]
static ALLOCATOR: CortexMHeap = CortexMHeap::empty();

const HEAP_SIZE: usize = 1024; // in bytes

const TEST_SEED: &[u8] = b"\xf8\xcb\x28\x85\x37\x60\x2b\x90\xd1\x29\x75\x4b\xdd\x0e\x4b\xed\xf9\xe2\x92\x3a\x04\xb6\x86\x7e\xdb\xeb\xc7\x93\xa7\x17\x6f\x5d\xca\xc5\xc9\x5d\x5f\xd2\x3a\x8e\x01\x6c\x95\x57\x69\x0e\xad\x1f\x00\x2b\x0f\x35\xd7\x06\xff\x8e\x59\x84\x1c\x09\xe0\xb6\xbb\x23\xf0\xa5\x91\x06\x42\xd0\x77\x98\x17\x40\x2e\x5e\x7a\x75\x54\x95\xe7\x44\xf5\x5c\xf1\x1e\x49\xee\xfd\x22\xa4\x60\xe9\xb2\xf7\x53";

#[entry]
fn main() -> ! {
    hprintln!("Test: Generating keys");

    // from example/allocator.rs
    unsafe { ALLOCATOR.init(cortex_m_rt::heap_start() as usize, HEAP_SIZE) }

    let root_xprv_key = Xprv::from_normalize(
        &TEST_SEED[..ED25519_EXPANDED_SECRET_KEY_SIZE],
        &TEST_SEED[ED25519_EXPANDED_SECRET_KEY_SIZE..],
    );

    let root_xpub_key = root_xprv_key.public();

    hprintln!("XPrv: {:?}", root_xprv_key).unwrap();
    hprintln!("XPub: {:?}", root_xpub_key.pubkey_bytes()).unwrap();

    debug::exit(debug::EXIT_SUCCESS);

    loop {}
}

#[alloc_error_handler]
fn alloc_error(_layout: Layout) -> ! {
    hprintln!("ALLOC ERROR").unwrap();
    debug::exit(debug::EXIT_FAILURE);

    loop {}
}
