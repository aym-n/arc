use std::io::Write;

mod lexer;
use crate::lexer::Lexer;

mod tokens;
use tokens::*;

mod errors;
use crate::errors::*;

// mod ast_printer;
// use crate::ast_printer::AstPrinter;

mod expr;

mod parser;
use parser::*;

mod interpreter;
use interpreter::Interpreter;

mod stmt;

mod enviroment;

mod callable;

mod native_functions;

mod functions;

mod resolver;
use resolver::*;

use std::rc::Rc;

mod instance;

fn eval(source: &str) -> Result<(), Error> {
    let lexer = Lexer::new(source.to_string());
    let mut tokens: Vec<Token> = lexer.collect();

    let mut parser = Parser::new(tokens);
    let statements = parser.parse();

    let interpreter = Interpreter::new();
    let s = Rc::new(statements?);
    let resolver = Resolver::new(&interpreter);
    resolver.resolve(&Rc::clone(&s));
    if resolver.success(){
        interpreter.interpret(&Rc::clone(&s));
    }
    Ok(())
}

fn repl() {
    let interpreter = Interpreter::new();
    loop {
        let mut input = String::new();
        print!(">");
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut input).unwrap();

        match input.trim() {
            "exit" => std::process::exit(0),
            "@" => interpreter.print_env(),
            _ => {
                let lexer = Lexer::new(input);
                let mut tokens: Vec<Token> = lexer.collect();

                let mut parser = Parser::new(tokens);
                let statements = parser.parse();

                match statements {
                    Ok(statements) => {
                        let s = Rc::new(statements);
                        let mut resolver = Resolver::new(&interpreter);
                        resolver.resolve(&Rc::clone(&s));

                        if resolver.success() {
                            if !interpreter.interpret(&Rc::clone(&s)) {
                                std::process::exit(1);
                            }
                        }

                    }
                    Err(e) => println!("Error"),
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
