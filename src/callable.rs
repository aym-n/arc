
use crate::interpreter::Interpreter;
use crate::tokens::*;
use crate::errors::Error;
use std::rc::Rc;
use core::fmt::Debug;
use core::fmt::Display;

#[derive(Clone)]
pub struct Callable{
    pub func: Rc<dyn CallableTrait>,
}

impl Debug for Callable{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<fn {}>", self.stringify())
    }
}

impl Display for Callable{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.stringify())
    }
}

impl PartialEq for Callable{
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.func, &other.func)
    }
}


pub trait CallableTrait {
    fn call(&self, interpreter: &Interpreter, arguments: &Vec<Object>, class: Option<Rc<ClassStruct>>) -> Result<Object, Error>;
    fn arity(&self) -> usize;
    fn stringify(&self) -> String;
}

impl CallableTrait for Callable {
    fn call(&self, interpreter: &Interpreter, arguments: &Vec<Object>, _class: Option<Rc<ClassStruct>>) -> Result<Object, Error> {
        self.func.call(interpreter, arguments, None)
    }

    fn arity(&self) -> usize {
        self.func.arity()
    }

    fn stringify(&self) -> String {
        self.func.stringify()
    }
}