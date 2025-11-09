//! Lexers for assignment and comparison operators.
//!
//! Handles `=`, `==`, `!=`, `<`, `<=`, `>`, `>=`, `<<`, `>>`, and their variants.

use crate::{token::TokenKind, Lexer};

impl Lexer {
  /// Lexes an equals sign (`=`), equality (`==`), or fat arrow (`=>`).
  ///
  /// Handles:
  /// - `=` - Assignment
  /// - `==` - Equality comparison
  /// - `=>` - Match arm arrow
  ///
  /// # Returns
  ///
  /// `Some(TokenKind::Eq)`, `Some(TokenKind::EqEq)`, or `Some(TokenKind::FatArrow)`
  pub(crate) fn lex_equal(&mut self) -> Option<TokenKind> {
    if self.match_char('=') {
      self.advance(); // consume the '='
      return Some(TokenKind::EqEq);
    } else if self.match_char('>') {
      self.advance(); // consume the '='
      return Some(TokenKind::FatArrow);
    }

    Some(TokenKind::Eq)
  }

  /// Lexes an exclamation mark (`!`) or inequality (`!=`).
  ///
  /// Handles:
  /// - `!` - Logical NOT
  /// - `!=` - Inequality comparison
  ///
  /// # Returns
  ///
  /// `Some(TokenKind::Bang)` or `Some(TokenKind::Ne)`
  pub(crate) fn lex_bang(&mut self) -> Option<TokenKind> {
    if self.match_char('=') {
      self.advance(); // consume the '='
      return Some(TokenKind::Ne);
    }

    Some(TokenKind::Bang)
  }

  /// Lexes a less-than sign (`<`), less-or-equal (`<=`), or left shift (`<<`, `<<=`).
  ///
  /// Handles:
  /// - `<` - Less than comparison
  /// - `<=` - Less than or equal
  /// - `<<` - Left shift
  /// - `<<=` - Left shift assignment
  ///
  /// # Returns
  ///
  /// `Some(TokenKind::Lt)`, `Some(TokenKind::Le)`, `Some(TokenKind::ShiftLeft)`, or `Some(TokenKind::ShiftLeftEq)`
  pub(crate) fn lex_less(&mut self) -> Option<TokenKind> {
    if self.match_char('=') {
      self.advance(); // consume the '='
      return Some(TokenKind::Le);
    } else if self.match_char('<') {
      self.advance(); // consume the '='

      if self.match_char('=') {
        self.advance(); // consume the '='
        return Some(TokenKind::ShiftLeftEq);
      }

      return Some(TokenKind::ShiftLeft);
    }

    Some(TokenKind::Lt)
  }

  /// Lexes a greater-than sign (`>`), greater-or-equal (`>=`), or right shift (`>>`, `>>=`).
  ///
  /// Handles:
  /// - `>` - Greater than comparison
  /// - `>=` - Greater than or equal
  /// - `>>` - Right shift
  /// - `>>=` - Right shift assignment
  ///
  /// # Returns
  ///
  /// `Some(TokenKind::Gt)`, `Some(TokenKind::Ge)`, `Some(TokenKind::ShiftRight)`, or `Some(TokenKind::ShiftRightEq)`
  pub(crate) fn lex_greater(&mut self) -> Option<TokenKind> {
    if self.match_char('=') {
      self.advance(); // consume the '='
      return Some(TokenKind::Ge);
    } else if self.match_char('>') {
      self.advance(); // consume the '>'

      if self.match_char('=') {
        self.advance(); // consume the '='
        return Some(TokenKind::ShiftRightEq);
      }

      return Some(TokenKind::ShiftRight);
    }

    Some(TokenKind::Gt)
  }
}
