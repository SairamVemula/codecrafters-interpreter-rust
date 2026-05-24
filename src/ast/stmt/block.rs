use super::*;

use crate::ast::expr::ExprEnum;


#[derive(Debug, Clone)]
pub struct Block {
    pub statements: Vec<StmtEnum>,
}

impl Block {
    pub fn new(statements: Vec<StmtEnum>) -> Self {
        Self { statements }
    }
}