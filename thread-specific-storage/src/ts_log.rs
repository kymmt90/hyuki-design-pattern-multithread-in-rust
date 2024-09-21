use std::{fs::File, io::Write as _, path::Path};

pub struct TsLog {
    file: File,
}

impl TsLog {
    pub fn new(path: impl AsRef<Path>) -> Self {
        Self {
            file: File::create(path).unwrap(),
        }
    }

    pub fn println(&mut self, s: &str) {
        self.file.write_all(s.as_bytes()).unwrap();
    }
}

impl Drop for TsLog {
    fn drop(&mut self) {
        self.println("==== End of log ====");
    }
}
