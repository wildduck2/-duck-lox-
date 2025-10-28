use std::sync::Arc;

use crate::{
  function::LoxCallable,
  interpreter::Interpreter,
  lox_value::{InterpreterError, LoxValue},
};

/// Print native function
pub struct PrintFunction;

impl PrintFunction {
  pub fn add(interpreter: &mut Interpreter) {
    interpreter.env.borrow_mut().define(
      "print".to_string(),
      LoxValue::NativeFunction(Arc::new(PrintFunction)),
    );
  }
}

impl LoxCallable for PrintFunction {
  fn arity(&self) -> usize {
    usize::MAX
  }

  fn call(
    &self,
    _interpreter: &mut crate::interpreter::Interpreter,
    arguments: Vec<(crate::lox_value::LoxValue, Option<scanner::token::Token>)>,
    _engine: &mut diagnostic::DiagnosticEngine,
  ) -> Result<crate::lox_value::LoxValue, InterpreterError> {
    // Map each (LoxValue, _) to string using Display
    let output = arguments
      .iter()
      .map(|(val, _)| val.to_string())
      .collect::<Vec<_>>()
      .join(" ");

    // Print to stdout
    println!("{}", output);

    // Return nil (like Lox `print` does)
    Ok(crate::lox_value::LoxValue::Nil)
  }
}
