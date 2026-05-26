use std::collections::HashMap;

use super::Interpreter;
use super::Result;
use super::RuntimeError;

use crate::ast::stmt::Fun;
use crate::ast::{
    expr::{Expr, ExprEnum, ExprVisitor},
    stmt::{Stmt, StmtEnum, StmtVisitor},
};
use crate::token::Token;

#[derive(Debug, PartialEq)]
pub enum InFunEnum {
    Fun,
    Method,
    Initializer,
}

#[derive(Debug, PartialEq)]
pub enum ClassType {
    Class,
    SubClass,
}

pub struct Resolver<'a> {
    interpreter: &'a mut Interpreter,
    scopes: Vec<HashMap<String, bool>>,
    in_function: Option<InFunEnum>,
    current_class: Option<ClassType>,
}
impl<'a> ExprVisitor for Resolver<'a> {
    type Output = Result<()>;

    fn visit_binary(&mut self, expr: &crate::ast::expr::Binary) -> Self::Output {
        self.resolve_expr(&expr.left)?;
        self.resolve_expr(&expr.right)
    }

    fn visit_grouping(&mut self, expr: &crate::ast::expr::Grouping) -> Self::Output {
        self.resolve_expr(&expr.expression)
    }

    fn visit_literal(&mut self, _expr: &crate::ast::expr::Object) -> Self::Output {
        Ok(())
    }

    fn visit_unary(&mut self, expr: &crate::ast::expr::Unary) -> Self::Output {
        self.resolve_expr(&expr.right)
    }

    fn visit_variable(&mut self, expr: &crate::ast::expr::Variable) -> Self::Output {
        if !self.scopes.is_empty()
            && self.scopes.last().unwrap().get(&expr.name.lexeme.clone()) == Some(&false)
        {
            return Err(RuntimeError::Error {
                line: expr.name.line,
                msg: "Can't read local variable in its own initializer.".to_owned(),
            });
        }
        self.resolve_local(&expr.clone().into(), &expr.name)?;
        Ok(())
    }

    fn visit_assign(&mut self, expr: &crate::ast::expr::Assign) -> Self::Output {
        self.resolve_expr(&expr.value)?;
        self.resolve_local(&expr.clone().into(), &expr.name)?;
        Ok(())
    }

    fn visit_logical(&mut self, expr: &crate::ast::expr::Logical) -> Self::Output {
        self.resolve_expr(&expr.left)?;
        self.resolve_expr(&expr.right)
    }

    fn visit_call(&mut self, expr: &crate::ast::expr::Call) -> Self::Output {
        self.resolve_expr(&expr.callee)?;

        for arg in &expr.args {
            self.resolve_expr(arg)?;
        }
        Ok(())
    }

    fn visit_get(&mut self, expr: &crate::ast::expr::Get) -> Self::Output {
        self.resolve_expr(&expr.object)
    }

    fn visit_set(&mut self, expr: &crate::ast::expr::Set) -> Self::Output {
        self.resolve_expr(&expr.value)?;
        self.resolve_expr(&expr.object)
    }

    fn visit_this(&mut self, expr: &crate::ast::expr::This) -> Self::Output {
        if let Some(_) = self.current_class {
            return self.resolve_local(&expr.clone().into(), &expr.keyword);
        }

        Err(RuntimeError::Error {
            line: expr.keyword.line,
            msg: "Can't use 'this' outside of a class.".into(),
        })
    }

    fn visit_super(&mut self, expr: &crate::ast::expr::Super) -> Self::Output {
        if self.current_class == None {
            return Err(RuntimeError::Error {
                line: expr.keyword.line,
                msg: "Can't use 'super' outside of a class.".to_owned(),
            });
        } else if self.current_class != Some(ClassType::SubClass) {
            return Err(RuntimeError::Error {
                line: expr.keyword.line,
                msg: "Can't use 'super' in a class with no superclass.".to_owned(),
            });
        }
        self.resolve_local(&expr.clone().into(), &expr.keyword)
    }
}

impl<'a> StmtVisitor for Resolver<'a> {
    type Output = Result<()>;

    fn visit_expression(&mut self, stmt: &mut crate::ast::stmt::Expression) -> Self::Output {
        self.resolve_expr(&stmt.expression)
    }

    fn visit_print(&mut self, stmt: &mut crate::ast::stmt::Print) -> Self::Output {
        self.resolve_expr(&stmt.expression)
    }

    fn visit_var(&mut self, stmt: &mut crate::ast::stmt::Var) -> Self::Output {
        self.declare(&stmt.name)?;
        if let Some(ini) = &stmt.initializer {
            self.resolve_expr(ini)?;
        }
        self.define(&stmt.name)?;
        Ok(())
    }

    fn visit_block(&mut self, stmt: &mut crate::ast::stmt::Block) -> Self::Output {
        self.begin_scope();
        self.resolve(&mut stmt.statements)?;
        self.end_scope();
        Ok(())
    }

