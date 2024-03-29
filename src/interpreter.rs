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
        let superclass = if let Some(superclass_expr) = &stmt.superclass {
            let superclass = self.evaluate(superclass_expr.clone())?;

            if let Object::Class(c) = superclass {
                Some(c)
            } else if let Expr::Variable(v) = superclass_expr.deref() {
                return Err(Error::runtime_error(&v.name, "Superclass must be a class."));
            } else {
                panic!("could not extract variable expr");
            }
        } else {
            None
        };

        self.environment
            .borrow()
            .borrow_mut()
            .define(stmt.name.lexeme.clone(), Object::Nil);

        let enclosing = if let Some(ref s) = superclass {
            let mut e = Environment::new_with_enclosing(self.environment.borrow().clone());
            e.define("super".to_string(), Object::Class(s.clone()));
            Some(self.environment.replace(Rc::new(RefCell::new(e))))
        } else {
            None
        };

        let mut methods = HashMap::new();
        for method in stmt.methods.deref() {
            if let Stmt::Function(func) = method.deref() {
                let is_initializer = func.name.lexeme == "init";
                let function = Object::Function(Rc::new(Function::new(
                    func,
                    self.environment.borrow().deref(),
                    is_initializer,
                )));
                methods.insert(func.name.lexeme.clone(), function);
            } else {
                return Err(Error::runtime_error(
                    &stmt.name,
                    "Class method did not resolve to a function.",
                ));
            };
        }

        let cls = Object::Class(Rc::new(ClassStruct::new(
            stmt.name.lexeme.clone(),
            superclass,
            methods,
        )));

        if let Some(previous) = enclosing {
            self.environment.replace(previous);
        }

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
    fn visit_super_expr(&self, wrapper: Rc<Expr>, expr: &SuperExpr) -> Result<Object, Error> {
        let distance = *self.locals.borrow().get(&wrapper).unwrap();
        let superclass = if let Some(sc) = self
            .environment
            .borrow()
            .borrow()
            .get_at(distance, "super")
            .ok()
        {
            if let Object::Class(superclass) = sc {
                superclass
            } else {
                panic!("Unable to extract superclass");
            }
        } else {
            panic!("Unable to extract superclass");
        };

        let object = self
            .environment
            .borrow()
            .borrow()
            .get_at(distance - 1, "this")
            .ok()
            .unwrap();

        if let Some(method) = superclass.find_method(expr.method.lexeme.clone()) {
            if let Object::Function(func) = method {
                Ok(func.bind(&object))
            } else {
                panic!("method was not a function");
            }
        } else {
            Err(Error::runtime_error(
                &expr.method,
                &format!("Undefined property '{}'.", expr.method.lexeme),
            ))
        }
    }

    fn visit_this_expr(&self, wrapper: Rc<Expr>, expr: &ThisExpr) -> Result<Object, Error> {
        self.look_up_variable(&expr.keyword, wrapper)
    }

    fn visit_set_expr(&self, _wrapper: Rc<Expr>, expr: &SetExpr) -> Result<Object, Error> {
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

    fn visit_get_expr(&self, _wrapper: Rc<Expr>, expr: &GetExpr) -> Result<Object, Error> {
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

        let (callfunc, cls): (Option<Rc<dyn CallableTrait>>, Option<Rc<ClassStruct>>) = match callee
        {
            Object::Function(func) => (Some(func), None),
            Object::Native(native) => (Some(native.func.clone()), None),
            Object::Class(cls) => {
                let class = Rc::clone(&cls);
                (Some(cls), Some(class))
            }
            _ => (None, None),
        };

        if let Some(callfunc) = callfunc {
            if arguments.len() != callfunc.arity() {
                return Err(Error::runtime_error(
                    &expr.paren,
                    &format!(
                        "Expected {} arguments but got {}.",
                        callfunc.arity(),
                        arguments.len()
                    ),
                ));
            }
            callfunc.call(self, &arguments, cls)
        } else {
            Err(Error::runtime_error(
                &expr.paren,
                "Can only call functions and classes.",
            ))
        }
    }

    fn visit_variable_expr(&self, wrapper: Rc<Expr>, expr: &VariableExpr) -> Result<Object, Error> {
        self.look_up_variable(&expr.name, wrapper)
    }
}

impl Interpreter {
    pub fn new() -> Interpreter {
        let global = Rc::new(RefCell::new(Environment::new()));

        global.borrow_mut().define(
            "clock".to_string(),
            Object::Native(Rc::new(Native {
                func: Rc::new(NativeClock {}),
            })),
        );

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
