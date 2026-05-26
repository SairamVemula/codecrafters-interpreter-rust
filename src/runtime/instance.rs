use crate::{ast::expr::Object, error::RuntimeError, runtime::Result, token::Token};

use super::Class;
use std::{cell::RefCell, collections::HashMap, fmt::Display, rc::Rc};

#[derive(Debug, Clone)]
pub struct Instance {
    class: Class,
    pub fields: HashMap<String, Object>,
}

impl Instance {
    pub fn new(class: Class) -> Self {
        Self {
            class,
            fields: HashMap::new(),
        }
    }

    pub fn get(instance: &Rc<RefCell<Instance>>, name: &Token) -> Result<Object> {
        let self_ref = instance.borrow();
        if let Some(obj) = self_ref.fields.get(&name.lexeme) {
            return Ok(obj.clone());
        }
        if let Some(method) = self_ref.class.find_method(&name.lexeme) {
            return Ok(Object::Callable(Rc::new(method.bind(instance.clone()))));
        }

        Err(RuntimeError::Error {
            line: name.line,
            msg: format!("Undefined property '{}'.", name.lexeme),
        })
    }

    pub fn set(&mut self, name: &Token, value: Object) {
        self.fields.insert(name.lexeme.clone(), value);
    }
}

impl Display for Instance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} instance", self.class.name)
    }
}
