use std::{
    fmt::Display,
    sync::{Arc, Mutex},
};

use anyhow::Result;

use super::*;

use crate::ast::{expr::Literal, stmt::Fun};
use crate::error::RuntimeError;

#[derive(Debug)]
pub struct Function {
    declaration: Fun,
    closure: Arc<Mutex<Environment>>,
}

impl Function {
    pub fn new(declaration: Fun, closure: Arc<Mutex<Environment>>) -> Self {
        Self {
            declaration,
            closure,
        }
    }
}

impl Callable for Function {
    fn call(&self, interpreter: &mut Interpreter, args: Vec<Literal>) -> Result<Literal> {
        let env = Arc::new(Mutex::new(Environment::new(Some(self.closure.clone()))));
        for (i, param) in self.declaration.params.iter().enumerate() {
            env.lock()
                .unwrap()
                .define(param.lexeme.clone(), args.get(i).cloned());
        }
        match interpreter.execute_block(&mut self.declaration.body.clone(), env) {
            Ok(()) => Ok(Literal::Null),
            Err(e) => {
                if let Some(runtime_err) = e.downcast_ref::<RuntimeError>() {
                    match runtime_err {
                        RuntimeError::ReturnValue { value } => Ok(value.clone()),
                        _ => Err(e),
                    }
                } else {
                    Err(e)
                }
            }
        }
    }

    fn arity(&self) -> usize {
        self.declaration.params.len()
    }
}

impl Display for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<fn {}>", self.declaration.name.lexeme)
    }
}
