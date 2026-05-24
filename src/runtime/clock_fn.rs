use anyhow::Result;

use super::*;
use std::{
    fmt::Display,
    time::{SystemTime, UNIX_EPOCH},
};

use crate::ast::expr::Literal;

#[derive(Debug)]
pub struct ClockFn;

impl Callable for ClockFn {
    fn call(&self, _interpreter: &mut Interpreter, _args: Vec<Literal>) -> Result<Literal> {
        let secs = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        Ok(Literal::Number(secs as f64, secs.to_string()))
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
