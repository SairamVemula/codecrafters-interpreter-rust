use std::fmt;

use crate::token::Token;

use super::ExprEnum;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct This {
    pub keyword: Token
}

impl This {
    pub fn new(keyword: Token) -> Self {
        Self {
            keyword
        }
    }
}

impl From<This> for ExprEnum {
    fn from(value: This) -> Self {
        ExprEnum::This(value)
    }
}

impl fmt::Display for This {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.keyword.lexeme)
    }
}
