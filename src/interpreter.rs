use crate::callable::*;
use crate::enviroment::Environment;
use crate::errors::*;
use crate::expr::*;
use crate::functions::*;
use crate::native_functions::*;
use crate::stmt::*;
use crate::tokens::*;

use std::cell::RefCell;
use std::collections::HashMap;
use std::ops::Deref;
use std::rc::Rc;
pub struct Interpreter {
    pub globals: Rc<RefCell<Environment>>,
    environment: RefCell<Rc<RefCell<Environment>>>,
    locals: RefCell<HashMap<Rc<Expr>, usize>>,
}

impl StmtVisitor<()> for Interpreter {

    fn visit_class_stmt(&self, _: Rc<Stmt>, stmt: &ClassStmt) -> Result<(), Error> {
        self.environment
            .borrow()
            .borrow_mut()
            .define(stmt.name.lexeme.clone(), Object::Nil);

        let mut methods = HashMap::new();
        for method in stmt.methods.deref() {
            if let Stmt::Function(func) = method.deref() {
                let is_initializer = func.name.lexeme == "init";
                let function = Object::Function(Rc::new(
                    Function::new(func, self.environment.borrow().deref(), is_initializer),
                ));
                methods.insert(func.name.lexeme.clone(), function);
            } else {
                return Err(Error::runtime_error(
                    &stmt.name,
                    "Class method did not resolve to a function.",
                ));
            };
        }

        let cls = Object::Class(Rc::new(ClassStruct::new(stmt.name.lexeme.clone(), methods)));

        self.environment
            .borrow()
            .borrow_mut()
            .assign(&stmt.name, cls.clone())?;
        Ok(())
    }

    fn visit_return_stmt(&self, _: Rc<Stmt>, stmt: &ReturnStmt) -> Result<(), Error> {
        if let Some(value) = stmt.value.clone() {
            let value = self.evaluate(value)?;
            Err(Error::return_value(value))
        } else {
            Err(Error::return_value(Object::Nil))
        }
    }
    fn visit_function_stmt(&self, _: Rc<Stmt>, stmt: &FunctionStmt) -> Result<(), Error> {
        let function = Function::new(stmt, self.environment.borrow().deref(), false);
        self.environment.borrow().borrow_mut().define(
            stmt.name.lexeme.clone(),
            Object::Function(Rc::new(function)),
        );
        Ok(())
    }

    fn visit_if_stmt(&self, _: Rc<Stmt>, stmt: &IfStmt) -> Result<(), Error> {
        if self.is_truthy(self.evaluate(stmt.condition.clone())?) {
            self.execute(stmt.then_branch.clone())?;
        } else if let Some(else_branch) = stmt.else_branch.clone() {
            self.execute(else_branch)?;
        }
        Ok(())
    }
    fn visit_block_stmt(&self, _: Rc<Stmt>, stmt: &BlockStmt) -> Result<(), Error> {
        let environment =
            Environment::new_with_enclosing(Rc::clone(&self.environment.borrow().clone()));
        self.execute_block(&stmt.statements, environment)
    }
    fn visit_print_stmt(&self, _: Rc<Stmt>, stmt: &PrintStmt) -> Result<(), Error> {
        let value = self.evaluate(stmt.expression.clone())?;
        println!("{}", value);
        Ok(())
    }

    fn visit_expression_stmt(&self, _: Rc<Stmt>, stmt: &ExpressionStmt) -> Result<(), Error> {
        self.evaluate(stmt.expression.clone())?;
        Ok(())
    }

    fn visit_var_stmt(&self, _: Rc<Stmt>, stmt: &VarStmt) -> Result<(), Error> {
        let value = if let Some(expr) = stmt.initializer.clone() {
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

    fn visit_while_stmt(&self, _: Rc<Stmt>, stmt: &WhileStmt) -> Result<(), Error> {
        while self.is_truthy(self.evaluate(stmt.condition.clone())?) {
            self.execute(stmt.body.clone())?;
        }
        Ok(())
    }
}

impl ExprVisitor<Object> for Interpreter {

    fn visit_this_expr(&self, wrapper: Rc<Expr>, expr: &ThisExpr) -> Result<Object, Error> {
        self.look_up_variable(&expr.keyword, wrapper)
    }
    
    fn visit_set_expr(&self, wrapper: Rc<Expr>, expr: &SetExpr) -> Result<Object, Error> {
        let object = self.evaluate(expr.object.clone())?;

        if let Object::Instance(inst) = object {
            let value = self.evaluate(expr.value.clone())?;
            inst.set(&expr.name, value.clone());
            return Ok(value);
        }

        Err(Error::runtime_error(
            &expr.name,
            "Only instances have fields.",
        ))
    }

    fn visit_get_expr(&self, wrapper: Rc<Expr>, expr: &GetExpr) -> Result<Object, Error> {
        let object = self.evaluate(expr.object.clone())?;

        if let Object::Instance(inst) = object {
            return Ok(inst.get(&expr.name, &inst)?);
        } else {
            Err(Error::runtime_error(
                &expr.name,
                "Only instances have fields.",
            ))
        }
    }
    fn visit_logical_expr(&self, _: Rc<Expr>, expr: &LogicalExpr) -> Result<Object, Error> {
        let left = self.evaluate(expr.left.clone())?;

        if expr.operator.kind == TokenKind::Or {
            if self.is_truthy(left.clone()) {
                return Ok(left);
            }
        } else {
            if !self.is_truthy(left.clone()) {
                return Ok(left);
            }
        }

        self.evaluate(expr.right.clone())
    }
    fn visit_assign_expr(&self, wrapper: Rc<Expr>, expr: &AssignExpr) -> Result<Object, Error> {
        let value = self.evaluate(expr.value.clone())?;
        if let Some(distance) = self.locals.borrow().get(&wrapper) {
            self.environment.borrow().borrow_mut().assign_at(
                *distance,
                &expr.name,
                value.clone(),
            )?;
        } else {
            self.globals
                .borrow_mut()
                .assign(&expr.name, value.clone())?;
        }

        Ok(value)
    }

    fn visit_literal_expr(&self, _: Rc<Expr>, expr: &LiteralExpr) -> Result<Object, Error> {
        Ok(expr.value.clone().unwrap())
    }

    fn visit_grouping_expr(&self, _: Rc<Expr>, expr: &GroupingExpr) -> Result<Object, Error> {
        Ok(self.evaluate(expr.expression.clone())?)
    }

    fn visit_unary_expr(&self, _: Rc<Expr>, expr: &UnaryExpr) -> Result<Object, Error> {
        let right = self.evaluate(expr.right.clone())?;

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
            _ => Err(Error::runtime_error(
                &expr.operator,
                "Invalid unary operator",
            )),
        }
    }

    fn visit_binary_expr(&self, _: Rc<Expr>, expr: &BinaryExpr) -> Result<Object, Error> {
        let left = self.evaluate(expr.left.clone())?;
        let right = self.evaluate(expr.right.clone())?;
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
            Err(Error::runtime_error(
                &expr.operator,
                "Invalid binary operator",
            ))
        } else {
            Ok(result)
        }
    }

