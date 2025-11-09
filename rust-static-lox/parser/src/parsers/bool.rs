use crate::{ast::Expr, Parser};
use diagnostic::DiagnosticEngine;
use lexer::token::Token;

impl Parser {
  pub(crate) fn parser_bool(
    &mut self,
    engine: &mut DiagnosticEngine,
    token: &Token,
  ) -> Result<Expr, ()> {
    let value = self
      .source_file
      .src
      .get(token.span.start..token.span.end)
      .unwrap()
      .parse::<bool>()
      .unwrap();

    self.advance(engine); // consume the identifier

    Ok(Expr::Bool {
      value,
      span: token.span,
    })
  }
}
