use super::*;

use crate::token::Token;

#[derive(Debug, Clone)]
pub struct Unary {
    pub operator: Token,
    pub right: Box<ExprEnum>,
}

impl Unary {
    pub fn new(operator: Token, right: ExprEnum) -> Self {
        Self {
            operator,
            right: Box::new(right),
        }
    }
}


impl From<Unary> for ExprEnum {
    fn from(value: Unary) -> Self {
        ExprEnum::Unary(value)
    }
}