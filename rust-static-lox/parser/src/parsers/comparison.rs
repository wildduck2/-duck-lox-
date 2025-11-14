use diagnostic::DiagnosticEngine;
use lexer::token::TokenKind;

use crate::ast::BinaryOp;
use crate::{ast::Expr, Parser};

impl Parser {
  /* -------------------------------------------------------------------------------------------- */
  /*                                     Comparison Parsing                                       */
  /* -------------------------------------------------------------------------------------------- */

  /// Parses chained comparison operators (`==`, `<`, `>=`, …) with left associativity.
  pub(crate) fn parse_comparison(&mut self, engine: &mut DiagnosticEngine) -> Result<Expr, ()> {
    let mut lhs = self.parse_bitwise_or(engine)?;
    'comparison_find: while !self.is_eof() {
      let token = self.current_token();
      match token.kind {
        TokenKind::EqEq
        | TokenKind::Ne
        | TokenKind::Lt
        | TokenKind::Le
        | TokenKind::Gt
        | TokenKind::Ge => {
          self.advance(engine); // consume the comparison operator

          let rhs = self.parse_bitwise_or(engine)?;

          lhs = Expr::Binary {
            op: match token.kind {
              TokenKind::EqEq => BinaryOp::Eq,
              TokenKind::Ne => BinaryOp::NotEq,
              TokenKind::Lt => BinaryOp::Less,
              TokenKind::Le => BinaryOp::LessEq,
              TokenKind::Gt => BinaryOp::Greater,
              TokenKind::Ge => BinaryOp::GreaterEq,
              _ => unreachable!(),
            },
            left: Box::new(lhs),
            right: Box::new(rhs),
            span: token.span,
          };
        },
        _ => break 'comparison_find,
      }
    }

    Ok(lhs)
  }
}
