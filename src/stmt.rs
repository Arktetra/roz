use crate::{lexer::Token, literal::Literal};

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Logical(Box<Expr>, Token, Box<Expr>),   // left operand, operator, right operand
    Binary(Box<Expr>, Token, Box<Expr>),    // left operand, operator, right operand
    Unary(Token, Box<Expr>),                // operator, operand
    Grouping(Box<Expr>),                    // (expression)
    Literal(Literal),                   
    Variable(Token),                        // name
    Assign(Token, Box<Expr>),               // name, value
    Call(Box<Expr>, Token, Vec<Expr>),      // callee, paren, list of argument
    None    
}

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    Expression(Expr),                       // expression
    If(Expr, Box<Stmt>, Box<Stmt>),         // condition, then branch, else branch
    While(Expr, Box<Stmt>),                 // condition, body
    Function(Token, Vec<Token>, Box<Stmt>), // name, params, body
    Return(Token, Expr),                    // keyword, value
    Print(Expr),                            // expression
    Var(Token, Expr),                       // name, initializer
    Block(Vec<Stmt>),                       // list of statement
    None
}

impl Stmt {
    pub fn get_block_body(&self) -> Option<&Vec<Stmt>> {
        match self {
            Stmt::Block(stmts) => Some(stmts),
            _ => None
        }
    }
}