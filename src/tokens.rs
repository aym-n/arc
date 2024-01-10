use std::fmt;
use crate::callable::Callable;
use crate::errors::*;
use crate::interpreter::Interpreter;
use crate::callable::*;
use std::rc::Rc;
use crate::instance::*;
use std::collections::HashMap;
use crate::functions::*;
use crate::native_functions::*;

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
    Function(Rc<Function>),
    Class(Rc<ClassStruct>),
    Instance(Rc<InstanceStruct>),
    Native(Rc<Native>),
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
            Object::Native(_) => write!(f, "<native>"),
            Object::Class(c) => write!(f, "<class {}>", c.name),
            Object::Instance(i) => write!(f, "<instance {}>", i.class.name),
        }
    }
}


impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ClassStruct {
    pub name: String,
    superclass: Option<Rc<ClassStruct>>,
    methods: HashMap<String, Object>,
}

impl ClassStruct {
    pub fn new(name: String, superclass: Option<Rc<ClassStruct>>, methods: HashMap<String, Object>) -> Self {
        ClassStruct {
            name,
            superclass,
            methods,
        }
    }

    pub fn instantiate(&self, interpreter: &Interpreter, arguments: Vec<Object>, cls: Rc<ClassStruct>) -> Result<Object, Error> {
        let instance = Object::Instance(Rc::new(InstanceStruct::new(cls)));
        if let Some(Object::Function(initializer)) = self.find_method("init".to_string()){
            if let Object::Function(initializer) = initializer.bind(&instance) {
                initializer.call(interpreter, &arguments, None)?;
            }
        }
        Ok(instance)
    }

    pub fn find_method(&self, name: String) -> Option<Object> {
        if let Some(method) = self.methods.get(&name){
            Some(method.clone())
        }else if let Some(superclass) = &self.superclass {
            superclass.find_method(name)
        }else{
            None
        }
    }
}

impl CallableTrait for ClassStruct {
    fn call(&self, interpreter: &Interpreter, arguments: &Vec<Object>, class: Option<Rc<ClassStruct>>) -> Result<Object, Error> {
        self.instantiate(interpreter, arguments.clone(), class.unwrap())
    }

    fn arity(&self) -> usize {
        if let Some(Object::Function(initializer)) = self.find_method("init".to_string()){
            return initializer.arity();
        }
        0
    }

    fn stringify(&self) -> String {
        self.name.clone()
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
