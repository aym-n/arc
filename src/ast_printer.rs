use crate::expr::*;

pub struct AstPrinter;

impl AstPrinter {
    pub fn print(&self, expr: &Expr) -> String {
        expr.accept(self)
    }

    pub fn parenthesize(&self, name: &str, exprs: &[&Expr]) -> String {
        let mut result = String::new();
        result.push_str("(");
        result.push_str(name);
        for expr in exprs {
            result.push_str(" ");
            result.push_str(&self.print(expr));
        }
        result.push_str(")");
        result      
    }

}

impl ExprVisitor<String> for AstPrinter {
    fn visit_binary_expr(&self, expr: &BinaryExpr) -> String {
        format!(
            "({} {} {})",
            expr.operator.lexeme.clone(),
            self.print(&expr.left),
            self.print(&expr.right)
        )
    }

    fn visit_grouping_expr(&self, expr: &GroupingExpr) -> String {
        format!("(group {})", self.print(&expr.expression))
    }

    fn visit_literal_expr(&self, expr: &LiteralExpr) -> String {
        match &expr.value {
            Some(v) => format!("{}", v),
            None => "nil".to_string(),
        }
    }

    fn visit_unary_expr(&self, expr: &UnaryExpr) -> String {
        format!(
            "({} {})",
            expr.operator.lexeme.clone(),
            self.print(&expr.right)
        )
    }
}