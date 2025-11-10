use crate::{ast::Expr, parser_utils::ExprContext, Parser};

use diagnostic::DiagnosticEngine;
use lexer::token::{Token, TokenKind};

impl Parser {
  pub(crate) fn parse_grouped_expr(
    &mut self,
    token: &mut Token,
    engine: &mut DiagnosticEngine,
  ) -> Result<Expr, ()> {
    self.advance(engine); // consume the open paren

    //TODO: later one parse the inner attributes (e.g., #[inline], (#![allow(unused)] x + 1))

    let expr = self.parse_expression(ExprContext::Default, engine)?; // parse the expression
    self.expect(TokenKind::CloseParen, engine)?; // consume the close paren
    token.span.merge(self.current_token().span);

    Ok(Expr::Group {
      expr: Box::new(expr),
      span: token.span,
    })
  }
}
