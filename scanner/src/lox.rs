use logger::{LogType, Logger};

#[derive(Debug, Clone)]
pub enum LoxError {
  CompileError(CompilerError),
  RuntimeError,
}

#[derive(Debug, Clone)]
pub enum CompilerError {
  SyntaxError,     // generic syntax error
  UnexpectedToken, // unexpected token found
  MissingToken,    // expected token missing
                   // ... add more as needed
}

pub struct Lox {
  pub has_error: bool,
}

impl Lox {
  pub fn new() -> Self {
    Self { has_error: false }
  }
}
