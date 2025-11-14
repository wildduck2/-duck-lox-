//! TODO: Literal parser missing features compared to full Rust grammar.
//!
//! Integer literals:
//! - Validate suffix names (u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize).
//! - Reject invalid or unknown suffixes.
//! - Validate binary digits (only 0 and 1).
//! - Validate octal digits (only 0 to 7).
//! - Validate hex digits (0 to 9 and a to f).
//! - Validate placement of underscores (no leading underscore, no trailing underscore,
//!   no consecutive underscores, no underscore directly after prefix like 0x_).
//!
//! Float literals:
//! - Support exponent syntax (1e10, 1.2e3, 5E-4).
//! - Validate exponent digits.
//! - Validate float suffixes (f32, f64).
//! - Distinguish between integer followed by identifier and real float suffix.
//! - Improve diagnostics for malformed floats (two dots, missing digits, bad exponent).
//!
//! String literals:
//! - Validate escape sequences (\" \\n \\t \\r \\0 \\\\ \\xNN \\u{NNNN} etc).
//! - Ensure byte strings (b"...") contain only ASCII after escape processing.
//! - Ensure c strings do not contain interior null bytes.
//! - Validate Unicode escapes inside normal strings.
//!
//! Raw strings:
//! - Validate starting and ending hash counts match.
//! - Validate that interior backslash escapes are not processed.
//!
//! Character literals:
//! - Reject multi character literals like 'ab'.
//! - Support escape sequences ('\\n', '\\t', '\\r', '\\0', '\\\\', '\\xNN', '\\u{NN}').
//! - Validate that exactly one Unicode scalar value remains after escape processing.
//!
//! Byte literals:
//! - Ensure final value is a single ASCII byte.
//! - Support escape sequences (b'\\n', b'\\t', b'\\xNN', b'\\\\').
//! - Reject any non ASCII code after escapes.
//!
//! General literal parsing:
//! - Improve span merging for better diagnostics.
//! - Add more specific errors for malformed prefixes (0b, 0o, 0x).
//! - Add more specific errors for invalid raw string delimiters.
//! - Provide suggestions when a float was intended but parsed as integer.
//! - Provide suggestions when underscores appear in invalid places.
//! - Ensure all literal errors point at the exact offending character.
//

use diagnostic::{
  code::DiagnosticCode,
  diagnostic::{Diagnostic, LabelStyle},
  types::error::DiagnosticError,
  DiagnosticEngine,
};
use lexer::token::{LiteralKind, Token};

use crate::{
  ast::{Expr, StrKind},
  Parser,
};

impl Parser {
  /// Parses a literal token and dispatches to the appropriate literal parser.
  /// Returns an error if the literal is malformed.
  pub(crate) fn parser_literal(
    &mut self,
    token: &Token,
    kind: LiteralKind,
    engine: &mut DiagnosticEngine,
  ) -> Result<Expr, ()> {
    self.advance(engine); // consume the literal token

    match kind {
      LiteralKind::Integer {
        suffix_start,
        base,
        empty_int,
      } => self.parser_integer(engine, token, empty_int, suffix_start, base),
      LiteralKind::Float { suffix_start, base } => {
        self.parser_float(engine, token, suffix_start, base)
      },
      LiteralKind::Str => self.parser_string(engine, token),
      LiteralKind::ByteStr => self.parser_byte_string(engine, token),
      LiteralKind::CStr => self.parser_c_string(engine, token),
      LiteralKind::RawStr { n_hashes } => self.parser_raw_string(engine, token, n_hashes),
      LiteralKind::RawByteStr { n_hashes } => self.parser_raw_byte_string(engine, token, n_hashes),
      LiteralKind::RawCStr { n_hashes } => self.parser_raw_c_string(engine, token, n_hashes),
      LiteralKind::Char => self.parser_char(engine, token),
      LiteralKind::Byte => self.parser_byte(engine, token),
    }
  }

  /* -------------------------------------------------------------------------------------------- */
  /*                                         Number Literal Parsing                                 */
  /* -------------------------------------------------------------------------------------------- */

