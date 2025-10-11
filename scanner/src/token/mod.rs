use crate::token::types::{Literal, TokenType};

pub mod types;

#[derive(Debug)]
pub struct Token {
  pub token_type: TokenType,
  pub lexeme: String,
  pub literal: Literal,
  pub position: (u32, u32),
}

impl Token {
  pub fn new(
    token_type: TokenType,
    lexeme: String,
    literal: Literal,
    position: (u32, u32),
  ) -> Self {
    Self {
      token_type,
      lexeme,
      literal,
      position,
    }
  }
}
