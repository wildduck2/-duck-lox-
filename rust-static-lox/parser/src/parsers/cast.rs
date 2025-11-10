use diagnostic::DiagnosticEngine;
use lexer::token::TokenKind;

use crate::ast::Type;
use crate::{ast::Expr, Parser};

impl Parser {
  /* -------------------------------------------------------------------------------------------- */
  /*                                     Cast Parsing                                           */
  /* -------------------------------------------------------------------------------------------- */

  pub(crate) fn parse_cast(&mut self, engine: &mut DiagnosticEngine) -> Result<Expr, ()> {
    let mut lhs = self.parse_unary(engine)?;

    'cast_find: while !self.is_eof() {
      let mut token = self.current_token();
      match token.kind {
        TokenKind::KwAs => {
          self.advance(engine); // consume the cast operator

          // let type_no_bounds = self.parse_type_no_bounds(engine)?;
          // NOTE: later one you will parse the type bounds
          self.advance(engine); // consume the type
          token.span.merge(self.current_token().span);

          lhs = Expr::Cast {
            expr: Box::new(lhs),
            ty: Type::U32,
            span: token.span,
          };
        },
        _ => break 'cast_find,
      }
    }

    Ok(lhs)
  }
}
