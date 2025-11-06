use crate::{token::TokenKind, Lexer};

impl Lexer {
  /// Lexes an equals sign
  ///
  /// # Examples
  /// ```rust
  /// let x = 1;
  ///       ^ equals sign
  /// let x = 1 == 1;
  ///           ^^ equals sign
  /// let x = match x { 1 => 2, _ => 3 };
  ///                     ^^ fat arrow
  /// ```
  pub fn lex_equal(&mut self) -> Option<TokenKind> {
    if self.match_char('=') {
      self.advance(); // consume the '='
      return Some(TokenKind::EqEq);
    } else if self.match_char('>') {
      self.advance(); // consume the '='
      return Some(TokenKind::FatArrow);
    }

    return Some(TokenKind::Eq);
  }

  /// Lexes an exclamation mark
  ///
  /// # Examples
  /// ```rust
  /// let x = !true;
  ///         ^ exclamation mark
  /// let x = 1 != 2;
  ///           ^^ not equals sign
  /// ```
  pub fn lex_bang(&mut self) -> Option<TokenKind> {
    if self.match_char('=') {
      self.advance(); // consume the '='
      return Some(TokenKind::Ne);
    }

    return Some(TokenKind::Bang);
  }

  /// Lexes a less than sign
  ///
  /// # Examples
  /// ```rust
  /// let x = 1 < 2;
  ///           ^ less than sign
  /// let x = 1 <= 2;
  ///           ^^ less than or equal to sign
  /// let x = 1 << 2;
  ///           ^^ less than sign
  /// let x = 1 <<= 2;
  ///           ^^^ less than or equal to sign
  /// ```
  pub fn lex_less(&mut self) -> Option<TokenKind> {
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

    return Some(TokenKind::Lt);
  }

  /// Lexes a greater than sign
  ///
  /// # Examples
  /// ```rust
  /// let x = 1 > 2;
  ///   x       ^ greater than sign
  /// let x = 1 >= 2;
  ///   x       ^^ greater than or equal to sign
  /// let x = 1 >> 2;
  ///   x       ^^ greater than sign
  /// let x = 1 >>> 2;
  ///   x       ^^^ greater than or equal to sign
  /// ```
  pub fn lex_greater(&mut self) -> Option<TokenKind> {
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

    return Some(TokenKind::Gt);
  }
}
