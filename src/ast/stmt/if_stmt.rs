use std::fmt;

use crate::ast::expr::ExprEnum;

use super::StmtEnum;

#[derive(Debug, Clone)]
pub struct IfStmt {
    pub condition: ExprEnum,
    pub then: Box<StmtEnum>,
    pub else_branch: Option<Box<StmtEnum>>,
}

impl IfStmt {
    pub fn new(condition: ExprEnum, then: StmtEnum, else_branch: Option<StmtEnum>) -> Self {
        Self {
            condition,
            then: Box::new(then),
            else_branch: else_branch.map(Box::new),
        }
    }
}

impl From<IfStmt> for StmtEnum {
    fn from(value: IfStmt) -> Self {
        StmtEnum::IfStmt(value)
    }
}

impl fmt::Display for IfStmt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let _ = write!(f, "if ({}) {}", self.condition, self.then);
        if let Some(eb) = &self.else_branch {
            write!(f, "else {eb}")
        } else {
            write!(f, "")
        }
    }
}
