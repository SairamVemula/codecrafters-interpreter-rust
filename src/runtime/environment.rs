use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use anyhow::Result;

use crate::{ast::expr::Literal, error::RuntimeError, token::Token};

#[derive(Debug)]
pub struct Environment {
    enclosing: Option<Arc<Mutex<Environment>>>,
    pub values: HashMap<String, Literal>,
}

impl Environment {
    pub fn new(enclosing: Option<Arc<Mutex<Environment>>>) -> Self {
        Self {
            values: HashMap::new(),
            enclosing,
        }
    }

    pub fn get(&self, name: Token) -> Result<Literal> {
        match self.values.get(&name.lexeme) {
            Some(value) => Ok(value.clone()),
            None => match &self.enclosing {
                Some(enclosing) => enclosing.lock().unwrap().get(name),
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
            Some(enclosing) => enclosing.lock().unwrap().assign(name, value),
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