use lexer::token::{Token, TokenKind};

use crate::{ast::Expr, parser_utils::ExprContext, DiagnosticEngine, Parser};

impl Parser {
  // *   arrayExpr        → "[" arrayElements? "]" ;
  // *
  // *   arrayElements    → expression (";" expression | ("," expression)* ","?) ;
  pub(crate) fn parse_array_expr(
    &mut self,
    token: &mut Token,
    engine: &mut DiagnosticEngine,
  ) -> Result<Expr, ()> {
    self.advance(engine); // consume the opening bracket

    let (elements, repeat) = self.parse_array_elements(engine)?;

    token.span.merge(self.current_token().span);

    Ok(Expr::Array {
      elements,
      repeat: repeat.map(Box::new),
      span: token.span,
    })
  }

  fn parse_array_elements(
    &mut self,
    engine: &mut DiagnosticEngine,
  ) -> Result<(Vec<Expr>, Option<Expr>), ()> {
    let mut repeat = None;
    let mut elements = vec![];

    // if it's empty, we're done
    if self.current_token().kind == TokenKind::CloseBracket {
      self.advance(engine); // consume the closing bracket
      return Ok((elements, repeat));
    }

    elements.push(self.parse_expression(ExprContext::Default, engine)?);

    while !self.is_eof() && !matches!(self.current_token().kind, TokenKind::CloseBracket) {
      if matches!(self.current_token().kind, TokenKind::Comma) {
        self.advance(engine); // consume the comma
      } else if matches!(self.current_token().kind, TokenKind::Semi) {
        self.advance(engine); // consume the semicolon
        repeat = Some(self.parse_expression(ExprContext::Default, engine)?);
      }

      if self.current_token().kind == TokenKind::CloseBracket {
        break;
      }

      elements.push(self.parse_expression(ExprContext::Default, engine)?);
    }

    self.expect(TokenKind::CloseBracket, engine)?;

    Ok((elements, repeat))
  }
}
