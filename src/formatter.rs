use crate::Record;
use std::sync::Once;

pub type Formatter = fn(record: &Record) -> String;

static FORMATTER_INIT: Once = Once::new();
static mut FORMATTER: Formatter = default_format;

/*
 * Sets the formatter
 */
pub fn set_formatter(formatter: Formatter) {
    if FORMATTER_INIT.is_completed() {
        panic!("Attempting to set more than one formatter");
    }

    FORMATTER_INIT.call_once(|| unsafe { FORMATTER = formatter });
}

pub fn formatter() -> Formatter {
    unsafe { FORMATTER }
}

fn default_format(record: &Record) -> String {
    format!(
        "[{}][{}] {}: {}",
        record.pathname(),
        record.timestamp(),
        record.level(),
        record.message()
    )
}
