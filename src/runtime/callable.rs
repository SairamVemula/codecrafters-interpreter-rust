use std::fmt::{Debug, Display};

use anyhow::Result;

use crate::ast::expr::Literal;

use super::*;

pub trait Callable: Debug + Display + Send + Sync {
    fn call(&self, interpreter: &mut Interpreter, args: Vec<Literal>) -> Result<Literal>;
    fn arity(&self) -> usize;
}