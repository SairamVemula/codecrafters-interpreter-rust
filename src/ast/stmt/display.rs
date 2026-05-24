use std::fmt;

use super::*;

use crate::ast::expr::{ExprEnum, Literal};

impl fmt::Display for StmtEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StmtEnum::Expression(e) => writeln!(f, "{}", e.expression),
            StmtEnum::Print(e) => writeln!(f, "{}", e.expression),
            StmtEnum::Var(var) => writeln!(
                f,
                "{} {}",
                var.name.lexeme,
                var.initializer
                    .clone()
                    .unwrap_or(Box::new(ExprEnum::Literal(Literal::Null)))
            ),
            StmtEnum::Block(block) => write!(
                f,
                "{{\n {} \n}}",
                block
                    .statements
                    .iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>()
                    .join("\n")
            ),
            StmtEnum::IfStmt(if_stmt) => writeln!(f, "{if_stmt}"),
            StmtEnum::WhileStmt(while_stmt) => writeln!(f, "{while_stmt}"),
        }
    }
}
