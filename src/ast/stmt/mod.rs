pub mod block;
pub mod display;
pub mod expression;
pub mod print;
pub mod traits;
pub mod var;

pub use block::*;
pub use display::*;
pub use expression::*;
pub use print::*;
pub use traits::*;
pub use var::*;

use std::fmt::Debug;

use crate::{ast::expr::ExprEnum, token::Token};

pub trait StmtVisitor {
    type Output;
    fn visit_expression(&mut self, expr: &mut Expression) -> Self::Output;
    fn visit_print(&mut self, expr: &mut Print) -> Self::Output;
    fn visit_var(&mut self, expr: &mut Var) -> Self::Output;
    fn visit_block(&mut self, expr: &mut Block) -> Self::Output;
}
pub trait Stmt: Debug {
    fn accept<T>(&mut self, visitor: &mut dyn StmtVisitor<Output = T>) -> T;
}

impl Stmt for StmtEnum {
    fn accept<T>(&mut self, visitor: &mut dyn StmtVisitor<Output = T>) -> T {
        match self {
            StmtEnum::Expression(expr) => visitor.visit_expression(expr),
            StmtEnum::Print(expr) => visitor.visit_print(expr),
            StmtEnum::Var(var) => visitor.visit_var(var),
            StmtEnum::Block(block) => visitor.visit_block(block),
        }
    }
}

#[derive(Debug, Clone)]
pub enum StmtEnum {
    Expression(Expression),
    Print(Print),
    Var(Var),
    Block(Block),
}

impl StmtEnum {
    pub fn new_expression(expr: ExprEnum) -> Self {
        Self::Expression(Expression::new(expr))
    }
    pub fn new_print(expr: ExprEnum) -> Self {
        Self::Print(Print::new(expr))
    }
    pub fn new_var(name: Token, expr: Option<ExprEnum>) -> Self {
        Self::Var(Var::new(name, expr))
    }

    pub fn new_block(statements: Vec<StmtEnum>) -> Self {
        Self::Block(Block::new(statements))
    }
}