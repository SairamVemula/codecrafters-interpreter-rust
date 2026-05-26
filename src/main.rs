use std::env;
use std::fs;
use std::io::{self, BufRead, Write};
use std::process;

use crate::error::{EXIT_PARSE_ERROR, EXIT_RESOLVE_ERROR, EXIT_RUNTIME_ERROR};
use crate::parser::Parser;
use crate::runtime::Interpreter;
use crate::runtime::resolver::Resolver;
use crate::scanner::Scanner;

mod ast;
mod error;
mod parser;
mod runtime;
mod scanner;
mod token;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() < 3 {
        repl();
        return;
    }

    let command = &args[1];
    let filename = &args[2];

    let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
        eprintln!("Failed to read file {}", filename);
        String::new()
    });

    match command.as_str() {
        "tokenize" => {
            if !file_contents.is_empty() {
                let mut scanner = Scanner::new(file_contents);
                let tokens = scanner.parse();
                for token in &tokens {
                    println!("{token}")
                }
                process::exit(scanner.exit_code());
            } else {
                println!("EOF  null");
            }
        }
        "parse" => {
            if !file_contents.is_empty() {
                let mut scanner = Scanner::new(file_contents);
                let tokens = scanner.parse();
                let mut parser = Parser::new(&tokens);
                let result = parser.parse_expression();
                match result {
                    Ok(expr) => println!("{expr}"),
                    Err(e) => {
                        eprintln!("{e}");
                        process::exit(EXIT_PARSE_ERROR);
                    }
                }
                process::exit(scanner.exit_code());
            } else {
                println!("EOF  null");
            }
        }
        "evaluate" => {
            if !file_contents.is_empty() {
                let mut scanner = Scanner::new(file_contents);
                let tokens = scanner.parse();
                let mut parser = Parser::new(&tokens);
                let mut interpreter = Interpreter::new();
                match parser.parse_expression() {
                    Ok(expr) => match interpreter.evaluate(&Box::new(expr)) {
                        Ok(value) => println!("{}", value.runtime_display()),
                        Err(e) => {
                            eprintln!("{e}");
                            process::exit(EXIT_RUNTIME_ERROR);
                        }
                    },
                    Err(e) => {
                        eprintln!("{e}");
                        process::exit(EXIT_PARSE_ERROR);
                    }
                }
                process::exit(scanner.exit_code());
            } else {
                println!("EOF  null");
            }
        }
        "run" => {
            if !file_contents.is_empty() {
                let mut scanner = Scanner::new(file_contents);
                let tokens = scanner.parse();
                let mut parser = Parser::new(&tokens);
                let result = parser.parse();
                match result {
                    Ok(mut statements) => {
                        let mut interpreter = Interpreter::new();
                        let mut resolver = Resolver::new(&mut interpreter);
                        if let Err(e) = resolver.resolve(&mut statements) {
                            eprintln!("{e}");
                            process::exit(EXIT_RESOLVE_ERROR);
                        }
                        if let Err(e) = interpreter.interpret(statements) {
                            eprintln!("{e}");
                            process::exit(EXIT_RUNTIME_ERROR);
                        }
                    }
                    Err(e) => {
                        eprintln!("{e}");
                        process::exit(EXIT_PARSE_ERROR);
                    }
                }
                process::exit(scanner.exit_code());
            } else {
                println!("EOF  null");
            }
        }
        _ => repl(),
    }
}

fn repl() {
    let mut interpreter = Interpreter::new();
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    loop {
        write!(stdout, "> ").unwrap();
        stdout.flush().unwrap();
        let mut line = String::new();
        match stdin.lock().read_line(&mut line) {
            Ok(0) => break,
            Err(_) => break,
            _ => {}
        }
        let line = line.trim().to_string();
        if line.is_empty() {
            continue;
        }
        let mut scanner = Scanner::new(line);
        let tokens = scanner.parse();
        let mut parser = Parser::new(&tokens);
        match parser.parse() {
            Ok(statements) => {
                if let Err(e) = interpreter.interpret(statements) {
                    eprintln!("{e}");
                }
            }
            Err(e) => {
                eprintln!("{e}");
            }
        }
    }
}

// testing cmd = ../interpreter-tester/test-stage.bat mp7
