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

use usb_device::bus;
use usb_device::prelude::*;
use usbd_serial::{DefaultBufferStore, SerialPort, USB_CLASS_CDC};

use cardano_embedded_sdk::bip::bip39::{dictionary, Entropy, Mnemonics};

extern crate alloc;
use alloc::{format, vec, vec::Vec};
use device::*;

#[global_allocator]
static ALLOCATOR: CortexMHeap = CortexMHeap::empty();

const HEAP_SIZE: usize = 1 * 1024; // in bytes

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

    let mut led_s = gpioe
        .pe13
        .into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);
    led_s.set_low().ok();

    let mut led_sw = gpioe
        .pe14
        .into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);
    led_sw.set_low().ok();

    let mut led_w = gpioe
        .pe15
        .into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);
    led_w.set_low().ok();

    let mut led_nw = gpioe
        .pe8
        .into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);
    led_nw.set_low().ok();

    let mut led_n = gpioe
        .pe9
        .into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);
    led_n.set_low().ok();

    let mut led_ne = gpioe
        .pe10
        .into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);
    led_ne.set_low().ok();

    let mut led_e = gpioe
        .pe11
        .into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);
    led_e.set_low().ok();

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
        if !usb_dev.poll(&mut [&mut serial]) || !serial.dtr() {
            continue;
        }
        led_s.set_high().ok();

        led_sw.set_high().ok();
        let message = recieve(&mut serial);
        led_sw.set_low().ok();

        led_w.set_high().ok();
        let message = minicbor::decode::<'_, In>(&message);
        delay(10_000_000);
        let _ = usb_dev.poll(&mut [&mut serial]);
        led_w.set_low().ok();

//        let _ = usb_dev.poll(&mut [&mut serial]);

        led_nw.set_high().ok();
        let out: Out = match message {
            Ok(In::Init(mnemonics)) => {
                let result = Mnemonics::from_string(&dictionary::ENGLISH, &mnemonics)
                    .map(|v| Entropy::from_mnemonics(&v))
                    .flatten()
                    .map(|v| entropy = Some(v));
                if let Err(e) = result {
                    Out::Error(format!("Decode mnemonics failed: {e}"))
                } else {
                    Out::Init
                }
            }
            Ok(In::Sign(tx_id, password, path)) => {
                if let Some(entropy) = &entropy {
                    sign(&tx_id, entropy, &password, &path)
                } else {
                    Out::Error(format!("Sign failed: no entropy"))
                }
            }
            Ok(In::Verifiy(tx_id, signature, password, path)) => {
                if let Some(entropy) = &entropy {
                    verify(&tx_id, signature, entropy, &password, &path)
                } else {
                    Out::Error(format!("Verifiy failed: no entropy"))
                }
            }
            Err(e) => Out::Error(format!("Decode message failed: {e:#?}")),
        };
        delay(10_000_000);
        let _ = usb_dev.poll(&mut [&mut serial]);
        led_nw.set_low().ok();

//        let _ = usb_dev.poll(&mut [&mut serial]);

        led_n.set_high().ok();
        let data = minicbor::to_vec(&out).unwrap();
        delay(10_000_000);
        let _ = usb_dev.poll(&mut [&mut serial]);
        led_n.set_low().ok();

//        let _ = usb_dev.poll(&mut [&mut serial]);

        led_ne.set_high().ok();
        let len = data.len();
        serial.write(&(len as u64).to_be_bytes()).unwrap();
        delay(10_000_000);
        let _ = usb_dev.poll(&mut [&mut serial]);
        led_ne.set_low().ok();

//        let _ = usb_dev.poll(&mut [&mut serial]);

        let mut offset = 0;
        while offset < len {
            match serial.write(&data[offset..len]) {
                Ok(0) => break,
                Ok(count) => {
                    offset += count;
                },
                Err(UsbError::WouldBlock) => {
                    delay(10)
                },
                _ => break,
            }
            led_sw.set_high().ok();
            delay(10_000_000);
            let _ = usb_dev.poll(&mut [&mut serial]);
            led_sw.set_low().ok();
        }

//        let _ = usb_dev.poll(&mut [&mut serial]);

        led_e.set_high().ok();
        let _ = usb_dev.poll(&mut [&mut serial]);
        serial.flush().unwrap();
        delay(10_000_000);

        led_e.set_low().ok();

//        let _ = usb_dev.poll(&mut [&mut serial]);

        led_s.set_low().ok();
    }
}

fn send<B: bus::UsbBus>(
    serial: &mut SerialPort<'_, B, DefaultBufferStore, DefaultBufferStore>,
    value: Out,
) {
    let data = minicbor::to_vec(&value).unwrap();
    let len = data.len();

    serial.write(&(len as u64).to_be_bytes()).unwrap();

    let mut offset = 0;
    while offset < len {
        match serial.write(&data[offset..len]) {
            Ok(0) => break,
            Ok(count) => offset += count,
            Err(UsbError::WouldBlock) => delay(10),
            _ => break,
        }
    }
    serial.flush().unwrap();
}

fn recieve<B: bus::UsbBus>(
    serial: &mut SerialPort<'_, B, DefaultBufferStore, DefaultBufferStore>,
) -> Vec<u8> {
let mut buf = [0u8; 64];
    let mut data = vec![];
    let mut read = 0;

    while read < 8 {
        match serial.read(&mut buf) {
            Ok(0) => break,
            Ok(count) => {
                data.extend_from_slice(&buf[..count]);
                read += count;
            }
            Err(UsbError::WouldBlock) => delay(10),
            _ => break,
        }
    }

    let mut length = [0u8; 8];
    length.copy_from_slice(&data[..8]);
    let length = u64::from_be_bytes(length);

    let mut message = vec![];
    if read > 8 {
        message.extend_from_slice(&data[8..read]);
        read = read - 8;
        while (read as u64) < length {
            match serial.read(&mut buf) {
                Ok(0) => break,
                Ok(count) => {
                    message.extend_from_slice(&buf[..count]);
                    read += count;
                }
                Err(UsbError::WouldBlock) => delay(1),
                _ => break,
            }
        }
    }

    message
}
