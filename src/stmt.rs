use crate::{lexer::Token, literal::Literal};

#[derive(PartialEq)]
pub enum Expr {
    Binary(Box<Expr>, Box<Token>, Box<Expr>),
    Unary(Box<Token>, Box<Expr>),
    Grouping(Box<Expr>),
    Literal(Box<Literal>),
    Variable(Token),
    Assign(Token, Box<Expr>)
}

pub enum Stmt {
    Expression(Expr),
    Print(Expr),
    Var(Token, Expr),
    Block(Vec<Stmt>)
}