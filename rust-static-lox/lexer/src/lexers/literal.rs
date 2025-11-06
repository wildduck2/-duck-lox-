use diagnostic::{
  code::DiagnosticCode,
  diagnostic::{Diagnostic, LabelStyle},
  types::error::DiagnosticError,
  DiagnosticEngine, Span,
};

use crate::{
  token::{Base, LiteralKind, TokenKind},
  Lexer,
};

impl Lexer {
  pub fn lex_number(&mut self) -> Option<TokenKind> {
    let kind = if self.get_current_lexeme() == "0" {
      if self.match_char('b') {
        self.lex_binary()
      } else if self.match_char('o') {
        self.lex_octal()
      } else if self.match_char('x') {
        self.lex_hexadecimal()
      } else {
        self.lex_decimal()
      }
    } else {
      self.lex_decimal()
    };

    let suffix_start = self.current as u32;
    Some(TokenKind::Literal { kind, suffix_start })
  }

  fn lex_binary(&mut self) -> LiteralKind {
    let mut has_digits = false;
    while let Some(c) = self.peek() {
      if c == '0' || c == '1' {
        self.advance();
        has_digits = true;
      } else if c == '_' && self.peek_next(1) != Some('_') {
        self.advance();
        continue;
      } else {
        break;
      }
    }
    LiteralKind::Int {
      base: Base::Binary,
      empty_int: !has_digits,
    }
  }

  fn lex_octal(&mut self) -> LiteralKind {
    let mut has_digits = false;
    while let Some(c) = self.peek() {
      if c >= '0' && c <= '7' {
        self.advance();
        has_digits = true;
      } else {
        break;
      }
    }
    LiteralKind::Int {
      base: Base::Octal,
      empty_int: !has_digits,
    }
  }

  fn lex_decimal(&mut self) -> LiteralKind {
    let mut has_digits = false;
    let mut has_dot = false;
    let mut has_exponent = false;
    let mut has_exp_digits = false;

    while let Some(c) = self.peek() {
      if c.is_ascii_digit() {
        self.advance();
        has_digits = true;
      } else if c == '_' && self.peek_next(1) != Some('_') {
        self.advance();
        continue;
      } else if c == '.' && !has_dot && !has_exponent {
        has_dot = true;
        self.advance();
      } else if (c == 'e' || c == 'E') && !has_exponent {
        has_exponent = true;
        self.advance();

        // Optional sign after e/E
        if let Some(sign) = self.peek() {
          if sign == '+' || sign == '-' {
            self.advance();
          }
        }

        // Exponent digits
        while let Some(ec) = self.peek() {
          if ec.is_ascii_digit() {
            self.advance();
            has_exp_digits = true;
          } else if ec == '_' && self.peek_next(1) != Some('_') {
            self.advance();
            continue;
          } else {
            break;
          }
        }
        break;
      } else {
        break;
      }
    }

    if has_dot || has_exponent {
      LiteralKind::Float {
        base: Base::Decimal,
        empty_exponent: has_exponent && !has_exp_digits,
      }
    } else {
      LiteralKind::Int {
        base: Base::Decimal,
        empty_int: !has_digits,
      }
    }
  }

  /// Floating-point literal with optional suffix
  ///
  /// # Examples
  /// ```rust
  /// 3.14            // basic float
  /// 1e10            // exponential notation
  /// 2.5E-3          // exponential with sign
  /// 1.0f32          // with type suffix
  /// 1e_             // empty_exponent = true (malformed)
  /// ```
  ///
  /// **Note**: `1f32` is lexed as `Int` with suffix "f32", not `Float`

  fn lex_hexadecimal(&mut self) -> LiteralKind {
    let mut has_digits = false;
    while let Some(c) = self.peek() {
      if c.is_ascii_hexdigit() {
        self.advance();
        has_digits = true;
      } else if c == '_' && self.peek_next(1) != Some('_') {
        self.advance();
        continue;
      } else {
        break;
      }
    }
    LiteralKind::Int {
      base: Base::Hexadecimal,
      empty_int: !has_digits,
    }
  }

  pub fn lex_string(&mut self, engine: &mut DiagnosticEngine) -> Option<TokenKind> {
    if self.get_current_lexeme() == "b" && self.peek() == Some('\'') {
      return self.lex_bchar(engine);
    } else if self.get_current_lexeme() == "b" && self.peek() == Some('"') {
      return self.lex_bstr(engine);
    } else if self.get_current_lexeme() == "c" && self.peek() == Some('"') {
      return self.lex_cstr(engine);
    // } else if self.get_current_lexeme() == "r" && self.peek() == Some('"') {
    //   return self.lex_raw_str(engine);
    } else if self.get_current_lexeme() == "\'" {
      return self.lex_char(engine);
    } else if self.get_current_lexeme() == "\"" {
      return self.lex_str(engine);
    }

    Some(TokenKind::Literal {
      kind: LiteralKind::Str { terminated: false },
      suffix_start: 0,
    })
  }
  //
  //   /// C string literal (null-terminated, type `&CStr`)
  //   ///
  //   /// Added in Rust 1.77 for FFI interop.
  //   ///
  //   /// # Examples
  //   /// ```rust
  //   /// c"hello"        // becomes "hello\0"
  //   /// c"with\0null"   // explicit null allowed
  //   /// c"unterminated  // terminated = false (malformed)
  //   /// ```
  //   CStr {
  //     /// False if the closing `"` is missing
  //     terminated: bool,
  //   },

