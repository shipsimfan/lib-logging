use crate::{LogLevel, Record};
use std::sync::Once;

pub type Filter = fn(record: &Record) -> bool;

static FILTER_INIT: Once = Once::new();
static mut FILTER: Option<Filter> = None;

static LOG_INIT: Once = Once::new();
static mut LOG_LEVEL: Option<LogLevel> = None;

/*
 * Sets the filter
 */
pub fn set_filter(filter: Filter) {
    if FILTER_INIT.is_completed() {
        panic!("Attempting to set more than one filter");
    }
    FILTER_INIT.call_once(|| unsafe { FILTER = Some(filter) })
}

pub fn filter() -> Option<Filter> {
    unsafe { FILTER }
}

/*
 * Sets the minimum log level for output
 */
pub fn set_minimum_log_level(level: LogLevel) {
    if LOG_INIT.is_completed() {
        panic!("Attempting to set more than one minimum log level");
    }
    LOG_INIT.call_once(|| unsafe { LOG_LEVEL = Some(level) })
}

pub fn minimum_log_level() -> Option<LogLevel> {
    unsafe { LOG_LEVEL }
}
