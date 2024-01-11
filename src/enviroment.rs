use std::collections::HashMap;
use crate::tokens::*;
use crate::errors::Error;
use std::rc::Rc;
use std::cell::RefCell;


#[derive(Debug)]
pub struct Environment {
    values: HashMap<String, Object>,
    enclosing: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
    pub fn new() -> Environment {
        Environment {
            values: HashMap::new(),
            enclosing: None,
        }
    }

    pub fn new_with_enclosing(enclosing: Rc<RefCell<Environment>>) -> Environment {
        Environment {
            values: HashMap::new(),
            enclosing: Some(enclosing),
        }
    }

    pub fn define(&mut self, name: String, value: Object) {
        self.values.insert(name, value);
    }

    pub fn get_at(&self, distance: usize, name: &str) -> Result<Object, Error> {
        if distance == 0 {
            Ok(self.values.get(name).unwrap().clone())
        }else{
            self.enclosing.as_ref().unwrap().borrow().get_at(distance - 1, name)
        }
    }

    pub fn assign_at(&mut self, distance: usize, name: &Token, value: Object) -> Result<(), Error> {
        if distance == 0 {
            self.values.insert(name.lexeme.clone(), value);
            Ok(())
        }else{
            self.enclosing.as_ref().unwrap().borrow_mut().assign_at(distance - 1, name, value)
        }
    }

    pub fn get(&self, token: &Token) -> Result<Object, Error> {
        if let Some(object) = self.values.get(&token.lexeme) {
            Ok(object.clone())
        } else if let Some(enclosing) = &self.enclosing {
            enclosing.borrow().get(token)
        } else {
            Err(Error::parse_error(
                token,
                &format!("Undefined variable '{}'.", token.lexeme),
            ))
        }
    }
    
    pub fn assign(&mut self, name: &Token, value: Object) -> Result<(), Error> {
        match self.values.get_mut(&name.lexeme) {
            Some(v) => {
                *v = value;
                Ok(())
            }
            None => {
                match &self.enclosing {
                    Some(env) => env.borrow_mut().assign(name, value),
                    None => Err(Error::parse_error(
                        name,
                        &format!("Undefined variable '{}'.", name.lexeme),
                    )),
                }
            }
        }
    }
}
