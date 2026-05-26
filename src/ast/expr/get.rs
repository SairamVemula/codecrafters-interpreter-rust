use std::fmt;

use crate::token::Token;

use super::ExprEnum;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Get {
    pub object: Box<ExprEnum>,
    pub name: Token,
}

impl Get {
    pub fn new(object: ExprEnum, name: Token) -> Self {
        Self {
            object: Box::new(object),
            name,
        }
    }
}

impl From<Get> for ExprEnum {
    fn from(value: Get) -> Self {
        ExprEnum::Get(value)
    }
}

impl fmt::Display for Get {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}", self.object, self.name.lexeme)
    }
}
