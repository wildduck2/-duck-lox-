use diagnostic::DiagnosticEngine;
use lexer::token::{LiteralKind, TokenKind};

use crate::{
  ast::{Expr, FieldAccess},
  parser_utils::ExprContext,
  Parser,
};

impl Parser {
  /* -------------------------------------------------------------------------------------------- */
  /*                                     Postfix Parsing                                          */
  /* -------------------------------------------------------------------------------------------- */

  pub(crate) fn parse_postfix(&mut self, engine: &mut DiagnosticEngine) -> Result<Expr, ()> {
    // First, parse the base expression (primary)
    let mut expr = self.parse_primary(engine)?;

    // Then, repeatedly apply postfix operators
    'postfix_find: loop {
      match self.current_token().kind {
        TokenKind::OpenParen => {
          expr = self.parse_method_call(expr, engine)?; // normal call
        },
        TokenKind::Dot => {
          // look ahead for .await vs .foo vs .0
          if self.peek(1).kind == TokenKind::KwAwait {
            expr = self.parse_await(expr, engine)?;
          } else {
            expr = self.parse_field_or_method(expr, engine)?;
          }
        },
        TokenKind::OpenBracket => {
          expr = self.parse_index(expr, engine)?;
        },
        TokenKind::Question => {
          expr = self.parse_try(expr, engine)?;
        },
        _ => break 'postfix_find,
      }
    }

    Ok(expr)
  }

  pub(crate) fn parse_await(
    &mut self,
    expr: Expr,
    engine: &mut DiagnosticEngine,
  ) -> Result<Expr, ()> {
    let mut span = expr.span();

    self.expect(TokenKind::Dot, engine)?;
    let token = self.current_token();
    self.advance(engine);
    span.merge(token.span);

    Ok(Expr::Await {
      expr: Box::new(expr),
      span,
    })
  }

  pub(crate) fn parse_try(
    &mut self,
    expr: Expr,
    engine: &mut DiagnosticEngine,
  ) -> Result<Expr, ()> {
    let mut start_span = expr.span();
    self.expect(TokenKind::Question, engine)?;
    start_span.merge(self.current_token().span);

    Ok(Expr::Try {
      expr: Box::new(expr),
      span: start_span,
    })
  }

  pub(crate) fn parse_index(
    &mut self,
    object: Expr,
    engine: &mut DiagnosticEngine,
  ) -> Result<Expr, ()> {
    let mut start_span = object.span();

    self.expect(TokenKind::OpenBracket, engine)?;
    let index = self.parse_expression(ExprContext::Default, engine)?;
    let close_bracket = self.current_token();
    self.expect(TokenKind::CloseBracket, engine)?;
    start_span.merge(close_bracket.span);

    Ok(Expr::Index {
      object: Box::new(object),
      index: Box::new(index),
      span: start_span,
    })
  }

  pub(crate) fn parse_field_or_method(
    &mut self,
    object: Expr,
    engine: &mut DiagnosticEngine,
  ) -> Result<Expr, ()> {
    let mut start_span = object.span();
    self.expect(TokenKind::Dot, engine)?;

    let token = self.current_token();

    // Look ahead for method call
    if let TokenKind::Ident = token.kind {
      let name = self
        .source_file
        .src
        .get(token.span.start..token.span.end)
        .unwrap()
        .to_string();

      self.advance(engine);

      // If next is '(', it's a method call
      if !self.is_eof() && self.current_token().kind == TokenKind::OpenParen {
        self.expect(TokenKind::OpenParen, engine)?;
        let args = self.parse_call_params(engine)?;
        let close_paren = self.current_token();
        self.expect(TokenKind::CloseParen, engine)?;
        start_span.merge(close_paren.span);

        return Ok(Expr::MethodCall {
          receiver: Box::new(object),
          method: name,
          turbofish: None,
          args,
          span: start_span,
        });
      }

      // Otherwise it's a field access
      start_span.merge(token.span);
      return Ok(Expr::Field {
        object: Box::new(object),
        field: FieldAccess::Named(name),
        span: start_span,
      });
    }

    // Tuple indexing: .0, .1, etc.
    if let TokenKind::Literal {
      kind: LiteralKind::Integer { .. },
    } = token.kind
    {
      let value_str = self
        .source_file
        .src
        .get(token.span.start..token.span.end)
        .unwrap();
      let index = value_str.parse::<usize>().unwrap_or(0);
      self.advance(engine);
      start_span.merge(token.span);

      return Ok(Expr::Field {
        object: Box::new(object),
        field: FieldAccess::Unnamed(index),
        span: start_span,
      });
    }

    Err(())
  }

  pub(crate) fn parse_method_call(
    &mut self,
    callee: Expr,
    engine: &mut DiagnosticEngine,
  ) -> Result<Expr, ()> {
    let mut start_span = callee.span();

    self.expect(TokenKind::OpenParen, engine)?;
    let args = self.parse_call_params(engine)?;
    let close_paren = self.current_token();
    self.expect(TokenKind::CloseParen, engine)?;
    start_span.merge(close_paren.span);

    Ok(Expr::Call {
      callee: Box::new(callee),
      args,
      span: start_span,
    })
  }

  pub(crate) fn parse_call_params(
    &mut self,
    engine: &mut DiagnosticEngine,
  ) -> Result<Vec<Expr>, ()> {
    let mut exprs = vec![];

    while !self.is_eof() && self.current_token().kind != TokenKind::CloseParen {
      // NOTE: move the higher precednce when you make more productions ready to aboid bugs like
      // `foo(foo(1, 2), 3)` thus it will be unable to parse the nested call
      let expr = self.parse_expression(ExprContext::Default, engine)?;
      exprs.push(expr);

      if matches!(self.current_token().kind, TokenKind::Comma) {
        self.expect(TokenKind::Comma, engine)?;
      }

      if matches!(self.current_token().kind, TokenKind::CloseParen) {
        break;
      }
    }

    Ok(exprs)
  }
}
