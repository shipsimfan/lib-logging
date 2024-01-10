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
    pub fn new(log_queue_size: usize) -> Self {
        let log_output_thread = LogOutputThread::new(log_queue_size);

        LogController { log_output_thread }
    }
}

impl Default for LogController {
    fn default() -> Self {
        LogController::new(Self::DEFAULT_QUEUE_SIZE)
    }
}
