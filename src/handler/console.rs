use std::io::{stderr, Write};

// Sends logs to standard error
pub struct ConsoleHandler;

impl super::Handler for ConsoleHandler {
    fn flush(&mut self) {
        stderr().flush().ok();
    }

    fn emit(&mut self, record: &crate::Record, formatter: crate::Formatter) {
        let output = formatter(record);

        stderr().write_all(output.as_bytes()).ok();
        stderr().write_all(&[b'\n']).ok();
    }
}