  fn lex_cstr(&mut self, engine: &mut DiagnosticEngine) -> Option<TokenKind> {
    self.advance();

    while let Some(c) = self.peek() {
      if c == '\n' {
        break;
      }

      if c == '"' {
        self.advance();
        break;
      }

      self.advance();
    }
    let terminated = if !self.get_current_lexeme().ends_with('"') {
      let diagnostic = Diagnostic::new(
        DiagnosticCode::Error(DiagnosticError::UnterminatedString),
        format!("Unterminated string literal: {}", self.get_current_lexeme()),
        self.source.path.to_string(),
      )
      .with_label(
        diagnostic::Span::new(self.start, self.current),
        Some("This string literal is not terminated".to_string()),
        LabelStyle::Primary,
      )
      .with_help("Use double quotes for string literals.".to_string());
      engine.add(diagnostic);

      false
    } else {
      true
    };

    Some(TokenKind::Literal {
      kind: LiteralKind::CStr { terminated },
      suffix_start: self.current as u32,
    })
  }

  fn lex_bstr(&mut self, engine: &mut DiagnosticEngine) -> Option<TokenKind> {
    self.advance();

    while let Some(c) = self.peek() {
      if c == '\n' {
        break;
      }

      if c == '"' {
        self.advance();
        break;
      }

      self.advance();
    }

    let terminated = if !self.get_current_lexeme().ends_with('"') {
      let diagnostic = Diagnostic::new(
        DiagnosticCode::Error(DiagnosticError::UnterminatedString),
        format!("Unterminated string literal: {}", self.get_current_lexeme()),
        self.source.path.to_string(),
      )
      .with_label(
        diagnostic::Span::new(self.start, self.current),
        Some("This string literal is not terminated".to_string()),
        LabelStyle::Primary,
      )
      .with_help("Use double quotes for string literals.".to_string());
      engine.add(diagnostic);

      false
    } else {
      true
    };

    Some(TokenKind::Literal {
      kind: LiteralKind::ByteStr { terminated },
      suffix_start: self.current as u32,
    })
  }

  fn lex_bchar(&mut self, engine: &mut DiagnosticEngine) -> Option<TokenKind> {
    let mut len = 1;
    let mut is_hex = false;
    self.advance();

    while let Some(c) = self.peek() {
      if c == '\n' {
        break;
      }

      if c == '\\' {
        is_hex = true;
        self.advance();
        if self.peek() == Some('"') {
          self.advance();
        }
        continue;
      }

      len += 1;
      if c == '\'' {
        self.advance();
        break;
      }

      self.advance();
    }

    let terminated = if !self.get_current_lexeme().ends_with('\'') {
      let diagnostic = Diagnostic::new(
        DiagnosticCode::Error(DiagnosticError::UnterminatedString),
        format!("Unterminated string literal: {}", self.get_current_lexeme()),
        self.source.path.to_string(),
      )
      .with_label(
        diagnostic::Span::new(self.start, self.current),
        Some("This string literal is not terminated".to_string()),
        LabelStyle::Primary,
      )
      .with_help("Use double quotes for string literals.".to_string());
      engine.add(diagnostic);

      false
    } else {
      true
    };

    if len > 3 && !is_hex {
      let diagnostic = Diagnostic::new(
        DiagnosticCode::Error(DiagnosticError::UnterminatedString),
        format!(
          "Too many characters in byte char literal: {}",
          self.get_current_lexeme()
        ),
        self.source.path.to_string(),
      )
      .with_label(
        diagnostic::Span::new(self.start, self.current),
        Some("This byte char literal is too long".to_string()),
        LabelStyle::Primary,
      )
      .with_help("byte char literals can only contain ASCII characters.".to_string());

      engine.add(diagnostic);
      return None;
    }

    Some(TokenKind::Literal {
      kind: LiteralKind::Byte { terminated },
      suffix_start: self.current as u32,
    })
  }

