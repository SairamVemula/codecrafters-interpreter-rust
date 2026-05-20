use std::fmt::Display;

#[derive(Clone, Copy, Debug)]
pub enum TokenType {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,

    Eof,
    Unknown,
}

impl Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenType::LeftParen => write!(f, "LEFT_PAREN"),
            TokenType::RightParen => write!(f, "RIGHT_PAREN"),
            TokenType::LeftBrace => write!(f, "LEFT_BRACE"),
            TokenType::RightBrace => write!(f, "RIGHT_BRACE"),
            TokenType::Eof => write!(f, "EOF"),
            TokenType::Unknown => write!(f, "UNKNOWN"),
        }
    }
}

impl TokenType {
    pub fn new(s: char) -> Self {
        match s {
            '(' => TokenType::LeftParen,
            ')' => TokenType::RightParen,
            '{' => TokenType::LeftBrace,
            '}' => TokenType::RightBrace,
            _ => TokenType::Unknown,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Token {
    _type: TokenType,
    lexeme: String,
    literal: Option<String>,
    line: usize,
}

impl Token {
    pub fn new(_type: TokenType, lexeme: String, literal: Option<String>, line: usize) -> Self {
        Self {
            _type,
            lexeme,
            literal,
            line,
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "{} {} {}",
            self._type,
            self.lexeme,
            self.literal.clone().unwrap_or("null".to_string())
        )
    }
}
