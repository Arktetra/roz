use crate::{
    environment::Environment,
    lexer::{TokenType, Token}, 
    literal::Literal,
    stmt::{Stmt, Expr}
};

#[derive(Debug)]
pub struct RuntimeError {
    pub token: Token,
    pub message: String
}

pub struct Interpreter {
    environment: Environment
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter { environment: Environment::new(None) }
    }

    fn evaluate(&mut self, expr: &Expr) -> Result<Literal, RuntimeError> {
        self.walk_expr(expr)
    }

    pub fn interpret(&mut self, stmts: &[Stmt]) -> Result<(), RuntimeError> {
        for stmt in stmts {
            self.execute(stmt)?;
        }
        Ok(())
    }

    fn execute(&mut self, stmt: &Stmt) -> Result<(), RuntimeError> {
        self.walk_stmt(stmt)?;
        Ok(())
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
                // self.check_number_operands(&left, operator, &right)?;
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
                Ok(Literal::Bool(left > right))
            }
            TokenType::Less => {
                Ok(Literal::Bool(left < right))
            }
            TokenType::GreaterEqual => {
                Ok(Literal::Bool(left >= right))
            }
            TokenType::LessEqual => {
                Ok(Literal::Bool(left <= right))
            }
            TokenType::EqualEqual => {
                Ok(Literal::Bool(self.is_equal(&left, &right)))
            }
            TokenType::BangEqual => {
                Ok(Literal::Bool(!self.is_equal(&left, &right)))
            }
            _ => Ok(Literal::Null)
        }
    }

    fn visit_variable_expr(&mut self, name: &Token) -> Result<Literal, RuntimeError> {
        self.environment.get(name.clone())
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

    fn execute_block(&mut self, stmts: &[Stmt], environment: Environment) -> Result<(), RuntimeError> {
        let previous = self.environment.clone();

        self.environment = environment;
        for stmt in stmts {
            self.execute(stmt)?;
        }

        self.environment = previous;
        Ok(())
    }
}

pub trait Visitor {
    fn visit_expr(&mut self, expr: &Expr) -> Result<Literal, RuntimeError>;
    fn walk_expr(&mut self, expr: &Expr) -> Result<Literal, RuntimeError>;
    fn visit_stmt(&mut self, stmt: &Stmt) -> Result<(), RuntimeError>;
    fn walk_stmt(&mut self, stmt: &Stmt) -> Result<(), RuntimeError>;
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
            Expr::Variable(name) => {
                self.visit_variable_expr(name)
            }
            Expr::Assign(name, rhs) => {
                let value = self.evaluate(rhs)?;
                self.environment.assign(name.clone(), value.clone())?;
                Ok(value)
            }
        }
    }

    fn walk_expr(&mut self, expr: &Expr) -> Result<Literal, RuntimeError> {
        self.visit_expr(expr)
    }

    fn visit_stmt(&mut self, stmt: &Stmt) -> Result<(), RuntimeError> {
        match stmt {
            Stmt::Expression(expr) => {
                self.evaluate(expr)?;
                Ok(())
            }
            Stmt::Print(expr) => {
                let value = self.evaluate(expr)?;
                println!("{}", value.to_string());
                Ok(())
            }
            Stmt::Var(name, initializer) => {
                let mut value = Literal::Null;

                if *initializer != Expr::Literal(Box::new(Literal::Null)) {
                    value = self.evaluate(initializer)?;
                }

                self.environment.define(name.lexeme.clone(), value);

                Ok(())
            }
            Stmt::Block(stmts) => self.execute_block(stmts, Environment::new(Some(self.environment.clone())))
        }
    }

    fn walk_stmt(&mut self, stmt: &Stmt) -> Result<(), RuntimeError> {
        self.visit_stmt(stmt)?;
        Ok(())
    }
}