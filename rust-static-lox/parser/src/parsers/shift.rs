use diagnostic::DiagnosticEngine;
use lexer::token::TokenKind;

use crate::ast::BinaryOp;
use crate::{ast::Expr, Parser};

impl Parser {
  /* -------------------------------------------------------------------------------------------- */
  /*                                     Shift Parsing                                            */
  /* -------------------------------------------------------------------------------------------- */

  /// Parses `<<` and `>>` binary expressions, ensuring tokens are paired.
  pub(crate) fn parse_shift(&mut self, engine: &mut DiagnosticEngine) -> Result<Expr, ()> {
    let mut lhs = self.parse_term(engine)?;

    'shift_find: while !self.is_eof() {
      let token = self.current_token();
      match token.kind {
        TokenKind::Gt | TokenKind::Lt
          if matches!(self.peek(1).kind, TokenKind::Gt | TokenKind::Lt) =>
        {
          self.advance(engine); // consume the shift operator

          let op = match token.kind {
            TokenKind::Gt => BinaryOp::Shl,
            TokenKind::Lt => BinaryOp::Shr,
            _ => unreachable!(),
          };
          self.advance(engine); // consume the other shift operator

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
