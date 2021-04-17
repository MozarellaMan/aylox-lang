use crate::{ast::AloxObject, error::RuntimeException, functions::Callable, token::Token};
use std::{cell::RefCell, collections::HashMap, rc::Rc};

#[derive(Debug, Clone)]
pub struct Environment {
    enclosing: Option<Rc<RefCell<Environment>>>,
    values: HashMap<String, EnvValue>,
}

#[derive(Clone, Debug)]
enum EnvValue {
    Object(Rc<Option<AloxObject>>),
    Function(Rc<dyn Callable>),
    Empty
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
       self.insert_env_value(name, value);
    }

    fn insert_env_value(&mut self, name: &str, value: Option<AloxObject>) {
        if let Some(value) = value {
            if let AloxObject::Function(func) = value {
                self.values.insert(name.to_string(), EnvValue::Function(func));
            } else {
                self.values.insert(name.to_string(), EnvValue::Object(Rc::new(Some(value))));
            }
        } else {
            self.values.insert(name.to_string(), EnvValue::Empty);
        }
    }

    pub fn assign(&mut self, name: &Token, value: Option<AloxObject>) -> Result<(), RuntimeException> {
        if self.values.contains_key(&name.lexeme) {
            self.insert_env_value(&name.lexeme, value);
            Ok(())
        } else {
            if let Some(enclosing) = &self.enclosing {
                return enclosing.borrow_mut().assign(name, value);
            }
            Err(RuntimeException::UndefinedVariable {
                lexeme: name.lexeme.clone(),
                line: name.line,
            })
        }
    }

    pub fn get(&self, name: &Token) -> Result<Rc<Option<AloxObject>>, RuntimeException> {
        let result =
            self.values
                .get(&name.lexeme)
                .cloned()
                .ok_or(RuntimeException::UndefinedVariable {
                    lexeme: name.lexeme.clone(),
                    line: name.line,
                });

        match result {
            Ok(res) => {
                if let EnvValue::Empty = res {
                    Err(RuntimeException::NilAccess {
                        line: name.line,
                        lexeme: name.lexeme.clone(),
                    })
                } else {
                    let res: Rc<Option<AloxObject>> = Environment::env_value_to_obj(res);
                    Ok(res)
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

    fn env_value_to_obj(val: EnvValue) -> Rc<Option<AloxObject>> {
        match val {
            EnvValue::Object(obj) => obj,
            EnvValue::Function(func) => Rc::new(Some(AloxObject::Function(func))),
            EnvValue::Empty => Rc::new(None)
        }
    }
}

impl Default for Environment {
    fn default() -> Self {
        Self::new()
    }
}
