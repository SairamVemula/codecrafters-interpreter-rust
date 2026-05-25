use std::fmt::{self, Display};
use std::rc::Rc;

use crate::runtime::Callable;

use super::ExprEnum;

#[derive(Debug, Clone)]
pub enum Literal {
    Null,
    String(String),
    Number(f64),
    Boolean(bool),
    Callable(Rc<dyn Callable>),
}

impl Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Literal::String(s) => write!(f, "{s}"),
            Literal::Number(n) => {
                if n.fract() == 0.0 {
                    write!(f, "{n:.1}")
                } else {
                    write!(f, "{n}")
                }
            }
            Literal::Null => write!(f, "nil"),
            Literal::Boolean(b) => write!(f, "{b}"),
            Literal::Callable(fun) => write!(f, "{fun}"),
        }
    }
}

impl From<Literal> for ExprEnum {
    fn from(value: Literal) -> Self {
        ExprEnum::Literal(value)
    }
}

impl Literal {
    pub fn is_truthy(&self) -> bool {
        match self {
            Literal::Null => false,
            Literal::Boolean(b) => *b,
            _ => true,
        }
    }
}

impl Literal {
    pub fn runtime_display(&self) -> String {
        match self {
            Literal::Null => "nil".to_string(),
            Literal::Number(n) => format!("{n}"),
            Literal::String(s) => s.clone(),
            Literal::Boolean(b) => b.to_string(),
            Literal::Callable(f) => f.to_string(),
        }
    }
}

impl PartialEq for Literal {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Literal::Null, Literal::Null) => true,
            (Literal::String(a), Literal::String(b)) => a == b,
            (Literal::Number(a), Literal::Number(b)) => a == b,
            (Literal::Boolean(a), Literal::Boolean(b)) => a == b,
            (Literal::Callable(a), Literal::Callable(b)) => Rc::ptr_eq(a, b),
            _ => false,
        }
    }
}
