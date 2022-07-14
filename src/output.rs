use crate::{Formatter, Record};

pub trait Output: Send {
    fn write_record(&self, record: &Record, formatter: &dyn Formatter);
}

static mut OUTPUT: Option<Box<dyn Output>> = None;

/*
 * Sets the output location
 * Unsafe to use multi-threaded
 */
pub fn set_output<O: Output + 'static>(output: O) {
    unsafe { OUTPUT = Some(Box::new(output)) }
}

/*
 * Clears the logging output
 * Unsafe to use multi-threaded
 */
pub fn clear_output() {
    unsafe { OUTPUT = None }
}

pub fn output() -> Option<&'static dyn Output> {
    unsafe { OUTPUT.as_ref().map(|output| output.as_ref()) }
}
