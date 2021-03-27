use crate::{ast::Expr, error::RuntimeError, token::Token};
use std::{cell::RefCell, collections::HashMap, rc::Rc};

pub struct Environment {
    enclosing: Option<Rc<RefCell<Environment>>>,
    values: HashMap<String, Rc<Expr>>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
            enclosing: None,
        }
    }

    pub fn with_enclosing(enclosing: Rc<RefCell<Environment>>) -> Self {
        Self {
            values: HashMap::new(),
            enclosing: Some(enclosing),
        }
    }

    pub fn define(&mut self, name: String, value: Rc<Expr>) {
        self.values.insert(name, value);
    }

    pub fn assign(&mut self, name: &Token, value: Rc<Expr>) -> Result<(), RuntimeError> {
        if self.values.contains_key(&name.lexeme) {
            self.values.insert(name.lexeme.clone(), value);
            Ok(())
        } else {
            if let Some(enclosing) = &self.enclosing {
                return enclosing.borrow_mut().assign(name, value);
            }
            Err(RuntimeError::UndefinedVariable {
                lexeme: name.lexeme.clone(),
            })
        }
    }

    pub fn get(&self, name: &Token) -> Result<Rc<Expr>, RuntimeError> {
        let result =
            self.values
                .get(&name.lexeme)
                .cloned()
                .ok_or(RuntimeError::UndefinedVariable {
                    lexeme: name.lexeme.clone(),
                });

        match result {
            Ok(res) => Ok(res),
            Err(err) => match &self.enclosing {
                Some(enclosing) => {
                    let enclosing = enclosing.borrow();
                    let result = enclosing.get(name)?;
                    Ok(result)
                }
                None => Err(err),
            },
        }
    }
}

impl Default for Environment {
    fn default() -> Self {
        Self::new()
    }
}
