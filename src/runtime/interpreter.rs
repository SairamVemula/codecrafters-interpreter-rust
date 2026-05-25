use std::cell::RefCell;
use std::rc::Rc;

use crate::ast::expr::{
    Assign, Binary, Call, Expr, ExprEnum, ExprVisitor, Grouping, Literal, Logical, Unary,
    Variable,
};
use crate::ast::stmt::{
    Block, Expression, Fun, IfStmt, Print, ReturnStmt, Stmt, StmtEnum, StmtVisitor, Var,
    WhileStmt,
};
use crate::error::RuntimeError;
use crate::runtime::clock_fn::ClockFn;
use crate::token::TokenType;

use super::{Environment, Function, Result};

pub struct Interpreter {
    pub globals: Rc<RefCell<Environment>>,
    pub environment: Rc<RefCell<Environment>>,
}

impl ExprVisitor for Interpreter {
    type Output = Result<Literal>;

    fn visit_binary(&mut self, expr: &Binary) -> Self::Output {
        let left = self.evaluate(&expr.left)?;
        let right = self.evaluate(&expr.right)?;

        match expr.operator.token_type {
            TokenType::Plus => match (left, right) {
                (Literal::Number(a), Literal::Number(b)) => Ok(Literal::Number(a + b)),
                (Literal::String(a), Literal::String(b)) => {
                    Ok(Literal::String(format!("{a}{b}")))
                }
                _ => Err(RuntimeError::PlusTypeMismatch),
            },
            TokenType::Minus => match (left, right) {
                (Literal::Number(a), Literal::Number(b)) => Ok(Literal::Number(a - b)),
                _ => Err(RuntimeError::NonNumericOperands),
            },
            TokenType::Star => match (left, right) {
                (Literal::Number(a), Literal::Number(b)) => Ok(Literal::Number(a * b)),
                _ => Err(RuntimeError::NonNumericOperands),
            },
            TokenType::Slash => match (left, right) {
                (Literal::Number(_), Literal::Number(b)) if b == 0.0 => {
                    Err(RuntimeError::DivisionByZero)
                }
                (Literal::Number(a), Literal::Number(b)) => Ok(Literal::Number(a / b)),
                _ => Err(RuntimeError::NonNumericOperands),
            },
            TokenType::Greater => match (left, right) {
                (Literal::Number(a), Literal::Number(b)) => Ok(Literal::Boolean(a > b)),
                _ => Err(RuntimeError::NonNumericOperands),
            },
            TokenType::GreaterEqual => match (left, right) {
                (Literal::Number(a), Literal::Number(b)) => Ok(Literal::Boolean(a >= b)),
                _ => Err(RuntimeError::NonNumericOperands),
            },
            TokenType::Less => match (left, right) {
                (Literal::Number(a), Literal::Number(b)) => Ok(Literal::Boolean(a < b)),
                _ => Err(RuntimeError::NonNumericOperands),
            },
            TokenType::LessEqual => match (left, right) {
                (Literal::Number(a), Literal::Number(b)) => Ok(Literal::Boolean(a <= b)),
                _ => Err(RuntimeError::NonNumericOperands),
            },
            TokenType::EqualEqual => Ok(Literal::Boolean(left == right)),
            TokenType::BangEqual => Ok(Literal::Boolean(left != right)),
            _ => Err(RuntimeError::UnaryTypeMismatch {
                operator: expr.operator.to_string(),
                operand: format!("{left} {right}"),
            }),
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

        match expr.operator.token_type {
            TokenType::Minus => match right {
                Literal::Number(n) => Ok(Literal::Number(-n)),
                _ => Err(RuntimeError::UnaryTypeMismatch {
                    operator: expr.operator.to_string(),
                    operand: format!("{right}"),
                }),
            },
            TokenType::Bang => match right {
                Literal::Boolean(n) => Ok(Literal::Boolean(!n)),
                Literal::Number(n) => Ok(Literal::Boolean(n == 0.0)),
                Literal::Null => Ok(Literal::Boolean(true)),
                _ => Err(RuntimeError::UnaryTypeMismatch {
                    operator: expr.operator.to_string(),
                    operand: format!("{right}"),
                }),
            },
            _ => Err(RuntimeError::UnaryTypeMismatch {
                operator: expr.operator.to_string(),
                operand: format!("{right}"),
            }),
        }
    }

    fn visit_variable(&mut self, expr: &Variable) -> Self::Output {
        self.environment.borrow().get(expr.name.clone())
    }

    fn visit_assign(&mut self, expr: &Assign) -> Self::Output {
        let value = self.evaluate(&expr.value)?;
        self.environment
            .borrow_mut()
            .assign(expr.name.clone(), value.clone())?;
        Ok(value)
    }

    fn visit_logical(&mut self, expr: &Logical) -> Self::Output {
        let left = self.evaluate(&expr.left)?;

        if expr.operator.token_type == TokenType::Or {
            if left.is_truthy() {
                return Ok(left);
            }
        } else if !left.is_truthy() {
            return Ok(left);
        }

        self.evaluate(&expr.right)
    }

    fn visit_call(&mut self, expr: &Call) -> Self::Output {
        let callee = self.evaluate(&expr.callee)?;

        let mut args = Vec::new();
        for arg in &expr.args {
            args.push(self.evaluate(arg)?);
        }

        if let Literal::Callable(callee) = callee {
            if callee.arity() != args.len() {
                return Err(RuntimeError::FunctionCallArgsError {
                    required: callee.arity(),
                    passed: args.len(),
                });
            }

            return callee.call(self, args);
        }

        Err(RuntimeError::FunctionCallError)
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
        println!("{}", result.runtime_display());
        Ok(())
    }

    fn visit_var(&mut self, stmt: &mut Var) -> Self::Output {
        let value = if let Some(initializer) = &stmt.initializer {
            Some(self.evaluate(initializer)?)
        } else {
            None
        };
        self.environment
            .borrow_mut()
            .define(stmt.name.lexeme.clone(), value);
        Ok(())
    }

    fn visit_block(&mut self, block: &mut Block) -> Self::Output {
        let child = Rc::new(RefCell::new(Environment::new(Some(Rc::clone(
            &self.environment,
        )))));
        self.execute_block(&mut block.statements, child)?;
        Ok(())
    }

    fn visit_if_stmt(&mut self, stmt: &mut IfStmt) -> Self::Output {
        if self.evaluate(&Box::new(stmt.condition.clone()))?.is_truthy() {
            self.execute(&mut stmt.then)?;
        } else if let Some(ref mut else_branch) = stmt.else_branch {
            self.execute(else_branch)?;
        }
        Ok(())
    }

    fn visit_while_stmt(&mut self, stmt: &mut WhileStmt) -> Self::Output {
        while self
            .evaluate(&Box::new(stmt.condition.clone()))?
            .is_truthy()
        {
            self.execute(&mut stmt.body)?;
        }
        Ok(())
    }

    fn visit_fun_stmt(&mut self, stmt: &mut Fun) -> Self::Output {
        let function = Rc::new(Function::new(stmt.clone(), self.environment.clone()));
        self.environment
            .borrow_mut()
            .define(stmt.name.lexeme.clone(), Some(Literal::Callable(function)));
        Ok(())
    }

    fn visit_return_stmt(&mut self, stmt: &mut ReturnStmt) -> Self::Output {
        let value = stmt
            .value
            .as_ref()
            .map(|v| self.evaluate(&Box::new(v.clone())))
            .transpose()?
            .unwrap_or(Literal::Null);
        Err(RuntimeError::ReturnValue { value })
    }
}

impl Interpreter {
    pub fn new() -> Self {
        let globals = Rc::new(RefCell::new(Environment::new(None)));
        let env = Rc::new(RefCell::new(Environment::new(Some(Rc::clone(&globals)))));
        let mut interpreter = Self {
            environment: Rc::clone(&env),
            globals: Rc::clone(&globals),
        };
        interpreter.init_globals();
        interpreter
    }

    fn init_globals(&mut self) {
        self.globals
            .borrow_mut()
            .define("clock".to_owned(), Some(Literal::Callable(Rc::new(ClockFn))));
    }

    pub fn evaluate(&mut self, expr: &ExprEnum) -> Result<Literal> {
        expr.accept(self)
    }

    fn execute(&mut self, stmt: &mut StmtEnum) -> Result<()> {
        stmt.accept(self)
    }

    pub fn interpret(&mut self, statements: Vec<StmtEnum>) -> Result<()> {
        for statement in statements {
            self.execute(&mut statement.clone())?;
        }
        Ok(())
    }

    pub fn execute_block(
        &mut self,
        statements: &mut [StmtEnum],
        child: Rc<RefCell<Environment>>,
    ) -> Result<()> {
        let previous = std::mem::replace(&mut self.environment, child);
        let result = (|| -> Result<()> {
            for stmt in statements.iter_mut() {
                self.execute(stmt)?;
            }
            Ok(())
        })();
        self.environment = previous;
        result
    }
}
