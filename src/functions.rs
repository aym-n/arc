use crate::callable::*;
use crate::enviroment::Environment;
use crate::errors::*;
use crate::stmt::*;
use crate::tokens::*;
use std::rc::Rc;

use crate::interpreter::Interpreter;
pub struct Function {
    name : Token,
    params : Rc<Vec<Token>>,
    body : Rc<Vec<Stmt>>,
}

impl Function {
    pub fn new(declaration: &FunctionStmt) -> Self {
        Self {
            name: declaration.name.clone(),
            params: Rc::clone(&declaration.params),
            body: Rc::clone(&declaration.body),
        }
    }
}

impl CallableTrait for Function {

    fn call(&self, interpreter: &Interpreter, arguments: &Vec<Object>) -> Result<Object, Error> {
        let mut e = Environment::new_with_enclosing(Rc::clone(&interpreter.globals));

        for (param , arg) in self.params.iter().zip(arguments.iter()) {
            e.define(param.lexeme.clone(), arg.clone());
        }

        interpreter.execute_block(&self.body, e)?;
        Ok(Object::Nil)   
    }

    fn arity(&self) -> usize {
        self.params.len()
    }

    fn stringify(&self) -> String {
        self.name.lexeme.clone()
    }
}
