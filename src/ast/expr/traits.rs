use super::{Assign, Binary, Call, Grouping, Logical, Object, Unary, Variable, Get};

pub trait ExprVisitor {
    type Output;

    fn visit_binary(&mut self, expr: &Binary) -> Self::Output;
    fn visit_grouping(&mut self, expr: &Grouping) -> Self::Output;
    fn visit_literal(&mut self, expr: &Object) -> Self::Output;
    fn visit_unary(&mut self, expr: &Unary) -> Self::Output;
    fn visit_variable(&mut self, expr: &Variable) -> Self::Output;
    fn visit_assign(&mut self, expr: &Assign) -> Self::Output;
    fn visit_logical(&mut self, expr: &Logical) -> Self::Output;
    fn visit_call(&mut self, expr: &Call) -> Self::Output;
    fn visit_get(&mut self, expr: &Get) -> Self::Output;
}

pub trait Expr: std::fmt::Debug {
    fn accept<T>(&self, visitor: &mut dyn ExprVisitor<Output = T>) -> T;
}
