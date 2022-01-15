use crate::{handler::HandlerContainer, Filter, LogLevel};
use std::collections::HashMap;

pub struct Logger {
    pub name: String,
    pub parent: Option<super::Logger>,
    pub children: HashMap<String, super::Logger>,
    pub propagate: bool,
    pub level: Option<LogLevel>,
    pub filters: Vec<(usize, Box<dyn Filter>)>,
    pub filter_idx: usize,
    pub handlers: Vec<(usize, HandlerContainer)>,
    pub handler_idx: usize,
}

impl Logger {
    pub fn new(name: String, parent: Option<super::Logger>) -> Self {
        Logger {
            name,
            parent,
            children: HashMap::new(),
            propagate: true,
            level: None,
            filters: Vec::new(),
            filter_idx: 0,
            handlers: Vec::new(),
            handler_idx: 0,
        }
    }
}
