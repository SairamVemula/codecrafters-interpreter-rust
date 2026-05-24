use super::*;
use crate::token::Token;

#[derive(Debug, Clone)]
pub struct Variable {
    pub name: Token,
}

impl Variable {
    pub fn new(name: Token) -> Self {
        Self { name }
    }
}

impl From<Variable> for ExprEnum {
    fn from(value: Variable) -> Self {
        ExprEnum::Variable(value)
    }
}