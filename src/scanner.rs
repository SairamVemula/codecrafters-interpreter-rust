use crate::token::{Token, TokenType};

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

    pub fn scan_tokens(&mut self) -> &Vec<Token> {
        while !self.is_at_end() {
            self.scan_token();
        }

        self.tokens
            .push(Token::new(TokenType::Eof, "".to_string(), None, 0));
        &self.tokens
    }

    fn scan_token(&mut self) {
        let ch = self.next();
        // println!("ch = {ch}");
        match ch {
            '\n' => {
                self.line += 1;
            }
            '(' => self.add_token(TokenType::LeftParen, None),
            ')' => self.add_token(TokenType::RightParen, None),
            '{' => self.add_token(TokenType::LeftBrace, None),
            '}' => self.add_token(TokenType::RightBrace, None),
            ',' => self.add_token(TokenType::Comma, None),
            '.' => self.add_token(TokenType::Dot, None),
            '-' => self.add_token(TokenType::Minus, None),
            '+' => self.add_token(TokenType::Plus, None),
            ';' => self.add_token(TokenType::Semicolon, None),
            '*' => self.add_token(TokenType::Star, None),
            '=' => {
                if self.matches('=') {
                    self.add_token(TokenType::EqualEqual, None);
                } else {
                    self.add_token(TokenType::Equal, None);
                }
            }
            '!' => {
                if self.matches('=') {
                    self.add_token(TokenType::BangEqual, None);
                } else {
                    self.add_token(TokenType::Bang, None);
                }
            }
            '<' => {
                if self.matches('=') {
                    self.add_token(TokenType::LessEqual, None);
                } else {
                    self.add_token(TokenType::Less, None);
                }
            }
            '>' => {
                if self.matches('=') {
                    self.add_token(TokenType::GreaterEqual, None);
                } else {
                    self.add_token(TokenType::Greater, None);
                }
            }
            '/' => {
                if self.matches('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.next();
                    }
                } else {
                    self.add_token(TokenType::Slash, None);
                }
            }
            ' ' | '\r' | '\t' => {}
            '"' => self.string(),
            _ => {
                eprintln!("[line {}] Error: Unexpected character: {}", self.line, ch);
                self.exit_code = 65;
            }
        };
        self.start = self.current;
    }

    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.current += 1;
            }
            self.next();
        }

        if self.is_at_end() {
            self.exit_code = 65;
            eprintln!("[line {}] Error: Unterminated string.", self.line);
            return
        }

        self.next();
        self.add_token(
            TokenType::String,
            Some(
                self.source[self.start + 1..self.current - 1]
                    .iter()
                    .collect(),
            ),
        );
    }

    fn add_token(&mut self, _type: TokenType, literal: Option<String>) {
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
