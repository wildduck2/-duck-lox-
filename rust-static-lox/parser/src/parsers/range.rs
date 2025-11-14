use crate::{
  ast::{Expr, RangeKind},
  Parser,
};
use diagnostic::DiagnosticEngine;
use lexer::token::TokenKind;

impl Parser {
  //! TODO: Enforce Rust's exact precedence for range expressions.
  //!       According to Rust grammar, ranges sit *below* `||` and above assignment,
  //!       but your implementation currently attaches ranges directly to `logicalOr`.
  //!
  //!       Example: `a + b .. c` must parse as `(a + b) .. c`, but
  //!       `a .. b + c` must parse as `a .. (b + c)`.
  //!
  //!       Also ensure that range expressions cannot nest or chain:
  //!       `a..b..c` must be rejected.
  //!
  //!       Relevant Rust spec: https://doc.rust-lang.org/reference/expressions/range-expr.html

  /// Parses Rust range expressions:
  /// ```
  /// rangeExpr → logicalOr ( (".." | "..=") logicalOr? )?
  /// ```
  ///
  /// Supported forms:
  /// - `a..b`         → `RangeKind::To`
  /// - `a..=b`        → `RangeKind::ToInclusive`
  /// - `a..`          → `RangeKind::From`
  /// - `..b`          → `RangeKind::To`
  /// - `..=b`         → `RangeKind::ToInclusive`
  /// - `..`           → `RangeKind::Full`
  ///
  /// Example:
  /// ```rust
  /// 0..10
  /// ..=len
  /// start.. // open-ended
  /// ```
  pub(crate) fn parse_range_expr(&mut self, engine: &mut DiagnosticEngine) -> Result<Expr, ()> {
    // Case 1: starts directly with a range operator
    if matches!(
      self.current_token().kind,
      TokenKind::DotDot | TokenKind::DotDotEq
    ) {
      return self.parse_range(None, engine);
    }

    // Case 2: starts with a left-hand expression
    let lhs = self.parse_logical_or(engine)?;

    // Only one range operator allowed per expression
    if matches!(
      self.current_token().kind,
      TokenKind::DotDot | TokenKind::DotDotEq
    ) {
      self.parse_range(Some(lhs), engine)
    } else {
      Ok(lhs)
    }
  }

  /// Parses a single `..` or `..=` range operator with optional start and end expressions.
  fn parse_range(
    &mut self,
    start: Option<Expr>,
    engine: &mut DiagnosticEngine,
  ) -> Result<Expr, ()> {
    let token = self.current_token();
    self.advance(engine); // consume the '..' or '..='

    // Detect whether an expression follows
    let has_end = !matches!(
      self.current_token().kind,
      TokenKind::CloseBracket
        | TokenKind::CloseParen
        | TokenKind::CloseBrace
        | TokenKind::Semi
        | TokenKind::Eof
    );

    let end = if has_end {
      Some(Box::new(self.parse_logical_or(engine)?))
    } else {
      None
    };

    let kind = match (token.kind, start.is_some(), end.is_some()) {
      (TokenKind::DotDot, false, false) => RangeKind::Full, // `..`
      (TokenKind::DotDot, true, false) => RangeKind::From,  // `a..`
      (TokenKind::DotDot, _, true) => RangeKind::To,        // `a..b` or `..b`
      (TokenKind::DotDotEq, _, true) => RangeKind::ToInclusive, // `a..=b` or `..=b`
      (TokenKind::DotDotEq, true, false) => RangeKind::FromInclusive, // `a..=` (rare, macros)
      _ => RangeKind::Exclusive,                            // fallback safety case
    };

    Ok(Expr::Range {
      kind,
      start: start.map(Box::new),
      end,
      span: token.span,
    })
  }
}
