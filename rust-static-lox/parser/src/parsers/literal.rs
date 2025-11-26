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
  /// Parses a literal token and delegates to the appropriate specialized parser.
  ///
  /// Grammar reference (`<literalExpr>`):
  ///
  /// ```bnf
  /// <literalExpr> ::=
  ///       CHAR
  ///     | STRING
  ///     | RAW_STRING
  ///     | BYTE
  ///     | BYTE_STRING
  ///     | RAW_BYTE_STRING
  ///     | C_STRING
  ///     | RAW_C_STRING
  ///     | INTEGER
  ///     | FLOAT
  ///     | "true"
  ///     | "false"
  /// ```
  ///
  /// Notes:
  /// - The lexer already validated structural correctness.
  /// - This phase only extracts the semantic value (numeric parsing,
  ///   char extraction, raw text capture, suffix extraction, etc.).
  /// - Invalid numeric values (overflow, malformed exponent, etc.) emit diagnostics here.
  pub(crate) fn parser_literal(
    &mut self,
    token: &Token,
    kind: LiteralKind,
    engine: &mut DiagnosticEngine,
  ) -> Result<Expr, ()> {
    self.advance(engine);

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

  /// Parses an `<INTEGER>` literal according to the grammar:
  ///
  /// ```bnf
  /// INTEGER ::=
  ///       DEC_INTEGER
  ///     | BIN_INTEGER
  ///     | OCT_INTEGER
  ///     | HEX_INTEGER
  ///
  /// DEC_INTEGER ::= [0-9][0-9_]* (integerSuffix)?
  /// BIN_INTEGER ::= "0b" [01_]+ (integerSuffix)?
  /// OCT_INTEGER ::= "0o" [0-7_]+ (integerSuffix)?
  /// HEX_INTEGER ::= "0x" [0-9A-Fa-f_]+ (integerSuffix)?
  ///
  /// integerSuffix ::= "u8" | "u16" | ... | "i128" | "isize"
  /// ```
  ///
  /// Notes:
  /// - `_` separators are removed before numeric conversion.
  /// - `empty_int=true` means `0x`, `0b`, `0o` without digits (lexer detected this).
  /// - Overflow, invalid suffix, or malformed digits produce diagnostics here.
  fn parser_integer(
    &mut self,
    engine: &mut DiagnosticEngine,
    token: &Token,
    empty_int: bool,
    suffix_start: usize,
    base: lexer::token::Base,
  ) -> Result<Expr, ()> {
    if empty_int {
      let diagnostic = Diagnostic::new(
        DiagnosticCode::Error(DiagnosticError::InvalidLiteral),
        "Invalid integer literal".into(),
        self.source_file.path.clone(),
      )
      .with_label(
        token.span,
        Some("Integer literal has no digits".into()),
        LabelStyle::Primary,
      );
      engine.add(diagnostic);
      return Err(());
    }

    // Extract suffix + numeric portion
    let src = &self.source_file.src;

    let suffix = Some(src[suffix_start..token.span.end].to_string());
    let value_str = &src[token.span.start..suffix_start].replace('_', "");

    let parsed = match base {
      lexer::token::Base::Binary => i128::from_str_radix(&value_str[2..], 2),
      lexer::token::Base::Octal => i128::from_str_radix(&value_str[2..], 8),
      lexer::token::Base::Hexadecimal => i128::from_str_radix(&value_str[2..], 16),
      lexer::token::Base::Decimal => value_str.parse::<i128>(),
    };

    let value = match parsed {
      Ok(v) => v,
      Err(_) => {
        let diagnostic = Diagnostic::new(
          DiagnosticCode::Error(DiagnosticError::InvalidLiteral),
          "Invalid integer literal".into(),
          self.source_file.path.clone(),
        )
        .with_label(
          token.span,
          Some("Integer literal is too large or malformed".into()),
          LabelStyle::Primary,
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

  /// Parses a `<FLOAT>` literal.
  ///
  /// Grammar reference:
  ///
  /// ```bnf
  /// FLOAT ::= DEC_FLOAT (floatSuffix)?
  ///
  /// DEC_FLOAT ::=
  ///       [0-9][0-9_]* "." [0-9_]+ (exponentPart)?
  ///     | [0-9][0-9_]* exponentPart
  ///
  /// exponentPart ::= ("e" | "E") ("+" | "-")? [0-9_]+
  ///
  /// floatSuffix ::= "f32" | "f64"
  /// ```
  ///
  /// Notes:
  /// - Decimal floats only (hex floats are rejected earlier by lexer).
  /// - Underscore separators are stripped before parsing.
  /// - Malformed exponent or overflow produces a diagnostic.
  fn parser_float(
    &mut self,
    engine: &mut DiagnosticEngine,
    token: &Token,
    suffix_start: usize,
    base: lexer::token::Base,
  ) -> Result<Expr, ()> {
    let src = &self.source_file.src;
    let suffix = Some(src[suffix_start..token.span.end].to_string());
    let value_str = &src[token.span.start..suffix_start];

    let parsed = match base {
      lexer::token::Base::Decimal => value_str.replace('_', "").parse::<f64>(),
      _ => Err(())?, // hex/other floats disallowed
    };

    let value = match parsed {
      Ok(v) => v,
      Err(_) => {
        let diagnostic = Diagnostic::new(
          DiagnosticCode::Error(DiagnosticError::InvalidLiteral),
          "Invalid float literal".into(),
          self.source_file.path.clone(),
        )
        .with_label(
          token.span,
          Some("Float literal is too large or malformed".into()),
          LabelStyle::Primary,
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

  /// Parses `<STRING>` literal:
  ///
  /// ```bnf
  /// STRING ::= "\"" (escapedChar | !["\\])* "\""
  /// ```
  ///
  /// Notes:
  /// - Escape resolution was already done by the lexer.
  /// - Here we simply store the reconstructed string.
  fn parser_string(&mut self, _engine: &mut DiagnosticEngine, token: &Token) -> Result<Expr, ()> {
    let raw = &self.source_file.src[token.span.start..token.span.end];
    Ok(Expr::String {
      kind: StrKind::Normal,
      value: raw.into(),
      span: token.span,
    })
  }

  /// Parses `<BYTE_STRING>` literal (`b"..."`).
  fn parser_byte_string(
    &mut self,
    _engine: &mut DiagnosticEngine,
    token: &Token,
  ) -> Result<Expr, ()> {
    let raw = &self.source_file.src[token.span.start..token.span.end];
    Ok(Expr::String {
      kind: StrKind::Byte,
      value: raw.into(),
      span: token.span,
    })
  }

  /// Parses `<C_STRING>` literal (`c"..."`).
  fn parser_c_string(&mut self, _engine: &mut DiagnosticEngine, token: &Token) -> Result<Expr, ()> {
    let raw = &self.source_file.src[token.span.start..token.span.end];
    Ok(Expr::String {
      kind: StrKind::C,
      value: raw.into(),
      span: token.span,
    })
  }

  /// Parses `<RAW_STRING>` literal (`r###"..."###`).
  ///
  /// ```bnf
  /// RAW_STRING ::= "r" "#"* "\"" rawText "\"" "#"*
  /// ```
  fn parser_raw_string(
    &mut self,
    _engine: &mut DiagnosticEngine,
    token: &Token,
    n_hashes: u16,
  ) -> Result<Expr, ()> {
    let raw = &self.source_file.src[token.span.start..token.span.end];
    Ok(Expr::String {
      kind: StrKind::Raw(n_hashes.into()),
      value: raw.into(),
      span: token.span,
    })
  }

  /// Parses `<RAW_BYTE_STRING>` (`br###"..."###`).
  fn parser_raw_byte_string(
    &mut self,
    _engine: &mut DiagnosticEngine,
    token: &Token,
    n_hashes: u16,
  ) -> Result<Expr, ()> {
    let raw = &self.source_file.src[token.span.start..token.span.end];
    Ok(Expr::String {
      kind: StrKind::RawByte(n_hashes.into()),
      value: raw.into(),
      span: token.span,
    })
  }

  /// Parses `<RAW_C_STRING>` (`cr###"..."###`).
  fn parser_raw_c_string(
    &mut self,
    _engine: &mut DiagnosticEngine,
    token: &Token,
    n_hashes: u16,
  ) -> Result<Expr, ()> {
    let raw = &self.source_file.src[token.span.start..token.span.end];
    Ok(Expr::String {
      kind: StrKind::RawC(n_hashes.into()),
      value: raw.into(),
      span: token.span,
    })
  }

  /// Parses `<CHAR>` literal.
  ///
  /// ```bnf
  /// CHAR ::= "'" (escapedChar | unicodeScalar) "'"
  /// ```
  ///
  /// Notes:
  /// - Lexer validated escape rules.
  /// - Only the inner character is extracted here.
  fn parser_char(&mut self, _engine: &mut DiagnosticEngine, token: &Token) -> Result<Expr, ()> {
    let ch = self.source_file.src[token.span.start + 1..token.span.end]
      .chars()
      .next()
      .unwrap();

    Ok(Expr::Char {
      value: ch,
      span: token.span,
    })
  }

  /// Parses `<BYTE>` literal (`b'a'`).
  ///
  /// ```bnf
  /// BYTE ::= "b'" asciiByte "'"
  /// ```
  ///
  /// Notes:
  /// - Must be exactly one ASCII byte.
  /// - Non-ASCII or multi-char already errored in the lexer.
  fn parser_byte(&mut self, engine: &mut DiagnosticEngine, token: &Token) -> Result<Expr, ()> {
    if token.span.start == token.span.end - 1 {
      let diag = Diagnostic::new(
        DiagnosticCode::Error(DiagnosticError::EmptyChar),
        "Invalid byte literal".into(),
        self.source_file.path.clone(),
      )
      .with_label(
        token.span,
        Some("Byte literal is malformed".into()),
        LabelStyle::Primary,
      );
      engine.add(diag);
      return Err(());
    }

    let byte = self.source_file.src[token.span.start + 2..token.span.end]
      .chars()
      .next()
      .unwrap()
      .to_string()
      .as_bytes()[0];

    Ok(Expr::Byte {
      value: byte,
      span: token.span,
    })
  }

  /// Parses boolean literals:
  ///
  /// ```bnf
  /// literalExpr ::= "true" | "false"
  /// ```
  pub(crate) fn parser_bool(
    &mut self,
    token: &mut Token,
    engine: &mut DiagnosticEngine,
  ) -> Result<Expr, ()> {
    let value = self.get_token_lexeme(token).parse::<bool>().unwrap();
    self.advance(engine);
    token.span.merge(self.current_token().span);

    Ok(Expr::Bool {
      value,
      span: token.span,
    })
  }
}
