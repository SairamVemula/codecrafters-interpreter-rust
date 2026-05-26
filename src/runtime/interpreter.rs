use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::ast::expr::{
    Assign, Binary, Call, Expr, ExprEnum, ExprVisitor, Grouping, Logical, Object, Unary, Variable,
    object,
};
use crate::ast::stmt::{
    Block, Expression, Fun, IfStmt, Print, ReturnStmt, Stmt, StmtEnum, StmtVisitor, Var, WhileStmt,
};
use crate::error::RuntimeError;
use crate::runtime::Class;
use crate::runtime::clock_fn::ClockFn;
use crate::token::{Token, TokenType};

use super::{Environment, Function, Result};

pub struct Interpreter {
    pub globals: Rc<RefCell<Environment>>,
    pub environment: Rc<RefCell<Environment>>,
    pub locals: HashMap<ExprEnum, usize>,
}

impl ExprVisitor for Interpreter {
    type Output = Result<Object>;

    fn visit_binary(&mut self, expr: &Binary) -> Self::Output {
        let left = self.evaluate(&expr.left)?;
        let right = self.evaluate(&expr.right)?;

        match expr.operator.token_type {
            TokenType::Plus => match (left, right) {
                (Object::Number(a), Object::Number(b)) => Ok(Object::Number(a + b)),
                (Object::String(a), Object::String(b)) => Ok(Object::String(format!("{a}{b}"))),
                _ => Err(RuntimeError::PlusTypeMismatch),
            },
            TokenType::Minus => match (left, right) {
                (Object::Number(a), Object::Number(b)) => Ok(Object::Number(a - b)),
                _ => Err(RuntimeError::NonNumericOperands),
            },
            TokenType::Star => match (left, right) {
                (Object::Number(a), Object::Number(b)) => Ok(Object::Number(a * b)),
                _ => Err(RuntimeError::NonNumericOperands),
            },
            TokenType::Slash => match (left, right) {
                (Object::Number(_), Object::Number(b)) if b == 0.0 => {
                    Err(RuntimeError::DivisionByZero)
                }
                (Object::Number(a), Object::Number(b)) => Ok(Object::Number(a / b)),
                _ => Err(RuntimeError::NonNumericOperands),
            },
            TokenType::Greater => match (left, right) {
                (Object::Number(a), Object::Number(b)) => Ok(Object::Boolean(a > b)),
                _ => Err(RuntimeError::NonNumericOperands),
            },
            TokenType::GreaterEqual => match (left, right) {
                (Object::Number(a), Object::Number(b)) => Ok(Object::Boolean(a >= b)),
                _ => Err(RuntimeError::NonNumericOperands),
            },
            TokenType::Less => match (left, right) {
                (Object::Number(a), Object::Number(b)) => Ok(Object::Boolean(a < b)),
                _ => Err(RuntimeError::NonNumericOperands),
            },
            TokenType::LessEqual => match (left, right) {
                (Object::Number(a), Object::Number(b)) => Ok(Object::Boolean(a <= b)),
                _ => Err(RuntimeError::NonNumericOperands),
            },
            TokenType::EqualEqual => Ok(Object::Boolean(left == right)),
            TokenType::BangEqual => Ok(Object::Boolean(left != right)),
            _ => Err(RuntimeError::UnaryTypeMismatch {
                operator: expr.operator.to_string(),
                operand: format!("{left} {right}"),
            }),
        }
    }

    fn visit_grouping(&mut self, expr: &Grouping) -> Self::Output {
        self.evaluate(&expr.expression)
    }

    fn visit_literal(&mut self, expr: &Object) -> Self::Output {
        Ok(expr.clone())
    }

