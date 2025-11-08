//! Lexers for numeric and string literals.
//!
//! Handles:
//! - Numeric literals: integers and floats in decimal, binary, octal, hexadecimal
//! - String literals: regular, byte, C strings, and their raw variants
//! - Character literals: regular and byte characters
//!
//! All literal lexers handle escape sequences and emit diagnostics for malformed literals.

use diagnostic::{
  code::DiagnosticCode,
  diagnostic::{Diagnostic, LabelStyle},
  types::error::DiagnosticError,
  DiagnosticEngine,
};

use crate::{
  token::{Base, LiteralKind, TokenKind},
  Lexer,
};

impl Lexer {
  /// Lexes a numeric literal (integer or float).
  ///
  /// Detects the numeric base from prefixes:
  /// - `0b` - Binary
  /// - `0o` - Octal
  /// - `0x` - Hexadecimal
  /// - No prefix - Decimal
  ///
  /// Handles floats with decimal points and exponential notation.
  ///
  /// # Returns
  ///
  /// `Some(TokenKind::Literal { kind, suffix_start })`
  pub fn lex_number(&mut self, engine: &mut DiagnosticEngine) -> Option<TokenKind> {
    let kind = if self.get_current_lexeme() == "0" {
      if self.match_char('b') {
        self.lex_binary(engine)
      } else if self.match_char('o') {
        self.lex_octal(engine)
      } else if self.match_char('x') {
        self.lex_hexadecimal(engine)
      } else {
        self.lex_decimal(engine)
      }
    } else {
      self.lex_decimal(engine)
    };

    let suffix_start = self.current as u32;
    Some(TokenKind::Literal { kind, suffix_start })
  }

  /// Lexes a binary integer literal (`0b...`).
  ///
  /// Consumes binary digits (0-1) and underscores (for readability).
  /// Sets `empty_int` to `true` if no digits follow the prefix.
  ///
  /// # Returns
  ///
  /// `LiteralKind::Int { base: Base::Binary, empty_int }`
  fn lex_binary(&mut self, engine: &mut DiagnosticEngine) -> LiteralKind {
    let mut has_digits = false;
    let suffix_start = self.current;
    while let Some(c) = self.peek() {
      if c == '0' || c == '1' {
        self.advance();
        has_digits = true;
      } else if c == '_' && self.peek_next(1) != Some('_') {
        self.advance();
        continue;
      } else {
        self.check_suffix_type(c, suffix_start, engine);
        break;
      }
    }
    LiteralKind::Int {
      base: Base::Binary,
      empty_int: !has_digits,
      suffix_start,
    }
  }

  /// Lexes an octal integer literal (`0o...`).
  ///
  /// Consumes octal digits (0-7). Sets `empty_int` to `true` if no digits follow the prefix.
  ///
  /// # Returns
  ///
  /// `LiteralKind::Int { base: Base::Octal, empty_int }`
  fn lex_octal(&mut self, engine: &mut DiagnosticEngine) -> LiteralKind {
    let mut has_digits = false;
    let suffix_start = self.current;
    while let Some(c) = self.peek() {
      if ('0'..='7').contains(&c) {
        self.advance();
        has_digits = true;
      } else {
        self.check_suffix_type(c, suffix_start, engine);
        break;
      }
    }
    LiteralKind::Int {
      base: Base::Octal,
      empty_int: !has_digits,
      suffix_start,
    }
  }

