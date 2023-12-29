use std::io::Write;

mod lexer;
use crate::lexer::Lexer;

mod tokens;
use tokens::*;

mod errors;
mod ast_printer;
use crate::ast_printer::AstPrinter;

mod expr;
use expr::*;

mod parser;
use parser::*;

fn eval(source: &str) -> String {

    let lexer = Lexer::new(source.to_string());

    //return a list of tokens
    let tokens: Vec<Token> = lexer.collect();

    let result: String = tokens.iter().map(|token| token.to_string()).collect::<Vec<String>>().join("\n");

    result
}

fn repl() {
    loop {
        let mut input = String::new();
        print!(">");
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut input).unwrap();

        let output = eval(&input);
        println!("{}", output);

    }
}

fn run(){
    let source = "-123 * (45.67) \0";
    let lexer = Lexer::new(source.to_string());
    let tokens: Vec<Token> = lexer.collect();
    
    let mut parser = Parser::new(tokens);
    let expr = parser.parse().unwrap();

    let ast_printer = AstPrinter;
    let result = ast_printer.print(&expr);
    println!("{}", result);
}

fn main(){
    match std::env::args().len() {

        1 => run(),
        2 => {
            let args: Vec<String> = std::env::args().collect();
            let filename = &args[1];
            let source = std::fs::read_to_string(filename).unwrap();
            let output = eval(&source);
            println!("{}", output);
        },
        _ => {
            println!("Usage: arc [filename]");
            std::process::exit(1);
        }
    }
}
