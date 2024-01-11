use crate::tokens::*;

#[derive(Debug, PartialEq, Clone)]
pub enum Error {
    LexerError{ lexeme: String , line: String, message: String},
    ParseError { token: Token, message: String },
    RuntimeError { token: Token, message: String },
    SystemError { message: String },
    Return { value: Object },
}

impl Error {
    pub fn lexer_error(lexeme: &str, line: &str, message: &str) -> Error {
        let err = Error::LexerError {
            lexeme: lexeme.to_string(),
            line: line.to_string(),
            message: message.to_string(),
        };
        err.report("");
        err
    }

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
            Error::LexerError { lexeme, line, message } => {
                eprintln!("[line {}] Error {}: {}", line, lexeme, message);
            }
            Error::ParseError { token, message }
            | Error::RuntimeError { token, message } => {
                if token.kind == TokenKind::EOF {
                    eprintln!("[line {}] Error at end: {}", token.line, message)
                } else {
                    eprintln!("[line {}] Error at '{}': {}", token.line, token.lexeme, message);
                }
            }
            Error::SystemError { message } => {
                eprintln!("System Error: {message}");
            }
            Error::Return { .. } => {}
        };
    }
}