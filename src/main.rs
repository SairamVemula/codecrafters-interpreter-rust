use std::env;
use std::fs;

use crate::error::{EXIT_PARSE_ERROR, EXIT_RUNTIME_ERROR, exit};
use crate::interpreter::Interpreter;
use crate::parser::Parser;
use crate::scanner::Scanner;

mod error;
mod expr;
mod interpreter;
mod parser;
mod scanner;
mod token;
mod environment;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} tokenize <filename>", args[0]);
        return;
    }

    let command = &args[1];
    let filename = &args[2];
    eprintln!("Logs from your program will appear here!");

    let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
        eprintln!("Failed to read file {}", filename);
        String::new()
    });

    // eprintln!("file_contents => `{}`", file_contents);

    match command.as_str() {
        "tokenize" => {
            if !file_contents.is_empty() {
                let mut scanner = Scanner::new(file_contents);
                let tokens = scanner.parse();
                for token in tokens {
                    println!("{token}")
                }
                exit(scanner.exit_code);
            } else {
                println!("EOF  null");
            }
        }
        "parse" => {
            if !file_contents.is_empty() {
                let mut scanner = Scanner::new(file_contents);
                let tokens = scanner.parse();
                let mut parser = Parser::new(tokens);
                let result = parser.parse_expression();
                match result {
                    Ok(expr) => {
                        println!("{expr}")
                    }
                    Err(e) => {
                        eprintln!("{}", e.to_string());
                        exit(EXIT_PARSE_ERROR);
                    }
                }
                exit(scanner.exit_code);
            } else {
                println!("EOF  null");
            }
        }
        "evaluate" | "run" => {
            if !file_contents.is_empty() {
                let mut scanner = Scanner::new(file_contents);
                let tokens = scanner.parse();
                let mut parser = Parser::new(tokens);
                let result = parser.parse();
                match result {
                    Ok(statements) => {
                        let mut interpreter = Interpreter::new();
                        let result = interpreter.interprete(statements);
                        match result {
                            Err(e) => {
                                eprintln!("{}", e.to_string());
                                exit(EXIT_RUNTIME_ERROR);
                            }
                            _ => {}
                        }
                    }
                    Err(e) => {
                        eprintln!("{}", e.to_string());
                        exit(EXIT_PARSE_ERROR);
                    }
                }
                exit(scanner.exit_code);
            } else {
                println!("EOF  null");
            }
        }
        _ => {
            eprintln!("Unknown command: {}", command);
        }
    }
}

// testing cmd = ../interpreter-tester/test-stage.bat mp7
