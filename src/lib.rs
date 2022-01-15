/*
 *  Rust Logging Library
 *      Inspired by Python 3 standard logging module
 *      https://docs.python.org/3/library/logging.html
 */

mod filter;
mod formatter;
mod handler;
mod logger;
mod record;

pub use filter::*;
pub use formatter::*;
pub use handler::*;
pub use logger::*;
pub use record::*;

#[doc(hidden)]
pub fn _log(
    logger: Logger,
    level: LogLevel,
    file: &str,
    line_number: u32,
    args: std::fmt::Arguments,
) {
    logger.log(level, file.to_owned(), line_number, format!("{}", args))
}

#[macro_export]
macro_rules! emergency {
    ($logger:expr, $($arg:tt)*) => ($crate::_log($logger, $crate::LogLevel::Emergency, file!(), line!(), format_args!($($arg)*)));
}

#[macro_export]
macro_rules! alert {
    ($logger:expr, $($arg:tt)*) => ($crate::_log($logger, $crate::LogLevel::Alert, file!(), line!(), format_args!($($arg)*)));
}

#[macro_export]
macro_rules! critical {
    ($logger:expr, $($arg:tt)*) => ($crate::_log($logger, $crate::LogLevel::Critical, file!(), line!(), format_args!($($arg)*)));
}

#[macro_export]
macro_rules! error {
    ($logger:expr, $($arg:tt)*) => ($crate::_log($logger, $crate::LogLevel::Error, file!(), line!(), format_args!($($arg)*)));
}

#[macro_export]
macro_rules! warning {
    ($logger:expr, $($arg:tt)*) => ($crate::_log($logger, $crate::LogLevel::Warning, file!(), line!(), format_args!($($arg)*)));
}

#[macro_export]
macro_rules! notice {
    ($logger:expr, $($arg:tt)*) => ($crate::_log($logger, $crate::LogLevel::Notice, file!(), line!(), format_args!($($arg)*)));
}

#[macro_export]
macro_rules! info {
    ($logger:expr, $($arg:tt)*) => ($crate::_log($logger, $crate::LogLevel::Informational, file!(), line!(), format_args!($($arg)*)));
}

#[macro_export]
macro_rules! debug {
    ($logger:expr, $($arg:tt)*) => ($crate::_log($logger, $crate::LogLevel::Debug, file!(), line!(), format_args!($($arg)*)));
}
