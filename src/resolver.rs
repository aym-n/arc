use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::ops::Deref;

use crate::errors::*;
use crate::expr::*;
use crate::interpreter::*;
use crate::stmt::*;
use crate::tokens::*;

pub struct Resolver<'a>{
    interpreter: &'a Interpreter,
    scopes: RefCell<Vec<RefCell<HashMap<String, bool>>>>,
    had_error: RefCell<bool>,
    current_function: RefCell<FunctionType>,
}

#[derive(PartialEq)]
enum FunctionType {
    None,
    Function,
    Method,
}

impl<'a> StmtVisitor<()> for Resolver<'a>{
    fn visit_class_stmt(&self , _: Rc<Stmt>, stmt: &ClassStmt) -> Result<(), Error> {
        self.declare(&stmt.name);
        self.define(&stmt.name);

        self.begin_scope();
        self.scopes.borrow().last().unwrap().borrow_mut().insert("this".to_string(), true);

        for method in stmt.methods.deref() {
            let declaration = FunctionType::Method;
            if let Stmt::Function(method) = method.deref() {
                self.resolve_function(method, declaration);
            }else{
                return Err(Error::runtime_error(&stmt.name, "Class method did not resolve to a function."));
            }
        }

        self.end_scope();

        Ok(())
    }

    fn visit_return_stmt(&self, _: Rc<Stmt> ,stmt: &ReturnStmt) -> Result<(), Error> {
        if *self.current_function.borrow() == FunctionType::None {
            return Err(Error::runtime_error(&stmt.keyword, "Cannot return from top-level code."));
        }
        
        if let Some(value) = stmt.value.clone() {
            self.resolve_expr(value);
        }

        Ok(())
    }
    fn visit_function_stmt(&self, _: Rc<Stmt>, stmt: &FunctionStmt) -> Result<(), Error> {
        self.declare(&stmt.name);
        self.define(&stmt.name);
        self.resolve_function(stmt, FunctionType::Function);
        Ok(())
    }
    fn visit_while_stmt(&self, _: Rc<Stmt>, stmt: &WhileStmt) -> Result<(), Error> {
        self.resolve_expr(stmt.condition.clone());
        self.resolve_stmt(stmt.body.clone());
        Ok(())
    }
    fn visit_if_stmt(&self, _: Rc<Stmt>, stmt: &IfStmt) -> Result<(), Error> {
        self.resolve_expr(stmt.condition.clone());
        self.resolve_stmt(stmt.then_branch.clone());
        if let Some(else_branch) = stmt.else_branch.clone() {
            self.resolve_stmt(else_branch);
        }
        Ok(())
    }

    fn visit_block_stmt(&self, _: Rc<Stmt>, stmt: &BlockStmt) -> Result<(), Error> {
        self.begin_scope();
        self.resolve(&stmt.statements);
        self.end_scope();
        Ok(())
    }

    fn visit_expression_stmt(&self, _: Rc<Stmt>, stmt: &ExpressionStmt) -> Result<(), Error> {
        self.resolve_expr(stmt.expression.clone());
        Ok(())
    }
    fn visit_print_stmt(&self, _: Rc<Stmt>, stmt: &PrintStmt) -> Result<(), Error> {
        self.resolve_expr(stmt.expression.clone());
        Ok(())
    }

    fn visit_var_stmt(&self, _: Rc<Stmt>, stmt: &VarStmt) -> Result<(), Error> {
        self.declare(&stmt.name);
        if let Some(init) = stmt.initializer.clone() {
            self.resolve_expr(init);
        }
        self.define(&stmt.name);
        Ok(())
    }
}

impl<'a> ExprVisitor<()> for Resolver<'a>{
    fn visit_this_expr(&self, wrapper: Rc<Expr>, expr: &ThisExpr) -> Result<(), Error> {
        self.resolve_local(wrapper, &expr.keyword);
        Ok(())
    }
    
    fn visit_set_expr(&self, _: Rc<Expr>, expr: &SetExpr) -> Result<(), Error> {
        self.resolve_expr(expr.value.clone());
        self.resolve_expr(expr.object.clone());
        Ok(())
    }

    fn visit_get_expr(&self, _: Rc<Expr>, expr: &GetExpr) -> Result<(), Error> {
        self.resolve_expr(expr.object.clone());
        Ok(())
    }

