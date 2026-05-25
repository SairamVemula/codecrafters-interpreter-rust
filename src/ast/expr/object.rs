use std::fmt::{self, Display};
use std::rc::Rc;

use crate::runtime::Callable;

use super::ExprEnum;

#[derive(Debug, Clone)]
pub enum Object {
    Null,
    String(String),
    Number(f64),
    Boolean(bool),
    Callable(Rc<dyn Callable>),
}

impl Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Object::String(s) => write!(f, "{s}"),
            Object::Number(n) => {
                if n.fract() == 0.0 {
                    write!(f, "{n:.1}")
                } else {
                    write!(f, "{n}")
                }
            }
            Object::Null => write!(f, "nil"),
            Object::Boolean(b) => write!(f, "{b}"),
            Object::Callable(fun) => write!(f, "{fun}"),
        }
    }
}

impl From<Object> for ExprEnum {
    fn from(value: Object) -> Self {
        ExprEnum::Object(value)
    }
}

impl Object {
    pub fn is_truthy(&self) -> bool {
        match self {
            Object::Null => false,
            Object::Boolean(b) => *b,
            _ => true,
        }
    }
}

impl Object {
    pub fn runtime_display(&self) -> String {
        match self {
            Object::Null => "nil".to_string(),
            Object::Number(n) => format!("{n}"),
            Object::String(s) => s.clone(),
            Object::Boolean(b) => b.to_string(),
            Object::Callable(f) => f.to_string(),
        }
    }
}

impl PartialEq for Object {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Object::Null, Object::Null) => true,
            (Object::String(a), Object::String(b)) => a == b,
            (Object::Number(a), Object::Number(b)) => a == b,
            (Object::Boolean(a), Object::Boolean(b)) => a == b,
            (Object::Callable(a), Object::Callable(b)) => Rc::ptr_eq(a, b),
            _ => false,
        }
    }
}
