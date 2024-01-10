use crate::tokens::*;

#[derive(Debug, PartialEq, Clone)]
pub enum Error {
    ParseError { token: Token, message: String },
    RuntimeError { token: Token, message: String },
    SystemError { message: String },
    Return { value: Object },
}

impl Error {
    pub fn return_value(value: Object) -> Error {
        Error::Return { value }
    }

    pub fn parse_error(token: &Token, message: &str) -> Error {
        let err = Error::ParseError {
            token: token.clone(),
            message: message.to_string(),
        };
        err.report("");
        err
    }

    pub fn runtime_error(token: &Token, message: &str) -> Error {
        let err = Error::RuntimeError {
            token: token.clone(),
            message: message.to_string(),
        };
        err.report("");
        err
    }

    pub fn system_error(message: &str) -> Error {
        let err = Error::SystemError {
            message: message.to_string(),
        };
        err.report("");
        err
    }

    pub fn report(&self, _loc: &str) {
        match self {
            Error::ParseError { token, message }
            | Error::RuntimeError { token, message } => {
                if token.kind == TokenKind::EOF {
                    eprintln!("{} at end {}", token.line, message);
                } else {
                    eprintln!("line {} at '{}' {}", token.line, token.lexeme, message);
                }
            }
            Error::SystemError { message } => {
                eprintln!("System Error: {message}");
            }
            Error::Return { .. } => {}
        };
    }
}