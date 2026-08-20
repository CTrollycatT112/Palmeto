// SPDX-License-Identifier: GPL-2.0-or-later
//
// Purpose: This module handles the serial driver for LIBRE POTATO BOARD (meson),
//          we must seperate serial drivers from each other because they all (i think?) use different constants,
//          although serial.rs will act as a shared super module,
//          these just provide the implementation for their specific target (meson for example)
//
use super::SerialDevice;

const REG_WFIFO:   usize   = 0x00;
const REG_RFIFO:   usize   = 0x04;
const REG_CONTROL: usize   = 0x08;
const REG_STATUS:  usize   = 0x0C;

const STATUS_TXFULL:     u32 = 1 << 21;
const STATUS_RXEMPTY:    u32 = 1 << 20;
const CONTROL_RESET_TX:  u32 = 1 << 22;
const CONTROL_RESET_RX:  u32 = 1 << 23;
const CONTROL_RX_INT_EN: u32 = 1 << 1;

pub struct MesonUart {
    vaddr: u64,
}

impl MesonUart {
    pub const fn new(vaddr: u64) -> Self {
        Self { vaddr }
    }

    #[inline]
    unsafe fn reg_ptr(&self, offset: usize) -> *mut u32 {
        (self.vaddr as usize + offset) as *mut u32
    }
}

impl SerialDevice for MesonUart {
    fn init(&mut self) {
        unsafe {
            let ptr = self.reg_ptr(REG_CONTROL);
            let val = core::ptr::read_volatile(ptr);
            core::ptr::write_volatile(ptr, val | CONTROL_RESET_TX | CONTROL_RESET_RX);
        }
    }
    
    fn write_byte(&mut self, byte: u8) {
        unsafe {
            while (core::ptr::read_volatile(self.reg_ptr(REG_STATUS)) & STATUS_TXFULL) != 0 {
                core::hint::spin_loop();
            }
            core::ptr::write_volatile(self.reg_ptr(REG_WFIFO), byte as u32);
        }
    }

    fn read_byte(&mut self) -> Option<u8> {
        unsafe {
            if (core::ptr::read_volatile(self.reg_ptr(REG_STATUS)) & STATUS_RXEMPTY) != 0 {
                None
            } else {
                Some(core::ptr::read_volatile(self.reg_ptr(REG_RFIFO)) as u8)
            }
        }
    }

    fn enable_interrupts(&mut self) {
        unsafe {
            let ptr = self.reg_ptr(REG_CONTROL);
            let val = core::ptr::read_volatile(ptr);
            core::ptr::write_volatile(ptr, val | CONTROL_RX_INT_EN);
        }
    }
}