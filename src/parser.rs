use anyhow::Result;

use crate::{
    ast::{
        expr::{Assign, Binary, Call, ExprEnum, Grouping, Literal, Logical, Unary, Variable},
        stmt::{Block, Expression, Fun, IfStmt, Print, ReturnStmt, StmtEnum, Var, WhileStmt},
    },
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
        if self.matches(vec![TokenType::Fun]) {
            return self.fun_declaration("function");
        }

        if self.matches(vec![TokenType::Var]) {
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

        let mut params = vec![];
        if !self.check(TokenType::RightParen) {
            loop {
                if params.len() >= 255 {
                    return Err(ParseError::FunctionArgsLimitExceeded {
                        line: self.peek().line,
                    }
                    .into());
                }

                params.push(
                    self.consume(TokenType::Identifier, "Expect parameter name.")?
                        .clone(),
                );

                if !self.matches(vec![TokenType::Comma]) {
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

        let initializer = if self.matches(vec![TokenType::Equal]) {
            Some(self.expression()?)
        } else {
            None
        };
        self.consume(
            TokenType::Semicolon,
            "Expect ';' after variable declaration.",
        )?;

        Ok(Var::new(name.clone(), initializer).into())
    }

    fn statement(&mut self) -> Result<StmtEnum> {
        if self.matches(vec![TokenType::For]) {
            return Ok(self.for_statement()?);
        }

        if self.matches(vec![TokenType::If]) {
            return Ok(self.if_statement()?);
        }

        if self.matches(vec![TokenType::Return]) {
            return Ok(self.return_statement()?);
        }

        if self.matches(vec![TokenType::Print]) {
            return Ok(self.print_statement()?);
        }

        if self.matches(vec![TokenType::While]) {
            return Ok(self.while_statement()?);
        }

        if self.matches(vec![TokenType::LeftBrace]) {
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

        let initializer = if self.matches(vec![TokenType::Semicolon]) {
            None
        } else if self.matches(vec![TokenType::Var]) {
            Some(self.var_declaration()?)
        } else {
            Some(self.expression_statement()?)
        };

        let condition = if !self.matches(vec![TokenType::Semicolon]) {
            self.expression()?
        } else {
            Literal::Boolean(true).into()
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

        let else_branch = if self.matches(vec![TokenType::Else]) {
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

        if self.matches(vec![TokenType::Equal]) {
            let equals = self.previous().clone();
            let value = self.assignment()?;
            if let ExprEnum::Variable(var) = expr {
                let name = var.name;
                return Ok(Assign::new(name, value).into());
            }

            return Err(ParseError::InvalidAssignment { line: equals.line }.into());
        }

        Ok(expr)
    }

    fn or(&mut self) -> Result<ExprEnum> {
        let mut expr = self.and()?;

        while self.matches(vec![TokenType::Or]) {
            let operator = self.previous().clone();
            let right = self.and()?;
            expr = Logical::new(expr, operator, right).into()
        }
        Ok(expr)
    }

    fn and(&mut self) -> Result<ExprEnum> {
        let mut expr = self.equality()?;

        while self.matches(vec![TokenType::And]) {
            let operator = self.previous().clone();
            let right = self.equality()?;
            expr = Logical::new(expr, operator, right).into()
        }

        Ok(expr)
    }

    fn equality(&mut self) -> Result<ExprEnum> {
        let mut comparison = self.comparison()?;

        while self.matches(vec![TokenType::EqualEqual, TokenType::BangEqual]) {
            let operator = self.previous().clone();
            let right = self.comparison()?;
            comparison = Binary::new(comparison, operator, right).into();
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
            term = Binary::new(term, operator, right).into();
        }

        Ok(term)
    }

    fn term(&mut self) -> Result<ExprEnum> {
        let mut factor = self.factor()?;

        while self.matches(vec![TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous().clone();
            let right = self.factor()?;
            factor = Binary::new(factor, operator, right).into();
        }

        Ok(factor)
    }
    fn factor(&mut self) -> Result<ExprEnum> {
        let mut unary = self.unary()?;

        while self.matches(vec![TokenType::Slash, TokenType::Star]) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            unary = Binary::new(unary, operator, right).into();
        }

        Ok(unary)
    }
    fn unary(&mut self) -> Result<ExprEnum> {
        while self.matches(vec![TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            return Ok(Unary::new(operator, right).into());
        }

        self.call()
    }

    fn call(&mut self) -> Result<ExprEnum> {
        let mut expr = self.primary()?;

        loop {
            if self.matches(vec![TokenType::LeftParen]) {
                expr = self.finish_call(expr)?;
            } else {
                break;
            }
        }

        Ok(expr)
    }

    fn finish_call(&mut self, callee: ExprEnum) -> Result<ExprEnum> {
        let mut args = vec![];

        if !self.check(TokenType::RightParen) {
            loop {
                if args.len() >= 255 {
                    return Err(ParseError::FunctionArgsLimitExceeded {
                        line: self.peek().line,
                    }
                    .into());
                }

                args.push(self.expression()?);
                if !self.matches(vec![TokenType::Comma]) {
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
        match token._type {
            TokenType::False | TokenType::True => Ok(ExprEnum::Literal(token.literal.clone())),
            TokenType::Nil => Ok(ExprEnum::Literal(Literal::Null)),
            TokenType::Number | TokenType::String => Ok(ExprEnum::Literal(token.literal.clone())),
            TokenType::Identifier => Ok(Variable::new(token.clone()).into()),
            TokenType::LeftParen => {
                let expr = self.expression()?;
                self.consume(TokenType::RightParen, "Expected ')' after expression")?;
                Ok(Grouping::new(expr).into())
            }
            _ => Err(ParseError::ExpectedExpression {
                line: token.line,
                got: token.lexeme.clone(),
            }
            .into()),
        }
    }
}
