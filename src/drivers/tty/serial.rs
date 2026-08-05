use core::fmt::{self, Write};

#[cfg(feature = "potato")]
pub mod s905x_uart;

#[cfg(feature = "potato")]
pub use s905x_uart::S905xUart as SerialHardware;

#[cfg(feature = "qemu")]
pub mod pl011_uart;

#[cfg(feature = "qemu")]
pub use pl011_uart::Pl011Uart as SerialHardware;

struct UnsafeSerial(Option<SerialHardware>);
unsafe impl Sync for UnsafeSerial {}

static mut SERIAL: UnsafeSerial = UnsafeSerial(None);

pub fn init(hhdm_offset: u64) {
    let uart = unsafe { SerialHardware::new(hhdm_offset) };
    unsafe {
        SERIAL.0 = Some(uart);
    }
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    unsafe {
        if let Some(ref mut serial) = SERIAL.0 {
            let _ = serial.write_fmt(args);
        }
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        $crate::drivers::tty::serial::_print(format_args!($($arg)*));
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