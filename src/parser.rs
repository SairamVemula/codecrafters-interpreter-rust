use crate::ast::expr::{
    Assign, Binary, Call, ExprEnum, Grouping, Logical, Object, Unary, Variable,
};
use crate::ast::stmt::{
    Block, Expression, Fun, IfStmt, Print, ReturnStmt, StmtEnum, Var, WhileStmt,
};
use crate::error::ParseError;
use crate::token::{Token, TokenType};

pub type Result<T> = std::result::Result<T, ParseError>;

#[derive(Debug)]
pub struct Parser<'a> {
    tokens: &'a [Token],
    current: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a [Token]) -> Self {
        Self { tokens, current: 0 }
    }

    fn matches(&mut self, types: &[TokenType]) -> bool {
        for &token_type in types {
            if self.check(token_type) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn check(&self, token_type: TokenType) -> bool {
        !self.is_at_end() && self.peek().token_type == token_type
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    fn advance(&mut self) -> &Token {
        let token = &self.tokens[self.current];
        self.current += 1;
        token
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.tokens.len() || self.tokens[self.current].token_type == TokenType::Eof
    }

    fn consume(&mut self, token_type: TokenType, expected: &str) -> Result<&Token> {
        if self.check(token_type) {
            Ok(self.advance())
        } else {
            Err(ParseError::ExpectedToken {
                line: self.peek().line,
                expected: expected.to_string(),
                found: self.peek().lexeme.clone(),
            })
        }
    }

    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().token_type == TokenType::Semicolon {
                return;
            }

            match self.peek().token_type {
                TokenType::Var
                | TokenType::Fun
                | TokenType::Print
                | TokenType::Class
                | TokenType::For
                | TokenType::If
                | TokenType::While
                | TokenType::Return => return,
                _ => {}
            }

            self.advance();
        }
    }
}
/**
 * program        → statement* EOF ;
 * declaration    → funDecl | varDecl | statement ;
 * funDecl        → "fun" function ;
 * function       → IDENTIFIER "(" parameters? ")" block ;
 * parameters     → IDENTIFIER ( "," IDENTIFIER )* ;
 * statement      → exprStmt | forStmt | ifStmt | printStmt | returnStmt | whileStmt | block;
 * returnStmt     → "return" expression? ";" ;
 * forStmt        → "for" "(" ( varDecl | exprStmt | ";" ) expression? ";" expression? ")" statement ;
 * whileStmt      → "while" "(" expression ")" statement ;
 * ifStmt         → "if" "(" expression ")" statement ( "else" statement )? ;
 * block          → "{" declaration* "}" ;
 * exprStmt       → expression ";" ;
 * printStmt      → "print" expression ";" ;
 * expression     → assignment ;
 * assignment     → IDENTIFIER "=" assignment | logic_or ;
 * logic_or       → logic_and ( "or" logic_and )* ;
 * logic_and      → equality ( "and" equality )* ;
 * equality       → comparison ( ( "!=" | "==" ) comparison )* ;
 * comparison     → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
 * term           → factor ( ( "-" | "+" ) factor )* ;
 * factor         → unary ( ( "/" | "*" ) unary )* ;
 * unary          → ( "!" | "-" ) unary | call ;
 * call           → primary ( "(" arguments? ")" )* ;
 * arguments      → expression ( "," expression )* ;
 * primary        → NUMBER | STRING | "true" | "false" | "nil" | "(" expression ")" ;
 */
impl<'a> Parser<'a> {
    pub fn parse(&mut self) -> Result<Vec<StmtEnum>> {
        let mut list = Vec::new();
        let mut had_error: Option<ParseError> = None;
        while !self.is_at_end() {
            match self.declaration() {
                Ok(stmt) => list.push(stmt),
                Err(err) => {
                    eprintln!("{err}");
                    had_error = Some(err);
                    self.synchronize();
                }
            }
        }
        if let Some(err) = had_error {
            Err(err)
        } else {
            Ok(list)
        }
    }

    fn declaration(&mut self) -> Result<StmtEnum> {
        if self.matches(&[TokenType::Fun]) {
            return self.fun_declaration("function");
        }

        if self.matches(&[TokenType::Var]) {
            return self.var_declaration();
        }

        self.statement()
    }