    fn visit_call_expr(&self, _: Rc<Expr>, expr: &CallExpr) -> Result<(), Error> {
        self.resolve_expr(expr.callee.clone());

        for arg in expr.arguments.iter() {
            self.resolve_expr(arg.clone());
        }
        Ok(())
    }
    fn visit_logical_expr(&self, _: Rc<Expr>, expr: &LogicalExpr) -> Result<(), Error> {
        self.resolve_expr(expr.left.clone());
        self.resolve_expr(expr.right.clone());
        Ok(())
    }
    fn visit_assign_expr(&self, wrapper: Rc<Expr>, expr: &AssignExpr) -> Result<(), Error> {
        self.resolve_expr(expr.value.clone());
        self.resolve_local(wrapper, &expr.name);
        Ok(())
    }
    fn visit_literal_expr(&self, _: Rc<Expr>, _expr: &LiteralExpr) -> Result<(), Error> {
        Ok(())
    }
    fn visit_grouping_expr(&self, _: Rc<Expr>, expr: &GroupingExpr) -> Result<(), Error> {
        self.resolve_expr(expr.expression.clone());
        Ok(())
    }
    fn visit_binary_expr(&self, _: Rc<Expr>, expr: &BinaryExpr) -> Result<(), Error> {
        self.resolve_expr(expr.left.clone());
        self.resolve_expr(expr.right.clone());

        Ok(())
    }
    fn visit_unary_expr(&self, _: Rc<Expr>, expr: &UnaryExpr) -> Result<(), Error> {
        self.resolve_expr(expr.right.clone());
        Ok(())
    }
    fn visit_variable_expr(&self, wrapper: Rc<Expr>, expr: &VariableExpr) -> Result<(), Error> {
        if !self.scopes.borrow().is_empty() && self.scopes.borrow().last().unwrap().borrow().get(&expr.name.lexeme) == Some(&false){
            return Err(Error::runtime_error(
                &expr.name,
                "Cannot read local variable in its own initializer.",
            ));
        }
        self.resolve_local(wrapper, &expr.name);
        Ok(())
    }
}

impl<'a> Resolver<'a> {
    pub fn new(interpreter: &'a Interpreter) -> Resolver {
        Resolver {
            interpreter,
            scopes: RefCell::new(Vec::new()),
            had_error: RefCell::new(false),
            current_function: RefCell::new(FunctionType::None),
        }
    }
    pub fn resolve(&self, statement: &Rc<Vec<Rc<Stmt>>>){
        for stmt in statement.deref(){
            self.resolve_stmt(stmt.clone());
        }
    }

    pub fn success(&self) -> bool{
        !self.had_error.borrow().clone()
    }

    fn resolve_stmt(&self, stmt: Rc<Stmt>){
        stmt.accept(stmt.clone(), self);
    }

    fn resolve_expr(&self, expr: Rc<Expr>){
        expr.accept(expr.clone(), self);
    }

    fn begin_scope(&self) {
        self.scopes.borrow_mut().push(RefCell::new(HashMap::new()));
    }

    fn end_scope(&self) {
        self.scopes.borrow_mut().pop();
    }

    fn declare(&self, name: &Token) {
        if let Some(scope) = self.scopes.borrow().last() {
            if scope.borrow().contains_key(&name.lexeme.clone()) {
                self.error(name, "Variable with this name already declared in this scope.");
                self.had_error.replace(true);
                return;
            }
            scope.borrow_mut().insert(name.lexeme.clone(), false);
        }
    }

    fn define(&self, name: &Token) {
        if let Some(scope) = self.scopes.borrow().last() {
            scope.borrow_mut().insert(name.lexeme.clone(), true);
        }
    }

    fn resolve_local(&self, expr: Rc<Expr>, name: &Token) {
        for (i, scope) in self.scopes.borrow().iter().rev().enumerate() {
            if scope.borrow().contains_key(&name.lexeme) {
                self.interpreter.resolve(expr.clone(), i);
                return;
            }
        }
    }

    fn resolve_function(&self, function: &FunctionStmt, func_type: FunctionType){

        let enclosing_function = self.current_function.replace(func_type);

        self.begin_scope();
        for param in function.params.iter(){
            self.declare(param);
            self.define(param);
        }
        self.resolve(&function.body);
        self.end_scope();

        self.current_function.replace(enclosing_function);
    }

    fn error(&self, token: &Token, message: &str) {
        self.had_error.replace(true);
        Error::runtime_error(token, message); 
    }
}