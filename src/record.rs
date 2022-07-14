use crate::LogLevel;
use chrono::{DateTime, Local};

#[derive(Debug)]
pub struct Record {
    name: &'static str,
    level: LogLevel,
    pathname: &'static str,
    line_number: u32,
    message: String,
    timestamp: DateTime<Local>,
}

impl Record {
    pub fn new(
        name: &'static str,
        level: LogLevel,
        pathname: &'static str,
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
        self.name
    }

    pub fn level(&self) -> LogLevel {
        self.level
    }

    pub fn pathname(&self) -> &str {
        self.pathname
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
