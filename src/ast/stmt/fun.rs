use std::fmt;

use crate::token::Token;

use super::StmtEnum;

#[derive(Debug, Clone)]
pub struct Fun {
    pub name: Token,
    pub params: Vec<Token>,
    pub body: Vec<StmtEnum>,
}

impl Fun {
    pub fn new(name: Token, params: Vec<Token>, body: Vec<StmtEnum>) -> Self {
        Self { name, params, body }
    }
}

impl From<Fun> for StmtEnum {
    fn from(value: Fun) -> Self {
        StmtEnum::Function(value)
    }
}

impl fmt::Display for Fun {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "fun {} ({}) {{\n{}\n}}",
            self.name,
            self.params
                .iter()
                .map(|p| p.to_string())
                .collect::<Vec<_>>()
                .join(", "),
            self.body
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}
