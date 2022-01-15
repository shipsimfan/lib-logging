use crate::Record;

// Provide a fine grained facility  for determining which log records to output
pub trait Filter {
    fn filter(&self, record: &Record) -> bool;
}
