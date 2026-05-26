use super::{
    Assign, Binary, Call, Get, Grouping, Logical, Object, Super, Set, This, Unary, Variable,
};

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
    fn visit_set(&mut self, expr: &Set) -> Self::Output;
    fn visit_this(&mut self, expr: &This) -> Self::Output;
    fn visit_super(&mut self, expr: &Super) -> Self::Output;
}

pub trait Expr: std::fmt::Debug {
    fn accept<T>(&self, visitor: &mut dyn ExprVisitor<Output = T>) -> T;
}
