use core::fmt::{self, Write};
use spin::Mutex;

#[cfg(feature = "potato")]
pub mod s905x_uart;

#[cfg(feature = "potato")]
pub use s905x_uart::S905xUart as SerialHardware;

#[cfg(feature = "qemu")]
pub mod pl011_uart;

#[cfg(feature = "qemu")]
pub use pl011_uart::Pl011Uart as SerialHardware;

static SERIAL: Mutex<Option<SerialHardware>> = Mutex::new(None);

pub fn init(hhdm_offset: u64) {
    let uart = unsafe { SerialHardware::new(hhdm_offset) };

    let mut lock = SERIAL.lock();
    *lock = Some(uart);
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
