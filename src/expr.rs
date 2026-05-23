use core::fmt;
use std::fmt::{Debug, Display};

use crate::token::Token;

pub trait ExprVisitor {
    type Output;
    fn visit_binary(&mut self, expr: &Binary) -> Self::Output;
    fn visit_grouping(&mut self, expr: &Grouping) -> Self::Output;
    fn visit_literal(&mut self, expr: &Literal) -> Self::Output;
    fn visit_unary(&mut self, expr: &Unary) -> Self::Output;
    fn visit_variable(&mut self, expr: &Variable) -> Self::Output;
    fn visit_assign(&mut self, expr: &Assign) -> Self::Output;
}
pub trait Expr: Debug {
    fn accept<T>(&self, visitor: &mut dyn ExprVisitor<Output = T>) -> T;
}

#[derive(Debug, Clone)]
pub enum ExprEnum {
    Assign(Assign),
    Binary(Binary),
    Grouping(Grouping),
    Literal(Literal),
    Unary(Unary),
    Variable(Variable),
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
    pub fn new_variable(name: Token) -> Self {
        Self::Variable(Variable::new(name))
    }

    pub fn new_assign(name: Token, value: ExprEnum) -> Self {
        Self::Assign(Assign::new(name, value))
    }
}

impl Expr for ExprEnum {
    fn accept<T>(&self, visitor: &mut dyn ExprVisitor<Output = T>) -> T {
        match self {
            ExprEnum::Binary(expr) => visitor.visit_binary(expr),
            ExprEnum::Grouping(expr) => visitor.visit_grouping(expr),
            ExprEnum::Literal(expr) => visitor.visit_literal(expr),
            ExprEnum::Unary(expr) => visitor.visit_unary(expr),
            ExprEnum::Variable(expr) => visitor.visit_variable(expr),
            ExprEnum::Assign(assign) => visitor.visit_assign(assign),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Assign {
    pub name: Token,
    pub value: Box<ExprEnum>,
}

impl Assign {
    pub fn new(name: Token, value: ExprEnum) -> Self {
        Self {
            name,
            value: Box::new(value),
        }
    }
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct Variable {
    pub name: Token,
}

impl Variable {
    pub fn new(name: Token) -> Self {
        Self { name }
    }
}

pub trait StmtVisitor {
    type Output;
    fn visit_expression(&mut self, expr: &mut Expression) -> Self::Output;
    fn visit_print(&mut self, expr: &mut Print) -> Self::Output;
    fn visit_var(&mut self, expr: &mut Var) -> Self::Output;
    fn visit_block(&mut self, expr: &mut Block) -> Self::Output;
}
pub trait Stmt: Debug {
    fn accept<T>(&mut self, visitor: &mut dyn StmtVisitor<Output = T>) -> T;
}

impl Stmt for StmtEnum {
    fn accept<T>(&mut self, visitor: &mut dyn StmtVisitor<Output = T>) -> T {
        match self {
            StmtEnum::Expression(expr) => visitor.visit_expression(expr),
            StmtEnum::Print(expr) => visitor.visit_print(expr),
            StmtEnum::Var(var) => visitor.visit_var(var),
            StmtEnum::Block(block) => visitor.visit_block(block),
        }
    }
}

#[derive(Debug, Clone)]
pub enum StmtEnum {
    Expression(Expression),
    Print(Print),
    Var(Var),
    Block(Block),
}

impl StmtEnum {
    pub fn new_expression(expr: ExprEnum) -> Self {
        Self::Expression(Expression::new(expr))
    }
    pub fn new_print(expr: ExprEnum) -> Self {
        Self::Print(Print::new(expr))
    }
    pub fn new_var(name: Token, expr: Option<ExprEnum>) -> Self {
        Self::Var(Var::new(name, expr))
    }

    pub fn new_block(statements: Vec<StmtEnum>) -> Self {
        Self::Block(Block::new(statements))
    }
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct Block {
    pub statements: Vec<StmtEnum>,
}

impl Block {
    pub fn new(statements: Vec<StmtEnum>) -> Self {
        Self { statements }
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
            ExprEnum::Variable(v) => writeln!(f, "{}", v.name.lexeme),
            ExprEnum::Assign(a) => writeln!(f, "(= {} {})", a.name.lexeme, a.value),
        }
    }
}

impl fmt::Display for StmtEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StmtEnum::Expression(e) => writeln!(f, "{}", e.expression),
            StmtEnum::Print(e) => writeln!(f, "{}", e.expression),
            StmtEnum::Var(var) => writeln!(
                f,
                "{} {}",
                var.name.lexeme,
                var.initializer
                    .clone()
                    .unwrap_or(Box::new(ExprEnum::Literal(Literal::Null)))
            ),
            StmtEnum::Block(block) => write!(
                f,
                "{{\n {} \n}}",
                block
                    .statements
                    .iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>()
                    .join("\n")
            ),
        }
    }
}
