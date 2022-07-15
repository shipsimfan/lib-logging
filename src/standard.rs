use crate::{set_output, Formatter, Record};

pub fn init_standard_error() {
    set_output(standard_error_output);
}

pub fn init_standard_output() {
    set_output(standard_output_output);
}

pub fn standard_error_output(record: &Record, formatter: Formatter) {
    eprintln!("{}", formatter(record))
}

pub fn standard_output_output(record: &Record, formatter: Formatter) {
    println!("{}", formatter(record))
}