    fn visit_unary(&mut self, expr: &Unary) -> Self::Output {
        let right = self.evaluate(&expr.right)?;

        match expr.operator.token_type {
            TokenType::Minus => match right {
                Object::Number(n) => Ok(Object::Number(-n)),
                _ => Err(RuntimeError::UnaryTypeMismatch {
                    operator: expr.operator.to_string(),
                    operand: format!("{right}"),
                }),
            },
            TokenType::Bang => match right {
                Object::Boolean(n) => Ok(Object::Boolean(!n)),
                Object::Number(n) => Ok(Object::Boolean(n == 0.0)),
                Object::Null => Ok(Object::Boolean(true)),
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
        self.lookup_variable(expr.name.clone(), expr.clone().into())
    }

    fn visit_assign(&mut self, expr: &Assign) -> Self::Output {
        let value = self.evaluate(&expr.value)?;

        match self.locals.get(&expr.clone().into()) {
            Some(distance) => {
                self.environment
                    .borrow_mut()
                    .assign_at(*distance, expr.name.clone(), value.clone())
            }
            None => self
                .globals
                .borrow_mut()
                .assign(expr.name.clone(), value.clone()),
        }?;

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

        if let Object::Callable(callee) = callee {
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

    fn visit_get(&mut self, expr: &crate::ast::expr::Get) -> Self::Output {
        let object = self.evaluate(&expr.object)?;
        if let Object::Instance(instance) = object {
            return instance.get(expr.name);
        }

        Err(RuntimeError::Error {
            line: expr.name.line,
            msg: "Only instances have properties.".to_owned(),
        })
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
        if self.evaluate(&stmt.condition)?.is_truthy() {
            self.execute(&mut stmt.then)?;
        } else if let Some(ref mut else_branch) = stmt.else_branch {
            self.execute(else_branch)?;
        }
        Ok(())
    }

    fn visit_while_stmt(&mut self, stmt: &mut WhileStmt) -> Self::Output {
        while self.evaluate(&stmt.condition)?.is_truthy() {
            self.execute(&mut stmt.body)?;
        }
        Ok(())
    }

    fn visit_fun_stmt(&mut self, stmt: &mut Fun) -> Self::Output {
        let function = Rc::new(Function::new(stmt.clone(), self.environment.clone()));
        self.environment
            .borrow_mut()
            .define(stmt.name.lexeme.clone(), Some(Object::Callable(function)));
        Ok(())
    }

    fn visit_return_stmt(&mut self, stmt: &mut ReturnStmt) -> Self::Output {
        let value = stmt
            .value
            .as_ref()
            .map(|v| self.evaluate(v))
            .transpose()?
            .unwrap_or(Object::Null);
        Err(RuntimeError::ReturnValue { value })
    }

    fn visit_class(&mut self, stmt: &mut crate::ast::stmt::Class) -> Self::Output {
        self.environment.borrow_mut().define(stmt.name.lexeme, None);
        let klass = Class::new(stmt.name.lexeme);
        self.environment
            .borrow_mut()
            .define(stmt.name.lexeme, klass);
        Ok(())
    }
}

impl Interpreter {
    pub fn new() -> Self {
        let globals = Rc::new(RefCell::new(Environment::new(None)));
        let mut interpreter = Self {
            environment: Rc::clone(&globals),
            globals: Rc::clone(&globals),
            locals: HashMap::new(),
        };
        interpreter.init_globals();
        interpreter
    }

    fn init_globals(&mut self) {
        self.globals
            .borrow_mut()
            .define("clock".to_owned(), Some(Object::Callable(Rc::new(ClockFn))));
    }

    pub fn evaluate(&mut self, expr: &ExprEnum) -> Result<Object> {
        expr.accept(self)
    }

    fn execute(&mut self, stmt: &mut StmtEnum) -> Result<()> {
        stmt.accept(self)
    }

    pub fn interpret(&mut self, statements: Vec<StmtEnum>) -> Result<()> {
        for mut statement in statements {
            self.execute(&mut statement)?;
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

    pub fn resolve(&mut self, expr: ExprEnum, depth: usize) {
        self.locals.insert(expr, depth);
    }

    fn lookup_variable(&self, name: Token, expr: ExprEnum) -> Result<Object> {
        let distance = self.locals.get(&expr);
        match distance {
            Some(distance) => self.environment.borrow().get_at(*distance, name),
            None => self.globals.borrow().get(name),
        }
    }
}
