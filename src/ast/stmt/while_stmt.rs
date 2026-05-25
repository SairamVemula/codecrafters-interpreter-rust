use std::fmt;

use crate::ast::expr::ExprEnum;

use super::StmtEnum;

#[derive(Debug, Clone)]
pub struct WhileStmt {
    pub condition: ExprEnum,
    pub body: Box<StmtEnum>,
}

impl WhileStmt {
    pub fn new(condition: ExprEnum, body: StmtEnum) -> Self {
        Self {
            condition,
            body: Box::new(body),
        }
    }
}

impl From<WhileStmt> for StmtEnum {
    fn from(value: WhileStmt) -> Self {
        StmtEnum::WhileStmt(value)
    }
}

impl fmt::Display for WhileStmt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "while ({}) {}", self.condition, self.body)
    }
}
