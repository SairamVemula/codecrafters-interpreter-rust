pub mod block;
pub mod display;
pub mod expression;
pub mod fun;
pub mod if_stmt;
pub mod print;
pub mod return_stmt;
pub mod traits;
pub mod var;
pub mod while_stmt;

pub use block::Block;
pub use expression::Expression;
pub use fun::Fun;
pub use if_stmt::IfStmt;
pub use print::Print;
pub use return_stmt::ReturnStmt;
pub use traits::{Stmt, StmtVisitor};
pub use var::Var;
pub use while_stmt::WhileStmt;

use std::fmt::Debug;

#[derive(Debug, Clone)]
pub enum StmtEnum {
    Expression(Expression),
    Print(Print),
    Var(Var),
    Block(Block),
    IfStmt(IfStmt),
    WhileStmt(WhileStmt),
    Function(Fun),
    ReturnStmt(ReturnStmt),
}

impl Stmt for StmtEnum {
    fn accept<T>(&mut self, visitor: &mut dyn StmtVisitor<Output = T>) -> T {
        match self {
            StmtEnum::Expression(expr) => visitor.visit_expression(expr),
            StmtEnum::Print(expr) => visitor.visit_print(expr),
            StmtEnum::Var(var) => visitor.visit_var(var),
            StmtEnum::Block(block) => visitor.visit_block(block),
            StmtEnum::IfStmt(if_stmt) => visitor.visit_if_stmt(if_stmt),
            StmtEnum::WhileStmt(while_stmt) => visitor.visit_while_stmt(while_stmt),
            StmtEnum::Function(fun) => visitor.visit_fun_stmt(fun),
            StmtEnum::ReturnStmt(return_stmt) => visitor.visit_return_stmt(return_stmt),
        }
    }
}
