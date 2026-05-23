use crate::{
    error::EXIT_SCAN_ERROR,
    expr::Literal,
    token::{Token, TokenType},
};

pub struct Scanner {
    source: Vec<char>,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    pub exit_code: i32,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Self {
            source: source.chars().collect(),
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1,
            exit_code: 0,
        }
    }

    pub fn parse(&mut self) -> &Vec<Token> {
        while !self.is_at_end() {
            self.scan_token();
        }

        self.tokens
            .push(Token::new(TokenType::Eof, "".to_string(), Literal::Null, 0));
        &self.tokens
    }

    fn scan_token(&mut self) {
        let ch = self.next();
        // println!("ch = {ch}");
        match ch {
            '\n' => {
                self.line += 1;
            }
            '(' => self.add_token(TokenType::LeftParen, Literal::Null),
            ')' => self.add_token(TokenType::RightParen, Literal::Null),
            '{' => self.add_token(TokenType::LeftBrace, Literal::Null),
            '}' => self.add_token(TokenType::RightBrace, Literal::Null),
            ',' => self.add_token(TokenType::Comma, Literal::Null),
            '.' => self.add_token(TokenType::Dot, Literal::Null),
            '-' => self.add_token(TokenType::Minus, Literal::Null),
            '+' => self.add_token(TokenType::Plus, Literal::Null),
            ';' => self.add_token(TokenType::Semicolon, Literal::Null),
            '*' => self.add_token(TokenType::Star, Literal::Null),
            '=' => {
                if self.matches('=') {
                    self.add_token(TokenType::EqualEqual, Literal::Null);
                } else {
                    self.add_token(TokenType::Equal, Literal::Null);
                }
            }
            '!' => {
                if self.matches('=') {
                    self.add_token(TokenType::BangEqual, Literal::Null);
                } else {
                    self.add_token(TokenType::Bang, Literal::Null);
                }
            }
            '<' => {
                if self.matches('=') {
                    self.add_token(TokenType::LessEqual, Literal::Null);
                } else {
                    self.add_token(TokenType::Less, Literal::Null);
                }
            }
            '>' => {
                if self.matches('=') {
                    self.add_token(TokenType::GreaterEqual, Literal::Null);
                } else {
                    self.add_token(TokenType::Greater, Literal::Null);
                }
            }
            '/' => {
                if self.matches('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.next();
                    }
                } else {
                    self.add_token(TokenType::Slash, Literal::Null);
                }
            }
            ' ' | '\r' | '\t' => {}
            '"' => self.string(),
            _ => {
                if ch.is_numeric() {
                    self.number();
                } else if ch.is_alphabetic() || ch == '_' {
                    self.identifier();
                } else {
                    eprintln!("[line {}] Error: Unexpected character: {}", self.line, ch);
                    self.exit_code = EXIT_SCAN_ERROR;
                }
            }
        };
        self.start = self.current;
    }

    fn identifier(&mut self) {
        while self.peek().is_alphanumeric() || self.peek() == '_' && !self.is_at_end() {
            self.next();
        }
        let str: String = self.source[self.start..self.current].iter().collect();
        let _type = TokenType::parse(str);
        let literal = match _type {
            TokenType::True => Literal::Boolean(true),
            TokenType::False => Literal::Boolean(false),
            _ => Literal::Null,
        };
        self.add_token(_type, literal);
    }

    fn number(&mut self) {
        while self.peek().is_numeric() && !self.is_at_end() {
            self.next();
        }
        if self.peek() == '.' && self.peek_next().is_numeric() {
            self.next();
            while self.peek().is_numeric() && !self.is_at_end() {
                self.next();
            }
        }
        let str: String = self.source[self.start..self.current].iter().collect();
        match str.parse::<f64>() {
            Ok(value) => {
                self.add_token(TokenType::Number, Literal::Number(value, str));
            }
            Err(_) => {
                eprintln!("[line {}] Error: Invalid number: {}", self.line, str);
                self.exit_code = EXIT_SCAN_ERROR;
            }
        }
    }

    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.next();
        }

        if self.is_at_end() {
            self.exit_code = EXIT_SCAN_ERROR;
            eprintln!("[line {}] Error: Unterminated string.", self.line);
            return;
        }

        self.next();
        self.add_token(
            TokenType::String,
            Literal::String(
                self.source[self.start + 1..self.current - 1]
                    .iter()
                    .collect(),
            ),
        );
    }

    fn add_token(&mut self, _type: TokenType, literal: Literal) {
        let token = Token::new(
            _type,
            self.source[self.start..self.current].iter().collect(),
            literal,
            self.line,
        );
        self.tokens.push(token);
    }

    fn next(&mut self) -> char {
        let ch = self.source[self.current];
        self.current += 1;
        ch
    }
    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }
        return self.source[self.current + 1];
    }
    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        return self.source[self.current];
    }

    fn matches(&mut self, ch: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.source[self.current] != ch {
            return false;
        }
        self.current += 1;
        true
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }
}
