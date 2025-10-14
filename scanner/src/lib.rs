use crate::token::Token;
use diagnostic::DiagnosticEngine;

pub mod token;
mod utils;

pub struct Scanner {
  pub tokens: Vec<Token>,
  pub source: String,
  pub line: usize,
  pub column: usize,
  pub current: usize,
  pub start: usize,
}

impl Scanner {
  /// Function that created a new scanner
  pub fn new() -> Self {
    Self {
      source: String::from(""),
      column: 0,
      line: 0,
      start: 0,
      current: 0,
      tokens: vec![],
    }
  }

  /// Function that executes the scanning operation on a lox content.
  pub fn scan(&mut self, engine: &mut DiagnosticEngine) {
    self.get_tokens(engine);
    println!("{:?}", self.tokens);
  }
}
