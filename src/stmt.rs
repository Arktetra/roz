use crate::{lexer::Token, literal::Literal};

#[derive(PartialEq)]
pub enum Expr {
    Logical(Box<Expr>, Token, Box<Expr>),
    Binary(Box<Expr>, Token, Box<Expr>),
    Unary(Token, Box<Expr>),
    Grouping(Box<Expr>),
    Literal(Literal),
    Variable(Token),
    Assign(Token, Box<Expr>)
}

#[derive(PartialEq)]
pub enum Stmt {
    Expression(Expr),
    If(Expr, Box<Stmt>, Box<Stmt>),
    While(Expr, Box<Stmt>),
    Print(Expr),
    Var(Token, Expr),
    Block(Vec<Stmt>),
    None
}