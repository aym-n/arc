use crate::errors::*;
use crate::expr::*;
use crate::tokens::*;
use std::rc::Rc;

pub enum Stmt {
    Block(Rc<BlockStmt>),
    Class(Rc<ClassStmt>),
    Expression(Rc<ExpressionStmt>),
    Function(Rc<FunctionStmt>),
    If(Rc<IfStmt>),
    Print(Rc<PrintStmt>),
    Return(Rc<ReturnStmt>),
    Var(Rc<VarStmt>),
    While(Rc<WhileStmt>),
}

impl PartialEq for Stmt {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Stmt::Block(a), Stmt::Block(b)) => Rc::ptr_eq(a, b),
            (Stmt::Class(a), Stmt::Class(b)) => Rc::ptr_eq(a, b),
            (Stmt::Expression(a), Stmt::Expression(b)) => Rc::ptr_eq(a, b),
            (Stmt::Function(a), Stmt::Function(b)) => Rc::ptr_eq(a, b),
            (Stmt::If(a), Stmt::If(b)) => Rc::ptr_eq(a, b),
            (Stmt::Print(a), Stmt::Print(b)) => Rc::ptr_eq(a, b),
            (Stmt::Return(a), Stmt::Return(b)) => Rc::ptr_eq(a, b),
            (Stmt::Var(a), Stmt::Var(b)) => Rc::ptr_eq(a, b),
            (Stmt::While(a), Stmt::While(b)) => Rc::ptr_eq(a, b),
            _ => false,
        }
    }
}

impl Eq for Stmt{}

use std::hash::{Hash, Hasher};
impl Hash for Stmt {
    fn hash<H>(&self, hasher: &mut H)
    where H: Hasher,
    { match self { 
        Stmt::Block(a) => { hasher.write_usize(Rc::as_ptr(a) as usize); }
        Stmt::Class(a) => { hasher.write_usize(Rc::as_ptr(a) as usize); }
        Stmt::Expression(a) => { hasher.write_usize(Rc::as_ptr(a) as usize); }
        Stmt::Function(a) => { hasher.write_usize(Rc::as_ptr(a) as usize); }
        Stmt::If(a) => { hasher.write_usize(Rc::as_ptr(a) as usize); }
        Stmt::Print(a) => { hasher.write_usize(Rc::as_ptr(a) as usize); }
        Stmt::Return(a) => { hasher.write_usize(Rc::as_ptr(a) as usize); }
        Stmt::Var(a) => { hasher.write_usize(Rc::as_ptr(a) as usize); }
        Stmt::While(a) => { hasher.write_usize(Rc::as_ptr(a) as usize); }
        }
    }
}

impl Stmt {
    pub fn accept<T>(&self, wrapper: Rc<Stmt>, stmt_visitor: &dyn StmtVisitor<T>) -> Result<T, Error> {
        match self {
            Stmt::Block(v) => stmt_visitor.visit_block_stmt(wrapper, v),
            Stmt::Class(v) => stmt_visitor.visit_class_stmt(wrapper, v),
            Stmt::Expression(v) => stmt_visitor.visit_expression_stmt(wrapper, v),
            Stmt::Function(v) => stmt_visitor.visit_function_stmt(wrapper, v),
            Stmt::If(v) => stmt_visitor.visit_if_stmt(wrapper, v),
            Stmt::Print(v) => stmt_visitor.visit_print_stmt(wrapper, v),
            Stmt::Return(v) => stmt_visitor.visit_return_stmt(wrapper, v),
            Stmt::Var(v) => stmt_visitor.visit_var_stmt(wrapper, v),
            Stmt::While(v) => stmt_visitor.visit_while_stmt(wrapper, v),
        }
    }
}

pub struct BlockStmt {
    pub statements: Rc<Vec<Rc<Stmt>>>,
}

pub struct ClassStmt {
    pub name: Token,
    pub methods: Rc<Vec<Rc<Stmt>>>,
}

pub struct ExpressionStmt {
    pub expression: Rc<Expr>,
}

pub struct FunctionStmt {
    pub name: Token,
    pub params: Rc<Vec<Token>>,
    pub body: Rc<Vec<Rc<Stmt>>>,
}

pub struct IfStmt {
    pub condition: Rc<Expr>,
    pub then_branch: Rc<Stmt>,
    pub else_branch: Option<Rc<Stmt>>,
}

pub struct PrintStmt {
    pub expression: Rc<Expr>,
}

pub struct ReturnStmt {
    pub keyword: Token,
    pub value: Option<Rc<Expr>>,
}

pub struct VarStmt {
    pub name: Token,
    pub initializer: Option<Rc<Expr>>,
}

pub struct WhileStmt {
    pub condition: Rc<Expr>,
    pub body: Rc<Stmt>,
}

pub trait StmtVisitor<T> {
    fn visit_block_stmt(&self, wrapper: Rc<Stmt>, stmt: &BlockStmt) -> Result<T, Error>;
    fn visit_class_stmt(&self, wrapper: Rc<Stmt>, stmt: &ClassStmt) -> Result<T, Error>;
    fn visit_expression_stmt(&self, wrapper: Rc<Stmt>, stmt: &ExpressionStmt) -> Result<T, Error>;
    fn visit_function_stmt(&self, wrapper: Rc<Stmt>, stmt: &FunctionStmt) -> Result<T, Error>;
    fn visit_if_stmt(&self, wrapper: Rc<Stmt>, stmt: &IfStmt) -> Result<T, Error>;
    fn visit_print_stmt(&self, wrapper: Rc<Stmt>, stmt: &PrintStmt) -> Result<T, Error>;
    fn visit_return_stmt(&self, wrapper: Rc<Stmt>, stmt: &ReturnStmt) -> Result<T, Error>;
    fn visit_var_stmt(&self, wrapper: Rc<Stmt>, stmt: &VarStmt) -> Result<T, Error>;
    fn visit_while_stmt(&self, wrapper: Rc<Stmt>, stmt: &WhileStmt) -> Result<T, Error>;
}

