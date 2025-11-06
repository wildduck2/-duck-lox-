use diagnostic::DiagnosticEngine;

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
      } else if c == '_' && self.peek_next() != Some('_') {
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
      } else if c == '_' && self.peek_next() != Some('_') {
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
          } else if ec == '_' && self.peek_next() != Some('_') {
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
      } else if c == '_' && self.peek_next() != Some('_') {
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
    None
  }
}
