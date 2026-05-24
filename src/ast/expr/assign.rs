use super::*;

use crate::token::Token;


#[derive(Debug, Clone)]
pub struct Assign {
    pub name: Token,
    pub value: Box<ExprEnum>,
}

impl Assign {
    pub fn new(name: Token, value: ExprEnum) -> Self {
        Self {
            name,
            value: Box::new(value),
        }
    }
}