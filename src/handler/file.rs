use std::{io::Write, path::Path};

// Sends logs to a file
pub struct FileHandler {
    file: std::fs::File,
}

impl FileHandler {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, std::io::Error> {
        let file = std::fs::OpenOptions::new().append(true).open(path)?;

        Ok(FileHandler { file })
    }
}

impl super::Handler for FileHandler {
    fn flush(&mut self) {
        self.file.flush().ok();
    }

    fn emit(&mut self, record: &crate::Record, formatter: crate::Formatter) {
        let output = formatter(record);

        self.file.write_all(output.as_bytes()).ok();
        self.file.write_all(&[b'\n']).ok();
    }
}
