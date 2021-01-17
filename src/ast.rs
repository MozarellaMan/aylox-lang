use crate::token::Token;

#[derive(Debug, Clone)]
pub enum LiteralVal {
    String(String),
    Number(f64),
}

#[derive(Debug, Clone, new)]
pub struct Binary {
    left: Box<Expr>,
    operator: Token,
    right: Box<Expr>,
}

#[derive(Debug, Clone, new)]
pub struct Grouping {
    expression: Box<Expr>,
}

#[derive(Debug, Clone, new)]
pub struct Literal {
    value: LiteralVal,
}

#[derive(Debug, Clone, new)]
pub struct Unary {
    operator: Token,
    right: Box<Expr>,
}

#[derive(Debug, Clone)]
pub enum Expr {
    Binary(Binary),
    Grouping(Grouping),
    Literal(Literal),
    Unary(Unary),
}
