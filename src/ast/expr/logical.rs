use crate::token::Token;

use super::ExprEnum;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Logical {
    pub left: Box<ExprEnum>,
    pub operator: Token,
    pub right: Box<ExprEnum>,
}

impl Logical {
    pub fn new(left: ExprEnum, operator: Token, right: ExprEnum) -> Self {
        Self {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        }
    }
}

impl From<Logical> for ExprEnum {
    fn from(value: Logical) -> Self {
        ExprEnum::Logical(value)
    }
}