  /// Lexes a decimal integer or float literal.
  ///
  /// Handles:
  /// - Integers: `42`, `1_000_000`
  /// - Floats: `3.14`, `1e10`, `2.5E-3`
  ///
  /// Detects floats by presence of decimal point or exponent marker (`e`/`E`).
  /// Sets `empty_exponent` to `true` if exponent marker exists but no digits follow.
  ///
  /// # Returns
  ///
  /// `LiteralKind::Int` or `LiteralKind::Float` with appropriate base and flags
  fn lex_decimal(&mut self, engine: &mut DiagnosticEngine) -> LiteralKind {
    let mut has_digits = false;
    let mut has_dot = false;
    let mut has_exponent = false;
    let mut has_exp_digits = false;
    let suffix_start = self.current;

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
        self.check_suffix_type(c, suffix_start, engine);
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
        suffix_start,
      }
    }
  }

  fn check_suffix_type(
    &mut self,
    c: char,
    mut suffix_start: usize,
    engine: &mut DiagnosticEngine,
  ) -> bool {
    if c == 'u' || c == 'i' {
      suffix_start = self.current;
      return self.inner_check_suffix_type(c, suffix_start, engine);
    }

    false
  }

  fn inner_check_suffix_type(
    &mut self,
    c: char,
    suffix_start: usize,
    engine: &mut DiagnosticEngine,
  ) -> bool {
    self.advance();
    match self.peek() {
      Some('8') => {
        self.advance();
        false
      },
      Some('1') => {
        self.advance();

        if self.peek() == Some('6') {
          self.advance();
          return true;
        } else if self.peek() == Some('2') {
          self.advance();

          if self.peek() == Some('8') {
            self.advance();
            return true;
          }
        }

        false
      },
      Some('3') => {
        self.advance();

        if self.peek() == Some('2') {
          self.advance();
          return true;
        }

        false
      },
      Some('6') => {
        self.advance();

        if self.peek() == Some('4') {
          self.advance();
          return true;
        }

        false
      },
      Some('s') => {
        self.advance();
        if self.peek() == Some('i') {
          self.advance();

          if self.peek() == Some('z') {
            self.advance();

            if self.peek() == Some('e') {
              self.advance();
              return true;
            }
          }
        }

        false
      },
      _ => {
        let diagnostic = Diagnostic::new(
          DiagnosticCode::Error(DiagnosticError::InvalidCharacter),
          format!("Invalid character: {}", c),
          self.source.path.to_string(),
        )
        .with_label(
          diagnostic::Span::new(suffix_start, self.current),
          Some("Invalid character here".to_string()),
          LabelStyle::Primary,
        )
        .with_help("Invalid character.".to_string());

        engine.add(diagnostic);
        false
      },
    }
  }

  /// Lexes a hexadecimal integer literal (`0x...`).
  ///
  /// Consumes hexadecimal digits (0-9, a-f, A-F) and underscores.
  /// Sets `empty_int` to `true` if no digits follow the prefix.
  ///
  /// # Returns
  ///
  /// `LiteralKind::Int { base: Base::Hexadecimal, empty_int }`
  fn lex_hexadecimal(&mut self, engine: &mut DiagnosticEngine) -> LiteralKind {
    let mut has_digits = false;
    let suffix_start = self.current;
    while let Some(c) = self.peek() {
      if c.is_ascii_hexdigit() {
        self.advance();
        has_digits = true;
      } else if c == '_' && self.peek_next(1) != Some('_') {
        self.advance();
        continue;
      } else {
        self.check_suffix_type(c, suffix_start, engine);
        break;
      }
    }
    LiteralKind::Int {
      base: Base::Hexadecimal,
      empty_int: !has_digits,
      suffix_start,
    }
  }

  /// Dispatches string/character literal lexing based on prefix.
  ///
  /// Routes to appropriate lexer based on the current lexeme and next character:
  /// - `"` or `'` - Regular string or character
  /// - `b"` or `b'` - Byte string or byte character
  /// - `c"` or `cr"` - C string or raw C string
  /// - `r"` or `r#"` - Raw string
  ///
  /// # Arguments
  ///
  /// * `engine` - Diagnostic engine for error reporting
  ///
  /// # Returns
  ///
  /// `Some(TokenKind::Literal)` with appropriate `LiteralKind`, or `` on error
  pub fn lex_string(&mut self, engine: &mut DiagnosticEngine) -> Option<TokenKind> {
    let first = self.get_current_lexeme(); // e.g. "b", "c", "r", or "\""
    let second = self.peek(); // next char, e.g. 'r', '"', etc.

    // Special case: this should never handle character literals
    if first == "'" {
      return self.lex_char(engine);
    }

    // Combine prefix (1–2 chars, e.g. br, cr)
    let mut prefix = first.to_string();
    if let Some(ch) = second {
      if ch.is_ascii_alphabetic() {
        prefix.push(ch);
      }
    }

    // Define allowed and reserved prefixes
    const VALID_PREFIXES: &[&str] = &["b", "br", "c", "cr", "r", "\""];
    const RESERVED_PREFIXES: &[&str] = &["f", "cf", "rf"];

    // Check prefix validity only for potential prefixed literals
    if first != "\"" {
      if RESERVED_PREFIXES.contains(&prefix.as_str()) {
        let diag = Diagnostic::new(
          DiagnosticCode::Error(DiagnosticError::ReservedPrefix),
          format!("'{}' is a reserved prefix for string literals", prefix),
          self.source.path.to_string(),
        )
        .with_label(
          diagnostic::Span::new(self.start, self.current),
          Some("Reserved literal prefix".to_string()),
          LabelStyle::Primary,
        )
        .with_help("This prefix is reserved for future use.".to_string());
        engine.add(diag);
        return Some(TokenKind::ReservedPrefix);
      }

      if !VALID_PREFIXES.contains(&prefix.as_str()) {
        let diag = Diagnostic::new(
          DiagnosticCode::Error(DiagnosticError::UnknownPrefix),
          format!("Unknown literal prefix '{}'", prefix),
          self.source.path.to_string(),
        )
        .with_label(
          diagnostic::Span::new(self.start, self.current),
          Some("Unrecognized literal prefix".to_string()),
          LabelStyle::Primary,
        )
        .with_help("Valid prefixes: b, br, c, cr, r.".to_string());
        engine.add(diag);
        return Some(TokenKind::UnknownPrefix);
      }
    }

    // Dispatch to the right literal lexer
    match (first, second) {
      ("b", Some('"')) => self.lex_bstr(engine),
      ("b", Some('r')) => self.lex_bstr(engine),
      ("c", Some('"')) => self.lex_cstr(engine),
      ("c", Some('r')) => self.lex_craw_str(engine),
      ("r", Some('"')) | ("r", Some('#')) => self.lex_raw_str(engine),
      ("\"", _) => self.lex_str(engine),
      ("b", Some('\'')) => self.lex_bchar(engine),
      _ => None,
    }
  }

  /// Lexes a raw C string literal (`cr#"..."#`).
  ///
  /// Handles hash-delimited raw C strings (e.g., `cr#"text"#`).
  /// Emits diagnostics for unterminated strings or too many hashes (>255).
  ///
  /// # Arguments
  ///
  /// * `engine` - Diagnostic engine for error reporting
  ///
  /// # Returns
  ///
  /// `Some(TokenKind::Literal { kind: LiteralKind::RawCStr { n_hashes } })`
  fn lex_craw_str(&mut self, engine: &mut DiagnosticEngine) -> Option<TokenKind> {
    self.advance();

    const MAX_HASHES: u16 = 255;

    // The 'cr' prefix has already been consumed by the caller.
    // We're now at the first '#' or '"' character.

    // Count the number of '#' characters.
    let mut n_hashes: u16 = 0;
    while self.peek() == Some('#') {
      n_hashes = n_hashes.saturating_add(1);
      self.advance();
    }

    // Optional upper bound for sanity (parity with Rust spec)
    if n_hashes > MAX_HASHES {
      let diag = Diagnostic::new(
        DiagnosticCode::Error(DiagnosticError::TooManyRawStrHashes),
        format!(
          "Raw C string uses {} hashes; maximum is {}.",
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

    // Expect opening quote
    if self.peek() != Some('"') {
      let diag = Diagnostic::new(
        DiagnosticCode::Error(DiagnosticError::UnterminatedString),
        format!(
          "Expected '\"' after 'cr{}' in raw C string literal",
          "#".repeat(n_hashes as usize)
        ),
        self.source.path.to_string(),
      )
      .with_label(
        diagnostic::Span::new(self.start, self.current),
        Some("Expected opening quote here".to_string()),
        LabelStyle::Primary,
      )
      .with_help("Raw C strings must start with cr\"...\", cr#\"...\"#, etc.".to_string());
      engine.add(diag);

      return Some(TokenKind::Literal {
        kind: LiteralKind::RawCStr { n_hashes },
        suffix_start: self.current as u32,
      });
    }

    // Consume opening quote
    self.advance();

    let mut found_end = false;

    // Scan until closing delimiter found
    'outer: while let Some(c) = self.peek() {
      if c == '"' {
        let saved = self.current;
        self.advance(); // consume quote
        let mut matched = 0u16;
        while matched < n_hashes && self.peek() == Some('#') {
          matched += 1;
          self.advance();
        }
        if matched == n_hashes {
          found_end = true;
          break 'outer;
        } else {
          // Not a real closing delimiter; reset
          self.current = saved + 1;
        }
      } else {
        self.advance();
      }
    }

    // Error handling for unterminated literal
    if !found_end {
      let diag = Diagnostic::new(
        DiagnosticCode::Error(DiagnosticError::UnterminatedString),
        format!(
          "Unterminated raw C string literal: expected closing '\"{}'",
          "#".repeat(n_hashes as usize)
        ),
        self.source.path.to_string(),
      )
      .with_label(
        diagnostic::Span::new(self.start, self.current),
        Some("This raw C string literal is not terminated".to_string()),
        LabelStyle::Primary,
      )
      .with_help(format!(
        "Raw C strings must end with \"{h}\" (e.g., cr{h}\"...\"{h}).",
        h = "#".repeat(n_hashes as usize),
      ));
      engine.add(diag);
    }

    Some(TokenKind::Literal {
      kind: LiteralKind::RawCStr { n_hashes },
      suffix_start: self.current as u32,
    })
  }

  /// Lexes a raw string literal (`r#"..."#`).
  ///
  /// Handles hash-delimited raw strings (e.g., `r#"text"#`, `r##"text"##`).
  /// Supports multi-line strings and includes recovery logic for unterminated literals.
  ///
  /// # Arguments
  ///
  /// * `engine` - Diagnostic engine for error reporting
  ///
  /// # Returns
  ///
  /// `Some(TokenKind::Literal { kind: LiteralKind::RawStr { n_hashes } })`
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
        kind: LiteralKind::RawStr { n_hashes },
        suffix_start: self.current as u32,
      });
    }

    // Consume opening quote
    self.advance();
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

      // If we're at a newline, consume exactly one so the next token begins on the next line
      if self.peek() == Some('\n') {
        self.advance();
      }

      // Safety: if nothing moved (e.g., EOF), bump one char to avoid stalling the lexer
      if self.current == recover_start && self.peek().is_some() {
        self.advance();
      }
    }

    Some(TokenKind::Literal {
      kind: LiteralKind::RawStr { n_hashes },
      // Right after the closing delimiter, or after recovery on error.
      suffix_start: self.current as u32,
    })
  }

  /// Lexes a C string literal (`c"..."`).
  ///
  /// C strings are null-terminated and used for FFI interop.
  /// Handles escape sequences and emits diagnostics for unterminated strings.
  ///
  /// # Arguments
  ///
  /// * `engine` - Diagnostic engine for error reporting
  ///
  /// # Returns
  ///
  /// `Some(TokenKind::Literal { kind: LiteralKind::CStr })`
  fn lex_cstr(&mut self, engine: &mut DiagnosticEngine) -> Option<TokenKind> {
    self.advance();
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
    }

    Some(TokenKind::Literal {
      kind: LiteralKind::CStr,
      suffix_start: self.current as u32,
    })
  }

  /// Lexes a byte string literal (`b"..."` or `br#"..."#`).
  ///
  /// Handles both regular byte strings and raw byte strings.
  /// Validates escape sequences and emits diagnostics for errors.
  ///
  /// # Arguments
  ///
  /// * `engine` - Diagnostic engine for error reporting
  ///
  /// # Returns
  ///
  /// `Some(TokenKind::Literal)` with `LiteralKind::ByteStr` or `LiteralKind::RawByteStr`
  fn lex_bstr(&mut self, engine: &mut DiagnosticEngine) -> Option<TokenKind> {
    // The 'b' prefix has already been consumed.
    if self.peek() == Some('r') {
      // --- RAW BYTE STRING ---
      self.advance(); // consume 'r'
      let mut n_hashes: u16 = 0;
      while self.peek() == Some('#') {
        n_hashes = n_hashes.saturating_add(1);
        self.advance();
      }

      if self.peek() != Some('"') {
        let diag = Diagnostic::new(
          DiagnosticCode::Error(DiagnosticError::UnterminatedString),
          format!(
            "Expected '\"' after 'br{}' in raw byte string literal",
            "#".repeat(n_hashes as usize)
          ),
          self.source.path.to_string(),
        )
        .with_label(
          diagnostic::Span::new(self.start, self.current),
          Some("Expected opening quote here".to_string()),
          LabelStyle::Primary,
        )
        .with_help("Raw byte strings must start with br\"...\", br#\"...\"#, etc.".to_string());
        engine.add(diag);
        return Some(TokenKind::Literal {
          kind: LiteralKind::RawByteStr { n_hashes },
          suffix_start: self.current as u32,
        });
      }

      self.advance(); // consume opening quote

      let mut found_end = false;
      'outer: while let Some(c) = self.peek() {
        if c == '"' {
          let saved = self.current;
          self.advance();
          let mut matched = 0u16;
          while matched < n_hashes && self.peek() == Some('#') {
            matched += 1;
            self.advance();
          }
          if matched == n_hashes {
            found_end = true;
            break 'outer;
          } else {
            self.current = saved + 1;
          }
        } else {
          self.advance();
        }
      }

      if !found_end {
        let diag = Diagnostic::new(
          DiagnosticCode::Error(DiagnosticError::UnterminatedString),
          format!(
            "Unterminated raw byte string literal: expected closing '\"{}'",
            "#".repeat(n_hashes as usize)
          ),
          self.source.path.to_string(),
        )
        .with_label(
          diagnostic::Span::new(self.start, self.current),
          Some("This raw byte string literal is not terminated".to_string()),
          LabelStyle::Primary,
        )
        .with_help(format!(
          "Raw byte strings must end with \"{h}\" (e.g., br{h}\"...\"{h}).",
          h = "#".repeat(n_hashes as usize)
        ));
        engine.add(diag);
      }

      return Some(TokenKind::Literal {
        kind: LiteralKind::RawByteStr { n_hashes },
        suffix_start: self.current as u32,
      });
    }

    // --- NORMAL BYTE STRING ---
    if self.peek() != Some('"') {
      let diag = Diagnostic::new(
        DiagnosticCode::Error(DiagnosticError::InvalidStringStart),
        "Expected '\"' after 'b' in byte string literal".to_string(),
        self.source.path.to_string(),
      )
      .with_label(
        diagnostic::Span::new(self.start, self.current),
        Some("Expected opening quote here".to_string()),
        LabelStyle::Primary,
      )
      .with_help("Byte strings must start with b\"...\" or br\"...\".".to_string());
      engine.add(diag);
      return Some(TokenKind::Literal {
        kind: LiteralKind::ByteStr,
        suffix_start: self.current as u32,
      });
    }

    self.advance(); // consume opening quote
    let mut terminated = false;

    while let Some(c) = self.peek() {
      self.advance();

      match c {
        '\\' => {
          // handle escapes inside normal b"..." literal
          match self.peek() {
            Some('n') | Some('r') | Some('t') | Some('\\') | Some('"') | Some('0') => {
              self.advance(); // valid simple escape
            },
            Some('x') => {
              self.advance(); // consume x
              let mut count = 0;
              while count < 2
                && self
                  .peek()
                  .map(|ch| ch.is_ascii_hexdigit())
                  .unwrap_or(false)
              {
                self.advance();
                count += 1;
              }
              if count < 2 {
                let diag = Diagnostic::new(
                  DiagnosticCode::Error(DiagnosticError::InvalidEscape),
                  "Invalid \\x escape: needs two hex digits".to_string(),
                  self.source.path.to_string(),
                );
                engine.add(diag);
              }
            },
            _ => {
              // invalid escape
              let diag = Diagnostic::new(
                DiagnosticCode::Error(DiagnosticError::InvalidEscape),
                "Invalid escape sequence in byte string".to_string(),
                self.source.path.to_string(),
              )
              .with_label(
                diagnostic::Span::new(self.current - 1, self.current),
                Some("Unknown escape".to_string()),
                LabelStyle::Primary,
              );
              engine.add(diag);
              // consume one char if exists to prevent infinite loop
              if self.peek().is_some() {
                self.advance();
              }
            },
          }
        },
        '"' => {
          terminated = true;
          break;
        },
        _ => {},
      }
    }

    if !terminated {
      let diag = Diagnostic::new(
        DiagnosticCode::Error(DiagnosticError::UnterminatedString),
        format!(
          "Unterminated byte string literal starting at {}",
          self.start
        ),
        self.source.path.to_string(),
      )
      .with_label(
        diagnostic::Span::new(self.start, self.current),
        Some("This byte string literal is not terminated".to_string()),
        LabelStyle::Primary,
      )
      .with_help("Byte strings must end with a closing '\"'.".to_string());
      engine.add(diag);
    }

    Some(TokenKind::Literal {
      kind: LiteralKind::ByteStr,
      suffix_start: self.current as u32,
    })
  }

  /// Lexes a byte character literal (`b'...'`).
  ///
  /// Byte characters must be ASCII (single byte). Emits diagnostics for
  /// unterminated literals or characters that are too long.
  ///
  /// # Arguments
  ///
  /// * `engine` - Diagnostic engine for error reporting
  ///
  /// # Returns
  ///
  /// `Some(TokenKind::Literal { kind: LiteralKind::Byte { terminated } })` or `None` on error
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

    if !self.get_current_lexeme().ends_with('\'') {
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

      return Some(TokenKind::Unknown);
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
      return Some(TokenKind::Unknown);
    }

    Some(TokenKind::Literal {
      kind: LiteralKind::Byte,
      suffix_start: self.current as u32,
    })
  }

  /// Lexes a character literal (`'...'`).
  ///
  /// Handles Unicode characters and escape sequences:
  /// - Simple escapes: `\n`, `\t`, `\\`, `\'`
  /// - Hex escapes: `\x7F`
  /// - Unicode escapes: `\u{1F980}`
  ///
  /// Emits diagnostics for unterminated literals or invalid escapes.
  ///
  /// # Arguments
  ///
  /// * `engine` - Diagnostic engine for error reporting
  ///
  /// # Returns
  ///
  /// `Some(TokenKind::Literal { kind: LiteralKind::Char { terminated } })`
  fn lex_char(&mut self, engine: &mut DiagnosticEngine) -> Option<TokenKind> {
    let mut terminated = false;

    // Consume opening '
    self.advance();

    while let Some(c) = self.peek() {
      match c {
        '\'' => {
          self.advance();
          terminated = true;
          break;
        },
        '\\' => {
          self.advance(); // consume '\'
          match self.peek() {
            Some('n' | 'r' | 't' | '\\' | '\'' | '0') => {
              self.advance();
            },
            Some('x') => {
              // Hex escape: \xNN
              self.advance();
              let mut count = 0;
              while count < 2
                && self
                  .peek()
                  .map(|ch| ch.is_ascii_hexdigit())
                  .unwrap_or(false)
              {
                self.advance();
                count += 1;
              }
              if count < 2 {
                let diag = Diagnostic::new(
                  DiagnosticCode::Error(DiagnosticError::InvalidEscape),
                  "Invalid \\x escape: needs two hex digits".to_string(),
                  self.source.path.to_string(),
                );
                engine.add(diag);
              }
            },
            Some('u') if self.peek_next(1) == Some('{') => {
              // Unicode escape: \u{...}
              self.advance(); // consume 'u'
              self.advance(); // consume '{'
              while let Some(ch) = self.peek() {
                if ch == '}' {
                  break;
                }
                if !ch.is_ascii_hexdigit() {
                  let diag = Diagnostic::new(
                    DiagnosticCode::Error(DiagnosticError::InvalidEscape),
                    "Invalid Unicode escape: non-hex digit".to_string(),
                    self.source.path.to_string(),
                  );
                  engine.add(diag);
                  break;
                }
                self.advance();
              }
              if self.peek() == Some('}') {
                self.advance(); // consume '}'
              } else {
                let diag = Diagnostic::new(
                  DiagnosticCode::Error(DiagnosticError::UnterminatedString),
                  "Unterminated Unicode escape, missing '}'".to_string(),
                  self.source.path.to_string(),
                );
                engine.add(diag);
              }
            },
            _ => {
              let diag = Diagnostic::new(
                DiagnosticCode::Error(DiagnosticError::InvalidEscape),
                "Unknown escape sequence in char literal".to_string(),
                self.source.path.to_string(),
              );
              engine.add(diag);
              if self.peek().is_some() {
                self.advance();
              }
            },
          }
        },
        '\n' => {
          break;
        },
        _ => {
          self.advance(); // normal ASCII char
        },
      }
    }

    if !terminated {
      return self.lex_lifetime(engine);
    }

    Some(TokenKind::Literal {
      kind: LiteralKind::Char,
      suffix_start: self.current as u32,
    })
  }

  /// Lexes a regular string literal (`"..."`).
  ///
  /// Handles escape sequences and multi-line strings.
  /// Emits diagnostics for unterminated strings.
  ///
  /// # Arguments
  ///
  /// * `engine` - Diagnostic engine for error reporting
  ///
  /// # Returns
  ///
  /// `Some(TokenKind::Literal { kind: LiteralKind::Str { terminated } })` or `None` on error
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

      // Return the unterminated string token so parser can continue
      return Some(TokenKind::Literal {
        kind: LiteralKind::Str,
        suffix_start: self.current as u32,
      });
    }

    Some(TokenKind::Literal {
      kind: LiteralKind::Str,
      suffix_start: self.current as u32,
    })
  }

  /// Helper function to lex a string line until closing quote or newline.
  ///
  /// Handles escape sequences (`\"`, `\\`). If `single` is `true`, stops at newline;
  /// otherwise allows multi-line strings.
  ///
  /// # Arguments
  ///
  /// * `single` - If `true`, stop at newline; if `false`, allow multi-line
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
