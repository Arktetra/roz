use crate::lexer::{Token, Literal};

pub enum Expr {
    Binary(Box<Expr>, Box<Token>, Box<Expr>),
    Unary(Box<Token>, Box<Expr>),
    Grouping(Box<Expr>),
    Literal(Box<Literal>)
}

pub trait Visitor<T> {
    fn visit_expr(&mut self, expr: &Expr) -> T;
    fn walk_expr(&mut self, expr: &Expr) ->T;
}

pub struct AstPrinter;

impl Visitor<String> for AstPrinter {
    fn visit_expr(&mut self, expr: &Expr) -> String {
        match *expr {
            Expr::Binary(ref left, ref operator, ref right) => {
                return "(".to_string() + &operator.lexeme + " " + &self.visit_expr(left) + " " + &self.visit_expr(right) + ")";
            }
            Expr::Unary(ref operator, ref expr) => {
                return "(".to_string() + &operator.lexeme + " " + &self.visit_expr(expr) + ")";
            }
            Expr::Grouping(ref expr) => {
                return "(group ".to_string() + &self.visit_expr(expr) + ")";
            }
            Expr::Literal(ref literal) => {
                match *literal.clone() {
                    Literal::Number(x) => x.to_string(),
                    Literal::String(x) => x,
                    Literal::Null => "null".to_string()
                }
            }
        }
    }

    fn walk_expr(&mut self, expr: &Expr) -> String{
        match *expr {
            Expr::Binary(_, _, _) => {
                self.visit_expr(expr)
            }
            Expr::Unary(_, _) => {
                self.visit_expr(expr)
            }
            Expr::Literal(_) => {
                self.visit_expr(expr)
            }
            Expr::Grouping(_) => {
                self.visit_expr(expr)
            }
        }
    }
}