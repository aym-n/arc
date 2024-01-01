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

fn eval(source: &str) -> Result<(), Error> {
    let lexer = Lexer::new(source.to_string());
    let mut tokens: Vec<Token> = lexer.collect();
    tokens.push(Token::new(TokenKind::EOF, "".to_string(), None, 0));

    let mut parser = Parser::new(tokens);
    let statements = parser.parse()?;

    let interpreter = Interpreter::new();
    if interpreter.interpret(&statements) {
        std::process::exit(1);
    }

    Ok(())
}

fn repl() {
    loop {
        let mut input = String::new();
        print!(">");
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut input).unwrap();

        eval(&input);
    }
}

fn main() {
    match std::env::args().len() {
        1 => repl(),
        2 => {
            let args: Vec<String> = std::env::args().collect();
            let filename = &args[1];
            let source = std::fs::read_to_string(filename).unwrap();
            eval(&source);
        }
        _ => {
            println!("Usage: arc [filename]");
            std::process::exit(1);
        }
    }
}
