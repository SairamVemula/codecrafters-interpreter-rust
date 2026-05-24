use super::*;

use crate::token::Token;


#[derive(Debug, Clone)]
pub struct Binary {
    pub left: Box<ExprEnum>,
    pub operator: Token,
    pub right: Box<ExprEnum>,
}

impl Binary {
    pub fn new(left: ExprEnum, operator: Token, right: ExprEnum) -> Self {
        Self {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        }
    }
}

impl From<Binary> for ExprEnum {
    fn from(value: Binary) -> Self {
        ExprEnum::Binary(value)
    }
}