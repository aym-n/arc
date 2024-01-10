use crate::callable::*;
use crate::enviroment::Environment;
use crate::errors::*;
use crate::stmt::*;
use crate::tokens::*;
use std::rc::Rc;
use std::cell::RefCell;
use std::fmt;

use crate::interpreter::Interpreter;

pub struct Function {
    name : Token,
    params : Rc<Vec<Token>>,
    body : Rc<Vec<Rc<Stmt>>>,
    closure: Rc<RefCell<Environment>>,
}

impl fmt::Debug for Function {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Function {{ name: {:?}, params: {:?}}}", self.name, self.params)
    }
}

impl Clone for Function {
    fn clone(&self) -> Self {
        Self {
            name: self.name.clone(),
            params: Rc::clone(&self.params),
            body: Rc::clone(&self.body),
            closure: Rc::clone(&self.closure),
        }
    }
}

impl PartialEq for Function {
    fn eq(&self, other: &Self) -> bool {
        self.name.kind == other.name.kind && Rc::ptr_eq(&self.params, &other.params) && Rc::ptr_eq(&self.body, &other.body) && Rc::ptr_eq(&self.closure, &other.closure)     
    }
}

impl Function {
    pub fn new(declaration: &FunctionStmt, closure: &Rc<RefCell<Environment>>) -> Self {
        Self {
            name: declaration.name.clone(),
            params: Rc::clone(&declaration.params),
            body: Rc::clone(&declaration.body),
            closure: Rc::clone(&closure),
        }
    }

    pub fn bind(&self, instance: &Object) -> Object {
        let mut e = RefCell::new(Environment::new_with_enclosing(Rc::clone(&self.closure)));
        e.borrow_mut().define("this".to_string(), instance.clone());
        Object::Function(Rc::new(Self {
            name: self.name.clone(),
            params: Rc::clone(&self.params),
            body: Rc::clone(&self.body),
            closure: Rc::new(e),
       }))
    }
}

impl CallableTrait for Function {

    fn call(&self, interpreter: &Interpreter, arguments: &Vec<Object>) -> Result<Object, Error> {
        let mut e = Environment::new_with_enclosing(Rc::clone(&self.closure));

        for (param , arg) in self.params.iter().zip(arguments.iter()) {
            e.define(param.lexeme.clone(), arg.clone());
        }

        match interpreter.execute_block(&self.body, e) {
            Ok(_) => Ok(Object::Nil),
            Err(e) => {
                match e {
                    Error::Return { value } => Ok(value),
                    _ => Err(e),
                }
            }
        }
    }

    fn arity(&self) -> usize {
        self.params.len()
    }

    fn stringify(&self) -> String {
        self.name.lexeme.clone()
    }
}
