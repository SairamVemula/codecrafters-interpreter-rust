use std::fmt;

use super::*;

use crate::token::Token;

#[derive(Debug, Clone)]
pub struct Call {
    pub callee: Box<ExprEnum>,
    // pub paren: Token,
    pub args: Vec<ExprEnum>,
}

impl Call {
    pub fn new(callee: ExprEnum, _paren: Token, args: Vec<ExprEnum>) -> Self {
        Self {
            callee: Box::new(callee),
            // paren,
            args,
        }
    }
}

impl From<Call> for ExprEnum {
    fn from(value: Call) -> Self {
        ExprEnum::Call(value)
    }
}

impl fmt::Display for Call {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "{} ({})",
            self.callee,
            self.args
                .iter()
                .map(|f| f.to_string())
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}
