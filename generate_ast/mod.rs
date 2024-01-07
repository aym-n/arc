use std::fs::File;
use std::io::{self, Write};

#[derive(Debug)]
struct TreeType {
    base_class_name: String,
    class_name: String,
    fields: Vec<String>,
}

pub fn generate_ast(output_dir: &String) -> io::Result<()> {
    define_ast(
        output_dir,
        &"Expr".to_string(),
        &vec![
            "Assign   : Token name, Box<Expr> value".to_string(),
            "Binary   : Box<Expr> left, Token operator, Box<Expr> right".to_string(),
            "Call     : Box<Expr> callee, Token paren, Vec<Expr> arguments".to_string(),
            "Grouping : Box<Expr> expression".to_string(),
            "Literal  : Option<Object> value".to_string(),
            "Logical  : Box<Expr> left, Token operator, Box<Expr> right".to_string(),
            "Unary    : Token operator, Box<Expr> right".to_string(),
            "Variable : Token name".to_string(),

        ],
        &vec![
            "crate::tokens::*".to_string(),
            "crate::errors::*".to_string(),
        ],
    )?;

    define_ast(
        output_dir,
        &"Stmt".to_string(),
        &vec![
            "Block      : Vec<Stmt> statements".to_string(),
            "Expression : Expr expression".to_string(),
            "Function   : Token name, Rc<Vec<Token>> params, Rc<Vec<Stmt>> body".to_string(),
            "If         : Expr condition, Box<Stmt> then_branch, Option<Box<Stmt>> else_branch".to_string(),
            "Print      : Expr expression".to_string(),
            "Var        : Token name, Option<Expr> initializer".to_string(),
            "While      : Expr condition, Box<Stmt> body".to_string(),
            "Return     : Token keyword, Option<Expr> value".to_string(),
        ],
        &vec![
            "crate::expr::Expr".to_string(),
            "crate::errors::*".to_string(),
            "crate::tokens::*".to_string(),
            "std::rc::Rc".to_string(),
        ],
    )?;
    Ok(())
}

fn define_ast(
    output_dir: &String,
    base_name: &String,
    types: &[String],
    imports: &[String],
) -> io::Result<()> {
    let path = format!("{output_dir}/{}.rs", base_name.to_lowercase());
    let mut file = File::create(path)?;
    let mut tree_types = Vec::new();

    for import in imports {
        write!(file, "use {};\n", import)?;
    }

    for ttype in types {
        let (base_class_name, args) = ttype.split_once(":").unwrap();
        let class_name = format!("{}{}", base_class_name.trim(), base_name);
        let arg_split = args.split(",");
        let mut fields = Vec::new();
        for arg in arg_split {
            let (t2type, name) = arg.trim().split_once(" ").unwrap();
            fields.push(format!("{}: {}", name, t2type));
        }
        tree_types.push(TreeType {
            base_class_name: base_class_name.trim().to_string(),
            class_name,
            fields,
        });
    }

    write!(file, "\npub enum {base_name} {{\n")?;
    for t in &tree_types {
        write!(file, "    {}({}),\n", t.base_class_name, t.class_name)?;
    }
    write!(file, "}}\n\n")?;

    write!(file, "impl {} {{\n", base_name)?;
    write!(file, "    pub fn accept<T>(&self, {}_visitor: &dyn {base_name}Visitor<T>) -> Result<T , Error> {{\n", base_name.to_lowercase())?;
    write!(file, "        match self {{\n")?;
    for t in &tree_types {
        write!(
            file,
            "            {}::{}(v) => v.accept({}_visitor),\n",
            base_name,
            t.base_class_name,
            base_name.to_lowercase()
        )?;
    }
    write!(file, "        }}\n")?;
    write!(file, "    }}\n")?;
    write!(file, "}}\n\n")?;

    for t in &tree_types {
        write!(file, "pub struct {} {{\n", t.class_name)?;
        for f in &t.fields {
            write!(file, "    pub {},\n", f)?;
        }
        write!(file, "}}\n\n")?;
    }

    write!(file, "pub trait {}Visitor<T> {{\n", base_name)?;
    for t in &tree_types {
        write!(
            file,
            "    fn visit_{}_{}(&self, expr: &{}) ->  Result<T , Error>;\n",
            t.base_class_name.to_lowercase(),
            base_name.to_lowercase(),
            t.class_name
        )?;
    }
    write!(file, "}}\n\n")?;

    for t in &tree_types {
        write!(file, "impl {} {{\n", t.class_name)?;
        write!(
            file,
            "    pub fn accept<T>(&self, visitor: &dyn {}Visitor<T>) ->  Result<T , Error> {{\n",
            base_name
        )?;
        write!(
            file,
            "        visitor.visit_{}_{}(self)\n",
            t.base_class_name.to_lowercase(),
            base_name.to_lowercase()
        )?;
        write!(file, "    }}\n")?;
        write!(file, "}}\n\n")?;
    }
    Ok(())
}
