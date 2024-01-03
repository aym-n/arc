use std::collections::HashMap;
use crate::tokens::*;
use crate::errors::Error;

pub struct Environment {
    values: HashMap<String, Object>,
}

impl Environment {
    pub fn new() -> Environment {
        Environment {
            values: HashMap::new(),
        }
    }

    pub fn define(&mut self, name: String, value: Object) {
        self.values.insert(name, value);
    }

    pub fn get(&self, token: &Token) -> Result<Object, Error> {
        match self.values.get(&token.lexeme) {
            Some(value) => Ok(value.clone()),
            None => Err(Error::new(
                token.line,
                format!("Undefined variable '{}'.", token.lexeme),
            )),
        }
    }

    pub fn assign(&mut self, name: &Token, value: Object) -> Result<(), Error> {
        match self.values.get_mut(&name.lexeme) {
            Some(v) => {
                *v = value;
                Ok(())
            }
            None => Err(Error::new(
                name.line,
                format!("Undefined variable '{}'.", name.lexeme),
            )),
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
}