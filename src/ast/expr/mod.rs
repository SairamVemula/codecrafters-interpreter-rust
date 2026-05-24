pub mod traits;
pub mod assign;
pub mod binary;
pub mod grouping;
pub mod literal;
pub mod unary;
pub mod variable;
pub mod display;

pub use traits::*;
pub use assign::*;
pub use binary::*;
pub use grouping::*;
pub use literal::*;
pub use unary::*;
pub use variable::*;

use crate::token::Token;


#[derive(Debug, Clone)]
pub enum ExprEnum {
    Assign(Assign),
    Binary(Binary),
    Grouping(Grouping),
    Literal(Literal),
    Unary(Unary),
    Variable(Variable),
}

impl ExprEnum {
    pub fn new_binary(left: ExprEnum, operator: Token, right: ExprEnum) -> Self {
        Self::Binary(Binary::new(left, operator, right))
    }

    pub fn new_grouping(expr: ExprEnum) -> Self {
        Self::Grouping(Grouping::new(expr))
    }

    pub fn new_unary(operator: Token, right: ExprEnum) -> Self {
        Self::Unary(Unary::new(operator, right))
    }

    pub fn new_variable(name: Token) -> Self {
        Self::Variable(Variable::new(name))
    }

    pub fn new_assign(name: Token, value: ExprEnum) -> Self {
        Self::Assign(Assign::new(name, value))
    }
}

impl Expr for ExprEnum {
    fn accept<T>(&self, visitor: &mut dyn ExprVisitor<Output = T>) -> T {
        match self {
            ExprEnum::Binary(expr) => visitor.visit_binary(expr),
            ExprEnum::Grouping(expr) => visitor.visit_grouping(expr),
            ExprEnum::Literal(expr) => visitor.visit_literal(expr),
            ExprEnum::Unary(expr) => visitor.visit_unary(expr),
            ExprEnum::Variable(expr) => visitor.visit_variable(expr),
            ExprEnum::Assign(expr) => visitor.visit_assign(expr),
        }
    }
}