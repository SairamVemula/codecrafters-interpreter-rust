use std::cell::RefCell;
use std::fmt::Display;
use std::rc::Rc;

use crate::ast::expr::Object;
use crate::ast::stmt::Fun;
use crate::error::RuntimeError;

use super::{Callable, Environment, Interpreter, Result};

#[derive(Debug)]
pub struct Function {
    declaration: Fun,
    closure: Rc<RefCell<Environment>>,
}

impl Function {
    pub fn new(declaration: Fun, closure: Rc<RefCell<Environment>>) -> Self {
        Self {
            declaration,
            closure,
        }
    }
}

impl Callable for Function {
    fn call(&self, interpreter: &mut Interpreter, args: Vec<Object>) -> Result<Object> {
        let env = Rc::new(RefCell::new(Environment::new(Some(self.closure.clone()))));
        for (param, arg) in self.declaration.params.iter().zip(args) {
            env.borrow_mut().define(param.lexeme.clone(), Some(arg));
        }
        match interpreter.execute_block(&mut self.declaration.body.clone(), env) {
            Ok(()) => Ok(Object::Null),
            Err(RuntimeError::ReturnValue { value }) => Ok(value),
            Err(e) => Err(e),
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
