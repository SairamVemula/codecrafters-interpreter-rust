use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::ast::expr::Object;
use crate::error::RuntimeError;
use crate::token::Token;

use super::Result;

#[derive(Debug)]
pub struct Environment {
    pub enclosing: Option<Rc<RefCell<Environment>>>,
    pub values: HashMap<String, Object>,
}

impl Environment {
    pub fn new(enclosing: Option<Rc<RefCell<Environment>>>) -> Self {
        Self {
            values: HashMap::new(),
            enclosing,
        }
    }

    pub fn get(&self, name: Token) -> Result<Object> {
        match self.values.get(&name.lexeme) {
            Some(value) => Ok(value.clone()),
            None => match &self.enclosing {
                Some(enclosing) => enclosing.borrow().get(name),
                None => Err(RuntimeError::UndefinedVariable { name: name.lexeme }),
            },
        }
    }

    pub fn assign(&mut self, name: Token, value: Object) -> Result<()> {
        if self.values.contains_key(&name.lexeme) {
            self.values.insert(name.lexeme, value);
            return Ok(());
        }
        match &self.enclosing {
            Some(enclosing) => enclosing.borrow_mut().assign(name, value),
            None => Err(RuntimeError::UndefinedVariable { name: name.lexeme }),
        }
    }

    pub fn assign_at(&mut self, distance: usize, name: Token, value: Object) -> Result<()> {
        if distance == 0 {
            return self.assign(name, value);
        }

        self.ancestor(distance).borrow_mut().assign(name, value)
    }

    pub fn define(&mut self, name: String, value: Option<Object>) {
        self.values.insert(name, value.unwrap_or(Object::Null));
    }

    pub fn get_at(&self, distance: usize, name: Token) -> Result<Object> {
        if distance == 0 {
            return self.get(name);
        }

        self.ancestor(distance).borrow().get(name)
    }

    fn ancestor(&self, distance: usize) -> Rc<RefCell<Environment>> {
        let mut environment = self
            .enclosing
            .as_ref()
            .expect("No enclosing environment")
            .clone();

        for _ in 1..distance {
            let next = environment
                .borrow()
                .enclosing
                .as_ref()
                .expect("No enclosing environment")
                .clone();

            environment = next;
        }

        environment
    }
}
