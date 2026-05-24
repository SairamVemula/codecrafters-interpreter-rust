use std::fmt;

use crate::token::Token;

use super::*;

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
        writeln!(
            f,
            "fun {} ({}) {{\n{}\n}}",
            self.name,
            self.params
                .iter()
                .map(|f| f.to_string())
                .collect::<Vec<String>>()
                .join(", "),
            self.body
                .iter()
                .map(|f| f.to_string())
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}
