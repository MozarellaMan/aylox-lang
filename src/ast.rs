use crate::token::Token;

#[derive(Debug, Copy, Clone, new)]
pub struct Nil;

#[derive(Debug, Clone)]
pub enum LiteralVal {
    String(String),
    Number(f64),
    Nil(Nil),
}

#[derive(Debug, Clone, new)]
pub struct Binary {
    pub left: Box<Expr>,
    pub operator: Token,
    pub right: Box<Expr>,
}

#[derive(Debug, Clone, new)]
pub struct Grouping {
    pub expression: Box<Expr>,
}

#[derive(Debug, Clone, new)]
pub struct Literal {
    pub value: LiteralVal,
}

#[derive(Debug, Clone, new)]
pub struct Unary {
    pub operator: Token,
    pub right: Box<Expr>,
}

#[derive(Debug, Clone)]
pub enum Expr {
    Binary(Binary),
    Grouping(Grouping),
    Literal(Literal),
    Unary(Unary),
}

pub trait Visitor<T> {
    fn visit_binary(&mut self, binary: &Binary) -> T;

    fn visit_grouping(&mut self, grouping: &Grouping) -> T;

    fn visit_literal(&mut self, literal: &Literal) -> T;

    fn visit_unary(&mut self, unary: &Unary) -> T;

    fn visit_expr(&mut self, expr: &Expr) -> T;
}
