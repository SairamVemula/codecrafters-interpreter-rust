use std::process;
use thiserror::Error;

#[allow(dead_code)]
pub const EXIT_SUCCESS: i32 = 0;
pub const EXIT_SCAN_ERROR: i32 = 65;
pub const EXIT_PARSE_ERROR: i32 = 65;
pub const EXIT_RUNTIME_ERROR: i32 = 70;

pub fn exit(code: i32) -> ! {
    process::exit(code)
}

#[allow(dead_code)]
#[derive(Error, Debug)]
pub enum ScanError {
    #[error("[line {line}] Error: Unexpected character: {ch}")]
    UnexpectedCharacter { line: usize, ch: char },
    #[error("[line {line}] Error: Unterminated string.")]
    UnterminatedString { line: usize },
    #[error("[line {line}] Error: Invalid number: {lexeme}")]
    InvalidNumber { line: usize, lexeme: String },
}

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("[line {line}] Error: Expected expression, got {got}")]
    ExpectedExpression { line: usize, got: String },
    #[error("[line {line}] Error: Expected {expected} but found {found}")]
    ExpectedToken {
        line: usize,
        expected: String,
        found: String,
    },
    #[error("[line {line}] Error: Invalid assignment target")]
    InvalidAssignment { line: usize },
}

#[derive(Error, Debug)]
pub enum RuntimeError {
    #[error("Operands must be two numbers or two strings")]
    PlusTypeMismatch,
    #[error("Operands must be numbers")]
    NonNumericOperands,
    #[error("Division by zero")]
    DivisionByZero,
    #[error("Unary not implemented for {operator} and {operand}")]
    UnaryTypeMismatch { operator: String, operand: String },
    #[error("Undefined variable '{name}'.")]
    UndefinedVariable { name: String },
}
