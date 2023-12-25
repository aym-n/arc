use std::process::exit;

// TODO: implement error handling with Result<T, E>

pub fn error(line: usize, message: &str) {
    report(line, "", message);
}

fn report(line: usize, location: &str, message: &str) {
    println!("[line {}] Error {}: {}", line, location, message);
    exit(65);
}