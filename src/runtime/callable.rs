use std::fmt::{Debug, Display};

use crate::ast::expr::Object;

use super::{Interpreter, Result};

pub trait Callable: Debug + Display {
    fn call(&self, interpreter: &mut Interpreter, args: Vec<Object>) -> Result<Object>;
    fn arity(&self) -> usize;
}
