use diagnostic::{
  code::DiagnosticCode,
  diagnostic::{Diagnostic, LabelStyle},
  types::error::DiagnosticError,
  DiagnosticEngine, Span,
};

use crate::{
  token::{Base, LiteralKind, RawStrError, TokenKind},
  Lexer,
};

impl Lexer {
  pub fn lex_number(&mut self) -> Option<TokenKind> {
    let kind = if self.get_current_lexeme() == "0" {
      match self.peek() {
        Some('b') | Some('B') => {
          self.advance();
          self.lex_binary()
        },
        Some('o') | Some('O') => {
          self.advance();
          self.lex_octal()
        },
        Some('x') | Some('X') => {
          self.advance();
          self.lex_hexadecimal()
        },
        _ => self.lex_decimal(),
      }
    } else {
      self.lex_decimal()
    };

    let suffix_start = self.current as u32;
    Some(TokenKind::Literal { kind, suffix_start })
  }

  /// Consume digits that match a predicate, handling underscores
  /// Returns true if at least one digit was consumed
  fn consume_digits<F>(&mut self, mut is_valid_digit: F) -> bool
  where
    F: FnMut(char) -> bool,
  {
    let mut has_digits = false;
    let mut last_was_underscore = false;

    while let Some(c) = self.peek() {
      if is_valid_digit(c) {
        self.advance();
        has_digits = true;
        last_was_underscore = false;
      } else if c == '_' {
        // Don't allow consecutive underscores or trailing underscores
        if last_was_underscore {
          break;
        }
        self.advance();
        last_was_underscore = true;
      } else {
        break;
      }
    }

    has_digits
  }

  fn lex_binary(&mut self) -> LiteralKind {
    let has_digits = self.consume_digits(|c| c == '0' || c == '1');

    LiteralKind::Int {
      base: Base::Binary,
      empty_int: !has_digits,
    }
  }

  fn lex_octal(&mut self) -> LiteralKind {
    let has_digits = self.consume_digits(|c| c >= '0' && c <= '7');

    LiteralKind::Int {
      base: Base::Octal,
      empty_int: !has_digits,
    }
  }

  fn lex_hexadecimal(&mut self) -> LiteralKind {
    let has_digits = self.consume_digits(|c| c.is_ascii_hexdigit());

    LiteralKind::Int {
      base: Base::Hexadecimal,
      empty_int: !has_digits,
    }
  }

  fn lex_decimal(&mut self) -> LiteralKind {
    // Consume integer part
    let has_int_digits = self.consume_digits(|c| c.is_ascii_digit());

    let mut has_dot = false;
    let mut has_frac_digits = false;

    // Check for decimal point
    if self.peek() == Some('.') {
      // Peek ahead to ensure it's not a range operator (..)
      let next = self.peek_next(1);
      if next.is_some() && next != Some('.') && !next.unwrap().is_ascii_alphabetic() {
        has_dot = true;
        self.advance(); // consume '.'

        // Consume fractional part
        has_frac_digits = self.consume_digits(|c| c.is_ascii_digit());
      }
    }

    // Check for exponent
    let (has_exponent, empty_exponent) = self.try_consume_exponent();

    // Determine literal type
    if has_dot || has_exponent {
      LiteralKind::Float {
        base: Base::Decimal,
        empty_exponent,
      }
    } else {
      LiteralKind::Int {
        base: Base::Decimal,
        empty_int: !has_int_digits,
      }
    }
  }

  /// Try to consume an exponent part (e/E followed by optional sign and digits)
  /// Returns (has_exponent, empty_exponent)
  fn try_consume_exponent(&mut self) -> (bool, bool) {
    if let Some(c) = self.peek() {
      if c == 'e' || c == 'E' {
        self.advance(); // consume 'e' or 'E'

        // Optional sign
        if let Some(sign) = self.peek() {
          if sign == '+' || sign == '-' {
            self.advance();
          }
        }

        // Consume exponent digits
        let has_exp_digits = self.consume_digits(|c| c.is_ascii_digit());

        return (true, !has_exp_digits);
      }
    }

    (false, false)
  }

