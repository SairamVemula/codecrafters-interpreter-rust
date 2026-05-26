use std::cell::RefCell;
use std::fmt::Display;
use std::rc::Rc;

use crate::ast::expr::Object;
use crate::ast::stmt::Fun;
use crate::error::RuntimeError;
use crate::token::Token;

use super::{Callable, Environment, Interpreter, Result, Instance};

#[derive(Debug, Clone)]
pub struct Function {
    declaration: Fun,
    closure: Rc<RefCell<Environment>>,
    is_initializer: bool
}

impl Function {
    pub fn new(declaration: Fun, closure: Rc<RefCell<Environment>>, is_initializer: bool) -> Self {
        Self {
            declaration,
            closure,
            is_initializer
        }
    }

    pub fn bind(&self, instance: Rc<RefCell<Instance>>) -> Function {
        let env = Rc::new(RefCell::new(Environment::new(Some(self.closure.clone()))));
        env.borrow_mut().define(
            "this".to_owned(),
            Some(Object::Instance(instance)),
        );
        Function::new(self.declaration.clone(), env, self.is_initializer)
    }
}

impl Callable for Function {
    fn call(&self, interpreter: &mut Interpreter, args: Vec<Object>) -> Result<Object> {
        let env = Rc::new(RefCell::new(Environment::new(Some(self.closure.clone()))));
        for (param, arg) in self.declaration.params.iter().zip(args) {
            env.borrow_mut().define(param.lexeme.clone(), Some(arg));
        }
        match interpreter.execute_block(&mut self.declaration.body.clone(), env) {
            Ok(()) => {
                if self.is_initializer {
                    let this_token = Token::new(
                        crate::token::TokenType::This,
                        "this".to_owned(),
                        Object::Null,
                        0,
                    );
                    return self.closure.borrow().get_at(0, this_token);
                }
                Ok(Object::Null)
            }
            Err(RuntimeError::ReturnValue { value }) => {
                if self.is_initializer {
                    let this_token = Token::new(
                        crate::token::TokenType::This,
                        "this".to_owned(),
                        Object::Null,
                        0,
                    );
                    return self.closure.borrow().get_at(0, this_token);
                }
                Ok(value)
            }
            Err(e) => Err(e),
        }
    }

    fn arity(&self) -> usize {
        self.declaration.params.len()
    }
    
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl Display for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<fn {}>", self.declaration.name.lexeme)
    }
}
