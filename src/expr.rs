use crate::tokens::Token;
use crate::tokens::TokenKind;
pub struct Binary {
    pub left: Box<Expr>,
    pub operator: Token,
    pub right: Box<Expr>,
}
impl Binary {
    pub fn new(left: Box<Expr> , operator: Token , right: Box<Expr> ) -> Self {
        left,
        operator,
        right,
    }
}
pub struct Grouping {
    pub expression: Box<Expr>,
}
impl Grouping {
    pub fn new(expression: Box<Expr> ) -> Self {
        expression,
    }
}
pub struct Literal {
    pub value: i64,
}
impl Literal {
    pub fn new(value: i64 ) -> Self {
        value,
    }
}
pub struct Unary {
    pub operator: Token,
    pub right: Box<Expr>,
}
impl Unary {
    pub fn new(operator: Token , right: Box<Expr> ) -> Self {
        operator,
        right,
    }
}
