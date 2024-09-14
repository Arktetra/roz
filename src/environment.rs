use std::collections::HashMap;

use crate::lexer::Token;
use crate::literal::Literal;
use crate::interpreter::RuntimeError;

#[derive(Clone)]
pub struct Environment {
    values: HashMap<String, Literal>,
    enclosing: Box<Option<Environment>>
}

impl Environment {
    pub fn new(enclosing: Option<Environment>) -> Self {
        Environment { 
            values: HashMap::new(),
            enclosing: Box::new(enclosing)
        }
    }

    pub fn define(&mut self, name: String, value: Literal) {
        self.values.insert(name, value);
    }

    pub fn get(&self, name: Token) -> Result<Literal, RuntimeError> {
        if let Some(value) = self.values.get(&name.lexeme) {
            Ok(value.clone())
        } else {
            match *self.enclosing.clone() {
                Some(enclosing) => enclosing.get(name),
                None => {
                    let message = format!("undefined variable '{}'", name.lexeme);
                    Err(RuntimeError{token: name, message: message})
                }
            }
        }
    }

    pub fn assign(&mut self, name: Token, value: Literal) -> Result<(), RuntimeError> {
        if self.values.contains_key(&name.lexeme) {
            self.values.insert(name.lexeme, value);
            Ok(())
        } else {
            match *self.enclosing.clone() {
                Some(enclosing) => {
                    self.values.insert(name.lexeme.clone(), enclosing.get(name)?);
                    Ok(())
                },
                None => {
                    let message = format!("undefined variable '{}'", name.lexeme);
                    Err(RuntimeError{ token: name, message })
                }
            }
        }
    }
}