use crate::LogOutput;
use named::Named;
use std::{borrow::Cow, cell::RefCell, rc::Rc};

mod named;

/// A configuration of [`LogOutput`]s and [`LogCategory`]s
pub struct LogConfiguration {
    outputs: Named<Rc<RefCell<dyn LogOutput>>>,
}

impl LogConfiguration {
    /// Creates a new empty [`LogConfiguration`]
    ///
    /// This will not output anything as is, use `LogConfiguration::default()` for a simple logger
    /// that outputs to stdout and stderr
    pub fn new() -> Self {
        LogConfiguration {
            outputs: Named::new(),
        }
    }

    /// Inserts a new [`LogOutput`] into this configuration
    ///
    /// Returns the parameters if there already is a [`LogOutput`] with `name`
    pub fn insert_output<S: Into<Cow<'static, str>>, O: LogOutput>(
        &mut self,
        name: S,
        output: O,
    ) -> Option<(Cow<'_, str>, O)> {
        let name = name.into();

        if self.outputs.contains(name.as_ref()) {
            return Some((name, output));
        }

        self.outputs.insert(name, Rc::new(RefCell::new(output)));
        None
    }
}
