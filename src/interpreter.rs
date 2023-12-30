use crate::expr::*;
use crate::tokens::*;

pub struct Interpreter {}

impl ExprVisitor<Object> for Interpreter{
    fn visit_literal_expr(&self, expr: &LiteralExpr) -> Object{
        expr.value.clone().unwrap()
    }

    fn visit_grouping_expr(&self, expr: &GroupingExpr) -> Object{
        self.evaluate(&expr.expression)
    }       

    fn visit_unary_expr(&self, expr: &UnaryExpr) -> Object{
        let right = self.evaluate(&expr.right);

        match expr.operator.kind {
            TokenKind::Minus => {
                if let Object::Num(x) = right {
                    return Object::Num(-x);
                }
                Object::Nil
            }
            TokenKind::Bang => {
                return Object::Bool(!self.is_truthy(right));
            }
            _ => {Object::Nil}
        } 
    }

    fn visit_binary_expr(&self, expr: &BinaryExpr) -> Object{
        let left = self.evaluate(&expr.left);
        let right = self.evaluate(&expr.right);

        let result = match expr.operator.kind{
            TokenKind::Minus => left - right,
            TokenKind::Slash => left / right,
            TokenKind::Asterisk => left * right,
            TokenKind::Plus => left + right,
            TokenKind::GreaterThan => Object::Bool(left > right),
            TokenKind::GreaterThanEqual => Object::Bool(left >= right),
            TokenKind::LessThan => Object::Bool(left < right),
            TokenKind::LessThanEqual => Object::Bool(left <= right),
            TokenKind::NotEqual => Object::Bool(left != right),
            TokenKind::EqualEqual => Object::Bool(left == right),
            _ => {Object::Nil}
        };

        if let Object::ArithmeticError = result {
            panic!("Arithmetic Error");
        }

        result
    }
} 

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter{}
    }

    pub fn interpret(&self, expr: &Expr) -> Object {
        self.evaluate(expr)
    }

    fn evaluate(&self, expr: &Expr) -> Object {
        expr.accept(self)
    }

    fn is_truthy(&self, object: Object) -> bool {
        match object {
            Object::Nil => false,
            Object::Bool(x) => x,
            _ => true,
        }
    }
}

#[cfg(test)]    
mod tests {
    use super::*;

    #[test]
    fn test_unary_minus() {
        let expr = Expr::Unary(UnaryExpr {
            operator: Token::new(TokenKind::Minus, "-".to_string(), None, 1),
            right: Box::new(Expr::Literal(LiteralExpr {
                value: Some(Object::Num(123.0)),
            })),
        });

        let interpreter = Interpreter::new();
        let result = interpreter.interpret(&expr);

        assert_eq!(result, Object::Num(-123.0));
    }

    #[test]
    fn test_subtraction(){
        let expr = Expr::Binary(BinaryExpr {
            left: Box::new(Expr::Literal(LiteralExpr {
                value: Some(Object::Num(125.0)),
            })),
            operator: Token::new(TokenKind::Minus, "-".to_string(), None, 1),
            right: Box::new(Expr::Literal(LiteralExpr {
                value: Some(Object::Num(123.0)),
            })),
        });

        let interpreter = Interpreter::new();
        let result = interpreter.interpret(&expr);

        assert_eq!(result, Object::Num(2.0));
    }

    #[test]
    fn test_unary_bang() {
        let expr = Expr::Unary(UnaryExpr {
            operator: Token::new(TokenKind::Bang, "!".to_string(), None, 1),
            right: Box::new(Expr::Literal(LiteralExpr {
                value: Some(Object::Bool(false)),
            })),
        });

        let interpreter = Interpreter::new();
        let result = interpreter.interpret(&expr);

        assert_eq!(result, Object::Bool(true));
    }

    #[test]
    fn test_multiplication() {
        let expr = Expr::Binary(BinaryExpr {
            left: Box::new(Expr::Literal(LiteralExpr {
                value: Some(Object::Num(15.0)),
            })),
            operator: Token::new(TokenKind::Asterisk, "*".to_string(), None, 1),
            right: Box::new(Expr::Literal(LiteralExpr {
                value: Some(Object::Num(10.0)),
            })),
        });

        let interpreter = Interpreter::new();
        let result = interpreter.interpret(&expr);

        assert_eq!(result, Object::Num(150.0));
    }

    #[test]
    fn test_division() {
        let expr = Expr::Binary(BinaryExpr {
            left: Box::new(Expr::Literal(LiteralExpr {
                value: Some(Object::Num(15.0)),
            })),
            operator: Token::new(TokenKind::Slash, "/".to_string(), None, 1),
            right: Box::new(Expr::Literal(LiteralExpr {
                value: Some(Object::Num(10.0)),
            })),
        });

        let interpreter = Interpreter::new();
        let result = interpreter.interpret(&expr);

        assert_eq!(result, Object::Num(1.5));
    }

