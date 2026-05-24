use std::{
    fmt::{self, Display},
    sync::Arc,
};

use crate::runtime::Callable;

use super::*;

#[derive(Debug, Clone)]
pub enum Literal {
    Null,
    String(String),
    Number(f64, String),
    Boolean(bool),
    Callable(Arc<dyn Callable>),
}

impl Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Literal::String(s) => write!(f, "{s}"),
            Literal::Number(_, s) => write!(f, "{s}"),
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

impl PartialEq for Literal {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Literal::Null, Literal::Null) => true,

            (Literal::String(a), Literal::String(b)) => a == b,

            (Literal::Number(a, _), Literal::Number(b, _)) => a == b,

            (Literal::Boolean(a), Literal::Boolean(b)) => a == b,

            (Literal::Callable(a), Literal::Callable(b)) => Arc::ptr_eq(a, b),

            _ => false,
        }
    }
}