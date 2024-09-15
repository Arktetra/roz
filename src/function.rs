use crate::{
    callable::Callable, environment::Environment, interpreter::Interpreter, lexer::Token,
    literal::Literal, stmt::Stmt,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Function {
    name: Token,
    parameters: Vec<Token>,
    body: Stmt,
}

impl Function {
    pub fn new(name: Token, parameters: &[Token], body: Stmt) -> Self {
        Function {
            name,
            parameters: parameters.to_vec(),
            body,
        }
    }

    pub fn name(&self) -> String {
        self.name.lexeme.clone()
    }
}

impl Callable for Function {
    fn arity(&self) -> usize {
        self.parameters.len()
    }

    fn call(&self, interpreter: &mut Interpreter, arguments: Vec<Literal>) -> () {
        let mut environment = Environment::new(Some(interpreter.globals.clone()));

        for i in 0..self.parameters.len() {
            environment.define(self.parameters[i].lexeme.clone(), arguments[i].clone());
        }

        interpreter
            .execute_block(self.body.get_block_body().unwrap(), environment)
            .unwrap()
    }
}