  pub fn lex_string(&mut self, engine: &mut DiagnosticEngine) -> Option<TokenKind> {
    let lexeme = self.get_current_lexeme();
    let next = self.peek();

    // Raw string variants
    if lexeme == "r" && next == Some('"') {
      return self.lex_raw_str(engine);
    } else if lexeme == "r" && next == Some('#') {
      return self.lex_raw_str(engine);
    } else if lexeme == "br" && next == Some('"') {
      return self.lex_raw_byte_str(engine);
    } else if lexeme == "br" && next == Some('#') {
      return self.lex_raw_byte_str(engine);
    } else if lexeme == "cr" && next == Some('"') {
      return self.lex_raw_cstr(engine);
    } else if lexeme == "cr" && next == Some('#') {
      return self.lex_raw_cstr(engine);
    }
    // Regular string variants
    else if lexeme == "b" && next == Some('\'') {
      return self.lex_bchar(engine);
    } else if lexeme == "b" && next == Some('"') {
      return self.lex_bstr(engine);
    } else if lexeme == "c" && next == Some('"') {
      return self.lex_cstr(engine);
    } else if lexeme == "\'" {
      return self.lex_char(engine);
    } else if lexeme == "\"" {
      return self.lex_str(engine);
    }

    Some(TokenKind::Literal {
      kind: LiteralKind::Str { terminated: false },
      suffix_start: 0,
    })
  }

  /// Consumes characters until the closing delimiter, handling escape sequences
  /// Returns true if properly terminated
  fn consume_until_delimiter(&mut self, delimiter: char, allow_escapes: bool) -> bool {
    while let Some(c) = self.peek() {
      // Stop at newline (unterminated)
      if c == '\n' {
        return false;
      }

      // Handle escape sequences
      if allow_escapes && c == '\\' {
        self.advance(); // consume backslash
        if let Some(escaped) = self.peek() {
          self.advance(); // consume escaped character
                          // Special handling for unicode escapes in char literals
          if escaped == 'u' && self.peek() == Some('{') {
            self.advance(); // consume '{'
                            // Consume until closing '}'
            while let Some(uc) = self.peek() {
              self.advance();
              if uc == '}' {
                break;
              }
            }
          }
        }
        continue;
      }

      // Check for closing delimiter
      if c == delimiter {
        self.advance();
        return true;
      }

      self.advance();
    }

    false
  }

  /// Emit an unterminated literal diagnostic
  fn emit_unterminated_diagnostic(
    &self,
    engine: &mut DiagnosticEngine,
    literal_type: &str,
    quote_type: &str,
  ) {
    let diagnostic = Diagnostic::new(
      DiagnosticCode::Error(DiagnosticError::UnterminatedString),
      format!(
        "Unterminated {} literal: {}",
        literal_type,
        self.get_current_lexeme()
      ),
      self.source.path.to_string(),
    )
    .with_label(
      diagnostic::Span::new(self.start, self.current),
      Some(format!("This {} literal is not terminated", literal_type)),
      LabelStyle::Primary,
    )
    .with_help(format!("Use {} for {} literals.", quote_type, literal_type));

    engine.add(diagnostic);
  }

  fn lex_cstr(&mut self, engine: &mut DiagnosticEngine) -> Option<TokenKind> {
    self.advance(); // consume 'c'

    let terminated = self.consume_until_delimiter('"', true);

    if !terminated {
      self.emit_unterminated_diagnostic(engine, "C string", "double quotes");
    }

    Some(TokenKind::Literal {
      kind: LiteralKind::CStr { terminated },
      suffix_start: self.current as u32,
    })
  }

  fn lex_bstr(&mut self, engine: &mut DiagnosticEngine) -> Option<TokenKind> {
    self.advance(); // consume 'b'

    let terminated = self.consume_until_delimiter('"', true);

    if !terminated {
      self.emit_unterminated_diagnostic(engine, "byte string", "double quotes");
    }

    Some(TokenKind::Literal {
      kind: LiteralKind::ByteStr { terminated },
      suffix_start: self.current as u32,
    })
  }

