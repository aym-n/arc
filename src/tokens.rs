use std::fmt;

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

use std::ops::*;
use std::cmp::*;

#[derive(Clone, Debug, PartialEq)]
pub enum Object {
    Num(f64),
    Str(String),
    Bool(bool),
    Nil,
    ArithmeticError,
}

impl Sub for Object {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        match (self, other) {
            (Object::Num(x), Object::Num(y)) => Object::Num(x - y),
            _ => Object::ArithmeticError,
        }
    }
}

impl Mul for Object {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        match (self, other) {
            (Object::Num(x), Object::Num(y)) => Object::Num(x * y),
            _ => Object::ArithmeticError,
        }
    }
}

impl Div for Object {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        match (self, other) {
            (Object::Num(x), Object::Num(y)) => Object::Num(x / y),
            _ => Object::ArithmeticError,
        }
    }
}

impl Add for Object {
    type Output = Object;

    fn add(self, other: Self) -> Object {
        match (self, other) {
            (Object::Num(left), Object::Num(right)) => Object::Num(left + right),
            (Object::Str(left), Object::Str(right)) => Object::Str(format!("{}{}", left, right)),
            _ => Object::ArithmeticError,
        }
    }
}

impl PartialOrd for Object {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {   
            (Object::Nil, Object::Nil) => Some(Ordering::Equal),
            (Object::Nil , _) => None,
            (Object::Num(x), Object::Num(y)) => x.partial_cmp(y),
            (Object::Str(x), Object::Str(y)) => x.partial_cmp(y),
            _ => None,
        }
    }
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Object::Num(x) => write!(f, "{x}"),
            Object::Str(x) => write!(f, "\"{x}\""),
            Object::Nil => write!(f, "nil"),
            Object::Bool(x) => write!(f, "{x}"),
            Object::ArithmeticError => panic!("Arithmetic Error"),
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
