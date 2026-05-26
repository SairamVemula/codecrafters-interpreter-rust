use std::fmt;

use super::ExprEnum;

impl fmt::Display for ExprEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ExprEnum::Object(l) => write!(f, "{l}"),
            ExprEnum::Grouping(g) => write!(f, "(group {})", g.expression),
            ExprEnum::Binary(b) => write!(f, "({} {} {})", b.operator.lexeme, b.left, b.right),
            ExprEnum::Unary(u) => write!(f, "({} {})", u.operator.lexeme, u.right),
            ExprEnum::Variable(v) => write!(f, "{}", v.name.lexeme),
            ExprEnum::Assign(a) => write!(f, "(= {} {})", a.name.lexeme, a.value),
            ExprEnum::Logical(l) => write!(f, "({} {} {})", l.operator.lexeme, l.left, l.right),
            ExprEnum::Call(call) => write!(f, "{call}"),
            ExprEnum::Get(get) => write!(f, "{get}"),
            ExprEnum::Set(set) => write!(f,"{set}"),
            ExprEnum::This(this) => write!(f,"{this}"),
            ExprEnum::Super(s) => write!(f,"{s}"),
        }
    }
}
