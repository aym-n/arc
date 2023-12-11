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

    Comment, // #

    //Keywords
    And, 
    Class,
    Else,
    False,
    Fn,
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

#[derive(Debug, PartialEq )]
pub struct Token {
    pub kind: TokenKind,
    pub literal: Option<String>,
    pub line: usize,
}

impl Default for Token {
    fn default() -> Self {
        Token {
            kind: TokenKind::Illegal,
            literal: None,
            line: 0,
        }
    }
}

impl Token {
    pub fn new(kind: TokenKind,line: usize) -> Self {
        Token {
            kind: kind,
            literal: None,
            line: line,
        }
    }

    pub fn new_with_literal(kind: TokenKind, literal: String, line: usize) -> Self {
        Token {
            kind,
            literal: Some(literal),
            line: line,
        }
    }
}
