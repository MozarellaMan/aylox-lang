use core::fmt::Debug;
use std::fmt::Display;

use crate::{ast::*, environment::Environment, interpreter::Interpreter};

pub trait Callable {
    fn needs_mut(&self) -> bool;
    fn arity(&self) -> usize;
    fn call(&self, interpreter: &Interpreter, args: &[AloxObject]) -> AloxObjResult;
    fn call_mut(&self, interpreter: &mut Interpreter, args: &[AloxObject]) -> AloxObjResult;
}

#[derive(new, Clone, Debug)]
pub struct AloxFunction {
    declaration: Function,
}

impl Callable for AloxFunction {
    fn needs_mut(&self) -> bool {
        true
    }
    fn arity(&self) -> usize {
        self.declaration.params.len()
    }

    fn call_mut(&self, interpreter: &mut Interpreter, args: &[AloxObject]) -> AloxObjResult {
        let mut environment = Environment::with_enclosing(interpreter.global_env.clone());
        for (i, param) in self.declaration.params.iter().enumerate() {
            environment.define(&param.lexeme, Some(args[i].clone()));
        }

        interpreter.interpret_block(&self.declaration.body, environment)?;
        Ok(AloxObject::Value(Value::Nil(Nil)))
    }

    fn call(&self, _interpreter: &Interpreter, _args: &[AloxObject]) -> AloxObjResult {
        todo!()
    }
}

impl Display for AloxFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.declaration.name.lexeme)
    }
}

impl Debug for dyn Callable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.arity())
    }
}
