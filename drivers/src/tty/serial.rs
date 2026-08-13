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

use core::fmt::{self};
use shared::library::ulogger;
use spin::Mutex;

use shared::core::types::status::{KResult, Status};
use shared::library::ulogger::sink::LogSink;

#[derive(Clone, Copy)]
pub struct UartConfig {
    pub dr_offset: u64,
    pub status_offset: u64,
    pub tx_full_mask: u32,
}

impl UartConfig {
    pub const PL011: Self = Self {
        dr_offset: 0x00,
        status_offset: 0x18,
        tx_full_mask: 1 << 5,
    };

    pub const S905X: Self = Self {
        dr_offset: 0x00,
        status_offset: 0x0C,
        tx_full_mask: 1 << 21,
    };
}

pub struct Uart {
    vaddr: u64,
    config: UartConfig,
}

impl Uart {
    pub const unsafe fn new(vaddr: u64, config: UartConfig) -> Self {
        Self { vaddr, config }
    }

    pub fn write_byte(&self, byte: u8) {
        let uart_ptr = self.vaddr as *mut u32;

        unsafe {
            let dr = uart_ptr.add((self.config.dr_offset / 4) as usize);
            let status = uart_ptr.add((self.config.status_offset / 4) as usize);

            while (core::ptr::read_volatile(status) & self.config.tx_full_mask) != 0 {
                core::hint::spin_loop();
            }

            core::ptr::write_volatile(dr, byte as u32);
        }
    }

    pub fn write_str_raw(&self, string: &str) {
        for byte in string.bytes() {
            if byte == b'\n' {
                self.write_byte(b'\r');
            }
            self.write_byte(byte);
        }
    }
}

impl fmt::Write for Uart {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_str_raw(s);
        Ok(())
    }
}

pub struct SerialSink;

impl LogSink for SerialSink {
    fn write(&self, data: &[u8]) {
        if let Some(uart) = SERIAL.lock().as_ref() {
            for &byte in data {
                if byte == b'\n' {
                    uart.write_byte(b'\r');
                }
                uart.write_byte(byte);
            }
        }
    }
}

static SERIAL: Mutex<Option<Uart>> = Mutex::new(None);
pub static SERIAL_SINK: SerialSink = SerialSink;

pub fn init_from_dtb(compatible: &fdt::standard_nodes::Compatible, vaddr: u64) -> KResult<()> {
    for comp in compatible.all() {
        let config = match comp {
            "arm,pl011" => Some(UartConfig::PL011),
            "amlogic,meson-gx-uart" | "amlogic,meson-s905-uart" => Some(UartConfig::S905X),
            _ => None,
        };

        if let Some(cfg) = config {
            let uart = unsafe { Uart::new(vaddr, cfg) };
            *SERIAL.lock() = Some(uart);

            ulogger::register_sink(&SERIAL_SINK)?;

            return Ok(());
        }
    }

    Err(Status::NOT_SUPPORTED)
}