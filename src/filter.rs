use crate::{LogLevel, Record};

pub trait Filter: Send {
    fn filter(&self, record: &Record) -> bool;
}

/*
 * In almost all use case for loggers, setting of these variables will not
 * be done in a situation for race conditions. That is why these variables
 * are not protected by guards.
 */

static mut FILTER: Option<Box<dyn Filter>> = None;
static mut LOG_LEVEL: Option<LogLevel> = None;

/*
 * Sets the filter
 * Unsafe to use multi-threaded
 */
pub fn set_filter(filter: Box<dyn Filter>) {
    unsafe { FILTER = Some(filter) }
}

/*
 * Removes the filter
 * Unsafe to use multi-threaded
 */
pub fn clear_filter() {
    unsafe { FILTER = None }
}

pub fn filter() -> Option<&'static dyn Filter> {
    unsafe { FILTER.as_ref().map(|filter| filter.as_ref()) }
}

/*
 * Sets the minimum log level for output
 * Unsafe to use multi-threaded
 */
pub fn set_log_level(level: LogLevel) {
    unsafe { LOG_LEVEL = Some(level) }
}

/*
 * Stops all logging output
 * Unsafe to use multi-threaded
 */
pub fn clear_logging() {
    unsafe { LOG_LEVEL = None }
}

pub fn log_level() -> Option<LogLevel> {
    unsafe { LOG_LEVEL }
}
