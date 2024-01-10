//! A Rust logging library

#![deny(missing_docs)]

mod controller;
mod message;
mod output;

pub use controller::LogController;
pub use message::LogMessage;
pub use output::LogOutput;
