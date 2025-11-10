use diagnostic::DiagnosticEngine;
use lexer::token::TokenKind;

use crate::ast::BinaryOp;
use crate::{ast::Expr, Parser};

impl Parser {
  /* -------------------------------------------------------------------------------------------- */
  /*                                     Term Parsing                                             */
  /* -------------------------------------------------------------------------------------------- */

  pub(crate) fn parse_term(&mut self, engine: &mut DiagnosticEngine) -> Result<Expr, ()> {
    let mut lhs = self.parse_factor(engine)?;

    'term_find: while !self.is_eof() {
      let token = self.current_token();
      match token.kind {
        TokenKind::Plus | TokenKind::Minus => {
          self.advance(engine); // consume the term operator

          let op = match token.kind {
            TokenKind::Plus => BinaryOp::Add,
            TokenKind::Minus => BinaryOp::Sub,
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
        _ => break 'term_find,
      }
    }

    Ok(lhs)
  }
}
