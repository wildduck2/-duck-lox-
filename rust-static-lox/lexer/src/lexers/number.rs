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
  DiagnosticEngine, Span,
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

    let number = self.get_current_lexeme();
    if number.starts_with('_') || number.ends_with('_') {
      let bad_span = diagnostic::Span::new(self.start, self.current);

      let diag = Diagnostic::new(
        DiagnosticCode::Error(DiagnosticError::InvalidInteger),
        "invalid integer literal".to_string(),
        self.source.path.to_string(),
      )
      .with_label(
        bad_span,
        Some(format!("`{number}` is not a valid integer literal")),
        LabelStyle::Primary,
      )
      .with_help(
        "underscores may be used only **between digits**, never at the start or end".to_string(),
      )
      .with_note("examples of valid literals: `1_000`, `0xFF_A0`, `123`".to_string())
      .with_note("examples of invalid literals: `_123`, `123_`, `0x_12`".to_string());

      engine.add(diag);
      return None;
    }

    Some(TokenKind::Literal { kind })
  }

  /// Lex a binary integer: `0b[01_]+`.
  ///
  /// Accepts `_` separators (not doubled). Records `empty_int` if no
  /// digits follow `0b`. Also probes for an optional integer suffix
  /// (e.g. `u8`, `i32`) starting at `suffix_start`.
  fn lex_binary(&mut self, engine: &mut DiagnosticEngine) -> LiteralKind {
    let mut empty_int = false;
    let mut suffix_start = 0;
    while let Some(c) = self.peek() {
      if c == '0' || c == '1' {
        self.advance();
        empty_int = true;
      } else if c == '_' && self.peek_next(1) != Some('_') {
        self.advance();
        continue;
      } else {
        self.check_suffix_type(c, &mut suffix_start, false, engine);
        break;
      }
    }

    if suffix_start == 0 {
      suffix_start = self.current;
    }

    LiteralKind::Integer {
      base: Base::Binary,
      empty_int: !empty_int,
      suffix_start,
    }
  }

  /// Lex an octal integer: `0o[0-7_]+`.
  ///
  /// Accepts `_` separators (not doubled). Records `empty_int` if no
  /// digits follow `0o`. Also probes for an optional integer suffix
  /// (e.g. `u16`, `usize`) starting at `suffix_start`.
  fn lex_octal(&mut self, engine: &mut DiagnosticEngine) -> LiteralKind {
    let mut empty_int = false;
    let mut suffix_start = 0;
    while let Some(c) = self.peek() {
      if ('0'..='7').contains(&c) {
        self.advance();
        empty_int = true;
      } else {
        self.check_suffix_type(c, &mut suffix_start, false, engine);
        break;
      }
    }

    if suffix_start == 0 {
      suffix_start = self.current;
    }

    LiteralKind::Integer {
      base: Base::Octal,
      empty_int: !empty_int,
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
    let mut has_dot = false;
    let mut has_exponent = false;
    let mut suffix_start = 0;

    while let Some(c) = self.peek() {
      if c.is_ascii_digit() {
        self.advance();
      } else if c == '_' && self.peek_next(1) != Some('_') {
        self.advance();
        continue;
      } else if c == '.' && !has_dot && !has_exponent {
        // NOTE: only treat '.' as float part if it's NOT immediately followed by an identifier or '('
        // and if the next char is a digit (i.e., part of a fractional number).
        let next = self.peek_next(1);
        if let Some(next_ch) = next {
          if next_ch.is_ascii_digit() {
            has_dot = true;
            self.advance(); // consume '.'
          } else {
            break; // e.g., tuple.0 → stop number lexing, '.' belongs to field access
          }
        } else {
          break;
        }
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
          } else if ec == '_' && self.peek_next(1) != Some('_') {
            self.advance();
            continue;
          } else {
            break;
          }
        }
      } else {
        self.check_suffix_type(c, &mut suffix_start, has_dot || has_exponent, engine);
        break;
      }
    }

    if suffix_start == 0 {
      suffix_start = self.current;
    }

    if has_dot || has_exponent {
      LiteralKind::Float {
        base: Base::Decimal,
        suffix_start,
      }
    } else {
      LiteralKind::Integer {
        base: Base::Decimal,
        // NOTE:  all the decimal number are not empty
        empty_int: false,
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
    suffix_start: &mut usize,
    is_float: bool,
    engine: &mut DiagnosticEngine,
  ) -> bool {
    *suffix_start = self.current;
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
      let value = self.inner_check_suffix_type(c, suffix_start, engine);
      return value;
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
    suffix_start: &mut usize,
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
          diagnostic::Span::new(*suffix_start, self.current),
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
    let mut empty_int = false;
    let mut has_dot = false;
    let mut has_exponent = false;
    let mut has_exp_digits = false;
    let mut suffix_start = 0;

    // consume hex digits and optional dot
    while let Some(c) = self.peek() {
      if c.is_ascii_hexdigit() {
        self.advance();
        empty_int = true;
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
      if c == 'p' || c == 'P' {
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
      self.check_suffix_type(c, &mut suffix_start, has_dot || has_exponent, engine);
    }

    // if the suffix start is 0 we set to to the current position, hence this is the end of the
    // actual value and if there's a suffix it will be parsed as a suffix
    if suffix_start == 0 {
      suffix_start = self.current;
    }

    if has_dot || has_exponent {
      LiteralKind::Float {
        base: Base::Hexadecimal,
        suffix_start,
      }
    } else {
      LiteralKind::Integer {
        base: Base::Hexadecimal,
        empty_int: !empty_int,
        suffix_start,
      }
    }
  }
}
