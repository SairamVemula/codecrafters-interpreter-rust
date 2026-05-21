use std::fmt::{Debug, Display};

use crate::token::{self, Token};

pub trait ExprVisitor {
    fn visit_binary(&self, expr: &Binary) -> String;
    fn visit_grouping(&self, expr: &Grouping) -> String;
    fn visit_literal(&self, expr: &Literal) -> String;
    fn visit_unary(&self, expr: &Unary) -> String;
}
pub trait Expr {
    fn accept(&self, visitor: &dyn ExprVisitor) -> String;
}

#[derive(Debug)]
pub enum ExprEnum {
    Binary(Binary),
    Grouping(Grouping),
    Literal(Literal),
    Unary(Unary),
}

impl Expr for ExprEnum {
    fn accept(&self, visitor: &dyn ExprVisitor) -> String {
        match self {
            ExprEnum::Binary(expr) => visitor.visit_binary(expr),
            ExprEnum::Grouping(expr) => visitor.visit_grouping(expr),
            ExprEnum::Literal(expr) => visitor.visit_literal(expr),
            ExprEnum::Unary(expr) => visitor.visit_unary(expr),
        }
    }
}

#[derive(Debug)]
pub struct Binary {
    pub left: Box<ExprEnum>,
    pub operator: Token,
    pub right: Box<ExprEnum>,
}

impl Binary {
    pub fn new(left: Box<ExprEnum>, operator: Token, right: Box<ExprEnum>) -> Self {
        Self {
            left,
            operator,
            right,
        }
    }
}

#[derive(Debug)]
pub struct Grouping {
    pub expression: Box<ExprEnum>,
}

impl Grouping {
    pub fn new(expression: Box<ExprEnum>) -> Self {
        Self { expression }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Literal {
    Null,
    String(String),
    Number(f64),
    Boolean(bool),
}

impl Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Literal::String(s) => write!(f, "{s}"),
            Literal::Number(n) => write!(f, "{:?}", n),
            Literal::Null => write!(f, "null"),
            Literal::Boolean(b) => write!(f, "{b}"),
        }
    }
}
#[derive(Debug)]
pub struct Unary {
    pub operator: Token,
    pub right: Box<ExprEnum>,
}

impl Unary {
    pub fn new(operator: Token, right: Box<ExprEnum>) -> Self {
        Self { operator, right }
    }
}
