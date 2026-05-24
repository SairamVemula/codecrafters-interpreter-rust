use std::fmt::{self, Display};

#[derive(Clone, Debug, PartialEq)]
pub enum Literal {
    Null,
    String(String),
    Number(f64, String),
    Boolean(bool),
}

impl Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Literal::String(s) => write!(f, "{s}"),
            Literal::Number(_, s) => write!(f, "{s}"),
            Literal::Null => write!(f, "nil"),
            Literal::Boolean(b) => write!(f, "{b}"),
        }
    }
}