use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::ops::Deref;

use crate::errors::*;
use crate::expr::*;
use crate::interpreter::*;
use crate::stmt::*;
use crate::tokens::*;

struct Resolver {
    interpreter: Interpreter,
    scopes: RefCell<Vec<RefCell<HashMap<String, bool>>>>,
}

impl StmtVisitor<()> for Resolver {
    fn visit_return_stmt(&self, _: &Rc<Stmt> ,stmt: &ReturnStmt) -> Result<(), Error> {
        if let Some(value) = &stmt.value {
            self.resolve_expr(value);
        }

        Ok(())
    }
    fn visit_function_stmt(&self, _: &Rc<Stmt>, stmt: &FunctionStmt) -> Result<(), Error> {
        self.declare(&stmt.name);
        self.define(&stmt.name);
        self.resolve_function(stmt);
        Ok(())
    }
    fn visit_while_stmt(&self, _: &Rc<Stmt>, stmt: &WhileStmt) -> Result<(), Error> {
        self.resolve_expr(&stmt.condition);
        self.resolve_stmt(&stmt.body);
        Ok(())
    }
    fn visit_if_stmt(&self, _: &Rc<Stmt>, stmt: &IfStmt) -> Result<(), Error> {
        self.resolve_expr(&stmt.condition);
        self.resolve_stmt(&stmt.then_branch);
        if let Some(else_branch) = &stmt.else_branch {
            self.resolve_stmt(else_branch);
        }
        Ok(())
    }

    fn visit_block_stmt(&self, _: &Rc<Stmt>, stmt: &BlockStmt) -> Result<(), Error> {
        self.begin_scope();
        self.resolve(&stmt.statements);
        self.end_scope();
        Ok(())
    }

    fn visit_expression_stmt(&self, _: &Rc<Stmt>, stmt: &ExpressionStmt) -> Result<(), Error> {
        self.resolve_expr(&stmt.expression);
        Ok(())
    }
    fn visit_print_stmt(&self, _: &Rc<Stmt>, stmt: &PrintStmt) -> Result<(), Error> {
        self.resolve_expr(&stmt.expression);
        Ok(())
    }

    fn visit_var_stmt(&self, _: &Rc<Stmt>, stmt: &VarStmt) -> Result<(), Error> {
        self.declare(&stmt.name);
        if let Some(init) = &stmt.initializer {
            self.resolve_expr(&init);
        }
        self.define(&stmt.name);
        Ok(())
    }
}

impl ExprVisitor<()> for Resolver {
    fn visit_call_expr(&self, _: &Rc<Expr>, expr: &CallExpr) -> Result<(), Error> {
        self.resolve_expr(&expr.callee);

        for arg in expr.arguments.iter() {
            self.resolve_expr(arg);
        }
        Ok(())
    }
    fn visit_logical_expr(&self, _: &Rc<Expr>, expr: &LogicalExpr) -> Result<(), Error> {
        self.resolve_expr(&expr.left);
        self.resolve_expr(&expr.right);
        Ok(())
    }
    fn visit_assign_expr(&self, wrapper: &Rc<Expr>, expr: &AssignExpr) -> Result<(), Error> {
        self.resolve_expr(&expr.value);
        self.resolve_local(wrapper, &expr.name);
        Ok(())
    }
    fn visit_literal_expr(&self, _: &Rc<Expr>, _expr: &LiteralExpr) -> Result<(), Error> {
        Ok(())
    }
    fn visit_grouping_expr(&self, _: &Rc<Expr>, expr: &GroupingExpr) -> Result<(), Error> {
        self.resolve_expr(&expr.expression);
        Ok(())
    }
    fn visit_binary_expr(&self, _: &Rc<Expr>, expr: &BinaryExpr) -> Result<(), Error> {
        self.resolve_expr(&expr.left);
        self.resolve_expr(&expr.right);

        Ok(())
    }
    fn visit_unary_expr(&self, _: &Rc<Expr>, expr: &UnaryExpr) -> Result<(), Error> {
        self.resolve_expr(&expr.right);
        Ok(())
    }
    fn visit_variable_expr(&self, wrapper: &Rc<Expr>, expr: &VariableExpr) -> Result<(), Error> {
        if !self.scopes.borrow().is_empty() && !self.scopes.borrow().last().unwrap().borrow().get(&expr.name.lexeme).unwrap(){
            return Err(Error::runtime_error(
                &expr.name,
                "Cannot read local variable in its own initializer.",
            ));
        }
        self.resolve_local(wrapper, &expr.name);
        Ok(())
    }
}

impl Resolver {
    fn resolve(&self, statements: &Rc<Vec<Rc<Stmt>>>){
        for statement in statements.deref() {
            self.resolve_stmt(statement);
        }
    }

    fn resolve_stmt(&self, stmt: &Rc<Stmt>){
        stmt.accept(stmt, self);
    }

    fn resolve_expr(&self, expr: &Rc<Expr>){
        expr.accept(expr, self);
    }

    fn begin_scope(&self) {
        self.scopes.borrow_mut().push(RefCell::new(HashMap::new()));
    }

    fn end_scope(&self) {
        self.scopes.borrow_mut().pop();
    }

    fn declare(&self, name: &Token) {
        if !self.scopes.borrow().is_empty() {
            self.scopes
                .borrow()
                .last()
                .unwrap()
                .borrow_mut()
                .insert(name.lexeme.clone(), false);
        }
    }

    fn define(&self, name: &Token) {
        if !self.scopes.borrow().is_empty() {
            self.scopes
                .borrow()
                .last()
                .unwrap()
                .borrow_mut()
                .insert(name.lexeme.clone(), true);
        }
    }

    fn resolve_local(&self, expr: &Rc<Expr>, name: &Token) {
        for (i, scope) in self.scopes.borrow().iter().enumerate().rev() {
            if scope.borrow().contains_key(&name.lexeme) {
                self.interpreter.resolve(expr.clone(), i);
                return;
            }
        }
    }

    fn resolve_function(&self, function: &FunctionStmt){
        self.begin_scope();
        for param in function.params.iter(){
            self.declare(param);
            self.define(param);
        }
        self.resolve(&function.body);
        self.end_scope();
    }
}