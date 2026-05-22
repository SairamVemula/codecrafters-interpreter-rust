use std::env;
use std::fs;

use crate::ast_printer::AstPrinter;
use crate::error::{exit, EXIT_PARSE_ERROR, EXIT_RUNTIME_ERROR};
use crate::interpreter::Interpreter;
use crate::parser::Parser;
use crate::scanner::Scanner;

mod ast_printer;
mod error;
mod expr;
mod interpreter;
mod parser;
mod scanner;
mod token;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} tokenize <filename>", args[0]);
        return;
    }

    let command = &args[1];
    let filename = &args[2];

    match command.as_str() {
        "tokenize" => {
            eprintln!("Logs from your program will appear here!");

            let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                eprintln!("Failed to read file {}", filename);
                String::new()
            });

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
            eprintln!("Logs from your program will appear here!");

            let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                eprintln!("Failed to read file {}", filename);
                String::new()
            });

            if !file_contents.is_empty() {
                let mut scanner = Scanner::new(file_contents);
                let tokens = scanner.parse();
                let mut parser = Parser::new(tokens);
                let result = parser.parse();
                match result {
                    Ok(ex) => {
                        let ast_printer = AstPrinter::new();
                        println!("{}", ast_printer.print(&ex));
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
        "evaluate" => {
            eprintln!("Logs from your program will appear here!");

            let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                eprintln!("Failed to read file {}", filename);
                String::new()
            });

            if !file_contents.is_empty() {
                let mut scanner = Scanner::new(file_contents);
                let tokens = scanner.parse();
                let mut parser = Parser::new(tokens);
                let result = parser.parse();
                match result {
                    Ok(ex) => {
                        let interpreter = Interpreter::new();
                        let result = interpreter.evaluate(&Box::new(ex));
                        match result {
                            Ok(evaluate) => {
                                println!("{}", evaluate);
                            }
                            Err(e) => {
                                eprintln!("{}", e.to_string());
                                exit(EXIT_RUNTIME_ERROR);
                            }
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
