use crate::{expr::Expr, lexer::TokenType, literal::Literal};

pub struct Interpreter;

pub trait Visitor {
    fn visit_expr(&mut self, expr: &Expr) -> Literal;
    fn walk_expr(&mut self, expr: &Expr) -> Literal;
}

impl Visitor for Interpreter {
    fn visit_expr(&mut self, expr: &Expr) -> Literal {
        match expr {
            Expr::Literal(ref literal) => {
                *literal.clone()
            }
            Expr::Grouping(group) => {
                return self.visit_expr(&group);
            }
            Expr::Unary(operator, expr) => {
                match operator.token_type {
                    TokenType::Plus => self.visit_expr(&expr),
                    TokenType::Minus => (- self.visit_expr(&expr)).unwrap(),
                    _ => Literal::Null,
                }
            }
            Expr::Binary(lhs, operator, rhs ) => {
                match operator.token_type {
                    TokenType::Plus => (self.visit_expr(&lhs) + self.visit_expr(&rhs)).unwrap(),
                    TokenType::Minus => (self.visit_expr(&lhs) - self.visit_expr(&rhs)).unwrap(),
                    TokenType::Star => (self.visit_expr(&lhs) * self.visit_expr(&rhs)).unwrap(),
                    TokenType::Slash => (self.visit_expr(&lhs) / self.visit_expr(&rhs)).unwrap(),
                    _ => Literal::Null,
                }
            }
        }
    }

    fn walk_expr(&mut self, expr: &Expr) -> Literal {
        match expr {
            Expr::Binary(_, _, _) => self.visit_expr(expr),
            Expr::Unary(_, _) => self.visit_expr(expr),
            Expr::Grouping(_) => self.visit_expr(expr),
            Expr::Literal(_) => self.visit_expr(expr)
        }
    }
}

#[cfg(test)]
mod  tests {
    use super::*;
    use crate::parser::Parser;
    use crate::lexer::Lexer;

    #[test]
    fn arithmetic() {
        let input = "(5 - 2) * 5 / 3".to_string();

        let mut lexer = Lexer::new(&input);
        lexer.scan_tokens();

        let mut parser = Parser::new(lexer.tokens);

        let expr = parser.expression();
        let mut interpreter = Interpreter;

        assert_eq!(Literal::Number(5.0), interpreter.walk_expr(&expr));
    }
}