  /// Parses integer literals of any base and reports malformed or overflowing values.
  fn parser_integer(
    &mut self,
    engine: &mut DiagnosticEngine,
    token: &Token,
    empty_int: bool,
    suffix_start: usize,
    base: lexer::token::Base,
  ) -> Result<Expr, ()> {
    // Check for empty integer literal (just a base prefix like "0x" with no digits)
    if empty_int {
      let diagnostic = Diagnostic::new(
        DiagnosticCode::Error(DiagnosticError::InvalidLiteral),
        "Invalid integer literal".to_string(),
        self.source_file.path.clone(),
      )
      .with_label(
        token.span,
        Some("Integer literal has no digits".to_string()),
        LabelStyle::Primary,
      );
      engine.add(diagnostic);
      return Err(());
    }

    // Parse the value based on the base
    let value_str = &self.source_file.src;
    // getting the suffix if the start is not 0, thus the suffix we need is
    // (e.g., u8, i32, etc.)
    let suffix = Some(
      value_str
        .get(suffix_start..token.span.end)
        .unwrap()
        .to_string(),
    );
    let value_str = value_str.get(token.span.start..suffix_start).unwrap();

    let value = match base {
      lexer::token::Base::Binary => {
        // Remove "0b" prefix
        i128::from_str_radix(&value_str[2..], 2)
      },
      lexer::token::Base::Octal => {
        // Remove "0o" prefix
        i128::from_str_radix(&value_str[2..], 8)
      },
      lexer::token::Base::Hexadecimal => {
        // Remove "0x" prefix
        i128::from_str_radix(&value_str[2..], 16)
      },
      lexer::token::Base::Decimal => {
        // Remove underscores if any (Rust allows 1_000_000)
        value_str.replace('_', "").parse::<i128>()
      },
    };

    // check if the parsing was successful and return the value
    let value = match value {
      Ok(v) => v,
      Err(_) => {
        let diagnostic = Diagnostic::new(
          DiagnosticCode::Error(DiagnosticError::InvalidLiteral),
          "Invalid integer literal".to_string(),
          self.source_file.path.clone(),
        )
        .with_label(
          token.span,
          Some("Integer literal is too large or malformed".to_string()),
          LabelStyle::Primary,
        )
        .with_help(
          "Integer literals must be in decimal, binary, octal, or hexadecimal format.".to_string(),
        );
        engine.add(diagnostic);
        return Err(());
      },
    };

    Ok(Expr::Integer {
      value,
      suffix,
      span: token.span,
    })
  }

  /// Parses decimal floating-point literals.
  fn parser_float(
    &mut self,
    engine: &mut DiagnosticEngine,
    token: &Token,
    suffix_start: usize,
    base: lexer::token::Base,
  ) -> Result<Expr, ()> {
    // Parse the value based on the base
    let value_str = &self.source_file.src;
    // getting the suffix if the start is not 0, thus the suffix we need is
    // (e.g., u8, i32, etc.)
    let suffix = Some(
      value_str
        .get(suffix_start..token.span.end)
        .unwrap()
        .to_string(),
    );
    let value_str = value_str.get(token.span.start..suffix_start).unwrap();

    let value = if let lexer::token::Base::Decimal = base {
      // Remove underscores if any (Rust allows 1_000_000)
      value_str.replace('_', "").parse::<f64>()
    } else {
      //
      Err(())?
    };

    let value = match value {
      Ok(v) => v,
      Err(_) => {
        let diagnostic = Diagnostic::new(
          DiagnosticCode::Error(DiagnosticError::InvalidLiteral),
          "Invalid float literal".to_string(),
          self.source_file.path.clone(),
        )
        .with_label(
          token.span,
          Some("Float literal is too large or malformed".to_string()),
          LabelStyle::Primary,
        )
        .with_help(
          "Float literals must be written in decimal form, e.g. `1.0` or `0.5`.".to_string(),
        );
        engine.add(diagnostic);
        return Err(());
      },
    };

    Ok(Expr::Float {
      value,
      suffix,
      span: token.span,
    })
  }

  /* -------------------------------------------------------------------------------------------- */
  /*                                         String Literal Parsing                                 */
  /* -------------------------------------------------------------------------------------------- */
  /// Parses standard UTF-8 string literals.
  fn parser_string(&mut self, _engine: &mut DiagnosticEngine, token: &Token) -> Result<Expr, ()> {
    let value = self
      .source_file
      .src
      .get(token.span.start..token.span.end)
      .unwrap();

    Ok(Expr::String {
      kind: StrKind::Normal,
      value: value.to_string(),
      span: token.span,
    })
  }

