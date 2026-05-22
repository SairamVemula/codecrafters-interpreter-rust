use anyhow::Result;

use crate::{
    error::ParseError,
    expr::{ExprEnum, Literal, StmtEnum},
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

    fn check(&mut self, _type: TokenType) -> bool {
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
        return self.peek()._type == TokenType::Eof;
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
}

// expression     → equality ;
// equality       → comparison ( ( "!=" | "==" ) comparison )* ;
// comparison     → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
// term           → factor ( ( "-" | "+" ) factor )* ;
// factor         → unary ( ( "/" | "*" ) unary )* ;
// unary          → ( "!" | "-" ) unary
//                | primary ;
// primary        → NUMBER | STRING | "true" | "false" | "nil"
//                | "(" expression ")" ;

impl<'a> Parser<'a> {
    pub fn parse(&mut self) -> Result<Vec<StmtEnum>> {
        let mut list = vec![];
        while !self.is_at_end() {
            list.push(self.statement()?);
        }
        Ok(list)
    }

    fn statement(&mut self) -> Result<StmtEnum> {
        if self.peek()._type == TokenType::Print {
            return Ok(self.print_statement()?);
        }

        Ok(self.expression_statement()?)
    }

    fn print_statement(&mut self) -> Result<StmtEnum> {
        self.advance();
        let expr = self.expression()?;
        self.consume(TokenType::Semicolon, "Expect ';' after value.")?;
        Ok(StmtEnum::new_print(expr))
    }

    fn expression_statement(&mut self) -> Result<StmtEnum> {
        let expr = self.expression()?;
        self.consume(TokenType::Semicolon, "Expect ';' after value.")?;
        Ok(StmtEnum::new_expression(expr))
    }

    fn expression(&mut self) -> Result<ExprEnum> {
        self.equality()
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
