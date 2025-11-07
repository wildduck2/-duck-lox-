use diagnostic::{
  code::DiagnosticCode,
  diagnostic::{Diagnostic, LabelStyle},
  types::error::DiagnosticError,
  DiagnosticEngine,
};

use crate::{
  token::{Base, LiteralKind, RawStrError, TokenKind},
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
    match (self.get_current_lexeme(), self.peek()) {
      ("b", Some('"')) => self.lex_bstr(engine),
      ("c", Some('"')) => self.lex_cstr(engine),
      ("r", Some('"')) | ("r", Some('#')) => self.lex_raw_str(engine),
      ("\"", _) => self.lex_str(engine),
      ("b", Some('\'')) => self.lex_bchar(engine),
      ("'", _) => self.lex_char(engine),
      _ => Some(TokenKind::Literal {
        kind: LiteralKind::Str { terminated: false },
        suffix_start: 0,
      }),
    }
  }

  fn lex_raw_str(&mut self, engine: &mut DiagnosticEngine) -> Option<TokenKind> {
    const MAX_HASHES: u16 = 255;

    // Count the number of '#' characters after 'r'
    let mut n_hashes: u16 = 0;
    while self.peek() == Some('#') {
      n_hashes = n_hashes.saturating_add(1);
      self.advance();
    }

    // Optional: parity with Rust (cap hashes)
    if n_hashes > MAX_HASHES {
      let diag = Diagnostic::new(
        DiagnosticCode::Error(DiagnosticError::TooManyRawStrHashes),
        format!(
          "Raw string uses {} hashes; maximum is {}.",
          n_hashes, MAX_HASHES
        ),
        self.source.path.to_string(),
      )
      .with_label(
        diagnostic::Span::new(self.start, self.current),
        Some("Too many '#' characters here".to_string()),
        LabelStyle::Primary,
      );
      engine.add(diag);
      n_hashes = MAX_HASHES;
    }

    // Expect opening quote `"`
    if self.peek() != Some('"') {
      let diag = Diagnostic::new(
        DiagnosticCode::Error(DiagnosticError::UnterminatedString),
        format!(
          "Expected '\"' after 'r{}' in raw string literal",
          "#".repeat(n_hashes as usize)
        ),
        self.source.path.to_string(),
      )
      .with_label(
        diagnostic::Span::new(self.start, self.current),
        Some("Expected opening quote here".to_string()),
        LabelStyle::Primary,
      )
      .with_help(
        "Raw strings must start with r\"...\", r#\"...\"#, r##\"...\"##, etc.".to_string(),
      );
      engine.add(diag);

      return Some(TokenKind::Literal {
        kind: LiteralKind::RawStr {
          err: Some(RawStrError::Unterminated),
          n_hashes,
        },
        suffix_start: self.current as u32,
      });
    }

    // Consume opening quote
    self.advance();

    let mut err: Option<RawStrError> = None;
    let mut found_end = false;

    // Scan until we find the closing delimiter: '"' + n_hashes of '#'
    'outer: while let Some(c) = self.peek() {
      // NEW: if we see a newline, do a *lookahead* for "raw prefix" at the start of next line.
      // If the next non-space/tab chars on the *next line* form r#*",
      // we treat this current raw string as unterminated and recover at EOL.
      if c == '\n' {
        let saved = self.current;

        // step to next line
        self.advance(); // consume '\n'

        // skip horizontal whitespace only (spaces/tabs); stop on others
        while matches!(self.peek(), Some(' ' | '\t')) {
          self.advance();
        }

        let mut looks_like_raw_prefix = false;
        if self.peek() == Some('r') {
          self.advance(); // consume 'r'
                          // zero or more '#'
          while self.peek() == Some('#') {
            self.advance();
          }
          // must be a '"'
          looks_like_raw_prefix = self.peek() == Some('"');
        }

        // restore position regardless of outcome
        self.current = saved;

        if looks_like_raw_prefix {
          break 'outer; // stop scanning; we'll recover at EOL below
        } else {
          // valid multi-line content; keep the newline
          self.advance(); // consume the '\n' we inspected earlier
          continue;
        }
      }

      if c == '"' {
        let saved = self.current;
        self.advance(); // consume '"'

        let mut matched: u16 = 0;
        while matched < n_hashes && self.peek() == Some('#') {
          matched += 1;
          self.advance();
        }

        if matched == n_hashes {
          found_end = true;
          break 'outer;
        } else {
          // Not a real closing delimiter; restore to just after the '"'
          self.current = saved + 1;
        }
      } else {
        // ordinary content
        self.advance();
      }
    }

    if !found_end {
      // Robust recovery: advance to end-of-line (or EOF) so next token starts cleanly
      let recover_start = self.current;
      while let Some(ch) = self.peek() {
        if ch == '\n' {
          break;
        }
        self.advance();
      }

      let diag = Diagnostic::new(
        DiagnosticCode::Error(DiagnosticError::UnterminatedString),
        format!(
          "Unterminated raw string literal: expected closing '\"{}'",
          "#".repeat(n_hashes as usize)
        ),
        self.source.path.to_string(),
      )
      .with_label(
        diagnostic::Span::new(self.start, self.current),
        Some("This raw string literal is not terminated".to_string()),
        LabelStyle::Primary,
      )
      .with_help(format!(
        "Raw strings must end with \"{h}\" (e.g., r{h}\"...\"{h}).",
        h = "#".repeat(n_hashes as usize),
      ));
      engine.add(diag);
      err = Some(RawStrError::Unterminated);

      // If we're at a newline, consume exactly one so the next token begins on the next line
      if self.peek() == Some('\n') {
        self.advance();
      }

      // Safety: if nothing moved (e.g., EOF), bump one char to avoid stalling the lexer
      if self.current == recover_start {
        if self.peek().is_some() {
          self.advance();
        }
      }
    }

    Some(TokenKind::Literal {
      kind: LiteralKind::RawStr { err, n_hashes },
      // Right after the closing delimiter, or after recovery on error.
      suffix_start: self.current as u32,
    })
  }

  fn lex_cstr(&mut self, engine: &mut DiagnosticEngine) -> Option<TokenKind> {
    self.advance();
    self.lex_string_line(true);

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
    self.lex_string_line(true);

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
    self.lex_string_line(true);

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

  fn lex_string_line(&mut self, single: bool) {
    while let Some(c) = self.peek() {
      if c == '\n' && single {
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
  }
}

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
