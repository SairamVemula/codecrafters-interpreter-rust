use std::fmt::{Debug, Display};

use crate::ast::expr::Literal;

use super::{Interpreter, Result};

pub trait Callable: Debug + Display {
    fn call(&self, interpreter: &mut Interpreter, args: Vec<Literal>) -> Result<Literal>;
    fn arity(&self) -> usize;
}
