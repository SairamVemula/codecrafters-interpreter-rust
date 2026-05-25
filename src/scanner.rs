use crate::ast::expr::Object;
use crate::error::EXIT_SCAN_ERROR;
use crate::token::{Token, TokenType};

pub struct Scanner {
    source: Vec<char>,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    had_error: bool,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Self {
            source: source.chars().collect(),
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
            had_error: false,
        }
    }

    pub fn parse(&mut self) -> Vec<Token> {
        while !self.is_at_end() {
            self.scan_token();
        }

        self.tokens
            .push(Token::new(TokenType::Eof, String::new(), Object::Null, self.line));
        std::mem::take(&mut self.tokens)
    }

    fn scan_token(&mut self) {
        let ch = self.next();
        match ch {
            '\n' => self.line += 1,
            '(' => self.add_token(TokenType::LeftParen, Object::Null),
            ')' => self.add_token(TokenType::RightParen, Object::Null),
            '{' => self.add_token(TokenType::LeftBrace, Object::Null),
            '}' => self.add_token(TokenType::RightBrace, Object::Null),
            ',' => self.add_token(TokenType::Comma, Object::Null),
            '.' => self.add_token(TokenType::Dot, Object::Null),
            '-' => self.add_token(TokenType::Minus, Object::Null),
            '+' => self.add_token(TokenType::Plus, Object::Null),
            ';' => self.add_token(TokenType::Semicolon, Object::Null),
            '*' => self.add_token(TokenType::Star, Object::Null),
            '=' => {
                if self.matches('=') {
                    self.add_token(TokenType::EqualEqual, Object::Null);
                } else {
                    self.add_token(TokenType::Equal, Object::Null);
                }
            }
            '!' => {
                if self.matches('=') {
                    self.add_token(TokenType::BangEqual, Object::Null);
                } else {
                    self.add_token(TokenType::Bang, Object::Null);
                }
            }
            '<' => {
                if self.matches('=') {
                    self.add_token(TokenType::LessEqual, Object::Null);
                } else {
                    self.add_token(TokenType::Less, Object::Null);
                }
            }
            '>' => {
                if self.matches('=') {
                    self.add_token(TokenType::GreaterEqual, Object::Null);
                } else {
                    self.add_token(TokenType::Greater, Object::Null);
                }
            }
            '/' => {
                if self.matches('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.next();
                    }
                } else {
                    self.add_token(TokenType::Slash, Object::Null);
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
                    self.had_error = true;
                }
            }
        }
        self.start = self.current;
    }

    fn identifier(&mut self) {
        while !self.is_at_end() && (self.peek().is_alphanumeric() || self.peek() == '_') {
            self.next();
        }
        let s: String = self.source[self.start..self.current].iter().collect();
        let token_type = TokenType::parse(&s);
        self.add_token(token_type, Object::Null);
    }

    fn number(&mut self) {
        while !self.is_at_end() && self.peek().is_numeric() {
            self.next();
        }
        if self.peek() == '.' && self.peek_next().is_numeric() {
            self.next();
            while !self.is_at_end() && self.peek().is_numeric() {
                self.next();
            }
        }
        let s: String = self.source[self.start..self.current].iter().collect();
        match s.parse::<f64>() {
            Ok(value) => {
                self.add_token(TokenType::Number, Object::Number(value));
            }
            Err(_) => {
                eprintln!("[line {}] Error: Invalid number: {}", self.line, s);
                self.had_error = true;
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
            self.had_error = true;
            eprintln!("[line {}] Error: Unterminated string.", self.line);
            return;
        }

        self.next();
        self.add_token(
            TokenType::String,
            Object::String(
                self.source[self.start + 1..self.current - 1]
                    .iter()
                    .collect(),
            ),
        );
    }

    pub fn exit_code(&self) -> i32 {
        if self.had_error {
            EXIT_SCAN_ERROR
        } else {
            0
        }
    }

    fn add_token(&mut self, token_type: TokenType, literal: Object) {
        let token = Token::new(
            token_type,
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
            '\0'
        } else {
            self.source[self.current + 1]
        }
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.source[self.current]
        }
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
