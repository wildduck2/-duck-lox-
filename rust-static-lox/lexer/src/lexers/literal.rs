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
  /// Lex a numeric literal starting at the current offset.
  ///
  /// Detects base by prefix:
  /// - `0b` binary, `0o` octal, `0x` hexadecimal, otherwise decimal
  ///
  /// Delegates to base-specific lexers (including float handling) and
  /// returns `TokenKind::Literal { kind, suffix_start }`.
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

  /// Lex a binary integer: `0b[01_]+`.
  ///
  /// Accepts `_` separators (not doubled). Records `empty_int` if no
  /// digits follow `0b`. Also probes for an optional integer suffix
  /// (e.g. `u8`, `i32`) starting at `suffix_start`.
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
        self.check_suffix_type(c, suffix_start, false, engine);
        break;
      }
    }
    LiteralKind::Int {
      base: Base::Binary,
      empty_int: !has_digits,
      suffix_start,
    }
  }

  /// Lex an octal integer: `0o[0-7_]+`.
  ///
  /// Accepts `_` separators (not doubled). Records `empty_int` if no
  /// digits follow `0o`. Also probes for an optional integer suffix
  /// (e.g. `u16`, `usize`) starting at `suffix_start`.
  fn lex_octal(&mut self, engine: &mut DiagnosticEngine) -> LiteralKind {
    let mut has_digits = false;
    let suffix_start = self.current;
    while let Some(c) = self.peek() {
      if ('0'..='7').contains(&c) {
        self.advance();
        has_digits = true;
      } else {
        self.check_suffix_type(c, suffix_start, false, engine);
        break;
      }
    }
    LiteralKind::Int {
      base: Base::Octal,
      empty_int: !has_digits,
      suffix_start,
    }
  }

  /// Lex a decimal number: integer or float.
  ///
  /// Integer: `[0-9][0-9_]*`
  /// Float forms supported:
  /// - fractional: `123.45`
  /// - exponent: `1e10`, `2.5E-3` (with optional `+`/`-`)
  ///
  /// Tracks `_` separators, flags missing exponent digits, and records
  /// integer/float kind. Probes for suffixes:
  /// - integer: `u8|u16|u32|u64|u128|usize|i8|i16|i32|i64|i128|isize`
  /// - float: `f32|f64`
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
        self.check_suffix_type(c, suffix_start, has_dot || has_exponent, engine);
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

  /// Internal: recognize and consume a numeric **suffix** at the current position.
  ///
  /// Supports:
  /// - integers: `u8|u16|u32|u64|u128|usize|i8|i16|i32|i64|i128|isize`
  /// - floats: `f32|f64` (only if `is_float == true`)
  ///
  /// Returns `true` if a valid suffix was fully consumed, `false` otherwise.
  /// Emits a diagnostic on obviously invalid trailing characters.
  fn check_suffix_type(
    &mut self,
    c: char,
    mut suffix_start: usize,
    is_float: bool,
    engine: &mut DiagnosticEngine,
  ) -> bool {
    if c == 'f' && is_float {
      self.advance(); // consume f

      if self.peek() == Some('3') {
        self.advance();
        if self.peek() == Some('2') {
          self.advance();
          return true;
        }
      } else if self.peek() == Some('6') {
        self.advance();

        if self.peek() == Some('4') {
          self.advance();
          return true;
        }
      }
    }

    if c == 'u' || c == 'i' {
      suffix_start = self.current;
      return self.inner_check_suffix_type(c, suffix_start, engine);
    }

    false
  }

  /// Internal: helper used by `check_suffix_type` to parse concrete integer suffixes.
  /// Consumes characters after the leading `u`/`i`. Returns `true` on success.
  ///
  /// On failure, emits a focused diagnostic pointing at the suffix span.
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

  /// Lex a hexadecimal number (`0x`/`0X`), supporting **ints and hex floats**.
  ///
  /// Integer: `0x[0-9A-Fa-f_]+`
  /// Hex-float (C/Rust-style): `0xA.BCp±E` where exponent is base-2 (`p`/`P`)
  ///   - optional fraction after `.`
  ///   - exponent requires at least one digit; optional `+`/`-`
  ///
  /// Accepts `_` separators (not doubled). Probes for valid suffixes
  /// after the numeric part. Emits diagnostics for malformed exponents.
  fn lex_hexadecimal(&mut self, engine: &mut DiagnosticEngine) -> LiteralKind {
    let mut has_digits = false;
    let mut has_dot = false;
    let mut has_exponent = false;
    let mut has_exp_digits = false;
    let suffix_start = self.current;

    // consume hex digits and optional dot
    while let Some(c) = self.peek() {
      if c.is_ascii_hexdigit() {
        self.advance();
        has_digits = true;
      } else if c == '_' && self.peek_next(1) != Some('_') {
        self.advance();
      } else if c == '.' && !has_dot {
        has_dot = true;
        self.advance();
      } else {
        break;
      }
    }

    // check for exponent part (p or P)
    if let Some(c) = self.peek() {
      if (c == 'p' || c == 'P') {
        has_exponent = true;
        self.advance();

        // optional sign
        if let Some(sign) = self.peek() {
          if sign == '+' || sign == '-' {
            self.advance();
          }
        }

        // exponent digits
        while let Some(ec) = self.peek() {
          if ec.is_ascii_digit() {
            self.advance();
            has_exp_digits = true;
          } else if ec == '_' && self.peek_next(1) != Some('_') {
            self.advance();
          } else {
            break;
          }
        }

        if !has_exp_digits {
          let diag = Diagnostic::new(
            DiagnosticCode::Error(DiagnosticError::InvalidCharacter),
            "Invalid hexadecimal float: missing exponent digits".to_string(),
            self.source.path.to_string(),
          );
          engine.add(diag);
        }
      }
    }

    // suffix check (like u8 or f64)
    if let Some(c) = self.peek() {
      self.check_suffix_type(c, suffix_start, has_dot || has_exponent, engine);
    }

    if has_dot || has_exponent {
      LiteralKind::Float {
        base: Base::Hexadecimal,
        empty_exponent: has_exponent && !has_exp_digits,
      }
    } else {
      LiteralKind::Int {
        base: Base::Hexadecimal,
        empty_int: !has_digits,
        suffix_start,
      }
    }
  }

  /// Dispatch string-like and character-like literals based on the current prefix.
  ///
  /// Routes:
  /// - `"` → normal string
  /// - `b"` / `br` → byte / raw byte string
  /// - `c"` / `cr` → C / raw C string
  /// - `r` + `#*` → raw string
  /// - `'` → character (delegates to `lex_char`)
  ///
  /// Emits “unknown/reserved prefix” diagnostics for unrecognized two-char prefixes.
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

  /// Lex a **raw C string**: `cr"..."`, `cr#"...\"..."#`, etc.
  ///
  /// Counts `#` fences, requires opening `"`, scans until matching `"###...###`.
  /// Emits diagnostics for too many `#` or unterminated literals. Returns
  /// `LiteralKind::RawCStr { n_hashes }`.
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

  /// Lex a **raw string**: `r"..."`, `r#"...\"..."#`, multi-line allowed.
  ///
  /// Counts `#` fences, requires opening `"`, scans until matching terminator.
  /// Includes recovery for probable next-line raw prefixes. Emits diagnostics
  /// for too many `#` or unterminated literals. Returns `LiteralKind::RawStr { n_hashes }`.
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

  /// Lex a **C string**: `c"..."`.
  ///
  /// Normal escape handling (as per your language rules) and unterminated
  /// diagnostics. Returns `LiteralKind::CStr`.
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

  /// Lex a byte string: `b"..."` or raw byte string: `br#"..."#`.
  ///
  /// For `b"..."`, validates escapes (e.g. `\\`, `\"`, `\xNN`) and reports errors.
  /// For `br...`, counts `#` fences and matches closing delimiter.
  ///
  /// Returns `LiteralKind::ByteStr` or `LiteralKind::RawByteStr { n_hashes }`.
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

  /// Lex a **byte character**: `b'X'` or escaped (`b'\xNN'`).
  ///
  /// Enforces single-byte content. Emits diagnostics for unterminated or
  /// overlong forms. Returns `LiteralKind::Byte`.
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

  /// Lex a **character literal**: `'x'`, `'\n'`, `'\x7F'`, `'\u{1F980}'`.
  ///
  /// Validates escapes and reports errors for unterminated forms or bad escapes.
  /// If no closing `'` is found before newline/EOF, falls back to `lex_lifetime`
  /// to interpret leading `'` as a lifetime token. Returns `LiteralKind::Char`.
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

  /// Lex a **normal string**: `"..."`.
  ///
  /// Handles escapes and reports unterminated strings. Returns `LiteralKind::Str`.
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

  /// Helper for string scanning until closing `"` (or newline if `single == true`).
  ///
  /// Understands simple escapes so the scanner doesn’t prematurely stop at `\"`.
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
