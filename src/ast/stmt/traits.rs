use super::{Block, Class, Expression, Fun, IfStmt, Print, ReturnStmt, Var, WhileStmt};

pub trait StmtVisitor {
    type Output;
    fn visit_expression(&mut self, stmt: &mut Expression) -> Self::Output;
    fn visit_print(&mut self, stmt: &mut Print) -> Self::Output;
    fn visit_var(&mut self, stmt: &mut Var) -> Self::Output;
    fn visit_block(&mut self, stmt: &mut Block) -> Self::Output;
    fn visit_if_stmt(&mut self, stmt: &mut IfStmt) -> Self::Output;
    fn visit_while_stmt(&mut self, stmt: &mut WhileStmt) -> Self::Output;
    fn visit_fun_stmt(&mut self, stmt: &mut Fun) -> Self::Output;
    fn visit_return_stmt(&mut self, stmt: &mut ReturnStmt) -> Self::Output;
    fn visit_class(&mut self, stmt: &mut Class) -> Self::Output;
}

pub trait Stmt: std::fmt::Debug {
    fn accept<T>(&mut self, visitor: &mut dyn StmtVisitor<Output = T>) -> T;
}
