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

    pub fn get(&self, token: &Token) -> Result<Object, Error> {
        match self.values.get(&token.lexeme) {
            Some(value) => Ok(value.clone()),
            None => {
                match &self.enclosing {
                    Some(env) => env.borrow().get(token),
                    None => Err(Error::parse_error(
                        token,
                        &format!("Undefined variable '{}'.", token.lexeme),
                    )),
                }
            }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_define() {
        let mut env = Environment::new();
        env.define("foo".to_string(), Object::Num(1.0));
        
        let token = Token::new(TokenKind::Identifier, "foo".to_string(), None, 0);
        let result = env.get(&token);
        assert!(result.is_ok());
        assert_eq!(result.ok().unwrap(), Object::Num(1.0));
    }

    #[test]
    fn test_get() {
        let mut env = Environment::new();
        env.define("foo".to_string(), Object::Num(1.0));
        
        let token = Token::new(TokenKind::Identifier, "foo".to_string(), None, 0);
        let result = env.get(&token);
        assert!(result.is_ok());
        assert_eq!(result.ok().unwrap(), Object::Num(1.0));
    }

    #[test]
    fn test_re_define() {
        let mut env = Environment::new();
        env.define("foo".to_string(), Object::Num(1.0));
        env.define("foo".to_string(), Object::Num(2.0));
        
        let token = Token::new(TokenKind::Identifier, "foo".to_string(), None, 0);
        let result = env.get(&token);
        assert!(result.is_ok());
        assert_eq!(result.ok().unwrap(), Object::Num(2.0));
    }

    #[test]
    fn test_get_undefined() {
        let env = Environment::new();
        
        let token = Token::new(TokenKind::Identifier, "foo".to_string(), None, 0);
        let result = env.get(&token);
        assert!(result.is_err());
        assert_eq!(result.err().unwrap().message, "Undefined variable 'foo'.");
    }

    #[test]
    fn error_when_writting_to_undefined(){
        let mut env = Environment::new();
        let token = Token::new(TokenKind::Identifier, "foo".to_string(), None, 0);
        assert!(env.assign(&token, Object::Num(1.0)).is_err());
    }

    #[test]
    fn reassign_variable() {
        let mut env = Environment::new();
        env.define("foo".to_string(), Object::Num(1.0));
        let token = Token::new(TokenKind::Identifier, "foo".to_string(), None, 0);
        assert!(env.assign(&token, Object::Num(2.0)).is_ok());
        let result = env.get(&token);
        assert!(result.is_ok());
        assert_eq!(result.ok().unwrap(), Object::Num(2.0));
    }

    #[test]
    fn test_enclose_an_enviroment(){
        let env = Rc::new(RefCell::new(Environment::new()));
        let env2 = Environment::new_with_enclosing(Rc::clone(&env));
        assert_eq!(env2.enclosing.unwrap().borrow().values, env.borrow().values);
    }

    #[test]
    fn test_read_from_enclosed_enviroment(){
        let e = Rc::new(RefCell::new(Environment::new()));
        e.borrow_mut().define("foo".to_string(), Object::Num(1.0));
        let e2 = Environment::new_with_enclosing(Rc::clone(&e));
        let token = Token::new(TokenKind::Identifier, "foo".to_string(), None, 0);
        let result = e2.get(&token);
        assert!(result.is_ok());
        assert_eq!(result.ok().unwrap(), Object::Num(1.0));
    }

    #[test]
    fn test_assign_to_enclosed_enviroment(){
        let e = Rc::new(RefCell::new(Environment::new()));
        e.borrow_mut().define("foo".to_string(), Object::Num(1.0));

        let mut e2 = Environment::new_with_enclosing(Rc::clone(&e));
        let token = Token::new(TokenKind::Identifier, "foo".to_string(), None, 0);

        assert!(e2.assign(&token, Object::Num(2.0)).is_ok());

        let result = e2.get(&token);

        assert!(result.is_ok());
        assert_eq!(result.ok().unwrap(), Object::Num(2.0));
    }
}