  fn lex_bchar(&mut self, engine: &mut DiagnosticEngine) -> Option<TokenKind> {
    self.advance(); // consume 'b'

    let start_pos = self.current;
    let terminated = self.consume_until_delimiter('\'', true);

    if !terminated {
      self.emit_unterminated_diagnostic(engine, "byte character", "single quotes");
      return Some(TokenKind::Literal {
        kind: LiteralKind::Byte { terminated: false },
        suffix_start: self.current as u32,
      });
    }

    // Validate byte char length (should be b'x' or b'\x')
    let content_len = self.current - start_pos - 1; // -1 for closing quote
    let lexeme = self.get_current_lexeme();

    // Check if it's an escape sequence
    let is_escape = lexeme.contains('\\');

    if content_len > 2 && !is_escape {
      let diagnostic = Diagnostic::new(
        DiagnosticCode::Error(DiagnosticError::UnterminatedString),
        format!("Too many characters in byte char literal: {}", lexeme),
        self.source.path.to_string(),
      )
      .with_label(
        diagnostic::Span::new(self.start, self.current),
        Some("This byte char literal is too long".to_string()),
        LabelStyle::Primary,
      )
      .with_help(
        "Byte char literals can only contain a single ASCII character or escape sequence."
          .to_string(),
      );

      engine.add(diagnostic);
      return None;
    }

    Some(TokenKind::Literal {
      kind: LiteralKind::Byte { terminated },
      suffix_start: self.current as u32,
    })
  }

  fn lex_char(&mut self, engine: &mut DiagnosticEngine) -> Option<TokenKind> {
    let start_pos = self.current;
    let terminated = self.consume_until_delimiter('\'', true);

    if !terminated {
      let diagnostic = Diagnostic::new(
        DiagnosticCode::Error(DiagnosticError::InvalidCharacter),
        format!("Unterminated char literal: {}", self.get_current_lexeme()),
        self.source.path.to_string(),
      )
      .with_label(
        diagnostic::Span::new(self.start, self.current),
        Some("Unterminated character literal".to_string()),
        LabelStyle::Primary,
      )
      .with_help("Use single quotes for character literals.".to_string());

      engine.add(diagnostic);

      return Some(TokenKind::Literal {
        kind: LiteralKind::Char { terminated: false },
        suffix_start: self.current as u32,
      });
    }

    // Validate char length
    let lexeme = self.get_current_lexeme();
    let content_len = self.current - start_pos - 1; // -1 for closing quote

    // Check for unicode escape
    let is_unicode = lexeme.contains("\\u{");

    if content_len > 1 && !lexeme.contains('\\') {
      // Multiple characters without escape
      let diagnostic = Diagnostic::new(
        DiagnosticCode::Error(DiagnosticError::UnterminatedString),
        format!("Too many characters in char literal: {}", lexeme),
        self.source.path.to_string(),
      )
      .with_label(
        diagnostic::Span::new(self.start, self.current),
        Some("This char literal is too long".to_string()),
        LabelStyle::Primary,
      )
      .with_help(
        "Char literals can only contain a single character or escape sequence.".to_string(),
      );

      engine.add(diagnostic);
      return None;
    } else if is_unicode && !lexeme.ends_with("}'") {
      // Malformed unicode escape
      let diagnostic = Diagnostic::new(
        DiagnosticCode::Error(DiagnosticError::UnterminatedString),
        format!("Invalid unicode escape: {}", lexeme),
        self.source.path.to_string(),
      )
      .with_label(
        diagnostic::Span::new(self.start, self.current),
        Some("Unicode escape sequence is not properly closed".to_string()),
        LabelStyle::Primary,
      )
      .with_help(
        "Use the correct escape sequence format: \\u{HEXDIGITS} (e.g., \\u{1F980})".to_string(),
      );

      engine.add(diagnostic);
      return None;
    }

    Some(TokenKind::Literal {
      kind: LiteralKind::Char { terminated },
      suffix_start: self.current as u32,
    })
  }

