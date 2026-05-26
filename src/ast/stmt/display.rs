use std::fmt;

use crate::ast::expr::{ExprEnum, Object};

use super::StmtEnum;

impl fmt::Display for StmtEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StmtEnum::Expression(e) => write!(f, "{}", e.expression),
            StmtEnum::Print(e) => write!(f, "{}", e.expression),
            StmtEnum::Var(var) => write!(
                f,
                "{} {}",
                var.name.lexeme,
                var.initializer
                    .clone()
                    .unwrap_or(Box::new(ExprEnum::Object(Object::Null)))
            ),
            StmtEnum::Block(block) => write!(
                f,
                "{{\n {} \n}}",
                block
                    .statements
                    .iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<_>>()
                    .join("\n")
            ),
            StmtEnum::IfStmt(if_stmt) => write!(f, "{if_stmt}"),
            StmtEnum::WhileStmt(while_stmt) => write!(f, "{while_stmt}"),
            StmtEnum::Function(fun) => write!(f, "{fun}"),
            StmtEnum::ReturnStmt(e) => write!(f, "{e}"),
            StmtEnum::Class(c) => write!(f, "{c}"),
        }
    }
}
