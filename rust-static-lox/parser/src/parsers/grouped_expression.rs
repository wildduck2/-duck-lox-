use diagnostic::DiagnosticEngine;
use lexer::token::{Token, TokenKind};

use crate::{ast::Expr, parser_utils::ExprContext, Parser};

impl Parser {
  pub(crate) fn parse_grouped_expr(
    &mut self,
    token: &mut Token,
    engine: &mut DiagnosticEngine,
  ) -> Result<Expr, ()> {
    self.advance(engine); // consume the open paren

    if matches!(self.current_token().kind, TokenKind::CloseParen) {
      self.advance(engine); // consume the close paren
      token.span.merge(self.current_token().span);
      return Ok(Expr::Unit(token.span));
    }

    let mut elements = vec![];

    let attributes = if matches!(self.current_token().kind, TokenKind::Pound) {
      self.parse_attributes(engine)?
    } else {
      vec![]
    };

    while !self.is_eof() && !matches!(self.current_token().kind, TokenKind::CloseParen) {
      if self.current_token().kind == TokenKind::Comma {
        self.advance(engine); // consume the comma
      }

      if self.current_token().kind == TokenKind::CloseParen {
        break;
      }

      elements.push(self.parse_expression(ExprContext::Default, engine)?);
    }

    self.expect(TokenKind::CloseParen, engine)?; // consume the close paren
    token.span.merge(self.current_token().span);

    if elements.len() == 1 {
      Ok(Expr::Group {
        attributes,
        expr: Box::new(elements[0].clone()),
        span: token.span,
      })
    } else {
      Ok(Expr::Tuple {
        attributes,
        elements,
        span: token.span,
      })
    }
  }
}
