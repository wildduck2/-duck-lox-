use crate::{ast::Expr, Parser};
use diagnostic::DiagnosticEngine;

use crate::ast::BinaryOp;
use lexer::token::TokenKind;

impl Parser {
  /* -------------------------------------------------------------------------------------------- */
  /*                                     Bitwise Parsing                                           */
  /* -------------------------------------------------------------------------------------------- */

  pub(crate) fn parse_bitwise_or(&mut self, engine: &mut DiagnosticEngine) -> Result<Expr, ()> {
    let mut lhs = self.parse_bitwise_xor(engine)?;

    'bitwise_or_find: while !self.is_eof() {
      let token = self.current_token();
      match token.kind {
        TokenKind::Or => {
          self.advance(engine); // consume the bitwise operator

          let rhs = self.parse_bitwise_xor(engine)?;

          lhs = Expr::Binary {
            op: BinaryOp::BitOr,
            left: Box::new(lhs),
            right: Box::new(rhs),
            span: token.span,
          };
        },
        _ => break 'bitwise_or_find,
      }
    }

    Ok(lhs)
  }

  pub(crate) fn parse_bitwise_xor(&mut self, engine: &mut DiagnosticEngine) -> Result<Expr, ()> {
    let mut lhs = self.parse_bitwise_and(engine)?;

    'bitwise_xor_find: while !self.is_eof() {
      let token = self.current_token();
      match token.kind {
        TokenKind::Caret => {
          self.advance(engine); // consume the bitwise operator

          let rhs = self.parse_bitwise_and(engine)?;

          lhs = Expr::Binary {
            op: BinaryOp::BitXor,
            left: Box::new(lhs),
            right: Box::new(rhs),
            span: token.span,
          };
        },
        _ => break 'bitwise_xor_find,
      }
    }

    Ok(lhs)
  }

  pub(crate) fn parse_bitwise_and(&mut self, engine: &mut DiagnosticEngine) -> Result<Expr, ()> {
    let mut lhs = self.parse_shift(engine)?;

    'bitwise_and_find: while !self.is_eof() {
      let token = self.current_token();
      match token.kind {
        TokenKind::And => {
          self.advance(engine); // consume the bitwise operator

          let rhs = self.parse_shift(engine)?;

          lhs = Expr::Binary {
            op: BinaryOp::BitAnd,
            left: Box::new(lhs),
            right: Box::new(rhs),
            span: token.span,
          };
        },
        _ => break 'bitwise_and_find,
      }
    }

    Ok(lhs)
  }
}
