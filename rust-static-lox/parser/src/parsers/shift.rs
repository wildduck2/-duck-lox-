use diagnostic::DiagnosticEngine;
use lexer::token::TokenKind;

use crate::ast::BinaryOp;
use crate::{ast::Expr, Parser};

impl Parser {
  /* -------------------------------------------------------------------------------------------- */
  /*                                     Shift Parsing                                            */
  /* -------------------------------------------------------------------------------------------- */

  pub(crate) fn parse_shift(&mut self, engine: &mut DiagnosticEngine) -> Result<Expr, ()> {
    let mut lhs = self.parse_term(engine)?;

    'shift_find: while !self.is_eof() {
      let token = self.current_token();
      match token.kind {
        TokenKind::ShiftLeft | TokenKind::ShiftRight => {
          self.advance(engine); // consume the shift operator

          let op = match token.kind {
            TokenKind::ShiftLeft => BinaryOp::Shl,
            TokenKind::ShiftRight => BinaryOp::Shr,
            _ => unreachable!(),
          };

          let rhs = self.parse_term(engine)?;

          lhs = Expr::Binary {
            op,
            left: Box::new(lhs),
            right: Box::new(rhs),
            span: token.span,
          };
        },
        _ => break 'shift_find,
      }
    }

    Ok(lhs)
  }
}
