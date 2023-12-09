use std::io::Write;

fn eval(source: &str) -> String {

    println!("{}", source);
    todo!()

}

fn repl() {
    loop {
        let mut input = String::new();
        print!("arc> ");
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut input).unwrap();

        println!("{}", input);
    }
}

fn main(){
    if std::env::args().len() == 1 {
        repl();
    } else {
        let args: Vec<String> = std::env::args().collect();
        let filename = &args[1];
        let source = std::fs::read_to_string(filename).unwrap();

        eval(&source);
        
    }   
}