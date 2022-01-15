/*
 *  Rust Logging Library
 *      Inspired by Python 3 standard logging module
 *      https://docs.python.org/3/library/logging.html
 */

mod filter;
mod formatter;
mod handler;
mod logger;

pub use filter::*;
pub use formatter::*;
pub use handler::*;
pub use logger::*;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum LogLevel {
    Emergency = 0,
    Alert = 1,
    Critical = 2,
    Error = 3,
    Warning = 4,
    Notice = 5,
    Informational = 6,
    Debug = 7,
}
