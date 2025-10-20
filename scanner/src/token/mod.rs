use diagnostic::diagnostic::Span;

use crate::token::types::{Literal, TokenType};

pub mod types;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Token {
  pub token_type: TokenType,
  pub lexeme: String,
  pub literal: Literal,
  pub position: (usize, usize),
}

impl Token {
  pub fn new(
    token_type: TokenType,
    lexeme: String,
    literal: Literal,
    position: (usize, usize),
  ) -> Self {
    Self {
      token_type,
      lexeme,
      literal,
      position,
    }
  }

  /// Function that takes a token and turn it to a span for the engine
  pub fn to_span(&self) -> Span {
    Span {
      file: "input.duck".to_string(),
      line: self.position.0,
      column: self.position.1,
      length: self.lexeme.len(),
    }
  }

  pub fn to_span_with_token(token: Token) -> Span {
    Span {
      file: "input.duck".to_string(),
      line: token.position.0,
      column: token.position.1,
      length: token.lexeme.len(),
    }
  }
}
