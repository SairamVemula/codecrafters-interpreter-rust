use super::*;

use crate::ast::expr::ExprEnum;

#[derive(Debug, Clone)]
pub struct Print {
    pub expression: Box<ExprEnum>,
}

impl Print {
    pub fn new(expression: ExprEnum) -> Self {
        Self {
            expression: Box::new(expression),
        }
    }
}

impl From<Print> for StmtEnum {
    fn from(value: Print) -> Self {
        StmtEnum::Print(value)
    }
}