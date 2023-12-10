#[derive(Debug, Default, PartialEq)]
pub enum TokenKind {
    #[default]
    Illegal,
    EOF,

    //Literals
    Number,
    String,
    Identifier,

    //Operators
    Plus,             // +
    Minus,            // -
    Asterisk,         // *
    Slash,            // /
    Bang,             // !
    Equal,            // =
    NotEqual,         // !=
    EqualEqual,       // ==
    LessThan,         // <
    LessThanEqual,    // <=
    GreaterThan,      // >
    GreaterThanEqual, // >=

    //Delimiters
    LeftParen,  // (
    RightParen, // )
    LeftBrace,  // {
    RightBrace, // }
    RightSquare, // [
    LeftSquare,  // ]
    Comma,      // ,
    Dot,        // .
    Semicolon,  // ;

    //Keywords
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,
}

use std::fmt;

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub lexeme: Option<String>,
    pub literal: Option<String>,
    pub line: usize,
}

impl Default for Token {
    fn default() -> Self {
        Token {
            kind: TokenKind::Illegal,
            literal: None,
            lexeme: None,
            line: 0,
        }
    }
}

impl Token {
    pub fn new(kind: TokenKind, lexeme: String, line: usize) -> Self {
        Token {
            kind: kind,
            literal: None,
            lexeme : Some(lexeme),
            line: line,
        }
    }

    pub fn new_with_literal(kind: TokenKind, lexeme: String, literal: String, line: usize) -> Self {
        Token {
            kind,
            literal: Some(literal),
            lexeme: Some(lexeme),
            line: line,
        }
    }
}
