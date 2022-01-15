use chrono::{DateTime, Local};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum LogLevel {
    Emergency = 0,
    Alert = 1,
    Critical = 2,
    Error = 3,
    Warning = 4,
    Notice = 5,
    Informational = 6,
    Debug = 7,
}

pub struct Record {
    name: String,
    level: LogLevel,
    pathname: String,
    line_number: u32,
    message: String,
    timestamp: DateTime<Local>,
}

impl Record {
    pub fn new(
        name: String,
        level: LogLevel,
        pathname: String,
        line_number: u32,
        message: String,
    ) -> Self {
        Record {
            name,
            level,
            pathname,
            line_number,
            message,
            timestamp: Local::now(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn level(&self) -> LogLevel {
        self.level
    }

    pub fn file(&self) -> &str {
        &self.pathname
    }

    pub fn line_number(&self) -> u32 {
        self.line_number
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn timestamp(&self) -> &DateTime<Local> {
        &self.timestamp
    }
}

impl std::fmt::Display for LogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                LogLevel::Emergency => "EMERGENCY",
                LogLevel::Alert => "ALERT",
                LogLevel::Critical => "CRITICAL",
                LogLevel::Error => "ERROR",
                LogLevel::Warning => "WARNING",
                LogLevel::Notice => "NOTICE",
                LogLevel::Informational => "INFO",
                LogLevel::Debug => "DEBUG",
            }
        )
    }
}
