use crate::token::Token;

use super::ExprEnum;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

impl From<Assign> for ExprEnum {
    fn from(value: Assign) -> Self {
        ExprEnum::Assign(value)
    }
}
