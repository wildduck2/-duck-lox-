//! Lexers for bitwise and logical operators.
//!
//! Handles `&`, `|`, `^` and their compound assignment and logical variants.

use crate::{token::TokenKind, Lexer};

impl Lexer {
  /// Lexes an ampersand (`&`), logical AND (`&&`), or compound assignment (`&=`).
  ///
  /// Handles:
  /// - `&` - Bitwise AND or borrow
  /// - `&&` - Logical AND
  /// - `&=` - Bitwise AND assignment
  ///
  /// # Returns
  ///
  /// `Some(TokenKind::And)`, `Some(TokenKind::AndAnd)`, or `Some(TokenKind::AndEq)`
  pub(crate) fn lex_and(&mut self) -> Option<TokenKind> {
    if self.match_char('&') {
      self.advance(); // consume the '='
      return Some(TokenKind::AndAnd);
    } else if self.match_char('=') {
      self.advance(); // consume the '='
      return Some(TokenKind::AndEq);
    }

    Some(TokenKind::And)
  }

  /// Lexes a pipe (`|`), logical OR (`||`), or compound assignment (`|=`).
  ///
  /// Handles:
  /// - `|` - Bitwise OR or closure parameter
  /// - `||` - Logical OR
  /// - `|=` - Bitwise OR assignment
  ///
  /// # Returns
  ///
  /// `Some(TokenKind::Or)`, `Some(TokenKind::OrOr)`, or `Some(TokenKind::OrEq)`
  pub(crate) fn lex_or(&mut self) -> Option<TokenKind> {
    if self.match_char('|') {
      self.advance(); // consume the '='
      return Some(TokenKind::OrOr);
    } else if self.match_char('=') {
      self.advance(); // consume the '='
      return Some(TokenKind::OrEq);
    }

    Some(TokenKind::Or)
  }

  /// Lexes a caret (`^`) or compound assignment (`^=`).
  ///
  /// Used for bitwise XOR operations.
  ///
  /// # Returns
  ///
  /// `Some(TokenKind::Caret)` or `Some(TokenKind::CaretEq)`
  pub(crate) fn lex_caret(&mut self) -> Option<TokenKind> {
    if self.match_char('=') {
      self.advance(); // consume the '='
      return Some(TokenKind::CaretEq);
    }

    Some(TokenKind::Caret)
  }
}
