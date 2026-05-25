use std::fmt::Display;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::ast::expr::Object;

use super::{Callable, Interpreter, Result};

#[derive(Debug)]
pub struct ClockFn;

impl Callable for ClockFn {
    fn call(&self, _interpreter: &mut Interpreter, _args: Vec<Object>) -> Result<Object> {
        let secs = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        Ok(Object::Number(secs as f64))
    }

    fn arity(&self) -> usize {
        0
    }
}

impl Display for ClockFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<native fn>")
    }
}
