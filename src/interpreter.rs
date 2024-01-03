use crate::enviroment::Environment;
use crate::errors::*;
use crate::expr::*;
use crate::stmt::*;
use crate::tokens::*;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Interpreter {
    environment: RefCell<Rc<RefCell<Environment>>>,
}

impl StmtVisitor<()> for Interpreter {
    fn visit_block_stmt(&self, stmt: &BlockStmt) -> Result<(), Error> {
        let environment =
            Environment::new_with_enclosing(Rc::clone(&self.environment.borrow().clone()));
        self.execute_block(&stmt.statements, environment)
    }
    fn visit_print_stmt(&self, stmt: &PrintStmt) -> Result<(), Error> {
        let value = self.evaluate(&stmt.expression)?;
        println!("{}", value);
        Ok(())
    }

    fn visit_expression_stmt(&self, stmt: &ExpressionStmt) -> Result<(), Error> {
        self.evaluate(&stmt.expression)?;
        Ok(())
    }

    fn visit_var_stmt(&self, stmt: &VarStmt) -> Result<(), Error> {
        let value = if let Some(expr) = &stmt.initializer {
            self.evaluate(expr)?
        } else {
            Object::Nil
        };

        self.environment
            .borrow()
            .borrow_mut()
            .define(stmt.name.lexeme.clone(), value);
        Ok(())
    }
}

impl ExprVisitor<Object> for Interpreter {
    fn visit_assign_expr(&self, expr: &AssignExpr) -> Result<Object, Error> {
        let value = self.evaluate(&expr.value)?;
        self.environment
            .borrow()
            .borrow_mut()
            .assign(&expr.name, value.clone())?;
        Ok(value)
    }

    fn visit_literal_expr(&self, expr: &LiteralExpr) -> Result<Object, Error> {
        Ok(expr.value.clone().unwrap())
    }

    fn visit_grouping_expr(&self, expr: &GroupingExpr) -> Result<Object, Error> {
        Ok(self.evaluate(&expr.expression)?)
    }

    fn visit_unary_expr(&self, expr: &UnaryExpr) -> Result<Object, Error> {
        let right = self.evaluate(&expr.right)?;

        match expr.operator.kind {
            TokenKind::Minus => {
                if let Object::Num(x) = right {
                    return Ok(Object::Num(-x));
                }
                Ok(Object::Nil)
            }
            TokenKind::Bang => {
                return Ok(Object::Bool(!self.is_truthy(right)));
            }
            _ => Err(Error::new(
                expr.operator.line,
                format!("Expect unary operator. Got {}", expr.operator.lexeme),
            )),
        }
    }

    fn visit_binary_expr(&self, expr: &BinaryExpr) -> Result<Object, Error> {
        let left = self.evaluate(&expr.left)?;
        let right = self.evaluate(&expr.right)?;
        let operator = &expr.operator.kind;

        let result = match (left, right) {
            (Object::Num(left), Object::Num(right)) => match operator {
                TokenKind::Minus => Object::Num(left - right),
                TokenKind::Slash => {
                    if right == 0.0 {
                        Object::ArithmeticError
                    } else {
                        Object::Num(left / right)
                    }
                }
                TokenKind::Asterisk => Object::Num(left * right),
                TokenKind::Plus => Object::Num(left + right),
                TokenKind::GreaterThan => Object::Bool(left > right),
                TokenKind::GreaterThanEqual => Object::Bool(left >= right),
                TokenKind::LessThan => Object::Bool(left < right),
                TokenKind::LessThanEqual => Object::Bool(left <= right),
                TokenKind::NotEqual => Object::Bool(left != right),
                TokenKind::EqualEqual => Object::Bool(left == right),
                _ => Object::ArithmeticError,
            },

            (Object::Str(left), Object::Str(right)) => match operator {
                TokenKind::Plus => Object::Str(format!("{}{}", left, right)),
                TokenKind::NotEqual => Object::Bool(left != right),
                TokenKind::EqualEqual => Object::Bool(left == right),
                _ => Object::ArithmeticError,
            },

            (Object::Str(left), Object::Num(right)) => match operator {
                TokenKind::Plus => Object::Str(format!("{}{}", left, right)),
                _ => Object::ArithmeticError,
            },

            (Object::Num(left), Object::Str(right)) => match operator {
                TokenKind::Plus => Object::Str(format!("{}{}", left, right)),
                _ => Object::ArithmeticError,
            },

            (Object::Bool(left), Object::Bool(right)) => match operator {
                TokenKind::NotEqual => Object::Bool(left != right),
                TokenKind::EqualEqual => Object::Bool(left == right),
                _ => Object::ArithmeticError,
            },

            (Object::Nil, Object::Nil) => match operator {
                TokenKind::NotEqual => Object::Bool(false),
                TokenKind::EqualEqual => Object::Bool(true),
                _ => Object::ArithmeticError,
            },

            (Object::Nil, _) => match operator {
                TokenKind::NotEqual => Object::Bool(true),
                TokenKind::EqualEqual => Object::Bool(false),
                _ => Object::ArithmeticError,
            },

            _ => Object::ArithmeticError,
        };

        if result == Object::ArithmeticError {
            Err(Error::new(expr.operator.line, format!("Arithmetic error")))
        } else {
            Ok(result)
        }
    }

