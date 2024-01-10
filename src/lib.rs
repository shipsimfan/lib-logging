//! A Rust logging library

#![deny(missing_docs)]

mod controller;
mod output;

pub use controller::{LogConfiguration, LogController};
pub use output::LogOutput;
