use crate::{
    callable::Callable,
    environment::Environment,
    function::Function,
    lexer::{Token, TokenType},
    literal::Literal,
    stmt::{Expr, Stmt},
};

#[derive(Debug)]
pub struct RuntimeError {
    pub token: Token,
    pub message: String,
}

pub struct Interpreter {
    pub globals: Environment,
    environment: Environment,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            globals: Environment::new(None),
            environment: Environment::new(None),
        }
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
            (a, b) => a == b,
        }
    }

    fn visit_literal_expr(&mut self, literal: &Literal) -> Result<Literal, RuntimeError> {
        Ok(literal.clone())
    }

    fn visit_grouping_expr(&mut self, expr: &Box<Expr>) -> Result<Literal, RuntimeError> {
        self.evaluate(expr)
    }

    fn visit_unary_expr(&mut self, operator: &Token, expr: &Expr) -> Result<Literal, RuntimeError> {
        let right = self.evaluate(expr)?;

        self.check_number_operand(operator, &right)?;

        match operator.token_type {
            TokenType::Minus => Ok((-right).unwrap()),
            TokenType::Plus => Ok(right),
            TokenType::Bang => Ok(Literal::Bool(!self.is_true(&right))),
            _ => Ok(Literal::Null),
        }
    }

    fn visit_call_expr(
        &mut self,
        callee: &Expr,
        paren: Token,
        arguments: &[Expr],
    ) -> Result<Literal, RuntimeError> {
        let callee = self.evaluate(callee)?;

        let mut arguments_ = Vec::new();

        for argument in arguments {
            arguments_.push(self.evaluate(&argument)?)
        }

        if callee.is_string() {
            return Err(RuntimeError {
                token: paren.clone(),
                message: "Can only call functions and classes.".to_string(),
            });
        }

        match callee {
            Literal::Function(function) => {
                if arguments_.len() != function.arity() {
                    return Err(RuntimeError {token: paren, message: format!("Expected {} arguments but got {}.", function.arity(), arguments_.len())});
                }

                self.environment.define(paren.lexeme, Literal::Function(function.clone()));
                function.call(self, arguments_);
                Ok(Literal::Null)
            }
            _ => Err(RuntimeError{ token: paren, message: "Couldn't execute function.".to_string()})
        }
    }

    fn visit_logical_expr(
        &mut self,
        left: &Expr,
        operator: &Token,
        right: &Expr,
    ) -> Result<Literal, RuntimeError> {
        let left = self.evaluate(left)?;

        if operator.token_type == TokenType::And {
            if !self.is_true(&left) {
                return Ok(left);
            }
        } else {
            if self.is_true(&left) {
                return Ok(left);
            }
        }

        self.evaluate(right)
    }

    fn visit_binary_expr(
        &mut self,
        left: &Expr,
        operator: &Token,
        right: &Expr,
    ) -> Result<Literal, RuntimeError> {
        let left = self.evaluate(left)?;
        let right = self.evaluate(right)?;

        match operator.token_type {
            TokenType::Minus => {
                self.check_number_operands(&left, operator, &right)?;
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
            TokenType::Greater => Ok(Literal::Bool(left > right)),
            TokenType::Less => Ok(Literal::Bool(left < right)),
            TokenType::GreaterEqual => Ok(Literal::Bool(left >= right)),
            TokenType::LessEqual => Ok(Literal::Bool(left <= right)),
            TokenType::EqualEqual => Ok(Literal::Bool(self.is_equal(&left, &right))),
            TokenType::BangEqual => Ok(Literal::Bool(!self.is_equal(&left, &right))),
            _ => Ok(Literal::Null),
        }
    }

    fn visit_variable_expr(&mut self, name: &Token) -> Result<Literal, RuntimeError> {
        self.environment.get(name.clone())
    }

    fn visit_expr_stmt(&mut self, expr: &Expr) -> Result<(), RuntimeError> {
        self.evaluate(expr)?;
        Ok(())
    }

    fn visit_print_stmt(&mut self, expr: &Expr) -> Result<(), RuntimeError> {
        let value = self.evaluate(expr)?;
        println!("{}", value.to_string());
        Ok(())
    }

    fn visit_var_stmt(&mut self, name: &Token, initializer: &Expr) -> Result<(), RuntimeError> {
        let mut value = Literal::Null;

        if *initializer != Expr::Literal(Literal::Null) {
            value = self.evaluate(initializer)?;
        }

        self.environment
            .define(name.lexeme.clone(), value);

        Ok(())
    }

    fn visit_if_stmt(
        &mut self,
        condition: &Expr,
        then_stmt: &Stmt,
        else_stmt: &Stmt,
    ) -> Result<(), RuntimeError> {
        let cond_eval_result = self.evaluate(condition)?;

        if self.is_true(&cond_eval_result) {
            self.execute(then_stmt)?;
        } else if *else_stmt != Stmt::None {
            self.execute(else_stmt)?;
        }

        Ok(())
    }

    fn visit_while_stmt(&mut self, condition: &Expr, body: &Stmt) -> Result<(), RuntimeError> {
        let mut cond_eval_result = self.evaluate(condition)?;

        while self.is_true(&cond_eval_result) {
            self.execute(body)?;
            cond_eval_result = self.evaluate(condition)?;
        }

        Ok(())
    }

    fn visit_block_stmt(&mut self, stmts: &[Stmt]) -> Result<(), RuntimeError> {
        let env = self.environment.clone();
        self.execute_block(&stmts, Environment::new(Some(env)))
    }

    fn visit_function_stmt(&mut self, name: &Token, parameters: &[Token], body: Stmt) -> Result<(), RuntimeError> {
        let function = Function::new(name.clone(), parameters, body);

        self.environment.define(name.lexeme.clone(), Literal::Function(Box::new(function)));

        Ok(())
    }

    fn check_number_operand(
        &self,
        operator: &Token,
        operand: &Literal,
    ) -> Result<(), RuntimeError> {
        if operand.is_double() {
            return Ok(());
        } else {
            return Err(RuntimeError {
                token: operator.clone(),
                message: "Expected the operand to be a double.".to_string(),
            });
        }
    }

    fn check_number_operands(
        &self,
        left: &Literal,
        operator: &Token,
        right: &Literal,
    ) -> Result<(), RuntimeError> {
        if left.is_double() && right.is_double() {
            return Ok(());
        } else {
            return Err(RuntimeError {
                token: operator.clone(),
                message: "Expected both operands to be double.".to_string(),
            });
        }
    }

    pub fn execute_block(
        &mut self,
        stmts: &[Stmt],
        environment: Environment,
    ) -> Result<(), RuntimeError> {
        self.environment = environment;
        for stmt in stmts {
            self.execute(stmt)?;
        }

        self.environment = self.environment.get_enclosing_environment().unwrap();
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
            Expr::Literal(ref literal) => self.visit_literal_expr(literal),
            Expr::Grouping(group) => self.visit_grouping_expr(group),
            Expr::Unary(operator, expr) => self.visit_unary_expr(operator, expr),
            Expr::Logical(lhs, operator, rhs) => self.visit_logical_expr(lhs, operator, rhs),
            Expr::Binary(lhs, operator, rhs) => self.visit_binary_expr(lhs, operator, rhs),
            Expr::Variable(name) => self.visit_variable_expr(name),
            Expr::Assign(name, rhs) => {
                let value = self.evaluate(rhs)?;
                self.environment
                    .assign(name.clone(), value.clone())?;
                Ok(value)
            }
            Expr::Call(callee, paren, arguments) => {
                self.visit_call_expr(callee, paren.clone(), arguments)
            }
            Expr::None => Ok(Literal::Null),
        }
    }

    fn walk_expr(&mut self, expr: &Expr) -> Result<Literal, RuntimeError> {
        self.visit_expr(expr)
    }

    fn visit_stmt(&mut self, stmt: &Stmt) -> Result<(), RuntimeError> {
        match stmt {
            Stmt::Expression(expr) => self.visit_expr_stmt(expr),
            Stmt::Print(expr) => self.visit_print_stmt(expr),
            Stmt::If(condition, then_statement, else_statement) => {
                self.visit_if_stmt(condition, then_statement, else_statement)
            }
            Stmt::While(condition, body) => self.visit_while_stmt(condition, body),
            Stmt::Var(name, initializer) => self.visit_var_stmt(name, initializer),
            Stmt::Block(stmts) => self.visit_block_stmt(stmts),
            Stmt::Function(name, parameters, body) => self.visit_function_stmt(name, parameters, *body.clone()),
            Stmt::None => Ok(()),
        }
    }

    fn walk_stmt(&mut self, stmt: &Stmt) -> Result<(), RuntimeError> {
        self.visit_stmt(stmt)?;
        Ok(())
    }
}
