#[cfg(feature = "potato")]
pub mod s905x_uart;

#[cfg(feature = "potato")]
pub use s905x_uart::S905xUart as SerialHardware;

#[cfg(feature = "qemu")]
pub mod pl011_uart;

#[cfg(feature = "qemu")]
pub use pl011_uart::Pl011Uart as SerialHardware;
