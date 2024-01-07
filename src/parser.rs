use crate::errors::*;
use crate::expr::*;
use crate::stmt::*;
use crate::tokens::*;
use std::rc::Rc;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Result<Vec<Stmt>, Error> {
        let mut statements: Vec<Stmt> = Vec::new();
        while !self.is_at_end() {
            statements.push(self.declaration()?);
        }
        Ok(statements)
    }

    fn expression(&mut self) -> Result<Expr, Error> {
        self.assignment()
    }

    fn declaration(&mut self) -> Result<Stmt, Error> {
        let result = if self.match_token(vec![TokenKind::Fn]) {
            self.function("function")
        } else if self.match_token(vec![TokenKind::Var]) {
            self.var_declaration()
        } else {
            self.statement()
        };

        if result.is_err() {
            self.synchronize();
        }

        result
    }

    fn statement(&mut self) -> Result<Stmt, Error> {
        if self.match_token(vec![TokenKind::For]) {
            return self.for_statement();
        }
        if self.match_token(vec![TokenKind::If]) {
            return self.if_statement();
        }

        if self.match_token(vec![TokenKind::Print]) {
            return self.print_statement();
        }

        if self.match_token(vec![TokenKind::Return]) {
            return self.return_statement();
        }

        if self.match_token(vec![TokenKind::While]) {
            return self.while_statement();
        }

        if self.match_token(vec![TokenKind::LeftBrace]) {
            return Ok(Stmt::Block(BlockStmt {
                statements: self.block()?,
            }));
        }
        self.expression_statement()
    }

    fn return_statement(&mut self) -> Result<Stmt, Error> {
        let keyword = self.previous();
        let value = if self.check(TokenKind::Semicolon) {
            None
        } else {
            Some(self.expression()?)
        };

        self.consume(TokenKind::Semicolon, "Expect ';' after return value.")?;
        Ok(Stmt::Return(ReturnStmt { keyword, value }))
    }

    fn for_statement(&mut self) -> Result<Stmt, Error> {
        self.consume(TokenKind::LeftParen, "Expect '(' after 'for'.")?;

        let initializer = if self.match_token(vec![TokenKind::Semicolon]) {
            None
        } else if self.match_token(vec![TokenKind::Var]) {
            Some(self.var_declaration()?)
        } else {
            Some(self.expression_statement()?)
        };

        let condition = if self.check(TokenKind::Semicolon) {
            None
        } else {
            Some(self.expression()?)
        };

        self.consume(TokenKind::Semicolon, "Expect ';' after loop condition.")?;

        let increment = if self.check(TokenKind::RightParen) {
            None
        } else {
            Some(self.expression()?)
        };

        self.consume(TokenKind::RightParen, "Expect ')' after for clauses.")?;

        let mut body = self.statement()?;

        if let Some(incr) = increment {
            body = Stmt::Block(BlockStmt {
                statements: vec![body, Stmt::Expression(ExpressionStmt { expression: incr })],
            });
        }

        body = Stmt::While(WhileStmt {
            condition: if let Some(cond) = condition {
                cond
            } else {
                Expr::Literal(LiteralExpr {
                    value: Some(Object::Bool(true)),
                })
            },
            body: Box::new(body),
        });

        if let Some(init) = initializer {
            body = Stmt::Block(BlockStmt {
                statements: vec![init, body],
            });
        }

        Ok(body)
    }

    fn if_statement(&mut self) -> Result<Stmt, Error> {
        self.consume(TokenKind::LeftParen, "Expect '(' after 'if'.")?;
        let condition = self.expression()?;
        self.consume(TokenKind::RightParen, "Expect ')' after if condition.")?;

        let then_branch = Box::new(self.statement()?);
        let else_branch = if self.match_token(vec![TokenKind::Else]) {
            Some(Box::new(self.statement()?))
        } else {
            None
        };

        Ok(Stmt::If(IfStmt {
            condition,
            then_branch,
            else_branch,
        }))
    }

    fn print_statement(&mut self) -> Result<Stmt, Error> {
        let value = self.expression()?;
        self.consume(TokenKind::Semicolon, "Expect ';' after value.")?;
        Ok(Stmt::Print(PrintStmt { expression: value }))
    }

    fn var_declaration(&mut self) -> Result<Stmt, Error> {
        let name = self.consume(TokenKind::Identifier, "Expect variable name.")?;

        let initializer = if self.match_token(vec![TokenKind::Equal]) {
            Some(self.expression()?)
        } else {
            None
        };

        self.consume(
            TokenKind::Semicolon,
            "Expect ';' after variable declaration.",
        )?;

        Ok(Stmt::Var(VarStmt { name, initializer }))
    }

    fn while_statement(&mut self) -> Result<Stmt, Error> {
        self.consume(TokenKind::LeftParen, "Expect '(' after 'while'.")?;
        let condition = self.expression()?;
        self.consume(TokenKind::RightParen, "Expect ')' after condition.")?;
        let body = Box::new(self.statement()?);

        Ok(Stmt::While(WhileStmt { condition, body }))
    }

    fn expression_statement(&mut self) -> Result<Stmt, Error> {
        let value = self.expression()?;
        self.consume(TokenKind::Semicolon, "Expect ';' after value.")?;
        Ok(Stmt::Expression(ExpressionStmt { expression: value }))
    }

    fn function(&mut self, kind: &str) -> Result<Stmt, Error> {
        let name = self.consume(TokenKind::Identifier, &format!("Expect {kind} name"))?;

        self.consume(TokenKind::LeftParen, &format!("Expect '(' after {kind} name"))?;

        let mut params = Vec::new();
        if !self.check(TokenKind::RightParen) {
            params.push(self.consume(TokenKind::Identifier, "Expect Parameter Name")?);
            while self.match_token(vec![TokenKind::Comma]) {
                if params.len() >= 255 {
                    return Err(Error::parse_error(
                        &self.peek(),
                        "Can't have more than 255 parameters.",
                    ))
                }
                params.push(self.consume(TokenKind::Identifier, "Expect Parameter Name")?);
            }
        }

        self.consume(TokenKind::RightParen, "Expect ')' after parameter")?;
        self.consume(TokenKind::LeftBrace, &format!("Expect '{{' after {kind} body"))?;

        let body = self.block()?;
        Ok(Stmt::Function(FunctionStmt { name, params: Rc::new(params), body: Rc::new(body) }))
    }

    fn block(&mut self) -> Result<Vec<Stmt>, Error> {
        let mut statements: Vec<Stmt> = Vec::new();

        while !self.check(TokenKind::RightBrace) && !self.is_at_end() {
            statements.push(self.declaration()?);
        }

        self.consume(TokenKind::RightBrace, "Expect '}' after block.")?;

        Ok(statements)
    }

    fn assignment(&mut self) -> Result<Expr, Error> {
        let expr = self.or()?;

        if self.match_token(vec![TokenKind::Equal]) {
            let equals = self.previous();
            let value = self.assignment()?;

            match expr {
                Expr::Variable(v) => {
                    return Ok(Expr::Assign(AssignExpr {
                        name: v.name,
                        value: Box::new(value),
                    }));
                }
                _ => {
                    return Err(Error::parse_error(
                        &equals,
                        "Invalid assignment target.",
                    ));
                }
            }
        }

        Ok(expr)
    }

    fn or(&mut self) -> Result<Expr, Error> {
        let mut expr = self.and()?;

        while self.match_token(vec![TokenKind::Or]) {
            let operator = self.previous();
            let right = self.and()?;
            expr = Expr::Logical(LogicalExpr {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            });
        }

        Ok(expr)
    }

    fn and(&mut self) -> Result<Expr, Error> {
        let mut expr = self.equality()?;

        while self.match_token(vec![TokenKind::And]) {
            let operator = self.previous();
            let right = self.equality()?;
            expr = Expr::Logical(LogicalExpr {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            });
        }

        Ok(expr)
    }

    fn equality(&mut self) -> Result<Expr, Error> {
        let mut expr = self.comparison()?;

        while self.match_token(vec![TokenKind::NotEqual, TokenKind::EqualEqual]) {
            let operator = self.previous();
            let right = self.comparison()?;
            expr = Expr::Binary(BinaryExpr {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            });
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, Error> {
        let mut expr = self.term()?;

        while self.match_token(vec![
            TokenKind::GreaterThan,
            TokenKind::GreaterThanEqual,
            TokenKind::LessThan,
            TokenKind::LessThanEqual,
        ]) {
            let operator = self.previous();
            let right = self.term()?;
            expr = Expr::Binary(BinaryExpr {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            });
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, Error> {
        let mut expr = self.factor()?;

        while self.match_token(vec![TokenKind::Minus, TokenKind::Plus]) {
            let operator = self.previous();
            let right = self.factor()?;
            expr = Expr::Binary(BinaryExpr {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            });
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, Error> {
        let mut expr = self.unary()?;

        while self.match_token(vec![TokenKind::Slash, TokenKind::Asterisk]) {
            let operator = self.previous();
            let right = self.unary()?;
            expr = Expr::Binary(BinaryExpr {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            });
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, Error> {
        if self.match_token(vec![TokenKind::Bang, TokenKind::Minus]) {
            let operator = self.previous();
            let right = self.unary()?;
            return Ok(Expr::Unary(UnaryExpr {
                operator,
                right: Box::new(right),
            }));
        }

        Ok(self.call()?)
    }

    fn finish_call(&mut self, callee: Expr) -> Result<Expr, Error> {
        let mut arguments: Vec<Expr> = Vec::new();

        if !self.check(TokenKind::RightParen) {
            loop {
                if arguments.len() >= 255 {
                    return Err(Error::parse_error(
                        &self.peek(),
                        "Can't have more than 255 arguments.",
                    ));
                }
                arguments.push(self.expression()?);
                if !self.match_token(vec![TokenKind::Comma]) {
                    break;
                }
            }
        }

        let paren = self.consume(TokenKind::RightParen, "Expect ')' after arguments.")?;

        Ok(Expr::Call(CallExpr {
            callee: Box::new(callee),
            paren,
            arguments,
        }))
    }

    fn call(&mut self) -> Result<Expr, Error> {
        let mut expr = self.primary()?;

        loop {
            if self.match_token(vec![TokenKind::LeftParen]) {
                expr = self.finish_call(expr)?;
            } else {
                break;
            }
        }
        Ok(expr)
    }

    fn primary(&mut self) -> Result<Expr, Error> {
        if self.match_token(vec![TokenKind::False]) {
            return Ok(Expr::Literal(LiteralExpr {
                value: Some(Object::Bool(false)),
            }));
        }

        if self.match_token(vec![TokenKind::True]) {
            return Ok(Expr::Literal(LiteralExpr {
                value: Some(Object::Bool(true)),
            }));
        }

        if self.match_token(vec![TokenKind::Nil]) {
            return Ok(Expr::Literal(LiteralExpr {
                value: Some(Object::Nil),
            }));
        }

        if self.match_token(vec![TokenKind::Number, TokenKind::String]) {
            return Ok(Expr::Literal(LiteralExpr {
                value: self.previous().literal,
            }));
        }

        if self.match_token(vec![TokenKind::Identifier]) {
            return Ok(Expr::Variable(VariableExpr {
                name: self.previous(),
            }));
        }

        if self.match_token(vec![TokenKind::LeftParen]) {
            let expr = self.expression()?;
            self.consume(TokenKind::RightParen, "Expect ')' after expression.")?;
            return Ok(Expr::Grouping(GroupingExpr {
                expression: Box::new(expr),
            }));
        }

        Err(Error::parse_error(&self.peek(), "Expect expression."))
    }

    fn match_token(&mut self, kinds: Vec<TokenKind>) -> bool {
        for kind in kinds {
            if self.check(kind) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn check(&self, kind: TokenKind) -> bool {
        if self.is_at_end() {
            return false;
        }
        self.peek().kind == kind
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn is_at_end(&self) -> bool {
        self.peek().kind == TokenKind::EOF
    }

    fn peek(&self) -> Token {
        self.tokens.get(self.current).unwrap().clone()
    }

    fn previous(&self) -> Token {
        self.tokens.get(self.current - 1).unwrap().clone()
    }

    fn consume(&mut self, kind: TokenKind, message: &str) -> Result<Token, Error> {
        if self.check(kind) {
            return Ok(self.advance());
        }

        Err(Error::parse_error(&self.peek(), message))
    }

    fn synchronize(&mut self) {
        self.advance();
        while !self.is_at_end() {
            if self.previous().kind == TokenKind::Semicolon {
                return;
            }

            match self.peek().kind {
                TokenKind::Class
                | TokenKind::Fn
                | TokenKind::Var
                | TokenKind::For
                | TokenKind::If
                | TokenKind::While
                | TokenKind::Print
                | TokenKind::Return => {
                    return;
                }
                _ => {}
            }

            self.advance();
        }
    }
}