  fn lex_str(&mut self, engine: &mut DiagnosticEngine) -> Option<TokenKind> {
    let terminated = self.consume_until_delimiter('"', true);

    if !terminated {
      self.emit_unterminated_diagnostic(engine, "string", "double quotes");
      return None;
    }

    Some(TokenKind::Literal {
      kind: LiteralKind::Str { terminated: true },
      suffix_start: self.current as u32,
    })
  }

  /// Count and consume the opening hash delimiters for raw strings
  /// Returns Ok(n_hashes) or Err(RawStrError)
  fn consume_raw_hashes(&mut self) -> Result<u16, RawStrError> {
    let mut n_hashes = 0usize;

    while let Some(c) = self.peek() {
      if c == '#' {
        self.advance();
        n_hashes += 1;
      } else if c == '"' {
        // Found opening quote, we're done counting
        break;
      } else {
        // Invalid character between r and opening quote
        return Err(RawStrError::InvalidStarter { bad_char: c });
      }
    }

    // Check if we exceeded u16::MAX
    if n_hashes > u16::MAX as usize {
      return Err(RawStrError::TooManyDelimiters { found: n_hashes });
    }

    Ok(n_hashes as u16)
  }

  /// Consume characters until we find the closing delimiter with matching hashes
  /// Returns Ok(()) or Err(RawStrError)
  fn consume_raw_string_content(&mut self, n_hashes: u16) -> Result<(), RawStrError> {
    let start_offset = self.current;

    // Expect opening quote
    if self.peek() != Some('"') {
      return Err(RawStrError::InvalidStarter {
        bad_char: self.peek().unwrap_or('\0'),
      });
    }
    self.advance(); // consume opening "

    loop {
      match self.peek() {
        None => {
          // End of file without termination
          return Err(RawStrError::NoTerminator {
            expected: n_hashes as usize,
            found: 0,
            possible_terminator_offset: None,
          });
        },
        Some('"') => {
          // Found potential closing quote, check if hashes match
          let quote_offset = self.current - start_offset;
          self.advance(); // consume "

          let mut found_hashes = 0usize;
          while self.peek() == Some('#') {
            self.advance();
            found_hashes += 1;

            // If we've found enough hashes, check if we're done
            if found_hashes == n_hashes as usize {
              // Check if next char is not another hash
              if self.peek() != Some('#') {
                // Successfully terminated!
                return Ok(());
              }
              // Otherwise, this is too many hashes, keep going
            }
          }

          // Not enough hashes or wrong number
          // If we found at least one hash, this was a possible terminator
          let possible_offset = if found_hashes > 0 {
            Some(quote_offset)
          } else {
            None
          };

          // If we hit EOF or another quote without matching hashes
          if self.peek().is_none() {
            return Err(RawStrError::NoTerminator {
              expected: n_hashes as usize,
              found: found_hashes,
              possible_terminator_offset: possible_offset,
            });
          }

          // Otherwise, the consumed " and partial hashes are part of content
          // Continue searching
        },
        Some(_) => {
          // Regular content character (no escape processing in raw strings!)
          self.advance();
        },
      }
    }
  }

