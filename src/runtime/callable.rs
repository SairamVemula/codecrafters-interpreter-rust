use std::{any::Any, fmt::{Debug, Display}, rc::Rc};

use crate::ast::expr::Object;

use super::{Interpreter, Result};

pub trait Callable: Debug + Display {
    fn call(&self, interpreter: &mut Interpreter, args: Vec<Object>) -> Result<Object>;
    fn arity(&self) -> usize;
    fn as_any(&self) -> &dyn Any;
}

impl From<Rc<dyn Callable>> for Object {
    fn from(value: Rc<dyn Callable>) -> Self {
        Object::Callable(value)
    }
}