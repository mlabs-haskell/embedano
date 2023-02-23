#![no_std]
#![no_main]

// pick a panicking behavior
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
                     // use panic_abort as _; // requires nightly
                     // use panic_itm as _; // logs messages over ITM; requires ITM support
                     // use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

use alloc_cortex_m::CortexMHeap;
use cortex_m::asm::delay;
use cortex_m_rt::entry;

use stm32f3xx_hal as hal;

use hal::pac;
use hal::prelude::*;
use hal::usb::{Peripheral, UsbBus};

use usb_device::prelude::*;
use usbd_serial::{SerialPort, USB_CLASS_CDC};

use cardano_embedded_sdk::api as embedano;
use cardano_embedded_sdk::bip::bip39::{dictionary, Entropy, Mnemonics};
use cardano_embedded_sdk::types::TxId;
use derivation_path::DerivationPath;

#[global_allocator]
static ALLOCATOR: CortexMHeap = CortexMHeap::empty();

const HEAP_SIZE: usize = 1024; // in bytes

#[entry]
fn main() -> ! {
    unsafe { ALLOCATOR.init(cortex_m_rt::heap_start() as usize, HEAP_SIZE) }

    let mnemonics = "aim wool into nose tell ball arm expand design push elevator multiply glove lonely minimum";
    let mnemonics = Mnemonics::from_string(&dictionary::ENGLISH, mnemonics).unwrap();
    let password = b"embedano";
    let entropy = Entropy::from_mnemonics(&mnemonics).unwrap();
    let path: DerivationPath = "m/1852'/1815'/0'/0/0".parse().unwrap();

    let dp = pac::Peripherals::take().unwrap();
    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();

    let clocks = rcc
        .cfgr
        .use_hse(8.MHz())
        .sysclk(48.MHz())
        .pclk1(24.MHz())
        .pclk2(24.MHz())
        .freeze(&mut flash.acr);
    assert!(clocks.usbclk_valid());

    // Configure the on-board LED (LD10, south red)
    let mut gpioe = dp.GPIOE.split(&mut rcc.ahb);
    let mut led = gpioe
        .pe13
        .into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);
    led.set_low().ok(); // Turn off

    let mut gpioa = dp.GPIOA.split(&mut rcc.ahb);

    // F3 Discovery board has a pull-up resistor on the D+ line.
    // Pull the D+ pin down to send a RESET condition to the USB bus.
    // This forced reset is needed only for development, without it host
    // will not reset your device when you upload new firmware.
    let mut usb_dp = gpioa
        .pa12
        .into_push_pull_output(&mut gpioa.moder, &mut gpioa.otyper);
    usb_dp.set_low().ok();
    delay(clocks.sysclk().0 / 100);

    let usb_dm = gpioa
        .pa11
        .into_af_push_pull(&mut gpioa.moder, &mut gpioa.otyper, &mut gpioa.afrh);
    let usb_dp = usb_dp.into_af_push_pull(&mut gpioa.moder, &mut gpioa.otyper, &mut gpioa.afrh);

    let usb = Peripheral {
        usb: dp.USB,
        pin_dm: usb_dm,
        pin_dp: usb_dp,
    };
    let usb_bus = UsbBus::new(usb);

    let mut serial = SerialPort::new(&usb_bus);

    let mut usb_dev = UsbDeviceBuilder::new(&usb_bus, UsbVidPid(0x16c0, 0x27dd))
        .manufacturer("Fake company")
        .product("Serial port")
        .serial_number("TEST")
        .device_class(USB_CLASS_CDC)
        .build();

    loop {
        if !usb_dev.poll(&mut [&mut serial]) {
            continue;
        }

        let mut buf = [0u8; 32];

        match serial.read(&mut buf) {
            Ok(count) if count == 32 => {
                led.set_high().ok(); // Turn on

                let tx_id = TxId::from_bytes(&buf).unwrap();
                let signature = embedano::sign_tx_id(&tx_id, &entropy, password, &path);

                serial.write(&signature.to_bytes()).unwrap();
            }
            _ => {}
        }

        led.set_low().ok(); // Turn off
    }
}
