mod lexer;
mod tokens;
mod errors;
mod ast_printer;
mod expr;

use std::io::Write;
use tokens::Token;
use crate::lexer::Lexer;

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

fn main(){
    match std::env::args().len() {

        1 => repl(),
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
