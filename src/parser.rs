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

    pub fn parse(&mut self) -> Result<Vec<Rc<Stmt>>, Error> {
        let mut statements: Vec<Rc<Stmt>> = Vec::new();
        while !self.is_at_end() {
            statements.push(self.declaration()?);
        }
        Ok(statements)
    }

    fn expression(&mut self) -> Result<Expr, Error> {
        self.assignment()
    }

    fn declaration(&mut self) -> Result<Rc<Stmt>, Error> {
        let result = if self.match_token(vec![TokenKind::Class]){
            self.class_declaration() 
        }else if self.match_token(vec![TokenKind::Fn]) {
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

    fn class_declaration(&mut self) -> Result<Rc<Stmt>, Error> {
        let name = self.consume(TokenKind::Identifier, "Expect class name.")?;

        let superclass = if self.match_token(vec![TokenKind::LessThan]) {
            self.consume(TokenKind::Identifier, "Expect superclass name.")?;
            Some(Rc::new(Expr::Variable(Rc::new(VariableExpr {
                name: self.previous(),
            }))))
        } else {
            None
        };

        let mut methods = Vec::new();
        self.consume(TokenKind::LeftBrace, "Expect '{' before class body.")?;
        while !self.check(TokenKind::RightBrace) && !self.is_at_end() {
            methods.push(self.function("method")?);
        }
        self.consume(TokenKind::RightBrace, "Expect '}' after class body.")?;

        Ok(Rc::new(Stmt::Class(Rc::new(ClassStmt {
            name,
            superclass,
            methods: Rc::new(methods),
        }))))
    }

    fn statement(&mut self) -> Result<Rc<Stmt>, Error> {
        if self.match_token(vec![TokenKind::For]) {
            return self.for_statement();
        }
        if self.match_token(vec![TokenKind::If]) {
            return Ok(Rc::new(self.if_statement()?));
        }

        if self.match_token(vec![TokenKind::Print]) {
            return Ok(Rc::new(self.print_statement()?));
        }

        if self.match_token(vec![TokenKind::Return]) {
            return Ok(self.return_statement()?);
        }

        if self.match_token(vec![TokenKind::While]) {
            return Ok(Rc::new(self.while_statement()?));
        }

        if self.match_token(vec![TokenKind::LeftBrace]) {
            return Ok(Rc::new(Stmt::Block(Rc::new(BlockStmt {
                statements: Rc::new(self.block()?),
            }))));
        }
        
        Ok(self.expression_statement()?)
    }

    fn return_statement(&mut self) -> Result<Rc<Stmt>, Error> {
        let keyword = self.previous();
        let value = if self.check(TokenKind::Semicolon) {
            None
        } else {
            Some(Rc::new(self.expression()?))
        };

        self.consume(TokenKind::Semicolon, "Expect ';' after return value.")?;
        Ok(Rc::new(Stmt::Return(Rc::new(ReturnStmt { keyword, value }))))
    }

    fn for_statement(&mut self) -> Result<Rc<Stmt>, Error> {
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
            body = Rc::new(Stmt::Block(Rc::new(BlockStmt {
                statements: Rc::new(vec![body, Rc::new(Stmt::Expression(Rc::new(ExpressionStmt {
                    expression: Rc::new(incr),
                })))]),
            })));
        }

        body = Rc::new(Stmt::While(Rc::new(WhileStmt {
            condition: if let Some(cond) = condition {
                Rc::new(cond)
            } else {
                Rc::new(Expr::Literal(Rc::new(LiteralExpr {
                    value: Some(Object::Bool(true)),
                })))
            },
            body,
        })));

        if let Some(init) = initializer {
            body = Rc::new(Stmt::Block(Rc::new(BlockStmt {
                statements: Rc::new(vec![init, body]),
            })));
        }

        Ok(body)
    }

    fn if_statement(&mut self) -> Result<Stmt, Error> {
        self.consume(TokenKind::LeftParen, "Expect '(' after 'if'.")?;
        let condition = Rc::new(self.expression()?);
        self.consume(TokenKind::RightParen, "Expect ')' after if condition.")?;

        let then_branch = self.statement()?;
        let else_branch = if self.match_token(vec![TokenKind::Else]) {
            Some(self.statement()?)
        } else {
            None
        };

        Ok(Stmt::If(Rc::new(IfStmt {
            condition,
            then_branch,
            else_branch,
        })))
    }

    fn print_statement(&mut self) -> Result<Stmt, Error> {
        let value = Rc::new(self.expression()?);
        self.consume(TokenKind::Semicolon, "Expect ';' after value.")?;
        Ok(Stmt::Print(Rc::new(PrintStmt { expression: value })))
    }

    fn var_declaration(&mut self) -> Result<Rc<Stmt>, Error> {
        let name = self.consume(TokenKind::Identifier, "Expect variable name.")?;

        let initializer = if self.match_token(vec![TokenKind::Equal]) {
            Some(Rc::new(self.expression()?))
        } else {
            None
        };

        self.consume(
            TokenKind::Semicolon,
            "Expect ';' after variable declaration.",
        )?;

        Ok(Rc::new(Stmt::Var(Rc::new(VarStmt { name, initializer }))))
    }

    fn while_statement(&mut self) -> Result<Stmt, Error> {
        self.consume(TokenKind::LeftParen, "Expect '(' after 'while'.")?;
        let condition = Rc::new(self.expression()?);
        self.consume(TokenKind::RightParen, "Expect ')' after condition.")?;
        let body = self.statement()?;

        Ok(Stmt::While(Rc::new(WhileStmt { condition, body })))
    }

    fn expression_statement(&mut self) -> Result<Rc<Stmt>, Error> {
        let value = Rc::new(self.expression()?);
        self.consume(TokenKind::Semicolon, "Expect ';' after value.")?;
        Ok(Rc::new(Stmt::Expression(Rc::new(ExpressionStmt { expression: value }))))
    }

    fn function(&mut self, kind: &str) -> Result<Rc<Stmt>, Error> {
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

        let body = Rc::new(self.block()?);
        Ok(Rc::new(Stmt::Function(Rc::new(FunctionStmt {
            name,
            params: Rc::new(params),
            body,
        }))))
    }

    fn block(&mut self) -> Result<Vec<Rc<Stmt>>, Error> {
        let mut statements: Vec<Rc<Stmt>> = Vec::new();

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
                    return Ok(Expr::Assign(Rc::new(AssignExpr {
                        name: v.name.clone(),
                        value: Rc::new(value),
                    })));
                }
                Expr::Get(g) => {
                    return Ok(Expr::Set(Rc::new(SetExpr {
                        object: Rc::clone(&g.object),
                        name: g.name.clone(),
                        value: Rc::new(value),
                    })));
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
            expr = Expr::Logical(Rc::new(LogicalExpr {
                left: Rc::new(expr),
                operator,
                right: Rc::new(right),
            }));
        }

        Ok(expr)
    }

    fn and(&mut self) -> Result<Expr, Error> {
        let mut expr = self.equality()?;

        while self.match_token(vec![TokenKind::And]) {
            let operator = self.previous();
            let right = self.equality()?;
            expr = Expr::Logical(Rc::new(LogicalExpr {
                left: Rc::new(expr),
                operator,
                right: Rc::new(right),
            }));
        }

        Ok(expr)
    }

    fn equality(&mut self) -> Result<Expr, Error> {
        let mut expr = self.comparison()?;

        while self.match_token(vec![TokenKind::NotEqual, TokenKind::EqualEqual]) {
            let operator = self.previous();
            let right = self.comparison()?;
            expr = Expr::Binary(Rc::new(BinaryExpr {
                left: Rc::new(expr),
                operator,
                right: Rc::new(right),
            }));
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
            expr = Expr::Binary(Rc::new(BinaryExpr {
                left: Rc::new(expr),
                operator,
                right: Rc::new(right),
            }));
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, Error> {
        let mut expr = self.factor()?;

        while self.match_token(vec![TokenKind::Minus, TokenKind::Plus]) {
            let operator = self.previous();
            let right = self.factor()?;
            expr = Expr::Binary(Rc::new(BinaryExpr {
                left: Rc::new(expr),
                operator,
                right: Rc::new(right),
            }));
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, Error> {
        let mut expr = self.unary()?;

        while self.match_token(vec![TokenKind::Slash, TokenKind::Asterisk]) {
            let operator = self.previous();
            let right = self.unary()?;
            expr = Expr::Binary(Rc::new(BinaryExpr {
                left: Rc::new(expr),
                operator,
                right: Rc::new(right),
            }));
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, Error> {
        if self.match_token(vec![TokenKind::Bang, TokenKind::Minus]) {
            let operator = self.previous();
            let right = self.unary()?;
            return Ok(Expr::Unary(Rc::new(UnaryExpr {
                operator,
                right: Rc::new(right),
            })));
        }

        Ok(self.call()?)
    }

    fn finish_call(&mut self, callee: Expr) -> Result<Expr, Error> {
        let mut arguments: Vec<Rc<Expr>> = Vec::new();

        if !self.check(TokenKind::RightParen) {
            loop {
                if arguments.len() >= 255 {
                    return Err(Error::parse_error(
                        &self.peek(),
                        "Can't have more than 255 arguments.",
                    ));
                }
                arguments.push(Rc::new(self.expression()?));
                if !self.match_token(vec![TokenKind::Comma]) {
                    break;
                }
            }
        }

        let paren = self.consume(TokenKind::RightParen, "Expect ')' after arguments.")?;

        Ok(Expr::Call(Rc::new(CallExpr {
            callee: Rc::new(callee),
            paren,
            arguments,
        })))
    }

    fn call(&mut self) -> Result<Expr, Error> {
        let mut expr = self.primary()?;

        loop {
            if self.match_token(vec![TokenKind::LeftParen]) {
                expr = self.finish_call(expr)?;
            }else if self.match_token(vec![TokenKind::Dot]) {
                let name = self.consume(TokenKind::Identifier, "Expect property name after '.'.")?;
                expr = Expr::Get(Rc::new(GetExpr {
                    object: Rc::new(expr),
                    name,
                }));
            }else {
                break;
            }
        }
        Ok(expr)
    }

    fn primary(&mut self) -> Result<Expr, Error> {
        if self.match_token(vec![TokenKind::False]) {
            return Ok(Expr::Literal(Rc::new(LiteralExpr {
                value: Some(Object::Bool(false)),
            })));
        }

        if self.match_token(vec![TokenKind::True]) {
            return Ok(Expr::Literal(Rc::new(LiteralExpr {
                value: Some(Object::Bool(true)),
            })));
        }

        if self.match_token(vec![TokenKind::Nil]) {
            return Ok(Expr::Literal(Rc::new(LiteralExpr {
                value: Some(Object::Nil),
            })));
        }

        if self.match_token(vec![TokenKind::Number, TokenKind::String]) {
            return Ok(Expr::Literal(Rc::new(LiteralExpr {
                value: self.previous().literal,
            })));
        }

        if self.match_token(vec![TokenKind::Super]){
            let keyword = self.previous();
            self.consume(TokenKind::Dot, "Expect . after super")?;
            let method = self.consume(TokenKind::Identifier, "Expect Superclass method name.")?;

            return Ok(Expr::Super(Rc::new(SuperExpr{
                keyword,
                method,
            })))
            
        }

        if self.match_token(vec![TokenKind::This]) {
            return Ok(Expr::This(Rc::new(ThisExpr {
                keyword: self.previous(),
            })));
        }

        if self.match_token(vec![TokenKind::Identifier]) {
            return Ok(Expr::Variable(Rc::new(VariableExpr {
                name: self.previous(),
            })));
        }

        if self.match_token(vec![TokenKind::LeftParen]) {
            let expr = self.expression()?;
            self.consume(TokenKind::RightParen, "Expect ')' after expression.")?;
            return Ok(Expr::Grouping(Rc::new(GroupingExpr {
                expression: Rc::new(expr),
            })));
        }
        
        if self.match_token(vec![TokenKind::Comment]) {
            return self.primary();
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
