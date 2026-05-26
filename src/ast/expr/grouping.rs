use super::ExprEnum;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

impl From<Grouping> for ExprEnum {
    fn from(value: Grouping) -> Self {
        ExprEnum::Grouping(value)
    }
}
