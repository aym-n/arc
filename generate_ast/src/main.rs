use std::fs;
use std::io::Write;

fn main(){
    match std::env::args().len() {
        2 => {
            let args: Vec<String> = std::env::args().collect();
            let output_dir = &args[1];
            define_ast(output_dir, "Expr", &[
                "Binary : Expr left, Token operator, Expr right",
                "Grouping : Expr expression",
                "Literal : Object value",
                "Unary : Token operator, Expr right",
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

    file.write_all(format!("pub enum {} {{\n", base_name).as_bytes()).unwrap();

    for type_ in types {
        let type_name = type_.split(":").next().unwrap().trim();
        let fields = type_.split(":").nth(1).unwrap().trim();
        define_type(&mut file, type_name, fields);
    }

    file.write_all(b"}\n").unwrap();

}

fn define_type(file: &mut fs::File, type_name: &str, fields: &str) {
    file.write_all(format!("    {} {{\n", type_name).as_bytes()).unwrap();

    let fields: Vec<&str> = fields.split(",").collect();
    for field in fields {
        let field = field.trim();
        let field_name = field.split(" ").nth(1).unwrap().trim();
        let mut field_type = field.split(" ").nth(0).unwrap().trim();
        if field_type == "Expr" {
            field_type = "Box<Expr>";
        }
        file.write_all(format!("        {}: {},\n", field_name, field_type).as_bytes()).unwrap();
    }


    file.write_all(b"    },\n").unwrap();
}

