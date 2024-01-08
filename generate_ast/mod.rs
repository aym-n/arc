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
            "Assign   : Token name, Rc<Expr> value".to_string(),
            "Binary   : Rc<Expr> left, Token operator, Rc<Expr> right".to_string(),
            "Call     : Rc<Expr> callee, Token paren, Vec<Rc<Expr>> arguments".to_string(),
            "Grouping : Rc<Expr> expression".to_string(),
            "Literal  : Option<Object> value".to_string(),
            "Logical  : Rc<Expr> left, Token operator, Rc<Expr> right".to_string(),
            "Unary    : Token operator, Rc<Expr> right".to_string(),
            "Variable : Token name".to_string(),

        ],
        &vec![
            "crate::tokens::*".to_string(),
            "crate::errors::*".to_string(),
            "std::rc::Rc".to_string(),
        ],
    )?;

    define_ast(
        output_dir,
        &"Stmt".to_string(),
        &vec![
            "Block      : Rc<Vec<Rc<Stmt>>> statements".to_string(),
            "Expression : Rc<Expr> expression".to_string(),
            "Function   : Token name, Rc<Vec<Token>> params, Rc<Vec<Rc<Stmt>>> body".to_string(),
            "If         : Rc<Expr> condition, Rc<Stmt> then_branch, Option<Rc<Stmt>> else_branch".to_string(),
            "Print      : Rc<Expr> expression".to_string(),
            "Var        : Token name, Option<Rc<Expr>> initializer".to_string(),
            "While      : Rc<Expr> condition, Rc<Stmt> body".to_string(),
            "Return     : Token keyword, Option<Rc<Expr>> value".to_string(),
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
        write!(file, "    {}(Rc<{}>),\n", t.base_class_name, t.class_name)?;
    }
    write!(file, "}}\n\n")?;

    writeln!(file, "impl PartialEq for {} {{", base_name)?;
    writeln!(file, "    fn eq(&self, other: &Self) -> bool {{")?;
    writeln!(file, "        match (self, other) {{")?;
    for t in &tree_types {
        writeln!(
            file,
            "            ({0}::{1}(a), {0}::{1}(b)) => Rc::ptr_eq(a, b),",
            base_name, t.base_class_name
        )?;
    }
    writeln!(file, "            _ => false,")?;
    writeln!(file, "        }}")?;
    writeln!(file, "    }}")?;
    writeln!(file, "}}\n\nimpl Eq for {}{{}}\n", base_name)?;

    writeln!(file, "use std::hash::{{Hash, Hasher}};")?;
    writeln!(file, "impl Hash for {} {{", base_name)?;
    writeln!(file, "    fn hash<H>(&self, hasher: &mut H)")?;
    writeln!(file, "    where H: Hasher,")?;
    writeln!(file, "    {{ match self {{ ")?;
    for t in &tree_types {
        writeln!(
            file,
            "        {}::{}(a) => {{ hasher.write_usize(Rc::as_ptr(a) as usize); }}",
            base_name, t.base_class_name
        )?;
    }
    writeln!(file, "        }}\n    }}\n}}\n")?;

    write!(file, "impl {} {{\n", base_name)?;
    write!(file, "    pub fn accept<T>(&self, wrapper: &Rc<{}>, {}_visitor: &dyn {base_name}Visitor<T>) -> Result<T , Error> {{\n", base_name, base_name.to_lowercase())?;
    write!(file, "        match self {{\n")?;
    for t in &tree_types {
        write!(
            file,
            "            {0}::{1}(v) => {3}_visitor.visit_{2}_{3}(wrapper, &v),\n",
            base_name,
            t.base_class_name,
            t.base_class_name.to_lowercase(),
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
            "    fn visit_{0}_{1}(&self, wrapper: &Rc<{3}>, {1}: &{2}) ->  Result<T , Error>;\n",
            t.base_class_name.to_lowercase(),
            base_name.to_lowercase(),
            t.class_name,
            base_name,
        )?;
    }
    write!(file, "}}\n\n")?;

    // for t in &tree_types {
    //     write!(file, "impl {} {{\n", t.class_name)?;
    //     write!(
    //         file,
    //         "    pub fn accept<T>(&self, visitor: &dyn {}Visitor<T>) ->  Result<T , Error> {{\n",
    //         base_name
    //     )?;
    //     write!(
    //         file,
    //         "        visitor.visit_{}_{}(self)\n",
    //         t.base_class_name.to_lowercase(),
    //         base_name.to_lowercase()
    //     )?;
    //     write!(file, "    }}\n")?;
    //     write!(file, "}}\n\n")?;
    // }
    Ok(())
}
