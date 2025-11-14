use crate::{
  ast::{Expr, RangeKind},
  Parser,
};
use diagnostic::DiagnosticEngine;
use lexer::token::TokenKind;

impl Parser {
  /* -------------------------------------------------------------------------------------------- */
  /*                                     Range Parsing                                           */
  /* -------------------------------------------------------------------------------------------- */

  /// Parses Rust range expressions (`..`, `..=`, `a..b`, etc.).
  pub(crate) fn parse_range_expr(&mut self, engine: &mut DiagnosticEngine) -> Result<Expr, ()> {
    // Case 1: range starts with ".." or "..="
    if matches!(
      self.current_token().kind,
      TokenKind::DotDot | TokenKind::DotDotEq
    ) {
      return self.parse_prefix_range(None, engine);
    }

    // Case 2: range starts with a single expression
    let mut lhs = Some(self.parse_logical_or(engine)?);

    'range_expr_find: while !self.is_eof() {
      let token = self.current_token();
      match token.kind {
        TokenKind::DotDot | TokenKind::DotDotEq => {
          lhs = Some(self.parse_prefix_range(lhs.take(), engine)?);
        },
        _ => break 'range_expr_find,
      }
    }

    Ok(lhs.unwrap())
  }

  /// Parses the `..` / `..=` portion once the start expression is known (or absent).
  fn parse_prefix_range(
    &mut self,
    start: Option<Expr>,
    engine: &mut DiagnosticEngine,
  ) -> Result<Expr, ()> {
    let token = self.current_token();
    self.advance(engine); // consume the range operator

    // Check if an expression follows
    let has_end = !matches!(
      self.current_token().kind,
      TokenKind::CloseBracket
        | TokenKind::CloseParen
        | TokenKind::CloseBrace
        | TokenKind::Semi
        | TokenKind::Eof
    );

    let rhs = if has_end {
      Some(Box::new(self.parse_logical_or(engine)?))
    } else {
      None
    };

    let kind = match token.kind {
      TokenKind::DotDot => RangeKind::To,
      TokenKind::DotDotEq => RangeKind::ToInclusive,
      _ => unreachable!(),
    };

    Ok(Expr::Range {
      kind,
      start: start.map(Box::new),
      end: rhs,
      span: token.span,
    })
  }
}
