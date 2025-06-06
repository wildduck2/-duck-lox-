use crate::logger::{Log, Logger};

#[derive(Debug)]
pub enum LoxError {
  CompileError(CompilerError),
  RuntimeError,
}

#[derive(Debug)]
pub enum CompilerError {
  SyntaxError,            // generic syntax error
  UnexpectedToken,        // unexpected token found
  MissingToken,           // expected token missing
  UnterminatedString,     // string literal not closed
  InvalidLiteral,         // literal value invalid (e.g., bad number format)
  TypeMismatch,           // type mismatch in expressions
  UndefinedVariable,      // variable used before declaration
  Redeclaration,          // variable/function redeclared
  InvalidAssignment,      // assigning to non-assignable expression
  MissingSemicolon,       // semicolon missing at end of statement
  InvalidFunctionCall,    // calling something that isn’t a function
  ParameterCountMismatch, // wrong number of arguments to function
  DivisionByZero,         // attempted division by zero (runtime but can be checked)
  InvalidReturn,          // return statement in non-function context or wrong type
  UnexpectedEOF,          // file ended unexpectedly
  DuplicateLabel,         // duplicate label in code (for goto or similar)
  InvalidOperator,        // invalid operator usage
  ConstantReassignment,   // reassignment to a constant
                          // ... add more as needed
}

pub struct Lox {
  pub has_error: bool,
}

impl Lox {
  /// Logs a message with a timestamp, level, and message, also writes to a log file but for
  /// the language.
  pub fn log_language(level: Log, message: &str, position: &str) -> () {
    Logger::log(level, format!("[line: {}] {}", position, message).as_str());
  }
}