    fn visit_variable_expr(&self, expr: &VariableExpr) -> Result<Object, Error> {
        self.environment.borrow().borrow().get(&expr.name)
    }
}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {
            environment: RefCell::new(Rc::new(RefCell::new(Environment::new()))),
        }
    }

    fn execute(&self, stmt: &Stmt) -> Result<(), Error> {
        stmt.accept(self)
    }

    fn execute_block(&self, statements: &[Stmt], environment: Environment) -> Result<(), Error> {
        let previous = self.environment.replace(Rc::new(RefCell::new(environment)));

        let result = statements
            .iter()
            .try_for_each(|statement| self.execute(statement));

        self.environment.replace(previous);

        result
    }

    pub fn interpret(&self, statements: &[Stmt]) -> bool {
        let mut success = true;
        for statment in statements {
            if let Err(e) = self.execute(statment) {
                e.report();
                success = false;
                break;
            }
        }
        success
    }

    fn evaluate(&self, expr: &Expr) -> Result<Object, Error> {
        expr.accept(self)
    }

    fn is_truthy(&self, object: Object) -> bool {
        match object {
            Object::Nil => false,
            Object::Bool(x) => x,
            _ => true,
        }
    }

    pub fn print_env(&self) {
        println!("{:?}", self.environment.borrow());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unary_minus() {
        let expr = UnaryExpr {
            operator: Token::new(TokenKind::Minus, "-".to_string(), None, 1),
            right: Box::new(Expr::Literal(LiteralExpr {
                value: Some(Object::Num(123.0)),
            })),
        };

        let interpreter = Interpreter::new();
        let result = interpreter.visit_unary_expr(&expr);
        assert!(result.is_ok());
        assert_eq!(result.ok(), Some(Object::Num(-123.0)));
    }

    #[test]
    fn test_subtraction() {
        let expr = BinaryExpr {
            left: Box::new(Expr::Literal(LiteralExpr {
                value: Some(Object::Num(125.0)),
            })),
            operator: Token::new(TokenKind::Minus, "-".to_string(), None, 1),
            right: Box::new(Expr::Literal(LiteralExpr {
                value: Some(Object::Num(123.0)),
            })),
        };

        let interpreter = Interpreter::new();
        let result = interpreter.visit_binary_expr(&expr);

        assert!(result.is_ok());
        assert_eq!(result.ok(), Some(Object::Num(2.0)));
    }

    #[test]
    fn test_unary_bang() {
        let expr = UnaryExpr {
            operator: Token::new(TokenKind::Bang, "!".to_string(), None, 1),
            right: Box::new(Expr::Literal(LiteralExpr {
                value: Some(Object::Bool(true)),
            })),
        };

        let interpreter = Interpreter::new();
        let result = interpreter.visit_unary_expr(&expr);

        assert!(result.is_ok());
        assert_eq!(result.ok(), Some(Object::Bool(false)));
    }

    #[test]
    fn test_multiplication() {
        let expr = BinaryExpr {
            left: Box::new(Expr::Literal(LiteralExpr {
                value: Some(Object::Num(15.0)),
            })),
            operator: Token::new(TokenKind::Asterisk, "*".to_string(), None, 1),
            right: Box::new(Expr::Literal(LiteralExpr {
                value: Some(Object::Num(10.0)),
            })),
        };

        let interpreter = Interpreter::new();
        let result = interpreter.visit_binary_expr(&expr);

        assert!(result.is_ok());
        assert_eq!(result.ok(), Some(Object::Num(150.0)));
    }

    #[test]
    fn test_division() {
        let expr = BinaryExpr {
            left: Box::new(Expr::Literal(LiteralExpr {
                value: Some(Object::Num(15.0)),
            })),
            operator: Token::new(TokenKind::Slash, "/".to_string(), None, 1),
            right: Box::new(Expr::Literal(LiteralExpr {
                value: Some(Object::Num(10.0)),
            })),
        };

        let interpreter = Interpreter::new();
        let result = interpreter.visit_binary_expr(&expr);

        assert!(result.is_ok());
        assert_eq!(result.ok(), Some(Object::Num(1.5)));
    }

    #[test]
    fn test_addition() {
        let expr = BinaryExpr {
            left: Box::new(Expr::Literal(LiteralExpr {
                value: Some(Object::Num(15.0)),
            })),
            operator: Token::new(TokenKind::Plus, "+".to_string(), None, 1),
            right: Box::new(Expr::Literal(LiteralExpr {
                value: Some(Object::Num(10.0)),
            })),
        };

        let interpreter = Interpreter::new();
        let result = interpreter.visit_binary_expr(&expr);

        assert!(result.is_ok());
        assert_eq!(result.ok(), Some(Object::Num(25.0)));
    }

    #[test]
    fn test_concatenation() {
        let expr = BinaryExpr {
            left: Box::new(Expr::Literal(LiteralExpr {
                value: Some(Object::Str("Hello".to_string())),
            })),
            operator: Token::new(TokenKind::Plus, "+".to_string(), None, 1),
            right: Box::new(Expr::Literal(LiteralExpr {
                value: Some(Object::Str("World".to_string())),
            })),
        };

        let interpreter = Interpreter::new();
        let result = interpreter.visit_binary_expr(&expr);

        assert!(result.is_ok());
        assert_eq!(result.ok(), Some(Object::Str("HelloWorld".to_string())));
    }

    #[test]
    fn test_greater_than() {
        // True case
        let expr = BinaryExpr {
            left: Box::new(Expr::Literal(LiteralExpr {
                value: Some(Object::Num(15.0)),
            })),
            operator: Token::new(TokenKind::GreaterThan, ">".to_string(), None, 1),
            right: Box::new(Expr::Literal(LiteralExpr {
                value: Some(Object::Num(10.0)),
            })),
        };

        let interpreter = Interpreter::new();
        let result = interpreter.visit_binary_expr(&expr);

        assert!(result.is_ok());
        assert_eq!(result.ok(), Some(Object::Bool(true)));

        // False case
        let expr = BinaryExpr {
            left: Box::new(Expr::Literal(LiteralExpr {
                value: Some(Object::Num(10.0)),
            })),
            operator: Token::new(TokenKind::GreaterThan, ">".to_string(), None, 1),
            right: Box::new(Expr::Literal(LiteralExpr {
                value: Some(Object::Num(15.0)),
            })),
        };

        let interpreter = Interpreter::new();
        let result = interpreter.visit_binary_expr(&expr);

        assert!(result.is_ok());
        assert_eq!(result.ok(), Some(Object::Bool(false)));
    }

    #[test]
    fn test_greater_than_equal() {
        //True case
        let expr = BinaryExpr {
            left: Box::new(Expr::Literal(LiteralExpr {
                value: Some(Object::Num(15.0)),
            })),
            operator: Token::new(TokenKind::GreaterThanEqual, ">=".to_string(), None, 1),
            right: Box::new(Expr::Literal(LiteralExpr {
                value: Some(Object::Num(10.0)),
            })),
        };

        let interpreter = Interpreter::new();
        let result = interpreter.visit_binary_expr(&expr);

        assert!(result.is_ok());
        assert_eq!(result.ok(), Some(Object::Bool(true)));

        //False case
        let expr = BinaryExpr {
            left: Box::new(Expr::Literal(LiteralExpr {
                value: Some(Object::Num(10.0)),
            })),
            operator: Token::new(TokenKind::GreaterThanEqual, ">=".to_string(), None, 1),
            right: Box::new(Expr::Literal(LiteralExpr {
                value: Some(Object::Num(15.0)),
            })),
        };

        let interpreter = Interpreter::new();
        let result = interpreter.visit_binary_expr(&expr);

        assert!(result.is_ok());
        assert_eq!(result.ok(), Some(Object::Bool(false)));
    }

    #[test]
    fn test_less_than() {
        //False case
        let expr = BinaryExpr {
            left: Box::new(Expr::Literal(LiteralExpr {
                value: Some(Object::Num(15.0)),
            })),
            operator: Token::new(TokenKind::LessThan, "<".to_string(), None, 1),
            right: Box::new(Expr::Literal(LiteralExpr {
                value: Some(Object::Num(10.0)),
            })),
        };

        let interpreter = Interpreter::new();
        let result = interpreter.visit_binary_expr(&expr);

        assert!(result.is_ok());
        assert_eq!(result.ok(), Some(Object::Bool(false)));

        //True case
        let expr = BinaryExpr {
            left: Box::new(Expr::Literal(LiteralExpr {
                value: Some(Object::Num(10.0)),
            })),
            operator: Token::new(TokenKind::LessThan, "<".to_string(), None, 1),
            right: Box::new(Expr::Literal(LiteralExpr {
                value: Some(Object::Num(15.0)),
            })),
        };

        let interpreter = Interpreter::new();
        let result = interpreter.visit_binary_expr(&expr);

        assert!(result.is_ok());
        assert_eq!(result.ok(), Some(Object::Bool(true)));
    }

    #[test]
    fn test_less_than_equal() {
        //True case
        let expr = BinaryExpr {
            left: Box::new(Expr::Literal(LiteralExpr {
                value: Some(Object::Num(10.0)),
            })),
            operator: Token::new(TokenKind::LessThanEqual, "<=".to_string(), None, 1),
            right: Box::new(Expr::Literal(LiteralExpr {
                value: Some(Object::Num(15.0)),
            })),
        };

        let interpreter = Interpreter::new();
        let result = interpreter.visit_binary_expr(&expr);

        assert!(result.is_ok());
        assert_eq!(result.ok(), Some(Object::Bool(true)));

        //False case
        let expr = BinaryExpr {
            left: Box::new(Expr::Literal(LiteralExpr {
                value: Some(Object::Num(15.0)),
            })),
            operator: Token::new(TokenKind::LessThanEqual, "<=".to_string(), None, 1),
            right: Box::new(Expr::Literal(LiteralExpr {
                value: Some(Object::Num(10.0)),
            })),
        };

        let interpreter = Interpreter::new();
        let result = interpreter.visit_binary_expr(&expr);

        assert!(result.is_ok());
        assert_eq!(result.ok(), Some(Object::Bool(false)));
    }

    #[test]
    fn test_not_equal() {
        // True case
        let expr = BinaryExpr {
            left: Box::new(Expr::Literal(LiteralExpr {
                value: Some(Object::Bool(true)),
            })),
            operator: Token::new(TokenKind::NotEqual, "!=".to_string(), None, 1),
            right: Box::new(Expr::Literal(LiteralExpr {
                value: Some(Object::Bool(false)),
            })),
        };

        let interpreter = Interpreter::new();
        let result = interpreter.visit_binary_expr(&expr);

        assert!(result.is_ok());
        assert_eq!(result.ok(), Some(Object::Bool(true)));

        // False case
        let expr = BinaryExpr {
            left: Box::new(Expr::Literal(LiteralExpr {
                value: Some(Object::Bool(true)),
            })),
            operator: Token::new(TokenKind::NotEqual, "!=".to_string(), None, 1),
            right: Box::new(Expr::Literal(LiteralExpr {
                value: Some(Object::Bool(true)),
            })),
        };

        let interpreter = Interpreter::new();
        let result = interpreter.visit_binary_expr(&expr);

        assert!(result.is_ok());
        assert_eq!(result.ok(), Some(Object::Bool(false)));
    }

    #[test]
    fn test_equal_equal() {
        // True case
        let expr = BinaryExpr {
            left: Box::new(Expr::Literal(LiteralExpr {
                value: Some(Object::Bool(true)),
            })),
            operator: Token::new(TokenKind::EqualEqual, "==".to_string(), None, 1),
            right: Box::new(Expr::Literal(LiteralExpr {
                value: Some(Object::Bool(true)),
            })),
        };

        let interpreter = Interpreter::new();
        let result = interpreter.visit_binary_expr(&expr);

        assert!(result.is_ok());
        assert_eq!(result.ok(), Some(Object::Bool(true)));

        // False case
        let expr = BinaryExpr {
            left: Box::new(Expr::Literal(LiteralExpr {
                value: Some(Object::Bool(true)),
            })),
            operator: Token::new(TokenKind::EqualEqual, "==".to_string(), None, 1),
            right: Box::new(Expr::Literal(LiteralExpr {
                value: Some(Object::Bool(false)),
            })),
        };

        let interpreter = Interpreter::new();
        let result = interpreter.visit_binary_expr(&expr);

        assert!(result.is_ok());
        assert_eq!(result.ok(), Some(Object::Bool(false)));
    }

    #[test]
    fn test_equal_equal_nil() {
        // True case
        let expr = BinaryExpr {
            left: Box::new(Expr::Literal(LiteralExpr {
                value: Some(Object::Nil),
            })),
            operator: Token::new(TokenKind::EqualEqual, "==".to_string(), None, 1),
            right: Box::new(Expr::Literal(LiteralExpr {
                value: Some(Object::Nil),
            })),
        };

        let interpreter = Interpreter::new();
        let result = interpreter.visit_binary_expr(&expr);

        assert!(result.is_ok());
        assert_eq!(result.ok(), Some(Object::Bool(true)));

        // False case
        let expr = BinaryExpr {
            left: Box::new(Expr::Literal(LiteralExpr {
                value: Some(Object::Nil),
            })),
            operator: Token::new(TokenKind::EqualEqual, "==".to_string(), None, 1),
            right: Box::new(Expr::Literal(LiteralExpr {
                value: Some(Object::Bool(false)),
            })),
        };

        let interpreter = Interpreter::new();
        let result = interpreter.visit_binary_expr(&expr);

        assert!(result.is_ok());
        assert_eq!(result.ok(), Some(Object::Bool(false)));
    }

    #[test]
    fn test_var_stmt() {
        let name = Token::new(TokenKind::Identifier, "foo".to_string(), None, 1);
        let stmt = VarStmt {
            name,
            initializer: Some(Expr::Literal(LiteralExpr {
                value: Some(Object::Num(1.0)),
            })),
        };

        let interpreter = Interpreter::new();
        let result = interpreter.visit_var_stmt(&stmt);

        let _ = interpreter.visit_var_stmt(&stmt);
        assert!(result.is_ok());
        assert_eq!(
            interpreter.environment.borrow().get(&stmt.name).ok(),
            Some(Object::Num(1.0))
        );
    }

    #[test]
    fn test_var_stmt_no_initializer() {
        let name = Token::new(TokenKind::Identifier, "foo".to_string(), None, 1);
        let stmt = VarStmt {
            name,
            initializer: None,
        };

        let interpreter = Interpreter::new();
        let result = interpreter.visit_var_stmt(&stmt);

        let _ = interpreter.visit_var_stmt(&stmt);
        assert!(result.is_ok());
        assert_eq!(
            interpreter.environment.borrow().get(&stmt.name).ok(),
            Some(Object::Nil)
        );
    }

    #[test]
    fn test_variable_expr() {
        let name = Token::new(TokenKind::Identifier, "foo".to_string(), None, 1);
        let expr = VariableExpr { name: name.clone() };

        let interpreter = Interpreter::new();
        let var = VarStmt {
            name: name,
            initializer: Some(Expr::Literal(LiteralExpr {
                value: Some(Object::Num(1.0)),
            })),
        };

        assert!(interpreter.visit_var_stmt(&var).is_ok());
        assert_eq!(
            interpreter.visit_variable_expr(&expr).ok(),
            Some(Object::Num(1.0))
        );
    }

    #[test]
    fn test_variable_expr_undefined() {
        let name = Token::new(TokenKind::Identifier, "foo".to_string(), None, 1);
        let expr = VariableExpr { name: name.clone() };

        let interpreter = Interpreter::new();

        assert!(interpreter.visit_variable_expr(&expr).is_err());
    }
}
