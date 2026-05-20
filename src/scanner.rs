use crate::token::{Token, TokenType};

pub struct Scanner {
    source: Vec<char>,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Self {
            source: source.chars().collect(),
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1,
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
            _ => {
                let _type = TokenType::new(ch);
                let token = Token::new(
                    _type,
                    self.source[self.start..self.current].iter().collect(),
                    None,
                    self.line,
                );
                self.tokens.push(token);
                self.start = self.current;
            }
        }
    }

    fn next(&mut self) -> char {
        let ch = self.source[self.current];
        self.current += 1;
        ch
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }
}
