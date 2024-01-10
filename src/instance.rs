use crate::errors::*;
use crate::tokens::*;
use std::cell::RefCell;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug, PartialEq, Clone)]
pub struct InstanceStruct {
    pub class: Rc<ClassStruct>,
    fields: RefCell<HashMap<String, Object>>,
}

impl InstanceStruct {
    pub fn new(class: Rc<ClassStruct>) -> Self {
        InstanceStruct {
            class: Rc::clone(&class),
            fields: RefCell::new(HashMap::new()),
        }
    }

    pub fn get(&self, name: &Token, this: &Rc<InstanceStruct>) -> Result<Object, Error> {
        if let Entry::Occupied(o) = self.fields.borrow_mut().entry(name.lexeme.clone()) {
            Ok(o.get().clone())
        } else if let Some(method) = self.class.find_method(name.lexeme.clone()) {
            if let Object::Function(function) = method {
                Ok(function.bind(&Object::Instance(Rc::clone(this))))
            } else {
                panic!("tried to bind `this` to a non-function")
            }
        } else {
            Err(Error::runtime_error(
                name,
                &format!("Undefined property '{}'.", name.lexeme),
            ))
        }
    }

    pub fn set(&self, name: &Token, value: Object) {
        self.fields.borrow_mut().insert(name.lexeme.clone(), value);
    }
}
