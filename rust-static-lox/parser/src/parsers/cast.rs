use diagnostic::DiagnosticEngine;
use lexer::token::TokenKind;

use crate::{ast::Expr, Parser};

impl Parser {
  /// Parses cast expressions using the Rust style `as` operator.
  ///
  /// Grammar:
  ///
  ///   castExpr ::= unaryExpr ( "as" type )*
  ///
  /// Description:
  /// - A cast expression begins with a unary expression.
  /// - Zero or more `as type` segments may follow.
  /// - Each cast forms a new expression node.
  ///
  /// Associativity:
  /// - Casts associate from left to right.
  ///   Example: `a as T1 as T2` becomes `(a as T1) as T2`.
  ///
  /// Type Parsing:
  /// - The type parser is not fully implemented.
  ///   A placeholder type node is returned until the full type grammar is added.
  ///
  /// Examples:
  ///   x as u32
  ///   value as i64 as f32
  ///
  pub(crate) fn parse_cast(&mut self, engine: &mut DiagnosticEngine) -> Result<Expr, ()> {
    // First parse the next higher precedence level: unary
    let mut lhs = self.parse_unary(engine)?;

    loop {
      let mut token = self.current_token();

      match token.kind {
        TokenKind::KwAs => {
          // Consume the `as` keyword
          self.advance(engine);
          let ty = self.parse_type(engine)?;
          token.span.merge(self.current_token().span);
          lhs = Expr::Cast {
            expr: Box::new(lhs),
            ty,
            span: token.span,
          };
        },

        _ => break,
      }
    }

    Ok(lhs)
  }
}
