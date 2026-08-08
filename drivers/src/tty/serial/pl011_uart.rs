//
// This is the serial driver for QEMU emulation
//
use core::fmt;

//
// TODO:
//   Right now, we hard-code the address
//   later, we need to instead parse the DTB
//   this is temporary, and works fine,
//   but parsing the DTB is a much better way
//
pub const UART_BASE_ADDRESS: u64 = 0x0900_0000;


const PL011_DR:   u64 = 0x00;
const PL011_FR:   u64 = 0x18;
const PL011_TXFF: u32 = 1 << 5;

pub struct Pl011Uart {
    vaddr: u64,
}

impl Pl011Uart {
    pub const unsafe fn new(hhdm_offset: u64) -> Self {
        Self {
            vaddr: UART_BASE_ADDRESS + hhdm_offset,
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
