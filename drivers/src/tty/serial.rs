use core::fmt::{self, Write};
use spin::Mutex;

pub mod pl011_uart;
pub mod s905x_uart;

use pl011_uart::Pl011Uart;
use s905x_uart::S905xUart;

//
// Maybe a trait is the better option,
// but there's no heap allocation yet..
// An enum pattern should be fine for now
// TODO:
//      Maybe use traits?
//
pub enum SerialDevice
{
    Pl011(Pl011Uart),
    S905x(S905xUart),
}

impl fmt::Write for SerialDevice {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        match self {
            SerialDevice::Pl011(uart) => uart.write_str(s),
            SerialDevice::S905x(uart) => uart.write_str(s),
        }
    }
}

static SERIAL: Mutex<Option<SerialDevice>> = Mutex::new(None);

pub fn init_from_dtb(dtb_ptr: *const u8, hhdm_offset: u64) -> bool {
    if dtb_ptr.is_null() {
        return false;
    }

    let fdt = match unsafe { fdt::Fdt::from_ptr(dtb_ptr) } {
        Ok(fdt) => fdt,
        Err(_) => return false,
    };

    for node in fdt.all_nodes() {
        let Some(compatible) = node.compatible() else { continue };
        let Some(reg) = node.reg().and_then(|mut r| r.next()) else { continue };

        let phys_base = reg.starting_address as u64;
        if phys_base == 0 {
            continue;
        }

        let vaddr = phys_base + hhdm_offset;

        for comp in compatible.all() {

            let dev = match comp {
                "arm,pl011" => {
                    Some(SerialDevice::Pl011(unsafe { Pl011Uart::new(vaddr) }))
                }
                "amlogic,meson-gx-uart" | "amlogic,meson-s905-uart" => {
                    Some(SerialDevice::S905x(unsafe { S905xUart::new(vaddr) }))
                }
                _ => None,
            };

            if let Some(uart) = dev {
                let mut lock = SERIAL.lock();
                *lock = Some(uart);
                return true;
            }
        }
    }

    false
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    let mut lock = SERIAL.lock();
    if let Some(ref mut serial) = *lock {
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
