use std::collections::HashMap;

use crate::lexer::Token;
use crate::literal::Literal;
use crate::interpreter::RuntimeError;

#[derive(Clone)]
pub struct Environment {
    values: HashMap<String, Literal>,
    enclosing: Option<Box<Environment>>
}

impl Environment {
    pub fn new(enclosing: Option<Environment>) -> Self {
        if let Some(enclosing) = enclosing {
            Environment {
                values: HashMap::new(),
                enclosing: Some(Box::new(enclosing))
            }
        } else {
            Environment {
                values: HashMap::new(),
                enclosing: None
            }
        }
    }

    /// This function can be used to get the enclosing environment whose values may have been changed by the current environment statements.
    pub fn get_enclosing_environment(&mut self) -> Option<Self> {
        if let Some(enclosing) = self.enclosing.clone() {
            Some(*enclosing)
        } else {
            None
        }
    }

    pub fn define(&mut self, name: String, value: Literal) {
        self.values.insert(name, value);
    }

    pub fn get(&self, name: Token) -> Result<Literal, RuntimeError> {
        if let Some(value) = self.values.get(&name.lexeme) {
            Ok(value.clone())
        } else {
            match &self.enclosing {
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
            match &mut self.enclosing {
                Some(enclosing) => {
                    // self.values.insert(name.lexeme.clone(), enclosing.get(name)?);
                    enclosing.values.insert(name.lexeme, value);
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