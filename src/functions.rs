use crate::callable::*;
use crate::enviroment::Environment;
use crate::errors::*;
use crate::stmt::*;
use crate::tokens::*;
use std::rc::Rc;

use crate::interpreter::Interpreter;
pub struct Function {
    declaration: Rc<Stmt>,
}

impl Function {
    pub fn new(declaration: &Rc<Stmt>) -> Self {
        Self {
            declaration: Rc::clone(declaration),
        }
    }
}

impl CallableTrait for Function {
    fn call(&self, interpreter: &Interpreter, arguments: &Vec<Object>) -> Result<Object, Error> {
        if let Stmt::Function(FunctionStmt { name, params, body }) = &*self.declaration {
            let mut e = Environment::new_with_enclosing(Rc::clone(&interpreter.globals));
            for (param, arg) in params.iter().zip(arguments.iter()) {
                e.define(param.lexeme.clone(), arg.clone());
            }

            interpreter.execute_block(body, e)?;
            Ok(Object::Nil)
        }else{
            Err(Error::new(0, "Function declaration is not a function".to_string()))
        }

    }

    fn arity(&self) -> usize {
        if let Stmt::Function(FunctionStmt { name, params, body }) = &*self.declaration {
            params.len()
        }else{
            panic!("Function declaration is not a function")
        }
    }

    fn stringify(&self) -> String {
        if let Stmt::Function(FunctionStmt { name, params, body }) = &*self.declaration {
            format!("<fn {}>", name.lexeme)
        }else{
            panic!("Function declaration is not a function")
        }
    }
}
