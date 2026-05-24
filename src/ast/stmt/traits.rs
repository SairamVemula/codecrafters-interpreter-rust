use super::*;

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
