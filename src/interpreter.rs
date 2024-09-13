use crate::{
    lexer::{TokenType, Token}, 
    literal::Literal,
    expr::Expr, 
};

#[derive(Debug)]
pub struct RuntimeError {
    pub token: Token,
    pub message: String
}

pub struct Interpreter;

impl Interpreter {
    fn evaluate(&mut self, expr: &Expr) -> Result<Literal, RuntimeError> {
        self.walk_expr(expr)
    }

    pub fn interpret(&mut self, expr: Expr) -> Result<String, RuntimeError> {
        let result = self.evaluate(&expr);

        match result {
            Ok(Literal::Number(x)) => Ok(x.to_string()),
            Ok(Literal::String(x)) => Ok(x),
            Ok(Literal::Bool(x)) => Ok(x.to_string()),
            Ok(Literal::Null) => Ok("Nil".to_string()),
            Err(error) => Err(error)
        }
    }

    fn is_true(&self, value: &Literal) -> bool {
        match value {
            Literal::Null => false,
            Literal::Bool(x) => *x,
            _ => true,
        }
    }

    fn is_equal(&self, left: &Literal, right: &Literal) -> bool {
        match (left, right) {
            (Literal::Null, Literal::Null) => false,
            (Literal::Null, _) => false,
            (a, b) => a == b
        }
    }

    fn visit_literal_expr(&mut self, literal: &Box<Literal>) -> Result<Literal, RuntimeError> {
        Ok(*literal.clone())
    }

    fn visit_grouping_expr(&mut self, expr: &Box<Expr>) -> Result<Literal, RuntimeError> {
        self.evaluate(expr)
    }

    fn visit_unary_expr(&mut self, operator: &Box<Token>, expr: &Box<Expr>) -> Result<Literal, RuntimeError> {
        let right = self.evaluate(expr)?;

        self.check_number_operand(operator, &right)?;

        match operator.token_type {
            TokenType::Minus => Ok((- right).unwrap()),
            TokenType::Plus => Ok(right),
            TokenType::Bang => Ok(Literal::Bool(!self.is_true(&right))),
            _ => Ok(Literal::Null)
        }
    }

    fn visit_binary_expr(&mut self, left: &Box<Expr>, operator: &Box<Token>, right: &Box<Expr>) -> Result<Literal, RuntimeError> {
        let left = self.evaluate(left)?;
        let right = self.evaluate(right)?;

        match operator.token_type {
            TokenType::Minus => {
                self.check_number_operands(&left, operator,  &right)?;
                Ok((left - right).unwrap())
            }
            TokenType::Plus => {
                self.check_number_operands(&left, operator, &right)?;
                Ok((left + right).unwrap())
            }
            TokenType::Star => {
                self.check_number_operands(&left, operator, &right)?;
                Ok((left * right).unwrap())
            }
            TokenType::Slash => {
                self.check_number_operands(&left, operator, &right)?;
                Ok((left / right).unwrap())
            }
            TokenType::Greater => {
                self.check_number_operands(&left, operator, &right)?;
                Ok(Literal::Bool(left > right))
            }
            TokenType::Less => {
                self.check_number_operands(&left, operator, &right)?;
                Ok(Literal::Bool(left < right))
            }
            TokenType::GreaterEqual => {
                self.check_number_operands(&left, operator, &right)?;
                Ok(Literal::Bool(left >= right))
            }
            TokenType::LessEqual => {
                self.check_number_operands(&left, operator, &right)?;
                Ok(Literal::Bool(left <= right))
            }
            TokenType::EqualEqual => {
                self.check_number_operands(&left, operator, &right)?;
                Ok(Literal::Bool(self.is_equal(&left, &right)))
            }
            TokenType::BangEqual => {
                self.check_number_operands(&left, operator, &right)?;
                Ok(Literal::Bool(!self.is_equal(&left, &right)))
            }
            _ => Ok(Literal::Null)
        }
    }

    fn check_number_operand(&self, operator: &Token, operand: &Literal) -> Result<(), RuntimeError> {
        if operand.is_double() {
            return Ok(())
        } else {
            return Err(RuntimeError{token: operator.clone(), message: "Expected the operand to be a double.".to_string()})
        }
    }

    fn check_number_operands(&self, left: &Literal, operator: &Token, right: &Literal) -> Result<(), RuntimeError> {
        if left.is_double() && right.is_double() { 
            return Ok(());
        } else {
            return Err(RuntimeError{token: operator.clone(), message: "Expected both operands to be double.".to_string()});
        }
    }
}

pub trait Visitor {
    fn visit_expr(&mut self, expr: &Expr) -> Result<Literal, RuntimeError>;
    fn walk_expr(&mut self, expr: &Expr) -> Result<Literal, RuntimeError>;
}

impl Visitor for Interpreter {
    fn visit_expr(&mut self, expr: &Expr) -> Result<Literal, RuntimeError> {
        match expr {
            Expr::Literal(ref literal) => {
                self.visit_literal_expr(literal)
            }
            Expr::Grouping(group) => {
                self.visit_grouping_expr(group)
            }
            Expr::Unary(operator, expr) => {
                self.visit_unary_expr(operator, expr)
            }
            Expr::Binary(lhs, operator, rhs ) => {
                self.visit_binary_expr(lhs, operator, rhs)
            }
        }
    }

    fn walk_expr(&mut self, expr: &Expr) -> Result<Literal, RuntimeError> {
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

        let expr = parser.parse().unwrap();
        let mut interpreter = Interpreter;

        assert_eq!(Literal::Number(5.0), interpreter.walk_expr(&expr).unwrap());
    }
}