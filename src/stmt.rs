use crate::{lexer::Token, literal::Literal};

pub enum Expr {
    Binary(Box<Expr>, Box<Token>, Box<Expr>),
    Unary(Box<Token>, Box<Expr>),
    Grouping(Box<Expr>),
    Literal(Box<Literal>)
}

pub enum Stmt {
    Expression(Expr),
    Print(Expr)
}

pub trait Visitor<T> {
    fn visit_expr(&mut self, expr: &Expr) -> T;
    fn walk_expr(&mut self, expr: &Expr) ->T;
}

pub struct AstPrinter;

impl AstPrinter {
    pub fn print(&mut self, expr: &Expr) -> String {
        self.walk_expr(expr)
    }
}

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
                    Literal::Bool(x) => x.to_string(),
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

#[cfg(test)]
mod tests {
    use crate::{lexer::Lexer, parser::Parser};

    use super::*;

    #[test]
    fn arithmetic() {
        let input = "- 2 * 2 * 2 / 2".to_string();

        let mut lexer = Lexer::new(&input);
        lexer.scan_tokens();
        let mut parser = Parser::new(lexer.tokens);

        let mut printer = AstPrinter;
        
        assert_eq!("(- (* 2 (* 2 (/ 2 2))))", printer.walk_expr(&parser.expression().unwrap()));
    }
}