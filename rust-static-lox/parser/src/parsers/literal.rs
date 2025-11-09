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
  pub(crate) fn parser_literal(
    &mut self,
    engine: &mut DiagnosticEngine,
    token: &Token,
    kind: LiteralKind,
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

  pub(crate) fn parser_integer(
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

  pub(crate) fn parser_float(
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
          "Float literals must be in decimal, binary, octal, or hexadecimal format.".to_string(),
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
  pub(crate) fn parser_string(
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
      kind: StrKind::Normal,
      value: value.to_string(),
      span: token.span,
    })
  }

  pub(crate) fn parser_byte_string(
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

  pub(crate) fn parser_c_string(
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
      kind: StrKind::C,
      value: value.to_string(),
      span: token.span,
    })
  }

  pub(crate) fn parser_raw_string(
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

  pub(crate) fn parser_raw_byte_string(
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

  pub(crate) fn parser_raw_c_string(
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

  pub(crate) fn parser_char(
    &mut self,
    _engine: &mut DiagnosticEngine,
    token: &Token,
  ) -> Result<Expr, ()> {
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

  pub(crate) fn parser_byte(
    &mut self,
    engine: &mut DiagnosticEngine,
    token: &Token,
  ) -> Result<Expr, ()> {
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
}
