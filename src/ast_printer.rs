use crate::expr::*;
use crate::tokens::*;

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
            expr.operator.literal.clone().unwrap(),
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
            expr.operator.literal.clone().unwrap_or_default(),
            self.print(&expr.right)
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ast_printer() {
        let expr = Expr::Binary(BinaryExpr {
            left: Box::new(Expr::Unary(UnaryExpr {
                operator: Token::new_with_literal(
                    TokenKind::Minus,
                    "-".to_string(),
                    1,
                ),
                right: Box::new(Expr::Literal(LiteralExpr {
                    value: Some(123.0),
                })),
            })),
            operator: Token::new_with_literal(
                TokenKind::Asterisk,
                "*".to_string(),
                1,
            ),
            right: Box::new(Expr::Grouping(GroupingExpr {
                expression: Box::new(Expr::Literal(LiteralExpr {
                    value: Some(45.67),
                })),
            })),
        });

        let ast_printer = AstPrinter;
        let result = ast_printer.print(&expr);
        assert_eq!(result, "(* (- 123) (group 45.67))");
    }
}