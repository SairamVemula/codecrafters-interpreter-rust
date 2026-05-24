use super::*;

use crate::ast::expr::ExprEnum;


#[derive(Debug, Clone)]
pub struct Expression {
    pub expression: Box<ExprEnum>,
}

impl Expression {
    pub fn new(expression: ExprEnum) -> Self {
        Self {
            expression: Box::new(expression),
        }
    }
}


impl From<Expression> for StmtEnum {
    fn from(value: Expression) -> Self {
        StmtEnum::Expression(value)
    }
}