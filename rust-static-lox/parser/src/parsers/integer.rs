use diagnostic::{
  code::DiagnosticCode,
  diagnostic::{Diagnostic, LabelStyle},
  types::error::DiagnosticError,
  DiagnosticEngine,
};
use lexer::token::Token;

use crate::{ast::Expr, Parser};

impl Parser {
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

    Ok(Expr::Integer {
      value: value.unwrap(),
      suffix,
      span: token.span,
    })
  }
}
