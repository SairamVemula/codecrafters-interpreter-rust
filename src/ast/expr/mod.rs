pub mod assign;
pub mod binary;
pub mod call;
pub mod display;
pub mod grouping;
pub mod literal;
pub mod logical;
pub mod traits;
pub mod unary;
pub mod variable;

pub use assign::Assign;
pub use binary::Binary;
pub use call::Call;
pub use grouping::Grouping;
pub use literal::Literal;
pub use logical::Logical;
pub use traits::{Expr, ExprVisitor};
pub use unary::Unary;
pub use variable::Variable;

#[derive(Debug, Clone)]
pub enum ExprEnum {
    Assign(Assign),
    Binary(Binary),
    Grouping(Grouping),
    Literal(Literal),
    Unary(Unary),
    Variable(Variable),
    Logical(Logical),
    Call(Call),
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
