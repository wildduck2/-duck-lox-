use diagnostic::DiagnosticEngine;
use lexer::token::TokenKind;

use crate::ast::Type;
use crate::{ast::Expr, Parser};

impl Parser {
  /* -------------------------------------------------------------------------------------------- */
  /*                                     Cast Parsing                                           */
  /* -------------------------------------------------------------------------------------------- */

  /// Parses postfix `as` casts while chaining when multiple appear.
  /// NOTE: RHS type parsing is still a placeholder and always returns `Type::U32`.
  pub(crate) fn parse_cast(&mut self, engine: &mut DiagnosticEngine) -> Result<Expr, ()> {
    let mut lhs = self.parse_unary(engine)?;

    'cast_find: while !self.is_eof() {
      let mut token = self.current_token();
      match token.kind {
        TokenKind::KwAs => {
          self.advance(engine); // consume the cast operator

          // NOTE: later one you will parse the type bounds
          // TODO: parse the full type after `as` once type grammar is wired up.
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
