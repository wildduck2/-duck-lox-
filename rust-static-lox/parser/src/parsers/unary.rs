use diagnostic::DiagnosticEngine;
use lexer::token::TokenKind;

use crate::ast::UnaryOp;
use crate::{ast::Expr, Parser};

impl Parser {
  /* -------------------------------------------------------------------------------------------- */
  /*                                     Unary Parsing                                            */
  /* -------------------------------------------------------------------------------------------- */

  pub(crate) fn parse_unary(&mut self, engine: &mut DiagnosticEngine) -> Result<Expr, ()> {
    let mut token = self.current_token();

    match token.kind {
      TokenKind::Minus | TokenKind::Bang | TokenKind::Star | TokenKind::And | TokenKind::AndAnd => {
        self.advance(engine); // consume the unary operator

        // consume the 'mut' keyword and set the mutable flag
        let mutable = if self.current_token().kind == TokenKind::KwMut {
          self.advance(engine);
          true
        } else {
          false
        };

        let op = match token.kind {
          TokenKind::Minus => UnaryOp::Neg,
          TokenKind::Bang => UnaryOp::Not,
          TokenKind::Star => UnaryOp::Deref,
          TokenKind::And => UnaryOp::Ref { mutable, depth: 1 },
          TokenKind::AndAnd => UnaryOp::Ref { mutable, depth: 2 },
          _ => unreachable!(),
        };

        let rhs = self.parse_unary(engine)?;

        token.span.merge(self.current_token().span);

        Ok(Expr::Unary {
          expr: Box::new(rhs),
          op,
          span: token.span,
        })
      },
      _ => self.parse_postfix(engine),
    }
  }
}
