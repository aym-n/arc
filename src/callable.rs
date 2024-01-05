
use crate::interpreter::Interpreter;
use crate::tokens::Object;
use crate::errors::Error;
use std::rc::Rc;
use core::fmt::Debug;

#[derive(Clone)]
pub struct Callable{
    pub func: Rc<dyn CallableTrait>,
}

impl Debug for Callable{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<func>")
    }
}

impl PartialEq for Callable{
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.func, &other.func)
    }
}


pub trait CallableTrait {
    fn call(&self, interpreter: &Interpreter, arguments: &Vec<Object>) -> Result<Object, Error>;
}

impl CallableTrait for Callable {
    fn call(&self, interpreter: &Interpreter, arguments: &Vec<Object>) -> Result<Object, Error> {
        self.func.call(interpreter, arguments)
    }
}