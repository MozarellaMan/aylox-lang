use crate::{ast::AloxObject, error::RuntimeError, token::Token};
use std::{cell::RefCell, collections::HashMap, rc::Rc};

pub struct Environment {
    enclosing: Option<Rc<RefCell<Environment>>>,
    values: HashMap<String, Rc<Option<AloxObject>>>,
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

    pub fn define(&mut self, name: &str, value: Option<AloxObject>) {
        self.values.insert(name.to_string(), Rc::new(value));
    }

    pub fn assign(&mut self, name: &Token, value: Option<AloxObject>) -> Result<(), RuntimeError> {
        if self.values.contains_key(&name.lexeme) {
            self.values.insert(name.lexeme.clone(), Rc::new(value));
            Ok(())
        } else {
            if let Some(enclosing) = &self.enclosing {
                return enclosing.borrow_mut().assign(name, value);
            }
            Err(RuntimeError::UndefinedVariable {
                lexeme: name.lexeme.clone(),
                line: name.line,
            })
        }
    }

    pub fn get(&self, name: &Token) -> Result<Rc<Option<AloxObject>>, RuntimeError> {
        let result =
            self.values
                .get(&name.lexeme)
                .cloned()
                .ok_or(RuntimeError::UndefinedVariable {
                    lexeme: name.lexeme.clone(),
                    line: name.line,
                });

        match result {
            Ok(res) => {
                if res.is_some() {
                    Ok(res)
                } else {
                    Err(RuntimeError::NilAccess {
                        line: name.line,
                        lexeme: name.lexeme.clone(),
                    })
                }
            }
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
