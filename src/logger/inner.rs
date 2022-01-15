use crate::LogLevel;
use std::collections::HashMap;

pub struct Logger {
    pub parent: Option<super::Logger>,
    pub children: HashMap<String, super::Logger>,
    pub propagate: bool,
    pub level: Option<LogLevel>,
}

impl Logger {
    pub fn new(parent: Option<super::Logger>) -> Self {
        Logger {
            parent,
            children: HashMap::new(),
            propagate: true,
            level: None,
        }
    }
}
