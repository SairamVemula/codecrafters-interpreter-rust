#![allow(unused_variables)]
use std::env;
use std::fs;
use std::process;

use crate::ast_printer::AstPrinter;
use crate::interpreter::Interpreter;
use crate::parser::Parser;
use crate::scanner::Scanner;

mod ast_printer;
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
            // You can use print statements as follows for debugging, they'll be visible when running tests.
            eprintln!("Logs from your program will appear here!");

            let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                eprintln!("Failed to read file {}", filename);
                // "<<<=>>>=".to_string()
                String::new()
            });

            // TODO: Uncomment the code below to pass the first stage
            if !file_contents.is_empty() {
                // eprintln!("file_contents = {file_contents}");
                let mut scanner = Scanner::new(file_contents);
                let tokens = scanner.parse();
                // println!("{:?}", tokens);
                for token in tokens {
                    println!("{token}")
                }
                process::exit(scanner.exit_code);
            } else {
                println!("EOF  null"); // Placeholder, replace this line when implementing the scanner
            }
        }
        "parse" => {
            // You can use print statements as follows for debugging, they'll be visible when running tests.
            eprintln!("Logs from your program will appear here!");

            let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                eprintln!("Failed to read file {}", filename);
                // "<<<=>>>=".to_string()
                String::new()
            });

            // TODO: Uncomment the code below to pass the first stage
            if !file_contents.is_empty() {
                // eprintln!("file_contents = {file_contents}");
                let mut scanner = Scanner::new(file_contents);
                let tokens = scanner.parse();
                // eprintln!("{:?}", tokens);
                let mut parser = Parser::new(tokens);
                let result = parser.parse();
                match result {
                    Ok(ex) => {
                        let ast_printer = AstPrinter::new();
                        println!("{}", ast_printer.print(&ex));
                    }
                    Err(_e) => {
                        eprintln!("{}", _e.to_string());
                        process::exit(65);
                    }
                }
                process::exit(scanner.exit_code);
            } else {
                println!("EOF  null"); // Placeholder, replace this line when implementing the scanner
            }
        }
        "evaluate" => {
            // You can use print statements as follows for debugging, they'll be visible when running tests.
            eprintln!("Logs from your program will appear here!");

            let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                eprintln!("Failed to read file {}", filename);
                // "<<<=>>>=".to_string()
                String::new()
            });

            // TODO: Uncomment the code below to pass the first stage
            if !file_contents.is_empty() {
                // eprintln!("file_contents = {file_contents}");
                let mut scanner = Scanner::new(file_contents);
                let tokens = scanner.parse();
                // eprintln!("{:?}", tokens);
                let mut parser = Parser::new(tokens);
                let result = parser.parse();
                match result {
                    Ok(ex) => {
                        let interpreter = Interpreter::new();
                        let evaluate = interpreter.evaluate(&Box::new(ex)).unwrap();
                        println!("{}", evaluate);
                    }
                    Err(_e) => {
                        eprintln!("{}", _e.to_string());
                        process::exit(65);
                    }
                }
                process::exit(scanner.exit_code);
            } else {
                println!("EOF  null"); // Placeholder, replace this line when implementing the scanner
            }
        }
        _ => {
            eprintln!("Unknown command: {}", command);
        }
    }
}

// testing cmd = ../interpreter-tester/test-stage.bat mp7
