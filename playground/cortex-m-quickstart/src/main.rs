#![no_std]
#![no_main]
#![feature(alloc_error_handler)]

// pick a panicking behavior
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
                     // use panic_abort as _; // requires nightly
                     // use panic_itm as _; // logs messages over ITM; requires ITM support
                     // use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

use core::alloc::Layout;

use cortex_m_rt::entry;
use cortex_m_semihosting::{debug, hprintln};

use stm32_hal2::{
    clocks::Clocks,
    flash::{Bank, Flash},
    pac,
};

use ed25519_dalek::{ExpandedSecretKey, SecretKey, SECRET_KEY_LENGTH};
use rand::prelude::*;
use rand_chacha::ChaCha20Rng;

use alloc_cortex_m::CortexMHeap;
#[global_allocator]
static ALLOCATOR: CortexMHeap = CortexMHeap::empty();

// use embedded_alloc::Heap;
// #[global_allocator]
// static HEAP: Heap = Heap::empty();

// STM32F303xB/STM32F303xC: 128 pages of 2 Kbytes
const FLASH_PAGE: usize = 127;
const PAGE_SIZE: usize = 2048;

// https://en.wikipedia.org/wiki/43_(number)
const KEY_FLAG: u8 = 43;

#[entry]
fn main() -> ! {
    {
        const HEAP_SIZE: usize = 1024;
        unsafe { ALLOCATOR.init(cortex_m_rt::heap_start() as usize, HEAP_SIZE) }
    }
    // {
    //     use core::mem::MaybeUninit;
    //     const HEAP_SIZE: usize = 1024;
    //     static mut HEAP_MEM: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];
    //     unsafe { HEAP.init(HEAP_MEM.as_ptr() as usize, HEAP_SIZE) }
    // }

    let mut _cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();

    let clock_cfg = Clocks::default();
    clock_cfg.setup().unwrap();

    let mut flash = Flash::new(dp.FLASH);

    let mut flag_bug = [0u8; 1];
    flash.read(
        Bank::B1,
        FLASH_PAGE,
        PAGE_SIZE - SECRET_KEY_LENGTH - 1,
        &mut flag_bug,
    );
    hprintln!("read: {:#?}", flag_bug).unwrap();
    if flag_bug[0] != KEY_FLAG {
        if let Err(e) = flash.erase_page(Bank::B1, FLASH_PAGE) {
            hprintln!("erase_page error: {:#?}", e).unwrap();
            debug::exit(debug::EXIT_FAILURE);
        }

        let seed: <ChaCha20Rng as SeedableRng>::Seed = Default::default();
        let mut csprng = ChaCha20Rng::from_seed(seed);
        let secret_key: SecretKey = SecretKey::generate(&mut csprng);
        let expanded_secret_key: ExpandedSecretKey = ExpandedSecretKey::from(&secret_key);
        let expanded_secret_key_bytes: [u8; 64] = expanded_secret_key.to_bytes();

        hprintln!(
            "expanded_secret_key_bytes: {:#?}",
            expanded_secret_key_bytes
        )
        .unwrap();

        let mut page = [0u8; PAGE_SIZE];
        page[PAGE_SIZE - SECRET_KEY_LENGTH - 1] = KEY_FLAG;
        page[PAGE_SIZE - SECRET_KEY_LENGTH..].copy_from_slice(&expanded_secret_key_bytes);
        if let Err(e) = flash.write_page(Bank::B1, FLASH_PAGE, &page) {
            hprintln!("write_page error: {:#?}", e).unwrap();
            debug::exit(debug::EXIT_FAILURE);
        }
    }

    let mut read_buf = [0u8; SECRET_KEY_LENGTH];
    flash.read(
        Bank::B1,
        FLASH_PAGE,
        PAGE_SIZE - SECRET_KEY_LENGTH,
        &mut read_buf,
    );

    hprintln!("read: {:#?}", read_buf).unwrap();

    loop {}
}

#[alloc_error_handler]
fn oom(_: Layout) -> ! {
    hprintln!("oom").unwrap();
    debug::exit(debug::EXIT_FAILURE);

    loop {}
}
