use std::fmt;

use super::*;

use crate::{ast::expr::Literal, token::Token};

#[derive(Debug, Clone)]
pub struct ReturnStmt {
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
        writeln!(
            f,
            "return {}",
            self.value.clone().unwrap_or(Literal::Null.into())
        )
    }
}
