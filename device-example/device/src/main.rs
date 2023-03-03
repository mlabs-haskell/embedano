#![no_std]
#![no_main]
#![feature(result_flattening)]

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

use cardano_embedded_sdk::bip::bip39::{dictionary, Entropy, Mnemonics};

extern crate alloc;
use alloc::format;
use device::*;

#[global_allocator]
static ALLOCATOR: CortexMHeap = CortexMHeap::empty();

const HEAP_SIZE: usize = 1024; // in bytes

#[entry]
fn main() -> ! {
    unsafe { ALLOCATOR.init(cortex_m_rt::heap_start() as usize, HEAP_SIZE) }

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

    let mut entropy: Option<Entropy> = None;

    loop {
        if !usb_dev.poll(&mut [&mut serial]) {
            continue;
        }

        let mut buf = [0u8; 4096];

        let out: Out = match serial.read(&mut buf) {
            Ok(_count) => {
                led.set_high().ok(); // Turn on
                match minicbor::decode::<'_, In>(&buf) {
                    Ok(In::Init(mnemonics)) => {
                        let result = Mnemonics::from_string(&dictionary::ENGLISH, &mnemonics)
                            .map(|v| Entropy::from_mnemonics(&v))
                            .flatten()
                            .map(|v| entropy = Some(v));
                        if let Err(e) = result {
                            Out::Error(format!("{e:#?}"))
                        } else {
                            Out::Init
                        }
                    }
                    Ok(In::Sign(tx_id, password, path)) => {
                        if let Some(entropy) = &entropy {
                            sign(&tx_id, entropy, &password, &path)
                        } else {
                            Out::Error(format!("No entropy"))
                        }
                    }
                    Ok(In::Verifiy(tx_id, signature, password, path)) => {
                        if let Some(entropy) = &entropy {
                            verify(&tx_id, signature, entropy, &password, &path)
                        } else {
                            Out::Error(format!("No entropy"))
                        }
                    }
                    Err(e) => Out::Error(format!("{e:#?}")),
                }
            }
            Err(e) => Out::Error(format!("{e:#?}")),
        };

        serial.write(&minicbor::to_vec(&out).unwrap()).unwrap();

        led.set_low().ok(); // Turn off
    }
}
