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

use hal::i2c::I2c;
use hal::time::rate::Hertz;
use lsm303dlhc::{I16x3, Lsm303dlhc};

use usb_device::prelude::*;
use usbd_serial::{SerialPort, USB_CLASS_CDC};

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

    let mut led_se = gpioe
        .pe12
        .into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);
    led_se.set_low().ok();

    let mut nss = gpioe
        .pe3
        .into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);
    nss.set_high().unwrap();
    let mut gpiob = dp.GPIOB.split(&mut rcc.ahb);
    let scl = gpiob
        .pb6
        .into_af_open_drain(&mut gpiob.moder, &mut gpiob.otyper, &mut gpiob.afrl);
    let sda = gpiob
        .pb7
        .into_af_open_drain(&mut gpiob.moder, &mut gpiob.otyper, &mut gpiob.afrl);
    let i2c = I2c::new(
        dp.I2C1,
        (scl, sda),
        Hertz::new(400_000),
        clocks,
        &mut rcc.apb1,
    );
    let mut lsm303dlhc = Lsm303dlhc::new(i2c).unwrap();

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

    let mut state = State::Read(Data::Head(vec![]));

    loop {
        if !usb_dev.poll(&mut [&mut serial]) || !serial.dtr() {
            continue;
        }

        'main: loop {
            match state {
                State::Read(Data::Head(mut data)) => {
                    led_s.set_high().ok();

                    let mut buf = [0u8; 64];
                    match serial.read(&mut buf) {
                        Ok(count) => {
                            data.extend_from_slice(&buf[..count]);
                            if data.len() >= 8 {
                                let mut rest = [0u8; 8];
                                rest.copy_from_slice(&data[..8]);
                                let rest = u64::from_be_bytes(rest);
                                let data = data.into_iter().skip(8).collect();
                                state = State::Read(Data::Body(data, rest as usize));
                            } else {
                                state = State::Read(Data::Head(data));
                            }
                        }
                        Err(UsbError::WouldBlock) => {
                            state = State::Read(Data::Head(data));
                        }
                        Err(e) => {
                            let out = Out::Error(format!("Decode mnemonics failed: {e:?}"));
                            state = State::Write(Data::Head(minicbor::to_vec(&out).unwrap()));
                        }
                    }

                    led_s.set_low().ok();
                }
                State::Read(Data::Body(data, 0)) => {
                    led_sw.set_high().ok();

                    match minicbor::decode::<'_, In>(&data) {
                        Ok(exec) => {
                            state = State::Exec(exec);
                        }
                        Err(e) => {
                            let out = Out::Error(format!("Decode mnemonics failed: {e}"));
                            state = State::Write(Data::Head(minicbor::to_vec(&out).unwrap()));
                        }
                    }

                    led_sw.set_low().ok();
                }
                State::Read(Data::Body(mut data, rest)) => {
                    led_w.set_high().ok();

                    let mut buf = [0u8; 64];
                    match serial.read(&mut buf) {
                        Ok(count) => {
                            data.extend_from_slice(&buf[..count]);
                            state = State::Read(Data::Body(data, rest - count));
                        }
                        Err(UsbError::WouldBlock) => {
                            state = State::Read(Data::Body(data, rest));
                        }
                        Err(e) => {
                            let out = Out::Error(format!("Decode mnemonics failed: {e:?}"));
                            state = State::Write(Data::Head(minicbor::to_vec(&out).unwrap()));
                        }
                    }

                    led_w.set_low().ok();
                }

                State::Write(Data::Head(data)) => {
                    led_nw.set_high().ok();

                    let len = data.len();
                    serial.write(&(len as u64).to_be_bytes()).unwrap();
                    state = State::Write(Data::Body(data, len));

                    led_nw.set_low().ok();
                }
                State::Write(Data::Body(_data, 0)) => {
                    led_n.set_high().ok();

                    state = State::Read(Data::Head(vec![]));

                    led_n.set_low().ok();

                    break 'main;
                }
                State::Write(Data::Body(data, rest)) => {
                    led_ne.set_high().ok();

                    match serial.write(&data) {
                        Ok(count) => {
                            let data = data.into_iter().skip(count).collect();
                            state = State::Write(Data::Body(data, rest - count));
                        }
                        Err(UsbError::WouldBlock) => {
                            state = State::Write(Data::Body(data, rest));
                        }
                        Err(e) => {
                            let out = Out::Error(format!("Decode mnemonics failed: {e:?}"));
                            state = State::Write(Data::Head(minicbor::to_vec(&out).unwrap()));
                        }
                    }

                    led_ne.set_low().ok();
                }

                State::Exec(In::Init(mnemonics)) => {
                    led_e.set_high().ok();

                    let result = Mnemonics::from_string(&dictionary::ENGLISH, &mnemonics)
                        .map(|v| Entropy::from_mnemonics(&v))
                        .flatten()
                        .map(|v| entropy = Some(v));
                    let out = if let Err(e) = result {
                        Out::Error(format!("Decode mnemonics failed: {e}"))
                    } else {
                        Out::Init
                    };
                    state = State::Write(Data::Head(minicbor::to_vec(&out).unwrap()));

                    led_e.set_low().ok();
                }
                State::Exec(In::Sign(tx_id, password, path)) => {
                    led_se.set_high().ok();

                    let out = if let Some(entropy) = &entropy {
                        sign(&tx_id, entropy, &password, &path)
                    } else {
                        Out::Error(format!("Sign failed: no entropy"))
                    };
                    state = State::Write(Data::Head(minicbor::to_vec(&out).unwrap()));

                    led_se.set_low().ok();
                }
                State::Exec(In::Verify(tx_id, signature, password, path)) => {
                    led_n.set_high().ok();
                    led_s.set_high().ok();

                    let out = if let Some(entropy) = &entropy {
                        verify(&tx_id, signature, entropy, &password, &path)
                    } else {
                        Out::Error(format!("Verify failed: no entropy"))
                    };
                    state = State::Write(Data::Head(minicbor::to_vec(&out).unwrap()));

                    led_n.set_low().ok();
                    led_s.set_low().ok();
                }
                State::Exec(In::Acc(password, path)) => {
                    use cardano_embedded_sdk::api::sign_data;
                    use derivation_path::DerivationPath;

                    let out = match (&entropy, path.parse::<DerivationPath>(), lsm303dlhc.accel()) {
                        (Some(entropy), Ok(path), Ok(I16x3 { x, y, z })) => {
                            let data = [x.to_be_bytes(), y.to_be_bytes(), z.to_be_bytes()]
                                .into_iter()
                                .flatten()
                                .collect::<Vec<u8>>();
                            let signature = sign_data(&data, entropy, &password, &path);
                            Out::Acc(x, y, z, signature.to_bytes())
                        }
                        (None, _, _) => Out::Error(format!("Accel failed: no entropy")),
                        (_, Err(e), _) => Out::Error(format!("Decode path failed: {e}")),
                        (_, _, Err(e)) => Out::Error(format!("Accel failed: {e:?}")),
                    };
                    state = State::Write(Data::Head(minicbor::to_vec(&out).unwrap()));
                }
            }
            usb_dev.poll(&mut [&mut serial]);
        }
    }
}
