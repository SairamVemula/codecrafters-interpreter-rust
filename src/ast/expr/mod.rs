pub mod traits;
pub mod assign;
pub mod binary;
pub mod grouping;
pub mod literal;
pub mod unary;
pub mod variable;
pub mod display;
pub mod logical;
pub mod call;

pub use traits::*;
pub use assign::*;
pub use binary::*;
pub use grouping::*;
pub use literal::*;
pub use unary::*;
pub use variable::*;
pub use logical::*;
pub use call::*;


#[derive(Debug, Clone)]
pub enum ExprEnum {
    Assign(Assign),
    Binary(Binary),
    Grouping(Grouping),
    Literal(Literal),
    Unary(Unary),
    Variable(Variable),
    Logical(Logical),
    Call(Call)
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
            ExprEnum::Logical(logical) => visitor.visit_logical(logical),
            ExprEnum::Call(call) => visitor.visit_call(call),
        }
    }
}