#![no_main]
#![no_std]
#![feature(result_flattening)]

use cortex_m_semihosting::hprintln;

use nrf52840_hal::Temp;
use panic_halt as _;

use alloc_cortex_m::CortexMHeap;

use cortex_m_rt::entry;
use nrf52840_hal::clocks::Clocks;
use nrf52840_hal::usbd::{UsbPeripheral, Usbd};
use usb_device::class_prelude::UsbBusAllocator;
use usb_device::device::{UsbDeviceBuilder, UsbVidPid};
use usb_device::UsbError;
use usbd_serial::{SerialPort, USB_CLASS_CDC};

use cardano_embedded_sdk::bip::bip39::{dictionary, Entropy, Mnemonics};

extern crate alloc;
use alloc::{format, vec, vec::Vec};

use embedano_device::*;

#[global_allocator]
static ALLOCATOR: CortexMHeap = CortexMHeap::empty();

const HEAP_SIZE: usize = 1 * 1024; // in bytes

#[entry]
fn main() -> ! {
    unsafe { ALLOCATOR.init(cortex_m_rt::heap_start() as usize, HEAP_SIZE) }

    // Setting up serial port and USB device
    let periph = nrf52840_hal::pac::Peripherals::take().unwrap();
    let clocks = Clocks::new(periph.CLOCK);
    let clocks = clocks.enable_ext_hfosc();

    let usb_bus = UsbBusAllocator::new(Usbd::new(UsbPeripheral::new(periph.USBD, &clocks)));
    let mut serial = SerialPort::new(&usb_bus);

    let mut usb_dev = UsbDeviceBuilder::new(&usb_bus, UsbVidPid(0x16c0, 0x27dd))
        .manufacturer("Fake company")
        .product("Serial port")
        .serial_number("TEST")
        .device_class(USB_CLASS_CDC)
        .max_packet_size_0(64) // (makes control transfers 8x faster)
        .build();

    let mut entropy: Option<Entropy> = None;
    let mut temp_sensor = Temp::new(periph.TEMP);

    let mut state = State::Read(Data::Head(vec![]));

    loop {
        if !usb_dev.poll(&mut [&mut serial]) || !serial.dtr() {
            continue;
        }

        'main: loop {
            match state {
                State::Read(Data::Head(mut data)) => {
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
                }
                State::Read(Data::Body(data, 0)) => match minicbor::decode::<'_, In>(&data) {
                    Ok(exec) => {
                        state = State::Exec(exec);
                    }
                    Err(e) => {
                        let out = Out::Error(format!("Decode mnemonics failed: {e}"));
                        state = State::Write(Data::Head(minicbor::to_vec(&out).unwrap()));
                    }
                },
                State::Read(Data::Body(mut data, rest)) => {
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
                }

                State::Write(Data::Head(data)) => {
                    let len = data.len();
                    serial.write(&(len as u64).to_be_bytes()).unwrap();
                    state = State::Write(Data::Body(data, len));
                }
                State::Write(Data::Body(_data, 0)) => {
                    state = State::Read(Data::Head(vec![]));

                    break 'main;
                }
                State::Write(Data::Body(data, rest)) => match serial.write(&data) {
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
                },

                State::Exec(In::Init(mnemonics)) => {
                    hprintln!("Firmware: initializing device");
                    let result = Mnemonics::from_string(&dictionary::ENGLISH, &mnemonics)
                        .map(|v| Entropy::from_mnemonics(&v))
                        .flatten()
                        .map(|v| entropy = Some(v));
                    let out = if let Err(e) = result {
                        Out::Error(format!("Decode mnemonics failed: {e}"))
                    } else {
                        hprintln!("Firmware: device initialized with mnemonics");
                        Out::Init
                    };
                    state = State::Write(Data::Head(minicbor::to_vec(&out).unwrap()));
                }
                State::Exec(In::Sign(tx_id, password, path)) => {
                    let out = if let Some(entropy) = &entropy {
                        hprintln!("Firmware: signing transaction id");
                        sign(&tx_id, entropy, &password, &path)
                    } else {
                        Out::Error(format!("Sign failed: no entropy"))
                    };
                    state = State::Write(Data::Head(minicbor::to_vec(&out).unwrap()));
                }
                State::Exec(In::Verify(tx_id, signature, password, path)) => {
                    let out = if let Some(entropy) = &entropy {
                        verify(&tx_id, signature, entropy, &password, &path)
                    } else {
                        Out::Error(format!("Verify failed: no entropy"))
                    };
                    state = State::Write(Data::Head(minicbor::to_vec(&out).unwrap()));
                }
                State::Exec(In::Temp(password, time, path)) => {
                    use cardano_embedded_sdk::api::sign_data;
                    use derivation_path::DerivationPath;

                    let temperature: i32 = temp_sensor.measure().to_num();
                    hprintln!("Firmware: temperature: {}", temperature);
                    hprintln!("Firmware: time: {}", time);
                    let out = match (&entropy, path.parse::<DerivationPath>()) {
                        (Some(entropy), Ok(path)) => {
                            let data: Vec<u8> = chain_data_bytes(temperature, time);
                            hprintln!("Firmware: temperature: signing");
                            let signature = sign_data(&data, entropy, &password, &path);
                            hprintln!("Firmware: temperature: sending");
                            Out::Temp(temperature, signature.to_bytes())
                        }
                        (None, _) => Out::Error(format!("Getting temperature failed: no entropy")),
                        (_, Err(e)) => Out::Error(format!("Decode path failed: {e}")),
                    };
                    state = State::Write(Data::Head(minicbor::to_vec(&out).unwrap()));
                }
                State::Exec(In::PubKey(password, path)) => {
                    let out = if let Some(entropy) = &entropy {
                        hprintln!("Firmware: Sending public key");
                        get_pub_key(entropy, &password, &path)
                    } else {
                        Out::Error(format!("Public key derivation failed: no entropy"))
                    };
                    state = State::Write(Data::Head(minicbor::to_vec(&out).unwrap()));
                }
            }
            usb_dev.poll(&mut [&mut serial]);
        }
        // MAIN LOOP END
    }
}

fn chain_data_bytes(a: i32, b: u64) -> Vec<u8> {
    a.to_be_bytes()
        .into_iter()
        .chain(b.to_be_bytes().into_iter())
        .collect::<Vec<u8>>()
}
