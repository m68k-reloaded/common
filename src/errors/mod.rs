use crate::Range;

pub struct Error {
    pub range: Range,
    pub message: String,
}

pub struct ErrorCollector {
    errors: Vec<Error>,
}

impl ErrorCollector {
    pub fn push(&mut self, error: Error) {
        self.errors.push(error);
    }

    pub fn dump(&self) {
        for error in &self.errors {
            // TODO(marcelgarus): print the error
            println!("Error at byte {}: {}", error.range.start, error.message);
        }
    }
}
