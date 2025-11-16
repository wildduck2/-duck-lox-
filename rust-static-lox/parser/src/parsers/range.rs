use crate::{
  ast::{Expr, RangeKind},
  Parser,
};
use diagnostic::{
  code::DiagnosticCode,
  diagnostic::{Diagnostic, LabelStyle},
  types::error::DiagnosticError,
  DiagnosticEngine,
};
use lexer::token::TokenKind;

impl Parser {
  /// Parses Rust-style range expressions.
  ///
  /// Grammar:
  /// ```
  /// rangeExpr
  ///     ::= logicalOr ( (".." | "..=") logicalOr? )?
  /// ```
  ///
  /// Supported forms:
  ///   a..b          -> RangeKind::To
  ///   a..=b         -> RangeKind::ToInclusive
  ///   a..           -> RangeKind::From
  ///   ..b           -> RangeKind::To
  ///   ..=b          -> RangeKind::ToInclusive
  ///   ..            -> RangeKind::Full
  ///
  /// Notes:
  /// - Only a single range operator may appear in an expression.
  /// - Ranges may omit the left or right operand.
  /// - Precedence matches Rust’s grammar: the operands come from `logicalOr`.
  ///
  /// Examples:
  ///   0..10
  ///   start..
  ///   ..=len
  ///   ..          // full range
  pub(crate) fn parse_range_expr(&mut self, engine: &mut DiagnosticEngine) -> Result<Expr, ()> {
    // Case 1: a range begins directly with ".." or "..="
    if matches!(
      self.current_token().kind,
      TokenKind::DotDot | TokenKind::DotDotEq
    ) {
      return self.parse_range(None, engine);
    }

    // Case 2: range may follow a left-hand side expression
    let lhs = self.parse_logical_or(engine)?;

    // At most one range operator is allowed
    if matches!(
      self.current_token().kind,
      TokenKind::DotDot | TokenKind::DotDotEq
    ) {
      self.parse_range(Some(lhs), engine)
    } else {
      Ok(lhs)
    }
  }

  /// Parses a single `..` or `..=` range operator and its optional start/end expressions.
  ///
  /// Grammar (fragment):
  /// ```
  /// range
  ///     ::= (".." | "..=") logicalOr?
  /// ```
  ///
  /// Behavior:
  /// - Accepts left-open (`..b`), right-open (`a..`), and full (`..`) ranges.
  /// - Classifies the syntax into `RangeKind` variants.
  /// - Rejects chained ranges like `a..b..c`.
  ///
  /// Examples:
  ///   a..b
  ///   ..=end
  ///   value..
  ///   ..
  fn parse_range(
    &mut self,
    start: Option<Expr>,
    engine: &mut DiagnosticEngine,
  ) -> Result<Expr, ()> {
    let token = self.current_token();
    self.advance(engine); // consume ".." or "..="

    // Determine whether an expression follows the operator
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

    // Classify the kind of range being constructed
    let kind = match (token.kind, start.is_some(), end.is_some()) {
      (TokenKind::DotDot, false, false) => RangeKind::Full,
      (TokenKind::DotDot, true, false) => RangeKind::From,
      (TokenKind::DotDot, _, true) => RangeKind::To,
      (TokenKind::DotDotEq, _, true) => RangeKind::ToInclusive,
      (TokenKind::DotDotEq, true, false) => RangeKind::FromInclusive,
      _ => RangeKind::Exclusive,
    };

    // Reject chained range operators (e.g., `a..b..c`)
    if matches!(
      self.current_token().kind,
      TokenKind::DotDot | TokenKind::DotDotEq
    ) {
      let bad = self.current_token();
      let lexeme = self.get_token_lexeme(&bad);

      let diagnostic = Diagnostic::new(
        DiagnosticCode::Error(DiagnosticError::UnexpectedToken),
        "chained range expressions are not allowed".to_string(),
        self.source_file.path.clone(),
      )
      .with_label(
        bad.span,
        Some(format!("found `{lexeme}` after a range expression")),
        LabelStyle::Primary,
      )
      .with_help("only one `..` or `..=` may appear in a range expression".to_string())
      .with_note("`a..b..c` is invalid; use `(a..b)` or `(b..c)` instead".to_string());

      engine.add(diagnostic);
      return Err(());
    }

    Ok(Expr::Range {
      kind,
      start: start.map(Box::new),
      end,
      span: token.span,
    })
  }
}
