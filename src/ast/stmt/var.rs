use super::*;

use crate::{ast::expr::ExprEnum, token::Token};

#[derive(Debug, Clone)]
pub struct Var {
    pub name: Token,
    pub initializer: Option<Box<ExprEnum>>,
}

impl Var {
    pub fn new(name: Token, expression: Option<ExprEnum>) -> Self {
        Self {
            name,
            initializer: if let Some(expr) = expression {
                Some(Box::new(expr))
            } else {
                None
            },
        }
    }
}
