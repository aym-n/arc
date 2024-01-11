mod callable;
mod enviroment;
mod errors;
mod expr;
mod functions;
mod instance;
mod interpreter;
mod lexer;
mod native_functions;
mod parser;
mod resolver;
mod stmt;
mod tokens;

use crate::errors::*;
use crate::lexer::Lexer;
use interpreter::Interpreter;
use parser::*;
use resolver::*;
use std::io::Write;
use std::rc::Rc;
use tokens::*;

fn eval(source: &str) -> Result<(), Error> {
    let mut lexer = Lexer::new(source.to_string());
    let tokens: Vec<Token> = lexer.collect();

    if lexer.success() {
        let mut parser = Parser::new(tokens);
        let statements = parser.parse();

        let interpreter = Interpreter::new();
        let s = Rc::new(statements?);
        let resolver = Resolver::new(&interpreter);
        resolver.resolve(&Rc::clone(&s));
        if resolver.success() {
            interpreter.interpret(&Rc::clone(&s));
        }
    }
    Ok(())
}

fn repl() {
    let interpreter = Interpreter::new();
    loop {
        let mut input = String::new();
        //ascii header
        println!(
            r#" 
            █████╗ ██████╗  ██████╗
            ██╔══██╗██╔══██╗██╔════╝
            ███████║██████╔╝██║     
            ██╔══██║██╔══██╗██║     
            ██║  ██║██║  ██║╚██████╗
            ╚═╝  ╚═╝╚═╝  ╚═╝ ╚═════╝ 
                    [v1.1.0]
            "#
        );            
        print!("arc~> ");
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut input).unwrap();
        input = input.trim_end_matches('\n').to_string();
        match input.trim() {
            "exit" => std::process::exit(0),
            "clear" => print!("{}[2J", 27 as char),
            "@" => interpreter.print_env(),
            _ => {
                let mut lexer = Lexer::new(input);
                let tokens: Vec<Token> = lexer.collect();

                if lexer.success() {
                    let mut parser = Parser::new(tokens);
                    let statements = parser.parse();

                    match statements {
                        Ok(statements) => {
                            let s = Rc::new(statements);
                            let resolver = Resolver::new(&interpreter);
                            resolver.resolve(&Rc::clone(&s));

                            if resolver.success() {
                                if !interpreter.interpret(&Rc::clone(&s)) {
                                    std::process::exit(1);
                                }
                            }
                        }
                        Err(_) => std::process::exit(2),
                    }
                }
            }
        }
    }
}

fn main() {
    match std::env::args().len() {
        1 => repl(),
        2 => {
            let args: Vec<String> = std::env::args().collect();
            let filename = &args[1];
            let source = std::fs::read_to_string(filename).unwrap();
            let _ = eval(&source);
        }
        _ => {
            println!("Usage: Arc [Filename]");
            std::process::exit(64);
        }
    }
}
