use crate::{ast::Expr, match_and_consume, Parser};
use diagnostic::DiagnosticEngine;
use lexer::token::TokenKind;

impl Parser {
  pub(crate) fn parse_block_expression(
    &mut self,
    is_unsafe: bool,
    label: Option<String>,
    engine: &mut DiagnosticEngine,
  ) -> Result<Expr, ()> {
    let mut token = self.current_token();
    if is_unsafe {
      self.expect(TokenKind::KwUnsafe, engine)?;
    }
    self.advance(engine); // consume the "{"

    let mut stmts = vec![];
    while !self.is_eof() && !matches!(self.current_token().kind, TokenKind::CloseBrace) {
      stmts.push(self.parse_stmt(engine)?);
      match_and_consume!(self, engine, TokenKind::Semi)?;
    }
    self.expect(TokenKind::CloseBrace, engine)?;

    token.span.merge(self.current_token().span);
    Ok(Expr::Block {
      stmts,
      label,
      is_unsafe,
      span: token.span,
    })
  }
}
