use std::fmt::Display;

#[derive(Clone, Copy, Debug)]
pub enum TokenType {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Star,
    Slash,

    Equal,
    EqualEqual,
    Bang,
    BangEqual,

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
            TokenType::Comma => write!(f, "COMMA"),
            TokenType::Dot => write!(f, "DOT"),
            TokenType::Minus => write!(f, "MINUS"),
            TokenType::Plus => write!(f, "PLUS"),
            TokenType::Semicolon => write!(f, "SEMICOLON"),
            TokenType::Star => write!(f, "STAR"),
            TokenType::Slash => write!(f, "SLASH"),
            TokenType::Equal => write!(f,"EQUAL"),
            TokenType::EqualEqual => write!(f,"EQUAL_EQUAL"),
            TokenType::Bang => write!(f,"BANG"),
            TokenType::BangEqual => write!(f,"BANG_EQUAL"),
            TokenType::Unknown => write!(f, "UNKNOWN"),
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
        write!(
            f,
            "{} {} {}",
            self._type,
            self.lexeme,
            self.literal.clone().unwrap_or("null".to_string())
        )
    }
}
