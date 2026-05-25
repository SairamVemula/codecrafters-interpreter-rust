use crate::ast::expr::ExprEnum;
use crate::token::Token;

use super::StmtEnum;

#[derive(Debug, Clone)]
pub struct Var {
    pub name: Token,
    pub initializer: Option<Box<ExprEnum>>,
}

impl Var {
    pub fn new(name: Token, expression: Option<ExprEnum>) -> Self {
        Self {
            name,
            initializer: expression.map(Box::new),
        }
    }
}

impl From<Var> for StmtEnum {
    fn from(value: Var) -> Self {
        StmtEnum::Var(value)
    }
}
