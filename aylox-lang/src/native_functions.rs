use std::time::{SystemTime, UNIX_EPOCH};

use crate::{ast::*, functions::Callable, interpreter::Interpreter};

pub struct Clock;

impl Callable for Clock {
    fn needs_mut(&self) -> bool {
        false
    }

    fn arity(&self) -> usize {
        0
    }

    fn call(&self, _interpreter: &Interpreter, _args: &[AloxObject]) -> AloxObjResult {
        Ok(AloxObject::Value(Value::Number(
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Clock has gone backwards!")
                .as_secs_f64(),
        )))
    }

    fn call_mut(&self, _interpreter: &mut Interpreter, _args: &[AloxObject]) -> AloxObjResult {
        todo!()
    }
}
