use anyhow::{Ok, Result};

use crate::{
    error::RuntimeError,
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
                _ => Err(RuntimeError::PlusTypeMismatch.into()),
            },

            TokenType::Minus => match (left, right) {
                (Literal::Number(a, _), Literal::Number(b, _)) => {
                    let r = a - b;
                    Ok(Literal::Number(r, r.to_string()))
                }
                _ => Err(RuntimeError::NonNumericOperands.into()),
            },

            TokenType::Star => match (left, right) {
                (Literal::Number(a, _), Literal::Number(b, _)) => {
                    let r = a * b;
                    Ok(Literal::Number(r, r.to_string()))
                }
                _ => Err(RuntimeError::NonNumericOperands.into()),
            },

            TokenType::Slash => match (left, right) {
                (Literal::Number(_, _), Literal::Number(0.0, _)) => {
                    Err(RuntimeError::DivisionByZero.into())
                }
                (Literal::Number(a, _), Literal::Number(b, _)) => {
                    let r = a / b;
                    Ok(Literal::Number(r, r.to_string()))
                }
                _ => Err(RuntimeError::NonNumericOperands.into()),
            },
            TokenType::Greater => match (left, right) {
                (Literal::Number(a, _), Literal::Number(b, _)) => Ok(Literal::Boolean(a > b)),
                _ => Err(RuntimeError::NonNumericOperands.into()),
            },

            TokenType::GreaterEqual => match (left, right) {
                (Literal::Number(a, _), Literal::Number(b, _)) => Ok(Literal::Boolean(a >= b)),
                _ => Err(RuntimeError::NonNumericOperands.into()),
            },

            TokenType::Less => match (left, right) {
                (Literal::Number(a, _), Literal::Number(b, _)) => Ok(Literal::Boolean(a < b)),
                _ => Err(RuntimeError::NonNumericOperands.into()),
            },

            TokenType::LessEqual => match (left, right) {
                (Literal::Number(a, _), Literal::Number(b, _)) => Ok(Literal::Boolean(a <= b)),
                _ => Err(RuntimeError::NonNumericOperands.into()),
            },

            TokenType::EqualEqual => Ok(Literal::Boolean(left == right)),

            TokenType::BangEqual => Ok(Literal::Boolean(left != right)),

            _ => Err(RuntimeError::UnaryTypeMismatch {
                operator: expr.operator.to_string(),
                operand: format!("{} {}", left, right),
            }
            .into()),
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
                _ => Err(RuntimeError::UnaryTypeMismatch {
                    operator: expr.operator.to_string(),
                    operand: format!("{}", right),
                }
                .into()),
            },
            TokenType::Bang => match right {
                Literal::Boolean(n) => Ok(Literal::Boolean(!n)),
                Literal::Number(n, _) => Ok(Literal::Boolean(!(n != 0.0))),
                Literal::Null => Ok(Literal::Boolean(true)),
                _ => Err(RuntimeError::UnaryTypeMismatch {
                    operator: expr.operator.to_string(),
                    operand: format!("{}", right),
                }
                .into()),
            },
            _ => Err(RuntimeError::UnaryTypeMismatch {
                operator: expr.operator.to_string(),
                operand: format!("{}", right),
            }
            .into()),
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
