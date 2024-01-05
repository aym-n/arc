use std::fmt;
use crate::callable::Callable;

#[derive(Debug, Default, PartialEq, Clone)]
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

#[derive(Clone, Debug, PartialEq)]
pub enum Object {
    Num(f64),
    Str(String),
    Bool(bool),
    Function(Callable),
    Nil,
    ArithmeticError,
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Object::Num(x) => write!(f, "{x}"),
            Object::Str(x) => write!(f, "{x}"),
            Object::Nil => write!(f, "nil"),
            Object::Bool(x) => write!(f, "{x}"),
            Object::ArithmeticError => write!(f, "Arithmetic Error"),
            Object::Function(_) => write!(f, "<func>"),
        }
    }
}


impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub lexeme: String,
    pub literal: Option<Object>,
    pub line: usize,
}

impl Default for Token {
    fn default() -> Self {
        Token {
            kind: TokenKind::Illegal,
            lexeme: "".to_string(),
            literal: None,
            line: 0,
        }
    }
}

impl Token {
    pub fn new(kind: TokenKind, lexeme: String, literal: Option<Object>, line: usize) -> Self {
        Token {
            kind,
            lexeme,
            literal,
            line,
        }
    }
}
