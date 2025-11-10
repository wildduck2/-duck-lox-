use diagnostic::DiagnosticEngine;
use lexer::token::TokenKind;

use crate::ast::BinaryOp;
use crate::{ast::Expr, Parser};

impl Parser {
  /* -------------------------------------------------------------------------------------------- */
  /*                                     Logical Parsing                                           */
  /* -------------------------------------------------------------------------------------------- */

  pub(crate) fn parse_logical_or(&mut self, engine: &mut DiagnosticEngine) -> Result<Expr, ()> {
    let mut lhs = self.parse_logical_and(engine)?;

    'logical_or_find: while !self.is_eof() {
      let token = self.current_token();
      match token.kind {
        TokenKind::OrOr => {
          self.advance(engine); // consume the logical operator

          let rhs = self.parse_logical_and(engine)?;

          lhs = Expr::Binary {
            op: BinaryOp::Or,
            left: Box::new(lhs),
            right: Box::new(rhs),
            span: token.span,
          };
        },
        _ => break 'logical_or_find,
      }
    }

    Ok(lhs)
  }

  pub(crate) fn parse_logical_and(&mut self, engine: &mut DiagnosticEngine) -> Result<Expr, ()> {
    let mut lhs = self.parse_comparison(engine)?;

    'logical_and_find: while !self.is_eof() {
      let token = self.current_token();
      match token.kind {
        TokenKind::AndAnd => {
          self.advance(engine); // consume the logical operator

          let rhs = self.parse_comparison(engine)?;

          lhs = Expr::Binary {
            op: BinaryOp::And,
            left: Box::new(lhs),
            right: Box::new(rhs),
            span: token.span,
          };
        },
        _ => break 'logical_and_find,
      }
    }

    Ok(lhs)
  }
}
