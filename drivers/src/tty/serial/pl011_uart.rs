// SPDX-License-Identifier: Apache-2.0
//
// Copyright (c) 2026 Palmeto OS Team
//
// Author:      Trollycat
//
// Module:      PL011 UART DRIVER
//
// Description: The PL011 UART DRIVER Is for QEMU aarch64              
//

use core::fmt;

const PL011_DR:   u64 = 0x00;
const PL011_FR:   u64 = 0x18;
const PL011_TXFF: u32 = 1 << 5;

pub struct Pl011Uart {
    vaddr: u64,
}

impl Pl011Uart {
    pub const unsafe fn new(vaddr: u64) -> Self {
        Self {
            vaddr
        }
    }

    pub fn write_byte(&self, byte: u8) {
        let uart_ptr = self.vaddr as *mut u32;
        
        unsafe {
            let dr = uart_ptr.add((PL011_DR / 4) as usize);
            let fr = uart_ptr.add((PL011_FR / 4) as usize);

            while (core::ptr::read_volatile(fr) & PL011_TXFF) != 0 {
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

impl fmt::Write for Pl011Uart {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_str_raw(s);
        Ok(())
    }
}
