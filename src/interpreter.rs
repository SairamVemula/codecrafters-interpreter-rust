use anyhow::{Ok, Result, anyhow};

use crate::{
    expr::{Binary, Expr, ExprEnum, ExprVisitor, Grouping, Literal, Unary},
    token::TokenType,
};

pub struct Interpreter {}

impl ExprVisitor for Interpreter {
    type Output = Result<Literal>;
    fn visit_binary(&self, expr: &Binary) -> Self::Output {
        let left = self.evaluate(&expr.left)?;
        let right = self.evaluate(&expr.right)?;

        match expr.operator._type {
            TokenType::Plus => match (left, right) {
                (Literal::Number(a, _), Literal::Number(b, _)) => {
                    let r = a + b;
                    Ok(Literal::Number(r, r.to_string()))
                }
                (Literal::String(a), Literal::String(b)) => {
                    Ok(Literal::String(format!("{}{}", a, b)))
                }
                _ => Err(anyhow!("Operands must be two numbers or two strings")),
            },

            TokenType::Minus => match (left, right) {
                (Literal::Number(a, _), Literal::Number(b, _)) => {
                    let r = a - b;
                    Ok(Literal::Number(r, r.to_string()))
                }
                _ => Err(anyhow!("Operands must be numbers")),
            },

            TokenType::Star => match (left, right) {
                (Literal::Number(a, _), Literal::Number(b, _)) => {
                    let r = a * b;
                    Ok(Literal::Number(r, r.to_string()))
                }
                _ => Err(anyhow!("Operands must be numbers")),
            },

            TokenType::Slash => match (left, right) {
                (Literal::Number(_, _), Literal::Number(0.0, _)) => {
                    Err(anyhow!("Division by zero"))
                }
                (Literal::Number(a, _), Literal::Number(b, _)) => {
                    let r = a / b;
                    Ok(Literal::Number(r, r.to_string()))
                }
                _ => Err(anyhow!("Operands must be numbers")),
            },
            // TokenType::Plus => match right {
            //     Literal::Boolean(n) => Ok(Literal::Boolean(!n)),
            //     _ => Err(anyhow!("Unary not implemented")),
            // },
            _ => Err(anyhow!("Unary operater not implemented")),
        }
    }

    fn visit_grouping(&self, expr: &Grouping) -> Self::Output {
        self.evaluate(&expr.expression)
    }

    fn visit_literal(&self, expr: &Literal) -> Self::Output {
        Ok(expr.clone())
    }

    fn visit_unary(&self, expr: &Unary) -> Self::Output {
        let right = self.evaluate(&expr.right)?;

        match expr.operator._type {
            TokenType::Minus => match right {
                Literal::Number(n, _) => {
                    let n = -n;
                    Ok(Literal::Number(n, n.to_string()))
                }
                _ => Err(anyhow!(
                    "Unary not implemented for {} and {}",
                    expr.operator,
                    right
                )),
            },
            TokenType::Bang => match right {
                Literal::Boolean(n) => Ok(Literal::Boolean(!n)),
                Literal::Number(n, _) => Ok(Literal::Boolean(!(n != 0.0))),
                Literal::Null => Ok(Literal::Boolean(true)),
                _ => Err(anyhow!(
                    "Unary not implemented for {} and {}",
                    expr.operator,
                    right
                )),
            },
            _ => Err(anyhow!("Unary operater not implemented")),
        }
    }
}

impl Interpreter {
    pub fn new() -> Self {
        Self {}
    }
    pub fn evaluate(&self, expr: &Box<ExprEnum>) -> Result<Literal> {
        expr.accept(self)
    }
}
