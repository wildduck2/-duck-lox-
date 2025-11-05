use crate::{token::TokenKind, Lexer};

impl Lexer {
  /// Lexes a colon colon symbol
  ///
  /// # Examples
  /// ```rust
  /// let x = 1;
  ///       ^ colon colon symbol
  /// ```
  pub fn lex_colon_colon(&mut self) -> Option<TokenKind> {
    if self.match_char(self.peek(), ':') {
      self.advance(); // consume the ':'
      return Some(TokenKind::ColonColon);
    }

    return Some(TokenKind::Colon);
  }

  /// Lexes a dot dot equals symbol
  ///
  /// # Examples
  /// ```rust
  /// let x = 1;
  ///       ^ dot dot equals symbol
  /// ```
  pub fn lex_dot_dot_eq(&mut self) -> Option<TokenKind> {
    if self.match_char(self.peek(), '.') {
      self.advance(); // consume the '.'
      if self.match_char(self.peek(), '=') {
        self.advance(); // consume the '='
        return Some(TokenKind::DotDotEq);
      }

      return Some(TokenKind::DotDot);
    }

    None
  }
}
