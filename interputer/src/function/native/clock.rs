use std::{
  sync::Arc,
  time::{SystemTime, UNIX_EPOCH},
};

use crate::{function::LoxCallable, interpreter::Interpreter, lox_value::LoxValue};

/// Clock native function
pub struct ClockFunction;

impl ClockFunction {
  pub fn add(interpreter: &mut Interpreter) {
    interpreter.env.borrow_mut().define(
      "clock".to_string(),
      LoxValue::NativeFunction(Arc::new(ClockFunction)),
    );
  }
}

impl LoxCallable for ClockFunction {
  fn arity(&self) -> usize {
    0
  }

  fn call(
    &self,
    _interpreter: &mut crate::interpreter::Interpreter,
    _arguments: Vec<(crate::lox_value::LoxValue, Option<scanner::token::Token>)>,
    _engine: &mut diagnostic::DiagnosticEngine,
  ) -> Result<crate::lox_value::LoxValue, ()> {
    let now = SystemTime::now()
      .duration_since(UNIX_EPOCH)
      .unwrap()
      .as_secs_f64();

    Ok(LoxValue::Number(now))
  }
}
