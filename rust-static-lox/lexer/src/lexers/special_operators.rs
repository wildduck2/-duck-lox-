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
  pub fn lex_colon_colon(&mut self) -> Option<TokenKind> {
    if self.match_char(':') {
      self.advance(); // consume the ':'
      return Some(TokenKind::ColonColon);
    }

    return Some(TokenKind::Colon);
  }

  /// Lexes a range operator (`..` or `..=`).
  ///
  /// Handles:
  /// - `..` - Exclusive range
  /// - `..=` - Inclusive range
  ///
  /// # Returns
  ///
  /// `Some(TokenKind::DotDot)` or `Some(TokenKind::DotDotEq)`, or `None` if not a range
  pub fn lex_dot_dot_eq(&mut self) -> Option<TokenKind> {
    if self.match_char('.') {
      self.advance(); // consume the '.'
      if self.match_char('=') {
        self.advance(); // consume the '='
        return Some(TokenKind::DotDotEq);
      }

      return Some(TokenKind::DotDot);
    }

    None
  }
}
