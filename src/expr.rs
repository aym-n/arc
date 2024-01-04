use crate::tokens::*;
use crate::errors::*;

pub enum Expr {
    Assign(AssignExpr),
    Binary(BinaryExpr),
    Grouping(GroupingExpr),
    Literal(LiteralExpr),
    Logical(LogicalExpr),
    Unary(UnaryExpr),
    Variable(VariableExpr),
}

impl Expr {
    pub fn accept<T>(&self, expr_visitor: &dyn ExprVisitor<T>) -> Result<T , Error> {
        match self {
            Expr::Assign(v) => v.accept(expr_visitor),
            Expr::Binary(v) => v.accept(expr_visitor),
            Expr::Grouping(v) => v.accept(expr_visitor),
            Expr::Literal(v) => v.accept(expr_visitor),
            Expr::Logical(v) => v.accept(expr_visitor),
            Expr::Unary(v) => v.accept(expr_visitor),
            Expr::Variable(v) => v.accept(expr_visitor),
        }
    }
}

pub struct AssignExpr {
    pub name: Token,
    pub value: Box<Expr>,
}

pub struct BinaryExpr {
    pub left: Box<Expr>,
    pub operator: Token,
    pub right: Box<Expr>,
}

pub struct GroupingExpr {
    pub expression: Box<Expr>,
}

pub struct LiteralExpr {
    pub value: Option<Object>,
}

pub struct LogicalExpr {
    pub left: Box<Expr>,
    pub operator: Token,
    pub right: Box<Expr>,
}

pub struct UnaryExpr {
    pub operator: Token,
    pub right: Box<Expr>,
}

pub struct VariableExpr {
    pub name: Token,
}

pub trait ExprVisitor<T> {
    fn visit_assign_expr(&self, expr: &AssignExpr) ->  Result<T , Error>;
    fn visit_binary_expr(&self, expr: &BinaryExpr) ->  Result<T , Error>;
    fn visit_grouping_expr(&self, expr: &GroupingExpr) ->  Result<T , Error>;
    fn visit_literal_expr(&self, expr: &LiteralExpr) ->  Result<T , Error>;
    fn visit_logical_expr(&self, expr: &LogicalExpr) ->  Result<T , Error>;
    fn visit_unary_expr(&self, expr: &UnaryExpr) ->  Result<T , Error>;
    fn visit_variable_expr(&self, expr: &VariableExpr) ->  Result<T , Error>;
}

impl AssignExpr {
    pub fn accept<T>(&self, visitor: &dyn ExprVisitor<T>) ->  Result<T , Error> {
        visitor.visit_assign_expr(self)
    }
}

impl BinaryExpr {
    pub fn accept<T>(&self, visitor: &dyn ExprVisitor<T>) ->  Result<T , Error> {
        visitor.visit_binary_expr(self)
    }
}

impl GroupingExpr {
    pub fn accept<T>(&self, visitor: &dyn ExprVisitor<T>) ->  Result<T , Error> {
        visitor.visit_grouping_expr(self)
    }
}

impl LiteralExpr {
    pub fn accept<T>(&self, visitor: &dyn ExprVisitor<T>) ->  Result<T , Error> {
        visitor.visit_literal_expr(self)
    }
}

impl LogicalExpr {
    pub fn accept<T>(&self, visitor: &dyn ExprVisitor<T>) ->  Result<T , Error> {
        visitor.visit_logical_expr(self)
    }
}

impl UnaryExpr {
    pub fn accept<T>(&self, visitor: &dyn ExprVisitor<T>) ->  Result<T , Error> {
        visitor.visit_unary_expr(self)
    }
}

impl VariableExpr {
    pub fn accept<T>(&self, visitor: &dyn ExprVisitor<T>) ->  Result<T , Error> {
        visitor.visit_variable_expr(self)
    }
}

