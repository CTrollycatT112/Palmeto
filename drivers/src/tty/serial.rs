// SPDX-License-Identifier: GPL-3.0-or-later
//
// Copyright (c) 2026 Trollycat
//
// Purpose: This module act's as a shared UART driver,
//          once we make more drivers or configure properly,
//          we will split this up into an abstract module,
//          but for now this file handles the 2 UART drivers.
//          it's chosen by the DTB,
//          wether to use 'PL011' or 'S905X'
//

use spin::Mutex;

pub mod meson_uart;
pub mod pl011uart;

//
// !!! SHARED IMPORTS
//
use shared::core::ringbuf::RingBuffer;
use shared::core::types::status::{KResult, Status};
use shared::library::ulogger::sink::{register_sink, LogSink};

//
// !!! KERNEL IMPORTS
//
use kernel::arch::arm64::exception::intrcntrl;
use kernel::arch::arm64::interrupts;
use kernel::arch::arm64::assembly::interrupt;

//
// COMPATIBLE DTB STRINGS
//
//  "arm,pl011"                 = QEMU SERIAL
//  "amlogic,meson-s905-uart"   = POTATO SERIAL
//  "amlogic,meson-gx-uart"     = FALLBACK FOR POTATO SERIAL
//
pub const COMPATIBLE_STRINGS: &[&str] = &
[
    "arm,pl011",
    "amlogic,meson-gx-uart",
    "amlogic,meson-s905-uart",
];

pub trait SerialDevice: Send + Sync {

    fn write_byte(&mut self, byte: u8);
    fn read_byte(&mut self) -> Option<u8>;
    fn enable_interrupts(&mut self);

    fn write_str_raw(&mut self, string: &str) {
        for byte in string.bytes() {
            if byte == b'\n' {
                self.write_byte(b'\r');
            }
            self.write_byte(byte);
        }
    }
}

//
// TODO:
//  WE SHOULD USE BOX<> HERE INSTEAD,
//  ONCE WE HAVE OUR HEAP ALLOCATOR..
//
pub enum ActiveSerial {
    Pl011(pl011uart::Pl011Uart),
    Meson(meson_uart::MesonUart),
}

impl SerialDevice for ActiveSerial {
    fn write_byte(&mut self, byte: u8) {
        match self {
            Self::Pl011(uart) => uart.write_byte(byte),
            Self::Meson(uart) => uart.write_byte(byte),
        }
    }

    fn read_byte(&mut self) -> Option<u8> {
        match self {
            Self::Pl011(uart) => uart.read_byte(),
            Self::Meson(uart) => uart.read_byte(),
        }
    }

    fn enable_interrupts(&mut self) {
        match self {
            Self::Pl011(uart) => uart.enable_interrupts(),
            Self::Meson(uart) => uart.enable_interrupts(),
        }
    }
}

pub struct SerialSink;

impl LogSink for SerialSink {
    fn write(&self, data: &[u8]) {
        if let Some(uart) = GLOBAL_SERIAL.lock().as_mut() {
            for &byte in data {
                if byte == b'\n' {
                    uart.write_byte(b'\r');
                }
                uart.write_byte(byte);
            }
        }
    }
}

pub static GLOBAL_SERIAL: Mutex<Option<ActiveSerial>> = Mutex::new(None);
pub static SERIAL_RX_BUFFER: Mutex<RingBuffer<512>>   = Mutex::new(RingBuffer::new());

pub static SERIAL_SINK: SerialSink = SerialSink;

pub fn try_init_node(node: &fdt::node::FdtNode, vaddr: u64) -> KResult<()> {
    let Some(compatible) = node.compatible() else {
        return Err(Status::NOT_SUPPORTED);
    };

    let reg = node.reg().and_then(|mut r| r.next()).ok_or(Status::INVALID_DEVICE_REQUEST)?;
    
    let base_vaddr = reg.starting_address as usize as u64 + vaddr;
    let mut device: Option<ActiveSerial> = None;

    for comp in compatible.all() {
        match comp {
            "arm,pl011" => {
                let mut uart = pl011uart::Pl011Uart::new(base_vaddr);
                uart.init();
                uart.enable_interrupts();
                device = Some(ActiveSerial::Pl011(uart));
                break;
            }
            "amlogic,meson-gx-uart" | "amlogic,meson-s905-uart" => {
                let mut uart = meson_uart::MesonUart::new(base_vaddr);
                uart.init();
                uart.enable_interrupts();
                device = Some(ActiveSerial::Meson(uart));
                break;
            }
            _ => {}
        }
    }

    let dev = device.ok_or(Status::NOT_SUPPORTED)?;

    if let Ok(irq) = intrcntrl::parse_interrupt(node, 0)
    {
        interrupts::register_handler(irq,serial_interrupt_handler);
        intrcntrl::enable_irq(irq);
    }

    *GLOBAL_SERIAL.lock() = Some(dev);
    register_sink(&SERIAL_SINK)?;

    Ok(())
}

pub fn serial_interrupt_handler() {
    if let Some(dev) = GLOBAL_SERIAL.lock().as_mut() {
        while let Some(byte) = dev.read_byte()
        {
            let mut rxbuf = SERIAL_RX_BUFFER.lock();
            let _ = rxbuf.push(byte);
        }
    }
}

pub fn read_char() -> Option<u8>
{
    //
    // DISABLE INTERRUPTS
    //
    let state = unsafe {interrupt::save_and_disable_interrupts()};

    //
    // LOCK
    //
    let mut rxbuf = SERIAL_RX_BUFFER.lock();
    
    //
    // Storage buffer
    //
    let mut dest  = [0u8; 1];

    //
    // IF THE BUFFER HAS A BYTE IN IT,
    // WRITE IT INTO THE DEST[0]
    //
    let result = if rxbuf.read(&mut dest) > 0
    {
        Some(dest[0])
    } else {
        None
    };

    //
    // We need to release the lock now,
    // because we have to re-enable interrupts
    //
    drop(rxbuf);

    //
    // Turn interrupts back on
    //
    unsafe {interrupt::restore_interrupts(state)};

    result
}