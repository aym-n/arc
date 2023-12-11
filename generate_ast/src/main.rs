use std::fs;
use std::io::Write;

fn main(){
    match std::env::args().len() {
        2 => {
            let args: Vec<String> = std::env::args().collect();
            let output_dir = &args[1];
            define_ast(output_dir, "Expr", &[
                "Binary : Box<Expr> left, Token operator, Box<Expr> right",
                "Grouping : Box<Expr> expression",
                "Literal : i64 value",
                "Unary : Token operator, Box<Expr> right",
            ]);
        },
        _ => {
            println!("Usage: generate_ast <output directory>");
            std::process::exit(1);
        }
    }
}

fn define_ast(output_dir: &str, base_name: &str, types: &[&str]) {
    let path = format!("../{}/{}.rs", output_dir, base_name.to_lowercase());
    let mut file = fs::File::create(path).unwrap();

    file.write_all(b"use crate::tokens::Token;\n").unwrap();
    file.write_all(b"use crate::tokens::TokenKind;\n").unwrap();

    for type_ in types {
        let type_name = type_.split(":").next().unwrap().trim();
        let fields = type_.split(":").nth(1).unwrap().trim();
        define_type(&mut file, type_name, fields);
        define_constructor(&mut file, type_name, fields);
    }
}

fn define_type(file: &mut fs::File, type_name: &str, fields: &str) {

    file.write_all(format!("pub struct {} {{\n", type_name).as_bytes()).unwrap();
    let fields: Vec<&str> = fields.split(",").collect();
    for field in fields {

        let field = field.trim();
        let field_name = field.split(" ").nth(1).unwrap().trim();
        let field_type = field.split(" ").nth(0).unwrap().trim();

        file.write_all(format!("    pub {}: {},\n", field_name, field_type).as_bytes()).unwrap();
    }
    file.write_all(b"}\n").unwrap();
}

fn define_constructor(file: &mut fs::File, type_name: &str, fields: &str) {
    // impl LiteralExpr {
    //     pub fn new(value: i64) -> Self {
    //         Self {
    //             value,
    //         }
    //     }
    // }
    file.write_all(format!("impl {} {{\n", type_name).as_bytes()).unwrap();
    file.write_all(format!("    pub fn new(").as_bytes()).unwrap();
    let fields: Vec<&str> = fields.split(",").collect();
    let mut first = true;
    let parameters = fields.clone();
    for field in fields {

        let field = field.trim();
        let field_name = field.split(" ").nth(1).unwrap().trim();
        let field_type = field.split(" ").nth(0).unwrap().trim();
        if first {
            first = false;
        } else {
            file.write_all(format!(", ").as_bytes()).unwrap();
        }
        file.write_all(format!("{}: {} ", field_name, field_type).as_bytes()).unwrap();
    }
    file.write_all(format!(") -> Self {{\n").as_bytes()).unwrap();
    
    for parameter in parameters {
        let parameter = parameter.trim();
        let parameter_name = parameter.split(" ").nth(1).unwrap().trim();
        file.write_all(format!("        {},\n", parameter_name).as_bytes()).unwrap();
    }

    file.write_all(format!("    }}\n").as_bytes()).unwrap();

    file.write_all(format!("}}\n").as_bytes()).unwrap();
}


