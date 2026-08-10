use core::fmt::{self, Write};
use shared::types::status::{KResult, Status};
use spin::Mutex;

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

static SERIAL: Mutex<Option<Uart>> = Mutex::new(None);

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
            return Ok(());
        }
    }

    Err(Status::NOT_SUPPORTED)
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    if let Some(ref mut serial) = *SERIAL.lock() {
        let _ = serial.write_fmt(args);
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        $crate::tty::serial::_print(format_args!($($arg)*));
    };
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\r\n"));
    ($($arg:tt)*) => {
        $crate::print!($($arg)*);
        $crate::print!("\r\n");
    };
}