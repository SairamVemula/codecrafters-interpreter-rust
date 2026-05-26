use std::fmt;

use crate::token::Token;

use super::ExprEnum;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Super {
    pub keyword: Token,
    pub method: Token,
}

impl Super {
    pub fn new(keyword: Token, method: Token) -> Self {
        Self {
            keyword,
            method
        }
    }
}

impl From<Super> for ExprEnum {
    fn from(value: Super) -> Self {
        ExprEnum::Super(value)
    }
}

impl fmt::Display for Super {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}", self.keyword.lexeme, self.method.lexeme)
    }
}
