pub mod types;

use types::{Literal, TokenType};

#[derive(Debug, Clone)]
pub struct Position {
  line: usize,
  column: usize,
}

#[derive(Debug, Clone)]
pub struct Token {
  pub token_type: TokenType,
  pub lexeme: String,
  pub literal: Literal,
  pub line: usize,
  pub position: Position,
}

impl Token {
  pub fn new(
    token_type: TokenType,
    lexeme: String,
    literal: Literal,
    line: usize,
    column: usize,
  ) -> Token {
    Token {
      token_type,
      lexeme,
      literal,
      line,
      position: Position {
        line,
        column: column,
      },
    }
  }
}
