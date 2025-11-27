use diagnostic::DiagnosticEngine;
use lexer::token::TokenKind;

use crate::{
  ast::{Attribute, Stmt},
  match_and_consume,
  parser_utils::ExprContext,
  Parser,
};

impl Parser {
  pub(crate) fn parse_let_statement(
    &mut self,
    attributes: Vec<Attribute>,
    engine: &mut DiagnosticEngine,
  ) -> Result<Stmt, ()> {
    let token = self.current_token();
    self.advance(engine); // consume let

    let pattern = self.parse_pattern_with_or(engine)?;

    println!("debug: {:#?}", pattern);
    let ty = if match_and_consume!(self, engine, TokenKind::Colon)? {
      Some(self.parse_type(engine)?)
    } else {
      None
    };

    let init = if match_and_consume!(self, engine, TokenKind::Eq)? {
      Some(self.parse_expression(ExprContext::Default, engine)?)
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
}
