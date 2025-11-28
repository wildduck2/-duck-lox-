use crate::{ast::*, match_and_consume, parser_utils::ExprContext, Parser};
use diagnostic::{
  code::DiagnosticCode,
  diagnostic::{Diagnostic, LabelStyle},
  types::error::DiagnosticError,
  DiagnosticEngine,
};
use lexer::token::TokenKind;

impl Parser {
  pub(crate) fn parse_match_expression(
    &mut self,
    context: ExprContext,
    engine: &mut DiagnosticEngine,
  ) -> Result<Expr, ()> {
    let mut token = self.current_token();
    self.advance(engine); // consume the "match"
    let scrutinee = self.parse_expression(context, engine)?;

    let mut arms = vec![];
    self.expect(TokenKind::OpenBrace, engine)?;
    while !self.is_eof() && !matches!(self.current_token().kind, TokenKind::CloseBrace) {
      arms.push(self.parse_match_arm(context, engine)?);
      self.expect(TokenKind::Comma, engine)?;
    }
    self.expect(TokenKind::CloseBrace, engine)?;

    token.span.merge(self.current_token().span);
    Ok(Expr::Match {
      scrutinee: Box::new(scrutinee),
      arms,
      span: token.span,
    })
  }

  fn parse_match_arm(
    &mut self,
    context: ExprContext,
    engine: &mut DiagnosticEngine,
  ) -> Result<MatchArm, ()> {
    let mut token = self.current_token();
    let attributes = if matches!(self.current_token().kind, TokenKind::Pound) {
      self.parse_attributes(engine)?
    } else {
      vec![]
    };

    let pattern = self.parse_pattern_with_or(context, engine)?;
    let guard = if matches!(self.current_token().kind, TokenKind::KwIf) {
      Some(self.parse_match_guard(context, engine)?)
    } else {
      None
    };

    self.expect(TokenKind::FatArrow, engine)?;
    let is_block = matches!(self.current_token().kind, TokenKind::OpenBrace);
    let body = self.parse_expression(ExprContext::Default, engine)?;

    if is_block {
      self.expect(TokenKind::Comma, engine)?;
    }

    token.span.merge(self.current_token().span);
    println!(
      "debug: {:#?}",
      MatchArm {
        attributes: attributes.clone(),
        pattern: pattern.clone(),
        guard: guard.clone(),
        body: body.clone(),
        span: token.span,
      }
    );

    Ok(MatchArm {
      attributes,
      pattern,
      guard,
      body,
      span: token.span,
    })
  }

  fn parse_match_guard(
    &mut self,
    context: ExprContext,
    engine: &mut DiagnosticEngine,
  ) -> Result<Expr, ()> {
    let mut token = self.current_token();
    self.advance(engine); // consume the "if"
    let condition = self.parse_expression(context, engine)?;

    token.span.merge(self.current_token().span);
    return Ok(Expr::If {
      condition: Box::new(condition),
      then_branch: Box::new(Expr::Block {
        stmts: vec![],
        label: None,
        is_unsafe: false,
        span: token.span,
      }),
      else_branch: None,
      span: token.span,
    });
  }
}
