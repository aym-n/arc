use crate::callable::*;
use crate::errors::*;
use crate::interpreter::*;
use crate::tokens::*;
use std::rc::Rc;
use std::fmt;
use std::time::SystemTime;

#[derive(Clone)]
pub struct Native{
    pub func: Rc<dyn CallableTrait>,
}

impl PartialEq for Native {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.func, &other.func)
    }
}

impl fmt::Debug for Native {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<native>")
    }
}

pub struct NativeClock {}

impl CallableTrait for NativeClock {
    fn call(&self, _terp: &Interpreter, _args: &Vec<Object>, _class: Option<Rc<ClassStruct>>) -> Result<Object, Error> {
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
