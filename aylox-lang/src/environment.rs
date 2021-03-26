use crate::{ast::Expr, error::RuntimeError, token::Token};
use std::collections::HashMap;

pub struct Environment {
    values: HashMap<String, Box<Expr>>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
        }
    }

    pub fn define(&mut self, name: String, value: Box<Expr>) {
        self.values.insert(name, value);
    }

    pub fn get(&self, name: &Token) -> Result<&Box<Expr>, RuntimeError> {
        let result = self
            .values
            .get(&name.lexeme)
            .ok_or(RuntimeError::UndefinedVariable {
                lexeme: name.lexeme.clone(),
            });
        result
    }
}

impl Default for Environment {
    fn default() -> Self {
        Self::new()
    }
}
