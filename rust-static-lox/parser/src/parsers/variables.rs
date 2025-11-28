use diagnostic::DiagnosticEngine;
use lexer::token::TokenKind;

use crate::{
  ast::{Attribute, Expr, Stmt},
  match_and_consume,
  parser_utils::ExprContext,
  Parser,
};

impl Parser {
  pub(crate) fn parse_let_statement(
    &mut self,
    context: ExprContext,
    attributes: Vec<Attribute>,
    engine: &mut DiagnosticEngine,
  ) -> Result<Stmt, ()> {
    let token = self.current_token();
    self.advance(engine); // consume let

    let pattern = self.parse_pattern_with_or(context, engine)?;

    println!("debug: {:#?}", pattern);
    let ty = if match_and_consume!(self, engine, TokenKind::Colon)? {
      Some(self.parse_type(engine)?)
    } else {
      None
    };

    let init = if match_and_consume!(self, engine, TokenKind::Eq)? {
      Some(self.parse_expression(vec![], ExprContext::Default, engine)?)
    } else {
      None
    };

    Err(())
    // Ok(Stmt::Let {
    //   attributes,
    //   pattern,
    //   ty,
    //   init: init.map(Box::new),
    //   else_block: None,
    //   span: token.span,
    // })
  }

  pub(crate) fn parse_let_expression(
    &mut self,
    context: ExprContext,
    engine: &mut DiagnosticEngine,
  ) -> Result<Expr, ()> {
    let mut token = self.current_token();
    self.advance(engine); // consume the "let"

    let pattern = self.parse_pattern(context, engine)?;
    self.expect(TokenKind::Eq, engine)?;
    let value = self.parse_expression(vec![], context, engine)?;

    token.span.merge(self.current_token().span);
    Ok(Expr::Let {
      expr: Box::new(value),
      pattern,
      span: token.span,
    })
  }
}
