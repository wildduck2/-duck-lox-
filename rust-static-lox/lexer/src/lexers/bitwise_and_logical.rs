use crate::{token::TokenKind, Lexer};

impl Lexer {
  /// Lexes an ampersand symbol
  ///
  /// # Examples
  /// ```rust
  /// let x = 1 & 2;
  ///           ^ ampersand symbol
  /// let x = 1 && 2;
  ///           ^^ ampersand and ampersand symbol
  /// let x &= 1;
  ///       ^^ ampersand equals sign
  /// ```
  pub fn lex_and(&mut self) -> Option<TokenKind> {
    if self.match_char('&') {
      self.advance(); // consume the '='
      return Some(TokenKind::AndAnd);
    } else if self.match_char('=') {
      self.advance(); // consume the '='
      return Some(TokenKind::AndEq);
    }

    return Some(TokenKind::And);
  }

  /// Lexes a pipe symbol
  ///
  /// # Examples
  /// ```rust
  /// let x = 1 | 2;
  ///           ^ pipe symbol
  /// let x = 1 || 2;
  ///           ^^ pipe and pipe symbol
  /// let x |= 1;
  ///       ^^ pipe equals sign
  /// ```
  pub fn lex_or(&mut self) -> Option<TokenKind> {
    if self.match_char('|') {
      self.advance(); // consume the '='
      return Some(TokenKind::OrOr);
    } else if self.match_char('=') {
      self.advance(); // consume the '='
      return Some(TokenKind::OrEq);
    }

    return Some(TokenKind::Or);
  }

  /// Lexes a caret symbol
  ///
  /// # Examples
  /// ```rust
  /// let x = 1 ^ 2;
  ///           ^ caret symbol
  /// let x ^= 2;
  ///       ^^ caret equals sign
  /// ```
  pub fn lex_caret(&mut self) -> Option<TokenKind> {
    if self.match_char('=') {
      self.advance(); // consume the '='
      return Some(TokenKind::CaretEq);
    }

    return Some(TokenKind::Caret);
  }
}
