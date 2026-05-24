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