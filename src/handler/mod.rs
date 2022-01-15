mod console;
mod file;
//mod socket;

pub use console::ConsoleHandler;
pub use file::FileHandler;
//pub use socket::SocketHandler;

// Responsible for sending the logs to the appropriate destination
pub trait Handler {}
