use crate::{token::TokenKind, Lexer};

impl Lexer {
  /// Lexes a plus sign
  ///
  /// # Examples
  /// ```rust
  /// let x = 1;
  ///       ^ plus sign
  /// let x = 1 + 1;
  ///           ^^ plus sign
  /// ```
  pub fn lex_plus(&mut self) -> Option<TokenKind> {
    if self.match_char('=') {
      self.advance(); // consume the '='
      return Some(TokenKind::PlusEq);
    }

    return Some(TokenKind::Plus);
  }

  /// Lexes a minus sign
  ///
  /// # Examples
  /// ```rust
  /// let x = 1;
  ///       ^ minus sign
  /// let x = 1 - 1;
  ///           ^^ minus sign
  /// let x = fn (x: i32) -> i32 { x - 1 };
  ///                     ^^ thin arrow
  /// ```
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

  /// Lexes a star sign
  ///
  /// # Examples
  /// ```rust
  /// let x = 1;
  ///       ^ star sign
  /// let x = 1 * 1;
  ///           ^^ star sign
  /// ```
  pub fn lex_star(&mut self) -> Option<TokenKind> {
    if self.match_char('=') {
      self.advance(); // consume the '='
      return Some(TokenKind::StarEq);
    }

    return Some(TokenKind::Star);
  }

  /// Lexes a slash sign
  ///
  /// # Examples
  /// ```rust
  /// let x = 1 / 1;
  ///           ^ slash sign
  /// let x /= 11;
  ///       ^^ slash sign
  ///
  /// // comment
  /// ^^ comment sign
  /// ```
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

  /// Lexes a percent sign
  ///
  /// # Examples
  /// ```rust
  /// let x = 1 % 1;
  ///           ^ percent sign
  /// let x %= 1;
  ///       ^^ percent equals sign
  /// ```
  pub fn lex_percent(&mut self) -> Option<TokenKind> {
    if self.match_char('=') {
      self.advance(); // consume the '='
      return Some(TokenKind::PercentEq);
    }

    return Some(TokenKind::Percent);
  }
}
