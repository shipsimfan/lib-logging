use inner::Logger as LoggerInner;
use std::sync::{Arc, Mutex, MutexGuard, Once};

use crate::LogLevel;

mod inner;

// Interface for client application
#[derive(Clone)]
pub struct Logger(Arc<Mutex<LoggerInner>>);

static ROOT_LOGGER_INIT: Once = Once::new();
static mut ROOT_LOGGER: Option<Logger> = None;

pub fn get_logger<S: AsRef<str>>(name: S) -> Logger {
    let mut root_logger = get_root_logger();
    if name.as_ref() == "" {
        root_logger.clone()
    } else {
        root_logger.get_child(name)
    }
}

fn get_root_logger() -> Logger {
    ROOT_LOGGER_INIT.call_once(|| unsafe {
        ROOT_LOGGER = Some(Logger(Arc::new(Mutex::new(LoggerInner::new(None)))))
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

    pub fn get_child<S: AsRef<str>>(&mut self, suffix: S) -> super::Logger {
        let mut path: Vec<&str> = suffix.as_ref().split("::").collect();
        path.retain(|part| *part == "");
        match path.pop() {
            Some(child) => self.get_logger(child, path),
            None => panic!("Attempting to get empty child!"),
        }
    }

    pub fn set_level(&mut self, level: Option<LogLevel>) {
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

    fn logger(&self) -> MutexGuard<LoggerInner> {
        self.0.lock().unwrap()
    }

    fn get_logger<S1: AsRef<str>, S2: AsRef<str>>(
        &mut self,
        child: S1,
        mut path: Vec<S2>,
    ) -> super::Logger {
        let mut logger = self.logger();

        if !logger.children.contains_key(child.as_ref()) {
            logger.children.insert(
                child.as_ref().to_owned(),
                Logger(Arc::new(Mutex::new(LoggerInner::new(Some(self.clone()))))),
            );
        }

        let child_logger = logger.children.get_mut(child.as_ref()).unwrap();
        match path.pop() {
            Some(child) => child_logger.get_logger(child, path),
            None => child_logger.clone(),
        }
    }
}
