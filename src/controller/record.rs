use crate::LogMessage;

/// A single log message ready to be sent to the log output thread
pub(super) struct LogRecord {
    message: Box<dyn LogMessage>,
}
