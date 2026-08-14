use super::SerialDevice;

const REG_DR:    usize = 0x00;
const REG_FR:    usize = 0x18;
const REG_IBRD:  usize = 0x24;
const REG_FBRD:  usize = 0x28;
const REG_LCR_H: usize = 0x2C;
const REG_CR:    usize = 0x30;
const REG_IMSC:  usize = 0x38;

const FR_TXFF:      u32 = 1 << 5;
const FR_RXFE:      u32 = 1 << 4;
const LCR_H_FEN:    u32 = 1 << 4;
const LCR_H_WLEN_8: u32 = 3 << 5;
const CR_UARTEN:    u32 = 1 << 0;
const CR_TXE:       u32 = 1 << 8;
const CR_RXE:       u32 = 1 << 9;
const IMSC_RXIM:    u32 = 1 << 4;

pub struct Pl011Uart {
    vaddr: u64,
}

impl Pl011Uart {
    pub const fn new(vaddr: u64) -> Self {
        Self { vaddr }
    }

    #[inline]
    unsafe fn reg_ptr(&self, offset: usize) -> *mut u32 {
        (self.vaddr as usize + offset) as *mut u32
    }

    pub fn init(&mut self) {
        unsafe {
            core::ptr::write_volatile(self.reg_ptr(REG_CR), 0);
            core::ptr::write_volatile(self.reg_ptr(REG_IBRD), 13);
            core::ptr::write_volatile(self.reg_ptr(REG_FBRD), 2);
            core::ptr::write_volatile(self.reg_ptr(REG_LCR_H), LCR_H_FEN | LCR_H_WLEN_8);
            core::ptr::write_volatile(self.reg_ptr(REG_CR), CR_UARTEN | CR_TXE | CR_RXE);
        }
    }
}

impl SerialDevice for Pl011Uart {
    fn write_byte(&mut self, byte: u8) {
        unsafe {
            while (core::ptr::read_volatile(self.reg_ptr(REG_FR)) & FR_TXFF) != 0 {
                core::hint::spin_loop();
            }
            core::ptr::write_volatile(self.reg_ptr(REG_DR), byte as u32);
        }
    }

    fn read_byte(&mut self) -> Option<u8> {
        unsafe {
            if (core::ptr::read_volatile(self.reg_ptr(REG_FR)) & FR_RXFE) != 0 {
                None
            } else {
                Some(core::ptr::read_volatile(self.reg_ptr(REG_DR)) as u8)
            }
        }
    }

    fn enable_interrupts(&mut self) {
        unsafe {
            let imsc = core::ptr::read_volatile(self.reg_ptr(REG_IMSC));
            core::ptr::write_volatile(self.reg_ptr(REG_IMSC), imsc | IMSC_RXIM);
        }
    }
}