    fn visit_if_stmt(&mut self, stmt: &mut crate::ast::stmt::IfStmt) -> Self::Output {
        self.resolve_expr(&stmt.condition)?;
        self.resolve_stmt(&mut stmt.then)?;
        if let Some(else_branch) = &mut stmt.else_branch {
            self.resolve_stmt(else_branch)?;
        }
        Ok(())
    }

    fn visit_while_stmt(&mut self, stmt: &mut crate::ast::stmt::WhileStmt) -> Self::Output {
        self.resolve_expr(&stmt.condition)?;
        self.resolve_stmt(&mut stmt.body)
    }

    fn visit_fun_stmt(&mut self, stmt: &mut crate::ast::stmt::Fun) -> Self::Output {
        self.declare(&stmt.name)?;
        self.define(&stmt.name)?;

        self.resolve_function(stmt, Some(InFunEnum::Fun))?;
        Ok(())
    }

    fn visit_return_stmt(&mut self, stmt: &mut crate::ast::stmt::ReturnStmt) -> Self::Output {
        if self.in_function.is_none() {
            return Err(RuntimeError::Error {
                line: stmt.keyword.line,
                msg: "Can't return from top-level code.".to_owned(),
            });
        }
        if let Some(value) = &stmt.value {
            if let Some(InFunEnum::Initializer) = self.in_function {
                return Err(RuntimeError::Error {
                    line: stmt.keyword.line,
                    msg: "Can't return a value from an initializer.".into(),
                });
            }
            self.resolve_expr(value)?;
        }
        Ok(())
    }

    fn visit_class(&mut self, stmt: &mut crate::ast::stmt::Class) -> Self::Output {
        let enclosing_class = self.current_class.replace(ClassType::Class);

        self.declare(&stmt.name)?;
        self.define(&stmt.name)?;

        if let Some(superclass) = &stmt.superclass {
            if superclass.name.lexeme == stmt.name.lexeme {
                return Err(RuntimeError::Error {
                    line: superclass.name.line,
                    msg: "A class can't inherit from itself.".into(),
                });
            }
            self.current_class = Some(ClassType::SubClass);
            self.resolve_expr(&superclass.clone().into())?;
        }

        if let Some(_) = &stmt.superclass {
            self.begin_scope();
            self.scopes
                .last_mut()
                .unwrap()
                .insert("super".to_owned(), true);
        }

        self.begin_scope();
        self.scopes
            .last_mut()
            .unwrap()
            .insert("this".to_owned(), true);

        for method in &mut stmt.methods {
            if let StmtEnum::Function(fun) = method {
                let declaration = if fun.name.lexeme == "init" {
                    Some(InFunEnum::Initializer)
                } else {
                    Some(InFunEnum::Method)
                };
                self.resolve_function(fun, declaration)?;
            }
        }

        self.end_scope();

        if let Some(_) = &stmt.superclass {
            self.end_scope();
        }

        self.current_class = enclosing_class;
        Ok(())
    }
}

impl<'a> Resolver<'a> {
    pub fn new(interpreter: &'a mut Interpreter) -> Self {
        Self {
            interpreter,
            scopes: Vec::new(),
            in_function: None,
            current_class: None,
        }
    }

    pub fn resolve(&mut self, statements: &mut [StmtEnum]) -> Result<()> {
        for stmt in statements {
            self.resolve_stmt(stmt)?;
        }
        Ok(())
    }

    fn resolve_stmt(&mut self, stmt: &mut StmtEnum) -> Result<()> {
        stmt.accept(self)?;
        Ok(())
    }

    fn resolve_expr(&mut self, expr: &ExprEnum) -> Result<()> {
        expr.accept(self)?;
        Ok(())
    }

    fn begin_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }
    fn end_scope(&mut self) {
        self.scopes.pop();
    }

    fn declare(&mut self, name: &Token) -> Result<()> {
        if !self.scopes.is_empty() {
            let scope = self.scopes.last_mut().unwrap();
            if scope.contains_key(&name.lexeme) {
                return Err(RuntimeError::Error {
                    line: name.line,
                    msg: format!(
                        "Already a variable with this name '{}' in this scope.",
                        name.lexeme
                    ),
                });
            }
            scope.insert(name.lexeme.clone(), false);
        }
        Ok(())
    }
    fn define(&mut self, name: &Token) -> Result<()> {
        if !self.scopes.is_empty() {
            self.scopes
                .last_mut()
                .unwrap()
                .insert(name.lexeme.clone(), true);
        }
        Ok(())
    }
    fn resolve_local(&mut self, expr: &ExprEnum, name: &Token) -> Result<()> {
        for (i, scope) in self.scopes.iter().rev().enumerate() {
            if scope.contains_key(&name.lexeme) {
                self.interpreter.resolve(expr.clone(), i);
                return Ok(());
            }
        }
        Ok(())
    }

    fn resolve_function(&mut self, fun: &mut Fun, mut in_fun: Option<InFunEnum>) -> Result<()> {
        self.begin_scope();
        let mut original = self.in_function.take();
        self.in_function = in_fun.take();
        for param in &fun.params {
            self.declare(&param)?;
            self.define(&param)?;
        }
        self.resolve(&mut fun.body)?;
        self.in_function = original.take();
        self.end_scope();
        Ok(())
    }
}
