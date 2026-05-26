use std::fmt;

use crate::token::Token;

use super::ExprEnum;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Call {
    pub callee: Box<ExprEnum>,
    pub args: Vec<ExprEnum>,
}

impl Call {
    pub fn new(callee: ExprEnum, _paren: Token, args: Vec<ExprEnum>) -> Self {
        Self {
            callee: Box::new(callee),
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
        write!(
            f,
            "{}({})",
            self.callee,
            self.args
                .iter()
                .map(|a| a.to_string())
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}
