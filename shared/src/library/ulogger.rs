use core::fmt::Write;

pub mod sink;
pub mod record;

pub use record::{Level, LogRecord};
pub use sink::{register_sink, dispatch, LogSink};

//
// WOWZERS!!!
// A STACK STRING??
// NO MORE HEAP..
// IT'S A MIRACLE.. AND ALSO STUPID
//
pub struct StackString<const N: usize>
{
    buf: [u8; N],
    len: usize,
}

impl<const N: usize> StackString<N> {
    pub const fn new() -> Self {
        Self {
            buf: [0; N],
            len: 0,
        }
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.buf[..self.len]
    }
}

impl<const N: usize> Default for StackString<N> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const N: usize> core::fmt::Write for StackString<N> {

    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        let bytes = s.as_bytes();
        let remaining = self.buf.len() - self.len;

        if bytes.len() > remaining {
            return Err(core::fmt::Error);
        }

        self.buf[self.len..self.len + bytes.len()].copy_from_slice(bytes);
        self.len += bytes.len();

        Ok(())
    }
}

#[doc(hidden)]
pub fn __log(record: &LogRecord) {
    let mut fmt_buf = StackString::<512>::new();
    
    let _ = core::write!(
        &mut fmt_buf,
        "{}[{}] {}:{}{} ",
        record.level.color_code(),
        record.level.as_str(),
        record.file,
        record.line,
        crate::core::color::ANSI_RESET
    );

    let _ = core::fmt::write(&mut fmt_buf, record.args);
    let _ = core::writeln!(&mut fmt_buf, "{}", crate::core::color::ANSI_RESET);

    dispatch(fmt_buf.as_bytes());
}

#[doc(hidden)]
pub fn __print(args: core::fmt::Arguments) {
    let mut fmt_buf = StackString::<512>::new();
    let _ = core::fmt::write(&mut fmt_buf, args);
    dispatch(fmt_buf.as_bytes());
}

//
// ACTUAL API
// ONLY USE MACRO'S PLS..
//

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        $crate::library::ulogger::__print(format_args!($($arg)*))
    };
}

#[macro_export]
macro_rules! println {
    () => {
        $crate::print!("\n")
    };
    ($($arg:tt)*) => {
        $crate::print!("{}\n", format_args!($($arg)*))
    };
}

#[macro_export]
macro_rules! log {
    ($level:expr, $($arg:tt)*) => {
        $crate::library::ulogger::__log(&$crate::library::ulogger::LogRecord {
            level: $level,
            args: format_args!($($arg)*),
            file: file!(),
            line: line!(),
        })
    };
}

#[macro_export]
macro_rules! trace {
    ($($arg:tt)*) => {
        if cfg!(debug_assertions) {
            $crate::log!($crate::library::ulogger::Level::Trace, $($arg)*)
        }
    };
}

#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {
        if cfg!(debug_assertions) {
            $crate::log!($crate::library::ulogger::Level::Debug, $($arg)*)
        }
    };
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
        $crate::log!($crate::library::ulogger::Level::Info, $($arg)*)
    };
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {
        $crate::log!($crate::library::ulogger::Level::Warn, $($arg)*)
    };
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        $crate::log!($crate::library::ulogger::Level::Error, $($arg)*)
    };
}

#[macro_export]
macro_rules! fatal {
    ($($arg:tt)*) => {
        $crate::log!($crate::library::ulogger::Level::Fatal, $($arg)*);
        panic!($($arg)*);
    };
}