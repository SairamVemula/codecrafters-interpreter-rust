use super::*;

#[derive(Debug, Clone)]
pub struct Grouping {
    pub expression: Box<ExprEnum>,
}

impl Grouping {
    pub fn new(expr: ExprEnum) -> Self {
        Self {
            expression: Box::new(expr),
        }
    }
}