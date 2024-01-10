use crate::LogOutput;
use record::LogRecord;
use thread::LogOutputThread;

mod record;
mod thread;

/// Manages the logging output thread and creates [`Logger`]s
pub struct LogController {
    log_output_thread: LogOutputThread,
}

impl LogController {
    /// The size of the queue when using a `default()` [`LogController`]
    pub const DEFAULT_QUEUE_SIZE: usize = 256;

    /// Creates a new [`LogController`]
    pub fn new(log_queue_size: usize, outputs: Vec<Box<dyn LogOutput>>) -> Self {
        let log_output_thread = LogOutputThread::new(log_queue_size, outputs);

        LogController { log_output_thread }
    }
}

impl Default for LogController {
    fn default() -> Self {
        // TODO: Add `stdout` and `stderr` to outputs
        LogController::new(Self::DEFAULT_QUEUE_SIZE, Vec::new())
    }
}
