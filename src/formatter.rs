use crate::Record;

// Specify the layour of logs in the final output
pub type Formatter = fn(record: &Record) -> String;

pub fn get_default_formatter() -> Formatter {
    default_formatter
}

fn default_formatter(record: &Record) -> String {
    format!(
        "{} | {} | {}",
        record.level(),
        record.name(),
        record.message()
    )
}
