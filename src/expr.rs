use core::fmt;
use std::fmt::{Debug, Display};

use crate::token::Token;

pub trait ExprVisitor {
    type Output;
    fn visit_binary(&self, expr: &Binary) -> Self::Output;
    fn visit_grouping(&self, expr: &Grouping) -> Self::Output;
    fn visit_literal(&self, expr: &Literal) -> Self::Output;
    fn visit_unary(&self, expr: &Unary) -> Self::Output;
}
pub trait Expr: Debug {
    fn accept<T>(&self, visitor: &dyn ExprVisitor<Output = T>) -> T;
}

#[derive(Debug)]
pub enum ExprEnum {
    Binary(Binary),
    Grouping(Grouping),
    Literal(Literal),
    Unary(Unary),
}

impl ExprEnum {
    pub fn new_binary(left: ExprEnum, operator: Token, right: ExprEnum) -> Self {
        Self::Binary(Binary::new(left, operator, right))
    }
    pub fn new_grouping(expr: ExprEnum) -> Self {
        Self::Grouping(Grouping::new(expr))
    }
    pub fn new_unary(operator: Token, right: ExprEnum) -> Self {
        Self::Unary(Unary::new(operator, right))
    }
}

impl Expr for ExprEnum {
    fn accept<T>(&self, visitor: &dyn ExprVisitor<Output = T>) -> T {
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
    pub fn new(left: ExprEnum, operator: Token, right: ExprEnum) -> Self {
        Self {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        }
    }
}

#[derive(Debug)]
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

#[derive(Clone, Debug, PartialEq)]
pub enum Literal {
    Null,
    String(String),
    Number(f64, String),
    Boolean(bool),
}

impl Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Literal::String(s) => write!(f, "{s}"),
            Literal::Number(_, s) => write!(f, "{s}"),
            Literal::Null => write!(f, "nil"),
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
    pub fn new(operator: Token, right: ExprEnum) -> Self {
        Self {
            operator,
            right: Box::new(right),
        }
    }
}

pub trait StmtVisitor {
    type Output;
    fn visit_expression(&self, expr: &Expression) -> Self::Output;
    fn visit_print(&self, expr: &Print) -> Self::Output;
}
pub trait Stmt: Debug {
    fn accept<T>(&self, visitor: &dyn StmtVisitor<Output = T>) -> T;
}

impl Stmt for StmtEnum {
    fn accept<T>(&self, visitor: &dyn StmtVisitor<Output = T>) -> T {
        match self {
            StmtEnum::Expression(expr) => visitor.visit_expression(expr),
            StmtEnum::Print(expr) => visitor.visit_print(expr),
        }
    }
}

#[derive(Debug)]
pub enum StmtEnum {
    Expression(Expression),
    Print(Print),
}

impl StmtEnum {
    pub fn new_expression(expr: ExprEnum) -> Self {
        Self::Expression(Expression::new(expr))
    }
    pub fn new_print(expr: ExprEnum) -> Self {
        Self::Print(Print::new(expr))
    }
}

#[derive(Debug)]
pub struct Expression {
    pub expression: Box<ExprEnum>,
}

impl Expression {
    pub fn new(expression: ExprEnum) -> Self {
        Self {
            expression: Box::new(expression),
        }
    }
}

#[derive(Debug)]
pub struct Print {
    pub expression: Box<ExprEnum>,
}

impl Print {
    pub fn new(expression: ExprEnum) -> Self {
        Self {
            expression: Box::new(expression),
        }
    }
}

impl fmt::Display for ExprEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ExprEnum::Literal(l) => write!(f, "{l}"),
            ExprEnum::Grouping(g) => write!(f, "(group {})", g.expression),
            ExprEnum::Binary(b) => write!(f, "({} {} {})", b.operator.lexeme, b.left, b.right),
            ExprEnum::Unary(u) => {
                write!(f, "({} {})", u.operator.lexeme, u.right)
            }
        }
    }
}

impl fmt::Display for StmtEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StmtEnum::Expression(e) => writeln!(f, "{}", e.expression),
            StmtEnum::Print(e) => writeln!(f, "{}", e.expression),
        }
    }
}