    #[test]
    fn test_addition() {
        let expr = Expr::Binary(BinaryExpr {
            left: Box::new(Expr::Literal(LiteralExpr {
                value: Some(Object::Num(15.0)),
            })),
            operator: Token::new(TokenKind::Plus, "+".to_string(), None, 1),
            right: Box::new(Expr::Literal(LiteralExpr {
                value: Some(Object::Num(10.0)),
            })),
        });

        let interpreter = Interpreter::new();
        let result = interpreter.interpret(&expr);

        assert_eq!(result, Object::Num(25.0));
    }

    #[test]
    fn test_concatenation() {
        let expr = Expr::Binary(BinaryExpr {
            left: Box::new(Expr::Literal(LiteralExpr {
                value: Some(Object::Str("Hello".to_string())),
            })),
            operator: Token::new(TokenKind::Plus, "+".to_string(), None, 1),
            right: Box::new(Expr::Literal(LiteralExpr {
                value: Some(Object::Str("World".to_string())),
            })),
        });

        let interpreter = Interpreter::new();
        let result = interpreter.interpret(&expr);

        assert_eq!(result, Object::Str("HelloWorld".to_string()));
    }

    #[test]
    fn test_greater_than() {
        // True case
        let expr = Expr::Binary(BinaryExpr {
            left: Box::new(Expr::Literal(LiteralExpr {
                value: Some(Object::Num(15.0)),
            })),
            operator: Token::new(TokenKind::GreaterThan, ">".to_string(), None, 1),
            right: Box::new(Expr::Literal(LiteralExpr {
                value: Some(Object::Num(10.0)),
            })),
        });

        let interpreter = Interpreter::new();
        let result = interpreter.interpret(&expr);

        assert_eq!(result, Object::Bool(true));

        // False case
        let expr = Expr::Binary(BinaryExpr {
            left: Box::new(Expr::Literal(LiteralExpr {
                value: Some(Object::Num(10.0)),
            })),
            operator: Token::new(TokenKind::GreaterThan, ">".to_string(), None, 1),
            right: Box::new(Expr::Literal(LiteralExpr {
                value: Some(Object::Num(15.0)),
            })),
        });

        let interpreter = Interpreter::new();
        let result = interpreter.interpret(&expr);

        assert_eq!(result, Object::Bool(false));
    }

    #[test]
    fn test_greater_than_equal() {
        //True case
        let expr = Expr::Binary(BinaryExpr {
            left: Box::new(Expr::Literal(LiteralExpr {
                value: Some(Object::Num(15.0)),
            })),
            operator: Token::new(TokenKind::GreaterThanEqual, ">=".to_string(), None, 1),
            right: Box::new(Expr::Literal(LiteralExpr {
                value: Some(Object::Num(10.0)),
            })),
        });

        let interpreter = Interpreter::new();
        let result = interpreter.interpret(&expr);

        assert_eq!(result, Object::Bool(true));

        //False case
        let expr = Expr::Binary(BinaryExpr {
            left: Box::new(Expr::Literal(LiteralExpr {
                value: Some(Object::Num(10.0)),
            })),
            operator: Token::new(TokenKind::GreaterThanEqual, ">=".to_string(), None, 1),
            right: Box::new(Expr::Literal(LiteralExpr {
                value: Some(Object::Num(15.0)),
            })),
        });

        let interpreter = Interpreter::new();
        let result = interpreter.interpret(&expr);

        assert_eq!(result, Object::Bool(false));
    }

    #[test]
    fn test_less_than() {
        //False case
        let expr = Expr::Binary(BinaryExpr {
            left: Box::new(Expr::Literal(LiteralExpr {
                value: Some(Object::Num(15.0)),
            })),
            operator: Token::new(TokenKind::LessThan, "<".to_string(), None, 1),
            right: Box::new(Expr::Literal(LiteralExpr {
                value: Some(Object::Num(10.0)),
            })),
        });

        let interpreter = Interpreter::new();
        let result = interpreter.interpret(&expr);

        assert_eq!(result, Object::Bool(false));

        //True case
        let expr = Expr::Binary(BinaryExpr {
            left: Box::new(Expr::Literal(LiteralExpr {
                value: Some(Object::Num(10.0)),
            })),
            operator: Token::new(TokenKind::LessThan, "<".to_string(), None, 1),
            right: Box::new(Expr::Literal(LiteralExpr {
                value: Some(Object::Num(15.0)),
            })),
        });

        let interpreter = Interpreter::new();
        let result = interpreter.interpret(&expr);

        assert_eq!(result, Object::Bool(true));
    }

