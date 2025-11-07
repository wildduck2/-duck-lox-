//! Lexers for arithmetic operators.
//!
//! Handles `+`, `-`, `*`, `/`, `%` and their compound assignment variants.

use crate::{token::TokenKind, Lexer};

impl Lexer {
  /// Lexes a plus sign (`+`) or compound assignment (`+=`).
  ///
  /// # Returns
  ///
  /// `Some(TokenKind::Plus)` or `Some(TokenKind::PlusEq)`
  pub fn lex_plus(&mut self) -> Option<TokenKind> {
    if self.match_char('=') {
      self.advance(); // consume the '='
      return Some(TokenKind::PlusEq);
    }

    return Some(TokenKind::Plus);
  }

  /// Lexes a minus sign (`-`), compound assignment (`-=`), or thin arrow (`->`).
  ///
  /// Handles:
  /// - `-` - Subtraction or unary negation
  /// - `-=` - Subtraction assignment
  /// - `->` - Function return type arrow
  ///
  /// # Returns
  ///
  /// `Some(TokenKind::Minus)`, `Some(TokenKind::MinusEq)`, or `Some(TokenKind::ThinArrow)`
  pub fn lex_minus(&mut self) -> Option<TokenKind> {
    if self.match_char('=') {
      self.advance(); // consume the '='
      return Some(TokenKind::MinusEq);
    } else if self.match_char('>') {
      self.advance(); // consume the '>'
      return Some(TokenKind::ThinArrow);
    }

    return Some(TokenKind::Minus);
  }

  /// Lexes a star (`*`) or compound assignment (`*=`).
  ///
  /// Used for multiplication or dereferencing.
  ///
  /// # Returns
  ///
  /// `Some(TokenKind::Star)` or `Some(TokenKind::StarEq)`
  pub fn lex_star(&mut self) -> Option<TokenKind> {
    if self.match_char('=') {
      self.advance(); // consume the '='
      return Some(TokenKind::StarEq);
    }

    return Some(TokenKind::Star);
  }

  /// Lexes a slash (`/`), compound assignment (`/=`), or comment (`//`, `/*`).
  ///
  /// Handles:
  /// - `/` - Division operator
  /// - `/=` - Division assignment
  /// - `//` - Line comment
  /// - `/*` - Block comment
  ///
  /// # Returns
  ///
  /// `Some(TokenKind::Slash)`, `Some(TokenKind::SlashEq)`, or comment token
  pub fn lex_slash(&mut self) -> Option<TokenKind> {
    if self.match_char('=') {
      self.advance(); // consume the '='
      return Some(TokenKind::SlashEq);
    } else if self.match_char('/') {
      return self.lex_line_comment();
    } else if self.match_char('*') {
      return self.lex_multi_line_comment();
    }

    return Some(TokenKind::Slash);
  }

  /// Lexes a percent sign (`%`) or compound assignment (`%=`).
  ///
  /// Used for modulo operations.
  ///
  /// # Returns
  ///
  /// `Some(TokenKind::Percent)` or `Some(TokenKind::PercentEq)`
  pub fn lex_percent(&mut self) -> Option<TokenKind> {
    if self.match_char('=') {
      self.advance(); // consume the '='
      return Some(TokenKind::PercentEq);
    }

    return Some(TokenKind::Percent);
  }
}