  fn lex_char(&mut self, engine: &mut DiagnosticEngine) -> Option<TokenKind> {
    let mut len = 1;
    let mut is_unicode = false;

    while let Some(c) = self.peek() {
      if c == '\n' {
        break;
      }

      len += 1;
      if c == '\'' {
        self.advance();
        break;
      }

      if c == '\\' && self.peek_next(1) == Some('u') && self.peek_next(2) == Some('{') {
        is_unicode = true;
      }

      if c == '\\' {
        len -= 1;
      }
      self.advance();
    }

    let terminated = if !self.get_current_lexeme().ends_with('\'') {
      let diagnostic = Diagnostic::new(
        DiagnosticCode::Error(DiagnosticError::InvalidCharacter),
        format!("unterminated char literal: {}", self.get_current_lexeme()),
        self.source.path.to_string(),
      )
      .with_label(
        diagnostic::Span::new(self.start, self.current),
        Some("Unterminated character literal".to_string()),
        LabelStyle::Primary,
      )
      .with_help("Use single quotes for character literals.".to_string());

      engine.add(diagnostic);
      false
    } else {
      true
    };

    if len > 3 && !is_unicode {
      let diagnostic = Diagnostic::new(
        DiagnosticCode::Error(DiagnosticError::UnterminatedString),
        format!(
          "Too many characters in char literal: {}",
          self.get_current_lexeme()
        ),
        self.source.path.to_string(),
      )
      .with_label(
        diagnostic::Span::new(self.start, self.current),
        Some("This char literal is too long".to_string()),
        LabelStyle::Primary,
      )
      .with_help("char literals can only contain ASCII characters.".to_string());

      engine.add(diagnostic);
      return None;
    } else if len > 3 && is_unicode && !self.get_current_lexeme().ends_with("}'") {
      let diagnostic = Diagnostic::new(
        DiagnosticCode::Error(DiagnosticError::UnterminatedString),
        format!("Wrong unicode escape: {}", self.get_current_lexeme()),
        self.source.path.to_string(),
      )
      .with_label(
        diagnostic::Span::new(self.start, self.current),
        Some("There should be a closing `}` after the unicode escape".to_string()),
        LabelStyle::Primary,
      )
      .with_help("Use the right escape sequence for unicode characters. `\\u{1F980}`".to_string());

      engine.add(diagnostic);
      return None;
    }

    Some(TokenKind::Literal {
      kind: LiteralKind::Char { terminated },
      suffix_start: self.current as u32,
    })
  }

  fn lex_str(&mut self, engine: &mut DiagnosticEngine) -> Option<TokenKind> {
    while let Some(c) = self.peek() {
      if c == '\n' {
        break;
      }

      if c == '\\' {
        self.advance();
        if self.peek() == Some('"') {
          self.advance();
        }
        continue;
      }

      if c == '"' {
        self.advance();
        break;
      }
      self.advance();
    }

    if !self.get_current_lexeme().ends_with('"') {
      let diagnostic = Diagnostic::new(
        DiagnosticCode::Error(DiagnosticError::UnterminatedString),
        format!("Unterminated string literal: {}", self.get_current_lexeme()),
        self.source.path.to_string(),
      )
      .with_label(
        diagnostic::Span::new(self.start, self.current),
        Some("This string literal is not terminated".to_string()),
        LabelStyle::Primary,
      )
      .with_help("Use double quotes for string literals.".to_string());
      engine.add(diagnostic);

      return None;
    }

    Some(TokenKind::Literal {
      kind: LiteralKind::Str { terminated: false },
      suffix_start: self.current as u32,
    })
  }
}

//
//   /// C string literal (null-terminated, type `&CStr`)
//   ///
//   /// Added in Rust 1.77 for FFI interop.
//   ///
//   /// # Examples
//   /// ```rust
//   /// c"hello"        // becomes "hello\0"
//   /// c"with\0null"   // explicit null allowed
//   /// c"unterminated  // terminated = false (malformed)
//   /// ```
//   CStr {
//     /// False if the closing `"` is missing
//     terminated: bool,
//   },
//
//   /// Raw string literal (no escape processing)
//   ///
//   /// # Examples
//   /// ```rust
//   /// r"no\escapes"           // n_hashes = 0
//   /// r#"with "quotes""#      // n_hashes = 1
//   /// r##"more # freedom"##   // n_hashes = 2
//   /// ```
//   RawStr {
//     /// Number of `#` delimiters used
//     n_hashes: u16,
//     /// Error if the raw string is malformed
//     err: Option<RawStrError>,
//   },
//
//   /// Raw byte string literal (raw + byte string combined)
//   ///
//   /// # Examples
//   /// ```rust
//   /// br"raw bytes"
//   /// br#"with "quotes""#
//   /// ```
//   RawByteStr {
//     /// Number of `#` delimiters used
//     n_hashes: u16,
//     /// Error if the raw byte string is malformed
//     err: Option<RawStrError>,
//   },
//
//   /// Raw C string literal (raw + C string combined)
//   ///
//   /// Added in Rust 1.77.
//   ///
//   /// # Examples
//   /// ```rust
//   /// cr"raw c string"
//   /// cr#"with "quotes""#
//   /// ```
//   RawCStr {
//     /// Number of `#` delimiters used
//     n_hashes: u16,
//     /// Error if the raw C string is malformed
//     err: Option<RawStrError>,
//   },
// }
