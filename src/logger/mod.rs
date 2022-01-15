use crate::{handler::HandlerContainer, Filter, Handler, LogLevel, Record};
use inner::Logger as LoggerInner;
use std::sync::{Arc, Mutex, MutexGuard, Once};

mod inner;

// Interface for client application
#[derive(Clone)]
pub struct Logger(Arc<Mutex<LoggerInner>>);

static ROOT_LOGGER_INIT: Once = Once::new();
static mut ROOT_LOGGER: Option<Logger> = None;

pub fn get_logger<S: AsRef<str>>(name: S) -> Logger {
    let root_logger = get_root_logger();
    if name.as_ref() == "" {
        root_logger.clone()
    } else {
        root_logger.get_child(name)
    }
}

fn get_root_logger() -> Logger {
    ROOT_LOGGER_INIT.call_once(|| unsafe {
        let logger = Logger(Arc::new(Mutex::new(LoggerInner::new(String::new(), None))));
        logger.add_handler(crate::ConsoleHandler::new());
        logger.set_level(Some(LogLevel::Warning));

        ROOT_LOGGER = Some(logger);
    });
    unsafe { ROOT_LOGGER.as_ref().unwrap() }.clone()
}

impl Logger {
    pub fn propagate(&self) -> bool {
        self.logger().propagate
    }

    pub fn set_propagate(&self, propagate: bool) {
        self.logger().propagate = propagate
    }

    pub fn set_level(&self, level: Option<LogLevel>) {
        self.logger().level = level
    }

    pub fn is_enabled_for(&self, level: LogLevel) -> bool {
        level <= self.get_effective_level()
    }

    pub fn get_effective_level(&self) -> LogLevel {
        let logger = self.logger();
        match logger.level {
            Some(level) => level,
            None => match &logger.parent {
                Some(parent) => parent.get_effective_level(),
                None => LogLevel::Debug,
            },
        }
    }

    pub fn get_child<S: AsRef<str>>(&self, suffix: S) -> super::Logger {
        let mut path: Vec<&str> = suffix.as_ref().split("::").collect();
        path.retain(|part| *part == "");
        match path.pop() {
            Some(child) => self.get_logger(child, path),
            None => panic!("Attempting to get empty child!"),
        }
    }

    pub fn emergency(&self, message: String, file: String, line_number: u32) {
        self.log(LogLevel::Emergency, file, line_number, message);
    }

    pub fn alert(&self, message: String, file: String, line_number: u32) {
        self.log(LogLevel::Alert, file, line_number, message);
    }

    pub fn critical(&self, message: String, file: String, line_number: u32) {
        self.log(LogLevel::Critical, file, line_number, message);
    }

    pub fn error(&self, message: String, file: String, line_number: u32) {
        self.log(LogLevel::Error, file, line_number, message);
    }

    pub fn warning(&self, message: String, file: String, line_number: u32) {
        self.log(LogLevel::Warning, file, line_number, message);
    }

    pub fn notice(&self, message: String, file: String, line_number: u32) {
        self.log(LogLevel::Notice, file, line_number, message);
    }

    pub fn info(&self, message: String, file: String, line_number: u32) {
        self.log(LogLevel::Informational, file, line_number, message);
    }

    pub fn debug(&self, message: String, file: String, line_number: u32) {
        self.log(LogLevel::Debug, file, line_number, message);
    }

    pub fn log(&self, level: LogLevel, file: String, line_number: u32, message: String) {
        let record = {
            let logger = self.logger();
            Record::new(logger.name.clone(), level, file, line_number, message)
        };

        self.handle(record)
    }

    pub fn add_filter(&self, filter: Box<dyn Filter>) -> usize {
        let mut logger = self.logger();

        let ret = logger.filter_idx;
        logger.filters.push((ret, filter));
        logger.filter_idx += 1;

        ret
    }

    pub fn remove_filter(&self, idx: usize) {
        self.logger()
            .filters
            .retain(|(filter_idx, _)| *filter_idx != idx);
    }

    pub fn filter(&self, record: &Record) -> bool {
        let logger = self.logger();
        for (_, filter) in &logger.filters {
            if !filter.filter(record) {
                return false;
            }
        }

        true
    }

    pub fn add_handler(&self, handler: Box<dyn Handler>) -> usize {
        let mut logger = self.logger();

        let ret = logger.handler_idx;
        logger.handlers.push((ret, HandlerContainer::new(handler)));
        logger.handler_idx += 1;

        ret
    }

    pub fn remove_handler(&self, idx: usize) {
        self.logger()
            .handlers
            .retain(|(handler_idx, _)| *handler_idx != idx);
    }

    pub fn handle(&self, record: Record) {
        if self.filter(&record) {
            self.propagate_handle(record);
        }
    }

    pub fn has_handlers(&self) -> bool {
        self.logger().handlers.len() != 0
    }

    fn propagate_handle(&self, record: Record) {
        let mut logger = self.logger();

        // Send to our handlers
        for (_, handler) in &mut logger.handlers {
            handler.handle(&record)
        }

        // Propagate to parent
        if logger.propagate {
            match &logger.parent {
                Some(parent) => parent.propagate_handle(record),
                None => {}
            }
        }
    }

    fn logger(&self) -> MutexGuard<LoggerInner> {
        self.0.lock().unwrap()
    }

    fn get_logger<S1: AsRef<str>, S2: AsRef<str>>(
        &self,
        child: S1,
        mut path: Vec<S2>,
    ) -> super::Logger {
        let mut logger = self.logger();

        if !logger.children.contains_key(child.as_ref()) {
            let new_name = if logger.name.as_str() == "" {
                child.as_ref().to_owned()
            } else {
                format!("{}::{}", logger.name, child.as_ref())
            };

            logger.children.insert(
                child.as_ref().to_owned(),
                Logger(Arc::new(Mutex::new(LoggerInner::new(
                    new_name,
                    Some(self.clone()),
                )))),
            );
        }

        let child_logger = logger.children.get_mut(child.as_ref()).unwrap();
        match path.pop() {
            Some(child) => child_logger.get_logger(child, path),
            None => child_logger.clone(),
        }
    }
}
