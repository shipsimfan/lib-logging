use crate::{get_default_formatter, Filter, Formatter, LogLevel, Record};

mod console;
mod file;
//mod socket;

pub use console::ConsoleHandler;
pub use file::FileHandler;
//pub use socket::SocketHandler;

// Responsible for sending the logs to the appropriate destination
pub trait Handler {
    fn emit(&mut self, record: &Record, formatter: Formatter);
    fn flush(&mut self);
}

pub struct HandlerContainer {
    level: Option<LogLevel>,
    formatter: Option<Formatter>,
    filters: Vec<(usize, Box<dyn Filter>)>,
    filter_idx: usize,
    inner: Box<dyn Handler>,
}

impl HandlerContainer {
    pub fn new(inner: Box<dyn Handler>) -> Self {
        HandlerContainer {
            level: None,
            formatter: None,
            filters: Vec::new(),
            filter_idx: 0,
            inner,
        }
    }

    pub fn set_level(&mut self, level: Option<LogLevel>) {
        self.level = level;
    }

    pub fn set_formatter(&mut self, formatter: Option<Formatter>) {
        self.formatter = formatter;
    }

    pub fn add_filter(&mut self, filter: Box<dyn Filter>) -> usize {
        let ret = self.filter_idx;
        self.filters.push((ret, filter));
        self.filter_idx += 1;

        ret
    }

    pub fn remove_filter(&mut self, idx: usize) {
        self.filters.retain(|(filter_idx, _)| *filter_idx != idx);
    }

    pub fn filter(&self, record: &Record) -> bool {
        match self.level {
            Some(level) => {
                if record.level() > level {
                    return false;
                }
            }
            None => {}
        }

        for (_, filter) in &self.filters {
            if !filter.filter(record) {
                return false;
            }
        }

        true
    }

    pub fn flush(&mut self) {
        self.inner.flush();
    }

    pub fn handle(&mut self, record: &Record) {
        if self.filter(record) {
            self.emit(record);
        }
    }

    pub fn format(&self, record: &Record) -> String {
        match self.formatter {
            Some(formatter) => formatter(record),
            None => get_default_formatter()(record),
        }
    }

    pub fn emit(&mut self, record: &Record) {
        self.inner.emit(
            record,
            match self.formatter {
                Some(formatter) => formatter,
                None => get_default_formatter(),
            },
        );
    }
}
