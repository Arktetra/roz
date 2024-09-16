use std::collections::HashMap;

use crate::{
    interpreter::{RuntimeError, RuntimeException},
    lexer::Token,
    literal::Literal,
};

#[derive(Debug, Clone)]
pub struct Environment {
    pub values: HashMap<String, Literal>,
    enclosing: Option<Box<Environment>>,
}

impl Environment {
    pub fn new(enclosing: Option<Environment>) -> Self {
        if let Some(enclosing) = enclosing {
            Environment {
                values: HashMap::new(),
                enclosing: Some(Box::new(enclosing)),
            }
        } else {
            Environment {
                values: HashMap::new(),
                enclosing: None,
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

    /// Create a binding of a name with a value.
    pub fn define(&mut self, name: String, value: Literal) {
        self.values.insert(name, value);
    }

    /// Get the value bound to a name.
    pub fn get(&self, name: Token) -> Result<Literal, RuntimeException> {
        if let Some(value) = self.values.get(&name.lexeme) {
            Ok(value.clone())
        } else {
            match &self.enclosing {
                Some(enclosing) => enclosing.get(name),
                None => {
                    let message = format!("undefined variable '{}'", name.lexeme);
                    Err(RuntimeException::Error(RuntimeError {
                        token: name,
                        message,
                    }))
                }
            }
        }
    }

    /// Assign new value to an existing name in the environment.
    pub fn assign(&mut self, name: Token, value: Literal) -> Result<(), RuntimeException> {
        if self.values.contains_key(&name.lexeme) {
            self.values.insert(name.lexeme, value);
            Ok(())
        } else {
            match &mut self.enclosing {
                Some(enclosing) => {
                    // self.values.insert(name.lexeme.clone(), enclosing.get(name)?);
                    // enclosing.values.insert(name.lexeme, value);
                    enclosing.assign(name, value)?;
                    Ok(())
                }
                None => {
                    let message = format!("undefined variable '{}'", name.lexeme);
                    Err(RuntimeException::Error(RuntimeError {
                        token: name,
                        message,
                    }))
                }
            }
        }
    }

    pub fn display(&self) {
        for (string, literal) in self.values.clone() {
            println!("{} => {}", string, literal.to_string());
        }

        println!("___________________________");

        match &self.enclosing {
            Some(enclosing) => {
                enclosing.display();
            }
            None => ()
        }
    }
}
