use diagnostic::{
  code::DiagnosticCode,
  diagnostic::{Diagnostic, LabelStyle},
  types::error::DiagnosticError,
  DiagnosticEngine,
};
use lexer::token::Token;

use crate::{ast::Expr, Parser};

impl Parser {
  pub(crate) fn parser_float(
    &mut self,
    engine: &mut DiagnosticEngine,
    token: &Token,
    empty_exponent: bool,
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
}
