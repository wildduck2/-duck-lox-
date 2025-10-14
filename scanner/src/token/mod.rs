use crate::token::types::{Literal, TokenType};

pub mod types;

#[derive(Debug)]
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
}