    fn fun_declaration(&mut self, kind: &str) -> Result<StmtEnum> {
        let name = self
            .consume(TokenType::Identifier, &format!("Expect {kind} name."))?
            .clone();
        self.consume(
            TokenType::LeftParen,
            &format!("Expect '(' after {kind} name."),
        )?;

        let mut params = Vec::new();
        if !self.check(TokenType::RightParen) {
            loop {
                if params.len() >= 255 {
                    return Err(ParseError::FunctionArgsLimitExceeded {
                        line: self.peek().line,
                    });
                }

                params.push(
                    self.consume(TokenType::Identifier, "Expect parameter name.")?
                        .clone(),
                );

                if !self.matches(&[TokenType::Comma]) {
                    break;
                }
            }
        }
        self.consume(
            TokenType::RightParen,
            &format!("Expect ')' after parameters."),
        )?;

        self.consume(
            TokenType::LeftBrace,
            &format!("Expect '{{' after {kind} body."),
        )?;

        let body = self.block()?;

        Ok(Fun::new(name, params, body).into())
    }

    fn var_declaration(&mut self) -> Result<StmtEnum> {
        let name = self
            .consume(TokenType::Identifier, "Expected variable name.")?
            .clone();

        let initializer = if self.matches(&[TokenType::Equal]) {
            Some(self.expression()?)
        } else {
            None
        };
        self.consume(
            TokenType::Semicolon,
            "Expect ';' after variable declaration.",
        )?;

        Ok(Var::new(name, initializer).into())
    }

    fn statement(&mut self) -> Result<StmtEnum> {
        if self.matches(&[TokenType::For]) {
            return Ok(self.for_statement()?);
        }

        if self.matches(&[TokenType::If]) {
            return Ok(self.if_statement()?);
        }

        if self.matches(&[TokenType::Return]) {
            return Ok(self.return_statement()?);
        }

        if self.matches(&[TokenType::Print]) {
            return Ok(self.print_statement()?);
        }

        if self.matches(&[TokenType::While]) {
            return Ok(self.while_statement()?);
        }

        if self.matches(&[TokenType::LeftBrace]) {
            let statements = self.block()?;
            return Ok(Block::new(statements).into());
        }

        Ok(self.expression_statement()?)
    }

    fn return_statement(&mut self) -> Result<StmtEnum> {
        let keyword = self.previous().clone();
        let value = if !self.check(TokenType::Semicolon) {
            Some(self.expression()?)
        } else {
            None
        };

        self.consume(TokenType::Semicolon, "Expect ';' after return value.")?;

        Ok(ReturnStmt::new(keyword, value).into())
    }

    fn for_statement(&mut self) -> Result<StmtEnum> {
        self.consume(TokenType::LeftParen, "Expect '(' after 'for'.")?;

        let initializer = if self.matches(&[TokenType::Semicolon]) {
            None
        } else if self.matches(&[TokenType::Var]) {
            Some(self.var_declaration()?)
        } else {
            Some(self.expression_statement()?)
        };

        let condition = if !self.matches(&[TokenType::Semicolon]) {
            self.expression()?
        } else {
            Object::Boolean(true).into()
        };
        self.consume(TokenType::Semicolon, "Expect ';' after loop condition.")?;

        let increment = if !self.check(TokenType::RightParen) {
            Some(self.expression()?)
        } else {
            None
        };
        self.consume(TokenType::RightParen, "Expect ')' after for clauses.")?;

        let mut body = self.statement()?;

        if let Some(inc) = increment {
            body = Block::new(vec![body, Expression::new(inc).into()]).into()
        }

        body = WhileStmt::new(condition, body).into();

        if let Some(ini) = initializer {
            body = Block::new(vec![ini, body]).into();
        }

        Ok(body)
    }

    fn while_statement(&mut self) -> Result<StmtEnum> {
        self.consume(TokenType::LeftParen, "Expect '(' after 'while'.")?;
        let condition = self.expression()?;
        self.consume(TokenType::RightParen, "Expect ')' after while condition.")?;

        let body = self.statement()?;

        Ok(WhileStmt::new(condition, body).into())
    }

    fn if_statement(&mut self) -> Result<StmtEnum> {
        self.consume(TokenType::LeftParen, "Expect '(' after 'if'.")?;
        let condition = self.expression()?;
        self.consume(TokenType::RightParen, "Expect ')' after if condition.")?;

        let then = self.statement()?;

        let else_branch = if self.matches(&[TokenType::Else]) {
            Some(self.statement()?)
        } else {
            None
        };

        Ok(IfStmt::new(condition, then, else_branch).into())
    }

    fn print_statement(&mut self) -> Result<StmtEnum> {
        let expr = self.expression()?;
        self.consume(TokenType::Semicolon, "Expect ';' after value.")?;
        Ok(Print::new(expr).into())
    }

