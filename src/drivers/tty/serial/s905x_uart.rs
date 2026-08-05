//
// This is the serial driver for the libre potato board
//
use core::fmt;

//
// TODO:
//   Right now, we hard-code the address
//   later, we need to instead parse the DTB
//   this is temporary, and works fine,
//   but parsing the DTB is a much better way
//
pub const UART_BASE_ADDRESS: u64 = 0xC810_0400;

const UART_WFIFO:   u64 = 0x0C;
const UART_STATUS:  u64 = 0x0C;
const UART_TX_FULL: u32 = 1 << 21;

pub struct S905xUart {
    vaddr: u64,
}

impl S905xUart {
    pub const unsafe fn new(hhdm_offset: u64) -> Self {
        Self {
            vaddr: UART_BASE_ADDRESS + hhdm_offset,
        }
    }

    pub fn write_byte(&self, byte: u8) {
        let uart_ptr = self.vaddr as *mut u32;
        
        unsafe {
            let wfifo = uart_ptr.add((UART_WFIFO / 4) as usize);
            let status = uart_ptr.add((UART_STATUS / 4) as usize);

            while (core::ptr::read_volatile(status) & UART_TX_FULL) != 0 {
                core::hint::spin_loop();
            }

            core::ptr::write_volatile(wfifo, byte as u32);
        }
    }

    pub fn write_str_raw(&self, string: &str) {
        for byte in string.bytes() {
            self.write_byte(byte);
            if byte == b'\n' {
                self.write_byte(b'\r');
            }
        }
    }
}

impl fmt::Write for S905xUart {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_str_raw(s);
        Ok(())
    }
}
