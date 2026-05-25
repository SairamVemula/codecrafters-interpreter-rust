use std::fmt;

use crate::ast::expr::{ExprEnum, Object};
use crate::token::Token;

use super::StmtEnum;

#[derive(Debug, Clone)]
pub struct ReturnStmt {
    #[allow(dead_code)]
    pub keyword: Token,
    pub value: Option<ExprEnum>,
}

impl ReturnStmt {
    pub fn new(keyword: Token, value: Option<ExprEnum>) -> Self {
        Self { keyword, value }
    }
}

impl From<ReturnStmt> for StmtEnum {
    fn from(value: ReturnStmt) -> Self {
        StmtEnum::ReturnStmt(value)
    }
}

impl fmt::Display for ReturnStmt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "return {}",
            self.value.clone().unwrap_or(Object::Null.into())
        )
    }
}