    fn visit_call_expr(&self, _: Rc<Expr>, expr: &CallExpr) -> Result<Object, Error> {
        let callee = self.evaluate(expr.callee.clone())?;

        let mut arguments = Vec::new();

        for argument in expr.arguments.clone() {
            arguments.push(self.evaluate(argument)?);
        }

        if let Object::Function(function) = callee {
            if arguments.len() != function.arity() {
                return Err(Error::runtime_error(
                    &expr.paren,
                    &format!(
                        "Expected {} arguments but got {}",
                        function.arity(),
                        arguments.len()
                    ),
                ));
            }
            function.call(self, &arguments)
        } else if let Object::Class(cls) = callee {
            if arguments.len() != cls.arity() {
                return Err(Error::runtime_error(
                    &expr.paren,
                    &format!(
                        "Expected {} arguments but got {}",
                        cls.arity(),
                        arguments.len()
                    ),
                ));
            }
            cls.instantiate(self, arguments, Rc::clone(&cls))
        } else {
            return Err(Error::runtime_error(
                &expr.paren,
                "Can only call functions and classes",
            ));
        }
    }

    fn visit_variable_expr(&self, wrapper: Rc<Expr>, expr: &VariableExpr) -> Result<Object, Error> {
        self.look_up_variable(&expr.name, wrapper)
    }
}

impl Interpreter {
    pub fn new() -> Interpreter {
        let global = Rc::new(RefCell::new(Environment::new()));

        // global.borrow_mut().define(
        //     "clock".to_string(),
        //     Object::Function(Callable {
        //         func: Rc::new(NativeClock {}),
        //     }),
        // );

        Interpreter {
            globals: Rc::clone(&global),
            environment: RefCell::new(Rc::clone(&global)),
            locals: RefCell::new(HashMap::new()),
        }
    }

    fn execute(&self, stmt: Rc<Stmt>) -> Result<(), Error> {
        stmt.accept(stmt.clone(), self)
    }

    pub fn execute_block(
        &self,
        statements: &Rc<Vec<Rc<Stmt>>>,
        environment: Environment,
    ) -> Result<(), Error> {
        let previous = self.environment.replace(Rc::new(RefCell::new(environment)));

        let result = statements
            .iter()
            .try_for_each(|statement| self.execute(statement.clone()));

        self.environment.replace(previous);

        result
    }

    pub fn interpret(&self, statements: &[Rc<Stmt>]) -> bool {
        let mut success = true;
        for statement in statements {
            if self.execute(statement.clone()).is_err() {
                success = false;
                break;
            }
        }
        success
    }

    fn evaluate(&self, expr: Rc<Expr>) -> Result<Object, Error> {
        expr.accept(expr.clone(), self)
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

    pub fn resolve(&self, expr: Rc<Expr>, depth: usize) {
        self.locals.borrow_mut().insert(expr, depth);
    }

    fn look_up_variable(&self, name: &Token, expr: Rc<Expr>) -> Result<Object, Error> {
        if let Some(distance) = self.locals.borrow().get(&expr) {
            self.environment
                .borrow()
                .borrow()
                .get_at(*distance, &name.lexeme)
        } else {
            self.globals.borrow().get(&name)
        }
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
        let result = interpreter.visit_var_stmt(stmt);

        let _ = interpreter.visit_var_stmt(stmt);
        assert!(result.is_ok());
        assert_eq!(
            interpreter
                .environment
                .borrow()
                .borrow()
                .get(stmt.name)
                .ok(),
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
        let result = interpreter.visit_var_stmt(stmt);

        let _ = interpreter.visit_var_stmt(stmt);
        assert!(result.is_ok());
        assert_eq!(
            interpreter
                .environment
                .borrow()
                .borrow()
                .get(stmt.name)
                .ok(),
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
