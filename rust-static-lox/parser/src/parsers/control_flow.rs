use crate::{ast::Expr, match_and_consume, parser_utils::ExprContext, Parser};
use diagnostic::DiagnosticEngine;
use lexer::token::TokenKind;

impl Parser {
  pub(crate) fn parse_if_expression(
    &mut self,
    context: ExprContext,
    engine: &mut DiagnosticEngine,
  ) -> Result<Expr, ()> {
    let mut token = self.current_token();
    self.advance(engine); // consume the "if"
    let condition = self.parse_expression(context, engine)?;

    self.expect(TokenKind::OpenBrace, engine)?;
    let then_branch = self.parse_expression(context, engine)?;
    self.expect(TokenKind::CloseBrace, engine)?;

    let mut else_branch = None;
    if match_and_consume!(self, engine, TokenKind::KwElse)? {
      else_branch = Some(self.parse_expression(context, engine)?);
    }

    token.span.merge(self.current_token().span);
    Ok(Expr::If {
      condition: Box::new(condition),
      then_branch: Box::new(then_branch),
      else_branch: else_branch.map(Box::new),
      span: token.span,
    })
  }
}
