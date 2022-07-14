mod filter;
mod formatter;
mod level;
mod output;
mod record;

pub use filter::{clear_filter, clear_logging, set_filter, set_log_level, Filter};
pub use formatter::{set_default_formatter, set_formatter, Formatter};
pub use level::*;
pub use output::{clear_output, set_output, Output};
pub use record::*;

#[doc(hidden)]
pub fn _log(
    name: &'static str,
    level: LogLevel,
    file: &'static str,
    line_number: u32,
    args: std::fmt::Arguments,
) {
    if !match filter::log_level() {
        Some(current_level) => current_level <= level,
        None => false,
    } {
        return;
    }

    let output = match output::output() {
        Some(output) => output,
        None => return,
    };

    let record = Record::new(name, level, file, line_number, format!("{}", args));

    if !match filter::filter() {
        Some(filter) => filter.filter(&record),
        None => true,
    } {
        return;
    }

    output.write_record(&record, formatter::formatter());
}

#[macro_export]
macro_rules! fatal {
    ($($arg:tt)*) => ($crate::_log(module_path!(),$crate::LogLevel::Fatal, file!(), line!(), format_args!($($arg)*)));
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => ($crate::_log(module_path!(),$crate::LogLevel::Error, file!(), line!(), format_args!($($arg)*)));
}

#[macro_export]
macro_rules! warning {
    ($($arg:tt)*) => ($crate::_log(module_path!(),$crate::LogLevel::Warning, file!(), line!(), format_args!($($arg)*)));
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => ($crate::_log(module_path!(),$crate::LogLevel::Informational, file!(), line!(), format_args!($($arg)*)));
}

#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => ($crate::_log(module_path!(),$crate::LogLevel::Debug, file!(), line!(), format_args!($($arg)*)));
}

#[macro_export]
macro_rules! trace {
    ($($arg:tt)*) => ($crate::_log(module_path!(),$crate::LogLevel::Trace, file!(), line!(), format_args!($($arg)*)));
}
