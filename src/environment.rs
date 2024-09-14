use std::collections::HashMap;

use crate::lexer::Token;
use crate::literal::Literal;
use crate::interpreter::RuntimeError;

pub struct Environment {
    values: HashMap<String, Literal>
}

impl Environment {
    pub fn new() -> Self {
        Environment { values: HashMap::new() }
    }

    pub fn define(&mut self, name: String, value: Literal) {
        self.values.insert(name, value);
    }

    pub fn get(&mut self, name: Token) -> Result<Literal, RuntimeError> {
        if let Some(value) = self.values.get(&name.lexeme) {
            Ok(value.clone())
        } else {
            let message = format!("undefined variable name'{}'", name.lexeme);
            Err(RuntimeError{token: name, message: message})
        }
    }

    pub fn assign(&mut self, name: Token, value: Literal) -> Result<(), RuntimeError> {
        if self.values.contains_key(&name.lexeme) {
            self.values.insert(name.lexeme, value);
            Ok(())
        } else {
            let message = format!("undefined variable '{}'", name.lexeme);
            Err(RuntimeError { token: name, message })
        }
    }
}