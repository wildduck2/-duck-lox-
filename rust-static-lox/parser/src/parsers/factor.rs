use diagnostic::DiagnosticEngine;
use lexer::token::TokenKind;

use crate::ast::BinaryOp;
use crate::{ast::Expr, Parser};

impl Parser {
  /* -------------------------------------------------------------------------------------------- */
  /*                                     Factor Parsing                                           */
  /* -------------------------------------------------------------------------------------------- */

  pub(crate) fn parse_factor(&mut self, engine: &mut DiagnosticEngine) -> Result<Expr, ()> {
    let mut lhs = self.parse_cast(engine)?;

    'factor_find: while !self.is_eof() {
      let token = self.current_token();
      match token.kind {
        TokenKind::Star | TokenKind::Slash | TokenKind::Percent => {
          self.advance(engine); // consume the factor operator

          let op = match token.kind {
            TokenKind::Star => BinaryOp::Mul,
            TokenKind::Slash => BinaryOp::Div,
            TokenKind::Percent => BinaryOp::Mod,
            _ => unreachable!(),
          };

          let rhs = self.parse_factor(engine)?;

          lhs = Expr::Binary {
            op,
            left: Box::new(lhs),
            right: Box::new(rhs),
            span: token.span,
          };
        },
        _ => break 'factor_find,
      }
    }

    Ok(lhs)
  }
}
