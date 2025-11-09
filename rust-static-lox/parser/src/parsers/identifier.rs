use crate::{ast::Expr, Parser};
use diagnostic::DiagnosticEngine;
use lexer::token::Token;

impl Parser {
  pub(crate) fn parser_ident(
    &mut self,
    engine: &mut DiagnosticEngine,
    token: &Token,
  ) -> Result<Expr, ()> {
    let name = self
      .source_file
      .src
      .get(token.span.start..token.span.end)
      .unwrap()
      .to_string();

    self.advance(engine); // consume the identifier

    Ok(Expr::Ident {
      name,
      span: token.span,
    })
  }
}