    #[test]
    fn test_less_than_equal() {
        //True case
        let expr = Expr::Binary(BinaryExpr {
            left: Box::new(Expr::Literal(LiteralExpr {
                value: Some(Object::Num(10.0)),
            })),
            operator: Token::new(TokenKind::LessThanEqual, "<=".to_string(), None, 1),
            right: Box::new(Expr::Literal(LiteralExpr {
                value: Some(Object::Num(15.0)),
            })),
        });

        let interpreter = Interpreter::new();
        let result = interpreter.interpret(&expr);

        assert_eq!(result, Object::Bool(true));

        //False case
        let expr = Expr::Binary(BinaryExpr {
            left: Box::new(Expr::Literal(LiteralExpr {
                value: Some(Object::Num(15.0)),
            })),
            operator: Token::new(TokenKind::LessThanEqual, "<=".to_string(), None, 1),
            right: Box::new(Expr::Literal(LiteralExpr {
                value: Some(Object::Num(10.0)),
            })),
        });

        let interpreter = Interpreter::new();
        let result = interpreter.interpret(&expr);

        assert_eq!(result, Object::Bool(false));
    }

    #[test]
    fn test_not_equal() {
        // True case
        let expr = Expr::Binary(BinaryExpr {
            left: Box::new(Expr::Literal(LiteralExpr {
                value: Some(Object::Bool(true)),
            })),
            operator: Token::new(TokenKind::NotEqual, "!=".to_string(), None, 1),
            right: Box::new(Expr::Literal(LiteralExpr {
                value: Some(Object::Bool(false)),
            })),
        });

        let interpreter = Interpreter::new();
        let result = interpreter.interpret(&expr);

        assert_eq!(result, Object::Bool(true));

        // False case
        let expr = Expr::Binary(BinaryExpr {
            left: Box::new(Expr::Literal(LiteralExpr {
                value: Some(Object::Bool(true)),
            })),
            operator: Token::new(TokenKind::NotEqual, "!=".to_string(), None, 1),
            right: Box::new(Expr::Literal(LiteralExpr {
                value: Some(Object::Bool(true)),
            })),
        });

        let interpreter = Interpreter::new();
        let result = interpreter.interpret(&expr);

        assert_eq!(result, Object::Bool(false));
    }

    #[test]
    fn test_equal_equal() {
        // True case
        let expr = Expr::Binary(BinaryExpr {
            left: Box::new(Expr::Literal(LiteralExpr {
                value: Some(Object::Bool(true)),
            })),
            operator: Token::new(TokenKind::EqualEqual, "==".to_string(), None, 1),
            right: Box::new(Expr::Literal(LiteralExpr {
                value: Some(Object::Bool(true)),
            })),
        });

        let interpreter = Interpreter::new();
        let result = interpreter.interpret(&expr);

        assert_eq!(result, Object::Bool(true));

        // False case
        let expr = Expr::Binary(BinaryExpr {
            left: Box::new(Expr::Literal(LiteralExpr {
                value: Some(Object::Bool(true)),
            })),
            operator: Token::new(TokenKind::EqualEqual, "==".to_string(), None, 1),
            right: Box::new(Expr::Literal(LiteralExpr {
                value: Some(Object::Bool(false)),
            })),
        });

        let interpreter = Interpreter::new();
        let result = interpreter.interpret(&expr);

        assert_eq!(result, Object::Bool(false));
    }

    #[test]
    fn test_equal_equal_nil(){
        // True case
        let expr = Expr::Binary(BinaryExpr {
            left: Box::new(Expr::Literal(LiteralExpr {
                value: Some(Object::Nil),
            })),
            operator: Token::new(TokenKind::EqualEqual, "==".to_string(), None, 1),
            right: Box::new(Expr::Literal(LiteralExpr {
                value: Some(Object::Nil),
            })),
        });

        let interpreter = Interpreter::new();
        let result = interpreter.interpret(&expr);

        assert_eq!(result, Object::Bool(true));

        // False case
        let expr = Expr::Binary(BinaryExpr {
            left: Box::new(Expr::Literal(LiteralExpr {
                value: Some(Object::Nil),
            })),
            operator: Token::new(TokenKind::EqualEqual, "==".to_string(), None, 1),
            right: Box::new(Expr::Literal(LiteralExpr {
                value: Some(Object::Bool(false)),
            })),
        });

        let interpreter = Interpreter::new();
        let result = interpreter.interpret(&expr);

        assert_eq!(result, Object::Bool(false));
    }
}