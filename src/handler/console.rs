use super::Handler;
use std::io::{stderr, Write};

// Sends logs to standard error
pub struct ConsoleHandler;

impl Handler for ConsoleHandler {
    fn flush(&mut self) {
        stderr().flush().ok();
    }

    fn emit(&mut self, record: &crate::Record, formatter: crate::Formatter) {
        let output = formatter(record);

        stderr().write_all(output.as_bytes()).ok();
        stderr().write_all(&[b'\n']).ok();
    }
}

impl ConsoleHandler {
    pub fn new() -> Box<dyn Handler> {
        Box::new(ConsoleHandler)
    }
}
