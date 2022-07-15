use crate::{Formatter, Record};
use core::panic;
use std::sync::Once;

pub type Output = fn(record: &Record, formatter: Formatter);

static OUTPUT_INIT: Once = Once::new();
static mut OUTPUT: Option<Output> = None;

/*
 * Sets the output location
 */
pub fn set_output(output: Output) {
    if OUTPUT_INIT.is_completed() {
        panic!("Attempting to set more than log output");
    }

    OUTPUT_INIT.call_once(|| unsafe { OUTPUT = Some(output) })
}

pub fn output() -> Option<Output> {
    unsafe { OUTPUT }
}