  /// Parses byte-string literals (`b"..."`).
  fn parser_byte_string(
    &mut self,
    _engine: &mut DiagnosticEngine,
    token: &Token,
  ) -> Result<Expr, ()> {
    let value = self
      .source_file
      .src
      .get(token.span.start..token.span.end)
      .unwrap();

    Ok(Expr::String {
      kind: StrKind::Byte,
      value: value.to_string(),
      span: token.span,
    })
  }

  /// Parses C-string literals (`c"..."`).
  fn parser_c_string(&mut self, _engine: &mut DiagnosticEngine, token: &Token) -> Result<Expr, ()> {
    let value = self
      .source_file
      .src
      .get(token.span.start..token.span.end)
      .unwrap();

    Ok(Expr::String {
      kind: StrKind::C,
      value: value.to_string(),
      span: token.span,
    })
  }

  /// Parses raw string literals with `n` `#` delimiters.
  fn parser_raw_string(
    &mut self,
    _engine: &mut DiagnosticEngine,
    token: &Token,
    n_hashes: u16,
  ) -> Result<Expr, ()> {
    let value = self
      .source_file
      .src
      .get(token.span.start..token.span.end)
      .unwrap();

    Ok(Expr::String {
      kind: StrKind::Raw(n_hashes.into()),
      value: value.to_string(),
      span: token.span,
    })
  }

  /// Parses raw byte-string literals (`br##"..."##`).
  fn parser_raw_byte_string(
    &mut self,
    _engine: &mut DiagnosticEngine,
    token: &Token,
    n_hashes: u16,
  ) -> Result<Expr, ()> {
    let value = self
      .source_file
      .src
      .get(token.span.start..token.span.end)
      .unwrap();

    Ok(Expr::String {
      kind: StrKind::RawByte(n_hashes.into()),
      value: value.to_string(),
      span: token.span,
    })
  }

  /// Parses raw C-string literals (`cr##"..."##`).
  fn parser_raw_c_string(
    &mut self,
    _engine: &mut DiagnosticEngine,
    token: &Token,
    n_hashes: u16,
  ) -> Result<Expr, ()> {
    let value = self
      .source_file
      .src
      .get(token.span.start..token.span.end)
      .unwrap();

    Ok(Expr::String {
      kind: StrKind::RawC(n_hashes.into()),
      value: value.to_string(),
      span: token.span,
    })
  }

  /// Parses character literals (`'a'`).
  fn parser_char(&mut self, _engine: &mut DiagnosticEngine, token: &Token) -> Result<Expr, ()> {
    let value = self
      .source_file
      .src
      .get(token.span.start + 1..token.span.end)
      .unwrap()
      .chars()
      .next()
      .unwrap();

    Ok(Expr::Char {
      value,
      span: token.span,
    })
  }

  /// Parses byte literals (`b'a'`) ensuring they contain a single ASCII byte.
  fn parser_byte(&mut self, engine: &mut DiagnosticEngine, token: &Token) -> Result<Expr, ()> {
    if token.span.start == token.span.end - 1 {
      let diagnostic = Diagnostic::new(
        DiagnosticCode::Error(DiagnosticError::EmptyChar),
        "Invalid byte literal".to_string(),
        self.source_file.path.clone(),
      )
      .with_label(
        token.span,
        Some("Byte literal is too large or malformed".to_string()),
        LabelStyle::Primary,
      )
      .with_help("Byte literals must be a single ASCII character.".to_string());
      engine.add(diagnostic);
      return Err(());
    }

    let value = self
      .source_file
      .src
      .get(token.span.start + 2..token.span.end)
      .unwrap()
      .chars()
      .next()
      .unwrap()
      // convert to string and get the first byte so this will be a char but in a byte
      .to_string()
      .as_bytes()[0];

    Ok(Expr::Byte {
      value,
      span: token.span,
    })
  }

  /// Parses the `true`/`false` keywords into a boolean literal expression.
  pub(crate) fn parser_bool(
    &mut self,
    token: &mut Token,
    engine: &mut DiagnosticEngine,
  ) -> Result<Expr, ()> {
    let value = self.get_token_lexeme(token).parse::<bool>().unwrap();
    self.advance(engine); // consume the identifier
    token.span.merge(self.current_token().span);

    Ok(Expr::Bool {
      value,
      span: token.span,
    })
  }
}
