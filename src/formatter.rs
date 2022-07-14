use crate::Record;

pub trait Formatter: Send {
    fn format(&self, record: &Record) -> String;
}

struct DefaultFormatter;

static mut FORMATTER: Option<Box<dyn Formatter>> = None;

/*
 * Sets the formatter
 * Unsafe to use multi-threaded
 */
pub fn set_formatter<F: Formatter + 'static>(formatter: F) {
    unsafe { FORMATTER = Some(Box::new(formatter)) }
}

/*
 * Sets the formatter to the default formatter
 * Unsafe to use multi-threaded
 */
pub fn set_default_formatter() {
    unsafe { FORMATTER = Some(Box::new(DefaultFormatter)) }
}

pub fn formatter() -> &'static dyn Formatter {
    unsafe {
        match &FORMATTER {
            Some(formatter) => formatter.as_ref(),
            None => {
                set_default_formatter();
                FORMATTER.as_ref().unwrap().as_ref()
            }
        }
    }
}

impl Formatter for DefaultFormatter {
    fn format(&self, record: &Record) -> String {
        format!(
            "[{}][{}] {}: {}",
            record.pathname(),
            record.timestamp(),
            record.level(),
            record.message()
        )
    }
}
