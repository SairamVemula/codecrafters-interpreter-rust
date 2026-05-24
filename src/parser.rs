use anyhow::Result;

use crate::{
    ast::{expr::ExprEnum, expr::Literal, stmt::StmtEnum},
    error::ParseError,
    token::{Token, TokenType},
};

#[derive(Debug)]
pub struct Parser<'a> {
    tokens: &'a Vec<Token>,
    current: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    fn matches(&mut self, types: Vec<TokenType>) -> bool {
        for _type in types {
            if self.check(_type) {
                self.advance();
                return true;
            }
        }
        return false;
    }

    fn check(&self, _type: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        return self.peek()._type == _type;
    }

    fn previous(&mut self) -> &Token {
        return &self.tokens[self.current - 1];
    }

    fn advance(&mut self) -> &Token {
        let token = &self.tokens[self.current];
        self.current += 1;
        token
    }
    fn peek(&self) -> &Token {
        return &self.tokens[self.current];
    }

    fn is_at_end(&self) -> bool {
        if self.current >= self.tokens.len() {
            return true;
        }
        return self.tokens[self.current]._type == TokenType::Eof;
    }

    fn consume(&mut self, token_type: TokenType, expected: &str) -> Result<&Token> {
        if self.check(token_type) {
            Ok(self.advance())
        } else {
            Err(ParseError::ExpectedToken {
                line: self.peek().line,
                expected: expected.to_string(),
                found: self.peek().lexeme.clone(),
            }
            .into())
        }
    }

    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous()._type == TokenType::Semicolon {
                return;
            }

            match self.peek()._type {
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
 * declaration    → varDecl | statement ;
 * statement      → exprStmt | ifStmt | printStmt | block |;
 * ifStmt         → "if" "(" expression ")" statement ( "else" statement )? ;
 * block          → "{" declaration* "}" ;
 * exprStmt       → expression ";" ;
 * printStmt      → "print" expression ";" ;
 * expression     → assignment ;
 * assignment     → IDENTIFIER "=" assignment | equality ;
 * equality       → comparison ( ( "!=" | "==" ) comparison )* ;
 * comparison     → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
 * term           → factor ( ( "-" | "+" ) factor )* ;
 * factor         → unary ( ( "/" | "*" ) unary )* ;
 * unary          → ( "!" | "-" ) unary | primary ;
 * primary        → NUMBER | STRING | "true" | "false" | "nil" | "(" expression ")" ;
 */
impl<'a> Parser<'a> {
    pub fn parse(&mut self) -> Result<Vec<StmtEnum>> {
        let mut list = vec![];
        let mut had_error = None;
        while !self.is_at_end() {
            match self.declaration() {
                Ok(stmt) => list.push(stmt),
                Err(err) => {
                    had_error = Some(err.to_string());
                    self.synchronize();
                    eprintln!("{err}");
                }
            }
        }
        if let Some(err) = had_error {
            Err(anyhow::anyhow!(err))
        } else {
            Ok(list)
        }
    }

    fn declaration(&mut self) -> Result<StmtEnum> {
        if self.matches(vec![TokenType::Var]) {
            return self.var_declaration();
        }

        self.statement()
    }

    fn var_declaration(&mut self) -> Result<StmtEnum> {
        let name = self
            .consume(TokenType::Identifier, "Expected variable name.")?
            .clone();

        let initializer = if self.matches(vec![TokenType::Equal]) {
            Some(self.expression()?)
        } else {
            None
        };
        self.consume(
            TokenType::Semicolon,
            "Expect ';' after variable declaration.",
        )?;

        Ok(StmtEnum::new_var(name.clone(), initializer))
    }

    fn statement(&mut self) -> Result<StmtEnum> {
        if self.matches(vec![TokenType::Print]) {
            return Ok(self.print_statement()?);
        }
        if self.matches(vec![TokenType::LeftBrace]) {
            let statements = self.block()?;
            return Ok(StmtEnum::new_block(statements));
        }

        Ok(self.expression_statement()?)
    }

    fn print_statement(&mut self) -> Result<StmtEnum> {
        let expr = self.expression()?;
        self.consume(TokenType::Semicolon, "Expect ';' after value.")?;
        Ok(StmtEnum::new_print(expr))
    }

    fn block(&mut self) -> Result<Vec<StmtEnum>> {
        let mut statements = vec![];

        while !self.check(TokenType::RightBrace) && !self.is_at_end() {
            statements.push(self.declaration()?);
        }

        self.consume(TokenType::RightBrace, "Expect '}' after block.")?;

        Ok(statements)
    }

    fn expression_statement(&mut self) -> Result<StmtEnum> {
        let expr = self.expression()?;
        self.consume(TokenType::Semicolon, "Expect ';' after value.")?;
        Ok(StmtEnum::new_expression(expr))
    }

    pub fn parse_expression(&mut self) -> Result<ExprEnum> {
        self.expression()
    }

    fn expression(&mut self) -> Result<ExprEnum> {
        self.assignment()
    }

    fn assignment(&mut self) -> Result<ExprEnum> {
        let expr = self.equality()?;

        if self.matches(vec![TokenType::Equal]) {
            let equals = self.previous().clone();
            let value = self.assignment()?;
            if let ExprEnum::Variable(var) = expr {
                let name = var.name;
                return Ok(ExprEnum::new_assign(name, value));
            }

            return Err(ParseError::InvalidAssignment { line: equals.line }.into());
        }

        Ok(expr)
    }

    fn equality(&mut self) -> Result<ExprEnum> {
        let mut comparison = self.comparison()?;

        while self.matches(vec![TokenType::EqualEqual, TokenType::BangEqual]) {
            let operator = self.previous().clone();
            let right = self.comparison()?;
            comparison = ExprEnum::new_binary(comparison, operator, right);
        }

        Ok(comparison)
    }

    fn comparison(&mut self) -> Result<ExprEnum> {
        let mut term = self.term()?;

        while self.matches(vec![
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = self.previous().clone();
            let right = self.term()?;
            term = ExprEnum::new_binary(term, operator, right);
        }

        Ok(term)
    }

    fn term(&mut self) -> Result<ExprEnum> {
        let mut factor = self.factor()?;

        while self.matches(vec![TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous().clone();
            let right = self.factor()?;
            factor = ExprEnum::new_binary(factor, operator, right);
        }

        Ok(factor)
    }
    fn factor(&mut self) -> Result<ExprEnum> {
        let mut unary = self.unary()?;

        while self.matches(vec![TokenType::Slash, TokenType::Star]) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            unary = ExprEnum::new_binary(unary, operator, right);
        }

        Ok(unary)
    }
    fn unary(&mut self) -> Result<ExprEnum> {
        while self.matches(vec![TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            return Ok(ExprEnum::new_unary(operator, right));
        }

        self.primary()
    }
    fn primary(&mut self) -> Result<ExprEnum> {
        let token = self.advance();
        match token._type {
            TokenType::False | TokenType::True => Ok(ExprEnum::Literal(token.literal.clone())),
            TokenType::Nil => Ok(ExprEnum::Literal(Literal::Null)),
            TokenType::Number | TokenType::String => Ok(ExprEnum::Literal(token.literal.clone())),
            TokenType::Identifier => Ok(ExprEnum::new_variable(token.clone())),
            TokenType::LeftParen => {
                let expr = self.expression()?;
                self.consume(TokenType::RightParen, "Expected ')' after expression")?;
                Ok(ExprEnum::new_grouping(expr))
            }
            _ => Err(ParseError::ExpectedExpression {
                line: token.line,
                got: token.lexeme.clone(),
            }
            .into()),
        }
    }
}