    fn block(&mut self) -> Result<Vec<StmtEnum>> {
        let mut statements = Vec::new();

        while !self.check(TokenType::RightBrace) && !self.is_at_end() {
            statements.push(self.declaration()?);
        }

        self.consume(TokenType::RightBrace, "Expect '}' after block.")?;

        Ok(statements)
    }

    fn expression_statement(&mut self) -> Result<StmtEnum> {
        let expr = self.expression()?;
        self.consume(TokenType::Semicolon, "Expect ';' after value.")?;
        Ok(Expression::new(expr).into())
    }

    pub fn parse_expression(&mut self) -> Result<ExprEnum> {
        self.expression()
    }

    fn expression(&mut self) -> Result<ExprEnum> {
        self.assignment()
    }

    fn assignment(&mut self) -> Result<ExprEnum> {
        let expr = self.or()?;

        if self.matches(&[TokenType::Equal]) {
            let equals = self.previous().clone();
            let value = self.assignment()?;
            if let ExprEnum::Variable(var) = expr {
                let name = var.name;
                return Ok(Assign::new(name, value).into());
            }

            return Err(ParseError::InvalidAssignment { line: equals.line });
        }

        Ok(expr)
    }

    fn or(&mut self) -> Result<ExprEnum> {
        let mut expr = self.and()?;

        while self.matches(&[TokenType::Or]) {
            let operator = self.previous().clone();
            let right = self.and()?;
            expr = Logical::new(expr, operator, right).into();
        }
        Ok(expr)
    }

    fn and(&mut self) -> Result<ExprEnum> {
        let mut expr = self.equality()?;

        while self.matches(&[TokenType::And]) {
            let operator = self.previous().clone();
            let right = self.equality()?;
            expr = Logical::new(expr, operator, right).into();
        }

        Ok(expr)
    }

    fn equality(&mut self) -> Result<ExprEnum> {
        let mut expr = self.comparison()?;

        while self.matches(&[TokenType::EqualEqual, TokenType::BangEqual]) {
            let operator = self.previous().clone();
            let right = self.comparison()?;
            expr = Binary::new(expr, operator, right).into();
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<ExprEnum> {
        let mut expr = self.term()?;

        while self.matches(&[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = self.previous().clone();
            let right = self.term()?;
            expr = Binary::new(expr, operator, right).into();
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<ExprEnum> {
        let mut expr = self.factor()?;

        while self.matches(&[TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous().clone();
            let right = self.factor()?;
            expr = Binary::new(expr, operator, right).into();
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<ExprEnum> {
        let mut expr = self.unary()?;

        while self.matches(&[TokenType::Slash, TokenType::Star]) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            expr = Binary::new(expr, operator, right).into();
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<ExprEnum> {
        if self.matches(&[TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            return Ok(Unary::new(operator, right).into());
        }

        self.call()
    }

    fn call(&mut self) -> Result<ExprEnum> {
        let mut expr = self.primary()?;

        loop {
            if self.matches(&[TokenType::LeftParen]) {
                expr = self.finish_call(expr)?;
            } else {
                break;
            }
        }

        Ok(expr)
    }

    fn finish_call(&mut self, callee: ExprEnum) -> Result<ExprEnum> {
        let mut args = Vec::new();

        if !self.check(TokenType::RightParen) {
            loop {
                if args.len() >= 255 {
                    return Err(ParseError::FunctionArgsLimitExceeded {
                        line: self.peek().line,
                    });
                }

                args.push(self.expression()?);
                if !self.matches(&[TokenType::Comma]) {
                    break;
                }
            }
        }

        let paren = self
            .consume(TokenType::RightParen, "Expect ')' after arguments.")?
            .clone();

        Ok(Call::new(callee, paren, args).into())
    }

    fn primary(&mut self) -> Result<ExprEnum> {
        let token = self.advance();
        match token.token_type {
            TokenType::True => Ok(ExprEnum::Object(Object::Boolean(true))),
            TokenType::False => Ok(ExprEnum::Object(Object::Boolean(false))),
            TokenType::Nil => Ok(ExprEnum::Object(Object::Null)),
            TokenType::Number | TokenType::String => Ok(ExprEnum::Object(token.literal.clone())),
            TokenType::Identifier => Ok(Variable::new(token.clone()).into()),
            TokenType::LeftParen => {
                let expr = self.expression()?;
                self.consume(TokenType::RightParen, "Expected ')' after expression")?;
                Ok(Grouping::new(expr).into())
            }
            _ => Err(ParseError::ExpectedExpression {
                line: token.line,
                got: token.lexeme.clone(),
            }),
        }
    }
}