  /// Emit a raw string diagnostic
  fn emit_raw_string_diagnostic(
    &self,
    engine: &mut DiagnosticEngine,
    error: &RawStrError,
    literal_type: &str,
  ) {
    let (message, help, label) = match error {
      RawStrError::InvalidStarter { bad_char } => (
        format!(
          "Invalid raw {} literal: unexpected character '{}'",
          literal_type, bad_char
        ),
        format!(
          "Only '#' characters are allowed between 'r' and the opening quote. Found '{}'",
          bad_char
        ),
        format!("Unexpected character '{}' in raw string prefix", bad_char),
      ),
      RawStrError::NoTerminator {
        expected,
        found,
        possible_terminator_offset,
      } => {
        let message = format!(
          "Unterminated raw {} literal: {}",
          literal_type,
          self.get_current_lexeme()
        );

        let help = if let Some(_offset) = possible_terminator_offset {
          format!(
            "Expected closing quote with {} hash{}, but found {} hash{}",
            expected,
            if *expected == 1 { "" } else { "es" },
            found,
            if *found == 1 { "" } else { "es" }
          )
        } else {
          format!(
            "Raw string needs closing quote followed by {} hash{} (e.g., \"{})",
            expected,
            if *expected == 1 { "" } else { "es" },
            "#".repeat(*expected)
          )
        };

        let label = if *found > 0 {
          format!(
            "Expected {} hash{}, found {}",
            expected,
            if *expected == 1 { "" } else { "es" },
            found
          )
        } else {
          "Raw string literal is not terminated".to_string()
        };

        (message, help, label)
      },
      RawStrError::TooManyDelimiters { found } => (
        format!("Too many hash delimiters in raw {} literal", literal_type),
        format!(
          "Found {} hashes, but the maximum is {} (u16::MAX)",
          found,
          u16::MAX
        ),
        format!("This raw string has {} hash delimiters", found),
      ),
    };

    let diagnostic = Diagnostic::new(
      DiagnosticCode::Error(DiagnosticError::UnterminatedString),
      message,
      self.source.path.to_string(),
    )
    .with_label(
      diagnostic::Span::new(self.start, self.current),
      Some(label),
      LabelStyle::Primary,
    )
    .with_help(help);

    engine.add(diagnostic);
  }

  fn lex_raw_str(&mut self, engine: &mut DiagnosticEngine) -> Option<TokenKind> {
    self.advance(); // consume 'r'

    let n_hashes = match self.consume_raw_hashes() {
      Ok(n) => n,
      Err(err) => {
        self.emit_raw_string_diagnostic(engine, &err, "string");
        return Some(TokenKind::Literal {
          kind: LiteralKind::RawStr {
            n_hashes: 0,
            err: Some(err),
          },
          suffix_start: self.current as u32,
        });
      },
    };

    let err = match self.consume_raw_string_content(n_hashes) {
      Ok(()) => None,
      Err(e) => {
        self.emit_raw_string_diagnostic(engine, &e, "string");
        Some(e)
      },
    };

    Some(TokenKind::Literal {
      kind: LiteralKind::RawStr { n_hashes, err },
      suffix_start: self.current as u32,
    })
  }

  fn lex_raw_byte_str(&mut self, engine: &mut DiagnosticEngine) -> Option<TokenKind> {
    self.advance(); // consume 'b'
    self.advance(); // consume 'r'

    let n_hashes = match self.consume_raw_hashes() {
      Ok(n) => n,
      Err(err) => {
        self.emit_raw_string_diagnostic(engine, &err, "byte string");
        return Some(TokenKind::Literal {
          kind: LiteralKind::RawByteStr {
            n_hashes: 0,
            err: Some(err),
          },
          suffix_start: self.current as u32,
        });
      },
    };

    let err = match self.consume_raw_string_content(n_hashes) {
      Ok(()) => None,
      Err(e) => {
        self.emit_raw_string_diagnostic(engine, &e, "byte string");
        Some(e)
      },
    };

    Some(TokenKind::Literal {
      kind: LiteralKind::RawByteStr { n_hashes, err },
      suffix_start: self.current as u32,
    })
  }

  fn lex_raw_cstr(&mut self, engine: &mut DiagnosticEngine) -> Option<TokenKind> {
    self.advance(); // consume 'c'
    self.advance(); // consume 'r'

    let n_hashes = match self.consume_raw_hashes() {
      Ok(n) => n,
      Err(err) => {
        self.emit_raw_string_diagnostic(engine, &err, "C string");
        return Some(TokenKind::Literal {
          kind: LiteralKind::RawCStr {
            n_hashes: 0,
            err: Some(err),
          },
          suffix_start: self.current as u32,
        });
      },
    };

    let err = match self.consume_raw_string_content(n_hashes) {
      Ok(()) => None,
      Err(e) => {
        self.emit_raw_string_diagnostic(engine, &e, "C string");
        Some(e)
      },
    };

    Some(TokenKind::Literal {
      kind: LiteralKind::RawCStr { n_hashes, err },
      suffix_start: self.current as u32,
    })
  }
}
