pub mod assign;
pub mod binary;
pub mod call;
pub mod display;
pub mod grouping;
pub mod object;
pub mod logical;
pub mod traits;
pub mod unary;
pub mod variable;
pub mod get;
pub mod super_expr;
pub mod this;
pub mod set;

pub use assign::Assign;
pub use binary::Binary;
pub use call::Call;
pub use grouping::Grouping;
pub use object::Object;
pub use logical::Logical;
pub use traits::{Expr, ExprVisitor};
pub use unary::Unary;
pub use variable::Variable;
pub use get::Get;
pub use this::This;
pub use set::Set;
pub use super_expr::Super;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ExprEnum {
    Assign(Assign),
    Binary(Binary),
    Grouping(Grouping),
    Object(Object),
    Unary(Unary),
    Variable(Variable),
    Logical(Logical),
    Call(Call),
    Get(Get),
    Set(Set),
    This(This),
    Super(Super),
}

impl Expr for ExprEnum {
    fn accept<T>(&self, visitor: &mut dyn ExprVisitor<Output = T>) -> T {
        match self {
            ExprEnum::Binary(expr) => visitor.visit_binary(expr),
            ExprEnum::Grouping(expr) => visitor.visit_grouping(expr),
            ExprEnum::Object(expr) => visitor.visit_literal(expr),
            ExprEnum::Unary(expr) => visitor.visit_unary(expr),
            ExprEnum::Variable(expr) => visitor.visit_variable(expr),
            ExprEnum::Assign(expr) => visitor.visit_assign(expr),
            ExprEnum::Logical(logical) => visitor.visit_logical(logical),
            ExprEnum::Call(call) => visitor.visit_call(call),
            ExprEnum::Get(get) => visitor.visit_get(get),
            ExprEnum::Set(set) => visitor.visit_set(set),
            ExprEnum::This(this) => visitor.visit_this(this),
            ExprEnum::Super(super_extr) => visitor.visit_super(super_extr),
        }
    }
}
