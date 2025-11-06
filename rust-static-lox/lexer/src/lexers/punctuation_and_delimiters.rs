use crate::{token::TokenKind, Lexer};
use diagnostic::DiagnosticEngine;

impl Lexer {
  /// Lexes a semicolon
  ///
  /// # Examples
  /// ```rust
  /// let x = 1;
  ///          ^ semicolon
  /// ```
  pub fn lex_semicolon(&mut self) -> Option<TokenKind> {
    return Some(TokenKind::Semi);
  }

  /// Lexes a comma
  ///
  /// # Examples
  /// ```rust
  /// let x = 1, y = 2;
  ///          ^ comma
  /// ```
  pub fn lex_comma(&mut self) -> Option<TokenKind> {
    return Some(TokenKind::Comma);
  }

  /// Lexes a dot
  ///
  /// # Examples
  /// ```rust
  /// let x = 1.0;
  ///          ^ dot
  /// let x = 1..2;
  ///          ^^ dot dot symbol
  /// let x = 1.0..=2.0;
  ///            ^^^ dot equals sign
  /// ```
  pub fn lex_dot(&mut self) -> Option<TokenKind> {
    if self.match_char('.') {
      self.advance(); // consume the '.'
      return Some(TokenKind::DotDot);
    } else if self.match_char('=') {
      self.advance(); // consume the '='
      return Some(TokenKind::DotDotEq);
    }

    return Some(TokenKind::Dot);
  }

  /// Lexes an open parenthesis
  ///
  /// # Examples
  /// ```rust
  /// let x = (1);
  ///         ^ open parenthesis
  /// ```
  pub fn lex_open_paren(&mut self) -> Option<TokenKind> {
    return Some(TokenKind::OpenParen);
  }

  /// Lexes a close parenthesis
  ///
  /// # Examples
  /// ```rust
  /// let x = (1);
  ///           ^ close parenthesis
  /// ```
  pub fn lex_close_paren(&mut self) -> Option<TokenKind> {
    return Some(TokenKind::CloseParen);
  }

  /// Lexes an open brace
  ///
  /// # Examples
  /// ```rust
  /// let x = { 1 };
  ///         ^ open brace
  /// ```
  pub fn lex_open_brace(&mut self) -> Option<TokenKind> {
    return Some(TokenKind::OpenBrace);
  }

  /// Lexes a close brace
  ///
  /// # Examples
  /// ```rust
  /// let x = { 1 };
  ///             ^ close brace
  /// ```
  pub fn lex_close_brace(&mut self) -> Option<TokenKind> {
    return Some(TokenKind::CloseBrace);
  }

  /// Lexes an open bracket
  ///
  /// # Examples
  /// ```rust
  /// let x = [1];
  ///         ^ open bracket
  /// ```
  pub fn lex_open_bracket(&mut self) -> Option<TokenKind> {
    return Some(TokenKind::OpenBracket);
  }

  /// Lexes a close bracket
  ///
  /// # Examples
  /// ```rust
  /// let x = [1];
  ///           ^ close bracket
  /// ```
  pub fn lex_close_bracket(&mut self) -> Option<TokenKind> {
    return Some(TokenKind::CloseBracket);
  }

  /// Lexes an at symbol
  ///
  /// # Examples
  /// ```rust
  /// let x = @1;
  ///         ^ at symbol
  /// ```
  pub fn lex_at(&mut self) -> Option<TokenKind> {
    return Some(TokenKind::At);
  }

  /// Lexes a pound symbol
  ///
  /// # Examples
  /// ```rust
  /// # // comment
  /// #!/usr/bin/env rustrc
  /// #![allow(dead_code)]
  /// #[cfg(test)]
  /// #[allow(unused)]
  /// ```
  pub fn lex_pound(&mut self, engine: &mut DiagnosticEngine) -> Option<TokenKind> {
    if self.match_char('!') {
      return self.lex_shebang(engine);
    }

    Some(TokenKind::Pound)
  }

  /// Lexes a tilde symbol
  ///
  /// # Examples
  /// ```rust
  /// let x = ~1;
  ///         ^ tilde symbol
  /// ```
  pub fn lex_tilde(&mut self) -> Option<TokenKind> {
    return Some(TokenKind::Tilde);
  }

  /// Lexes a question mark
  ///
  /// # Examples
  /// ```rust
  /// let x = ?1;
  ///         ^ question mark
  /// ```
  pub fn lex_question(&mut self) -> Option<TokenKind> {
    return Some(TokenKind::Question);
  }

  /// Lexes a colon
  ///
  /// # Examples
  /// ```rust
  /// let x = 1:2;
  ///         ^ colon
  /// User::new(1, 2);
  ///     ^^ fat arrow
  /// ```
  pub fn lex_colon(&mut self) -> Option<TokenKind> {
    if self.match_char(':') {
      self.advance(); // consume the ':'
      return Some(TokenKind::ColonColon);
    }

    return Some(TokenKind::Colon);
  }

  /// Lexes a dollar symbol
  ///
  /// # Examples
  /// ```rust
  /// let x = $1;
  ///         ^ dollar symbol
  /// ```
  pub fn lex_dollar(&mut self) -> Option<TokenKind> {
    return Some(TokenKind::Dollar);
  }
}
