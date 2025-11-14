use diagnostic::DiagnosticEngine;
use lexer::token::TokenKind;

use crate::ast::{BinaryOp, Expr};
use crate::Parser;

impl Parser {
  //! TODO: Disallow interpreting >> as a shift operator inside type or generic argument contexts.
  //!       Rust lexes >> and >>> as separate > tokens inside generics, not as shift operators.
  //!       Example: Vec<Vec<u8>> should parse as Vec < Vec < u8 > >, not as a shift.

  /// Parses left (`<<`) and right (`>>`) bitwise shift expressions.
  ///
  /// Grammar:
  /// ```
  /// shiftExpr → term (("<<" | ">>") term)*
  /// ```
  ///
  /// Example:
  /// ```rust
  /// a << 1
  /// b >> 2 << 3
  /// ```
  ///
  /// Notes:
  /// - Delegates operand parsing to [`parse_term`].
  /// - Stops when no further shift operators are present.
  /// - Does *not* consume single `<` or `>` tokens; those are handled by comparison parsing.
  pub(crate) fn parse_shift(&mut self, engine: &mut DiagnosticEngine) -> Result<Expr, ()> {
    let mut lhs = self.parse_term(engine)?;

    while !self.is_eof() {
      let token = self.current_token();
      let next = self.peek(1);

      let op = match (token.kind, next.kind) {
        (TokenKind::Lt, TokenKind::Lt) => Some(BinaryOp::Shl),
        (TokenKind::Gt, TokenKind::Gt) => Some(BinaryOp::Shr),
        _ => None,
      };

      // Not a shift pair → stop parsing
      if op.is_none() {
        break;
      }

      // Consume both symbols of the operator
      self.advance(engine);
      self.advance(engine);

      let rhs = self.parse_term(engine)?;
      lhs = Expr::Binary {
        op: op.unwrap(),
        left: Box::new(lhs),
        right: Box::new(rhs),
        span: token.span,
      };
    }

    Ok(lhs)
  }
}
