use std::fmt;

use crate::token::Token;

use super::ExprEnum;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Set {
    pub object: Box<ExprEnum>,
    pub name: Token,
    pub value: Box<ExprEnum>,
}

impl Set {
    pub fn new(object: ExprEnum,name: Token, value: ExprEnum) -> Self {
        Self {
            object: Box::new(object),
            name,
            value: Box::new(value),
        }
    }
}

impl From<Set> for ExprEnum {
    fn from(value: Set) -> Self {
        ExprEnum::Set(value)
    }
}

impl fmt::Display for Set {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}", self.value, self.name.lexeme)
    }
}
