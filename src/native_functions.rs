use crate::callable::*;
use crate::errors::*;
use crate::interpreter::*;
use crate::tokens::*;

use std::time::SystemTime;

pub struct NativeClock {}

impl CallableTrait for NativeClock {
    fn call(&self, _terp: &Interpreter, _args: &Vec<Object>) -> Result<Object, Error> {
        match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
            Ok(n) => Ok(Object::Num(n.as_millis() as f64)),
            Err(_) => Err(Error::system_error("Failed to get time.")),
        }
    }

    fn arity(&self) -> usize {
        0
    }

    fn stringify(&self) -> String {
        "Native::Clock".to_string()
    }
}
