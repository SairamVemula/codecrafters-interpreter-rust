use std::{cell::RefCell, collections::HashMap, rc::Rc};

use anyhow::Result;

use crate::{error::RuntimeError, expr::Literal, token::Token};

pub struct Environment {
    enclosing: Option<Rc<RefCell<Environment>>>,
    pub values: HashMap<String, Literal>,
}

impl Environment {
    pub fn new(enclosing: Option<Rc<RefCell<Environment>>>) -> Self {
        Self {
            values: HashMap::new(),
            enclosing,
        }
    }

    pub fn get(&self, name: Token) -> Result<Literal> {
        match self.values.get(&name.lexeme) {
            Some(value) => Ok(value.clone()),
            None => match &self.enclosing {
                Some(enclosing) => enclosing.borrow().get(name),
                None => Err(RuntimeError::UndefinedVariable { name: name.lexeme }.into()),
            },
        }
    }

    pub fn assign(&mut self, name: Token, value: Literal) -> Result<()> {
        if self.values.contains_key(&name.lexeme) {
            self.values.insert(name.lexeme, value);
            return Ok(());
        }
        match &self.enclosing {
            Some(enclosing) => enclosing.borrow_mut().assign(name, value),
            None => Err(RuntimeError::UndefinedVariable {
                name: name.lexeme.clone(),
            }
            .into()),
        }
    }

    pub fn define(&mut self, name: String, value: Option<Literal>) {
        self.values.insert(name, value.unwrap_or(Literal::Null));
    }
}
