//! Lexers for special operators and sequences.
//!
//! Handles path separators, range operators, and other multi-character sequences.

use crate::{token::TokenKind, Lexer};

impl Lexer {
  /// Lexes a path separator (`::`).
  ///
  /// Used for module paths (e.g., `std::io`).
  ///
  /// # Returns
  ///
  /// `Some(TokenKind::ColonColon)` if `::` is found, otherwise `Some(TokenKind::Colon)`
  pub(crate) fn lex_colon_colon(&mut self) -> Option<TokenKind> {
    if self.match_char(':') {
      self.advance(); // consume the ':'
      return Some(TokenKind::ColonColon);
    }

    Some(TokenKind::Colon)
  }
}
