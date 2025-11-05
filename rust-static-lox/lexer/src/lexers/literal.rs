use diagnostic::DiagnosticEngine;

use crate::{token::TokenKind, Lexer};

impl Lexer {
  pub fn lex_number(&mut self, engine: &mut DiagnosticEngine) -> Option<TokenKind> {
    None
  }
  pub fn lex_string(&mut self) -> Option<TokenKind> {
    None
  }
}
