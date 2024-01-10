use crate::LogMessage;

/// A place for [`LogMessage`]s to be written
pub trait LogOutput: 'static + Send {
    /// Write a [`LogMessage`] to this output
    fn write(&mut self, message: &dyn LogMessage);
}
