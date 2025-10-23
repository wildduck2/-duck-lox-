use diagnostic::DiagnosticEngine;
use scanner::token::Token;

use crate::{interpreter::Interpreter, lox_value::LoxValue};

pub mod native;
pub mod normal;

pub trait LoxCallable {
  fn arity(&self) -> usize;
  fn call(
    &self,
    interpreter: &mut Interpreter,
    arguments: Vec<(LoxValue, Option<Token>)>,
    engine: &mut DiagnosticEngine,
  ) -> Result<LoxValue, ()>;
}
