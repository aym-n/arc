use std::process::exit;

// TODO: implement error handling with Result<T, E>
pub struct Error {
    pub line: usize,
    pub message: String,
}

impl Error {
    pub fn new(line: usize, message: String) -> Self {
        Error {
            line,
            message,
        }
    }

    pub fn report(&self) {
        eprintln!("[line {}] Error: {}", self.line, self.message);
        exit(65);
    }
}