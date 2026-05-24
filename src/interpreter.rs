use std::{cell::RefCell, rc::Rc};

use anyhow::{Ok, Result};

use crate::{
    ast::{
        expr::{Assign, Binary, Expr, ExprEnum, ExprVisitor, Grouping, Literal, Unary, Variable},
        stmt::{Block, Expression, IfStmt, Print, Stmt, StmtEnum, StmtVisitor, Var},
    },
    environment::Environment,
    error::RuntimeError,
    token::TokenType,
};

pub struct Interpreter {
    environment: Rc<RefCell<Environment>>,
}

impl ExprVisitor for Interpreter {
    type Output = Result<Literal>;
    fn visit_binary(&mut self, expr: &Binary) -> Self::Output {
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

    fn visit_grouping(&mut self, expr: &Grouping) -> Self::Output {
        self.evaluate(&expr.expression)
    }

    fn visit_literal(&mut self, expr: &Literal) -> Self::Output {
        Ok(expr.clone())
    }

    fn visit_unary(&mut self, expr: &Unary) -> Self::Output {
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

    fn visit_variable(&mut self, expr: &Variable) -> Self::Output {
        self.environment.borrow_mut().get(expr.name.clone())
    }

    fn visit_assign(&mut self, expr: &Assign) -> Self::Output {
        let value = self.evaluate(&expr.value)?;
        self.environment
            .borrow_mut()
            .assign(expr.name.clone(), value.clone())?;
        Ok(value)
    }

    fn visit_logical(&mut self, expr: &crate::ast::expr::Logical) -> Self::Output {
        let left = self.evaluate(&expr.left)?;

        if expr.operator._type == TokenType::Or {
            if left.is_truthy() {
                return Ok(left);
            }
        } else {
            if !left.is_truthy() {
                return Ok(left);
            }
        }

        self.evaluate(&expr.right)
    }
}

impl StmtVisitor for Interpreter {
    type Output = Result<()>;

    fn visit_expression(&mut self, stmt: &mut Expression) -> Self::Output {
        self.evaluate(&stmt.expression)?;
        Ok(())
    }

    fn visit_print(&mut self, stmt: &mut Print) -> Self::Output {
        let result = self.evaluate(&stmt.expression)?;
        println!("{result}");
        Ok(())
    }

    fn visit_var(&mut self, stmt: &mut Var) -> Self::Output {
        let value = if let Some(initializer) = &stmt.initializer {
            Some(self.evaluate(&initializer)?)
        } else {
            None
        };
        self.environment
            .borrow_mut()
            .define(stmt.name.lexeme.clone(), value);
        Ok(())
    }

    fn visit_block(&mut self, block: &mut Block) -> Self::Output {
        self.execute_block(&mut block.statements)?;
        Ok(())
    }

    fn visit_if_stmt(&mut self, stmt: &mut IfStmt) -> Self::Output {
        if self
            .evaluate(&Box::new(stmt.condition.clone()))?
            .is_truthy()
        {
            self.execute(&mut stmt.then)?;
        } else if let Some(mut else_branch) = stmt.else_branch.clone() {
            self.execute(&mut else_branch)?;
        }
        Ok(())
    }

    fn visit_while_stmt(&mut self, stmt: &mut crate::ast::stmt::WhileStmt) -> Self::Output {
        while self
            .evaluate(&Box::new(stmt.condition.clone()))?
            .is_truthy()
        {
            self.execute(&mut stmt.body)?;
        }
        Ok(())
    }
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            environment: Rc::new(RefCell::new(Environment::new(None))),
        }
    }
    pub fn evaluate(&mut self, expr: &Box<ExprEnum>) -> Result<Literal> {
        expr.accept(self)
    }

    fn execute(&mut self, stmt: &mut StmtEnum) -> Result<()> {
        stmt.accept(self)
    }

    pub fn interprete(&mut self, statements: Vec<StmtEnum>) -> Result<()> {
        for mut statement in statements {
            // eprintln!("{:?}", statement);
            self.execute(&mut statement)?
        }
        Ok(())
    }

    fn execute_block(&mut self, statements: &mut Vec<StmtEnum>) -> Result<()> {
        let child = Rc::new(RefCell::new(Environment::new(Some(Rc::clone(
            &self.environment,
        )))));
        let previous = std::mem::replace(&mut self.environment, child);
        for mut stmt in statements {
            self.execute(&mut stmt)?;
        }
        self.environment = previous;
        Ok(())
    }
}
