use std::fmt;

use crate::{ast::expr::Variable, token::Token};

use super::StmtEnum;

#[derive(Debug, Clone)]
pub struct Class {
    pub name: Token,
    pub superclass: Option<Variable>,
    pub methods: Vec<StmtEnum>,
}

impl Class {
    pub fn new(name: Token, methods: Vec<StmtEnum>, superclass: Option<Variable>) -> Self {
        Self {
            name,
            superclass,
            methods,
        }
    }
}

impl From<Class> for StmtEnum {
    fn from(value: Class) -> Self {
        StmtEnum::Class(value)
    }
}

impl fmt::Display for Class {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "class {} {{\n{}\n}}",
            self.name,
            self.methods
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}
