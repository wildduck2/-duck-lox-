use diagnostic::DiagnosticEngine;
use lexer::token::Token;

use crate::{ast::Expr, Parser};

impl Parser {
  /// Parses a standard identifier expression.
  pub(crate) fn parser_ident(
    &mut self,
    token: &mut Token,
    engine: &mut DiagnosticEngine,
  ) -> Result<Expr, ()> {
    let name = self
      .source_file
      .src
      .get(token.span.start..token.span.end)
      .unwrap()
      .to_string();

    self.advance(engine); // consume the identifier
    token.span.merge(self.current_token().span);

    Ok(Expr::Ident {
      name,
      span: token.span,
    })
  }

  /// Parses contextual keywords (`self`, `super`, `crate`, `Self`) as identifiers.
  pub(crate) fn parse_keyword_ident(
    &mut self,
    token: &mut Token,
    engine: &mut DiagnosticEngine,
  ) -> Result<Expr, ()> {
    let name = self.get_token_lexeme(token);
    self.advance(engine);
    token.span.merge(self.current_token().span);

    Ok(Expr::Ident {
      name,
      span: token.span,
    })
  }
}
