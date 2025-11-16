use diagnostic::code::DiagnosticCode;
use diagnostic::diagnostic::{Diagnostic, LabelStyle};
use diagnostic::types::error::DiagnosticError;
use diagnostic::DiagnosticEngine;
use lexer::token::TokenKind;

use crate::ast::{BinaryOp, Expr};
use crate::Parser;

impl Parser {
  /// Parses bitwise shift expressions.
  ///
  /// Grammar:
  /// ```
  /// shiftExpr ::= term (("<<" | ">>") term)*
  /// ```
  ///
  /// Supported operators:
  /// - `<<` - left shift
  /// - `>>` - right shift
  ///
  /// Notes:
  /// - This rule only handles **double-character** shift operators.
  /// - Single `<` or `>` tokens are *not* processed here; they belong to
  ///   the comparison operator grammar layer.
  /// - Operands are parsed using [`parse_term`].
  /// - Shifts are left-associative, matching Rust:
  ///   `(a << b) << c`
  ///
  /// Examples:
  /// ```rust
  /// a << 1
  /// value >> 3
  /// x >> 2 << 1
  /// ```
  ///
  /// Errors:
  /// - If a range operator (`..` or `..=`) appears immediately after a shift,
  ///   a diagnostic is emitted because range expressions cannot chain.
  pub(crate) fn parse_shift(&mut self, engine: &mut DiagnosticEngine) -> Result<Expr, ()> {
    let mut lhs = self.parse_term(engine)?;

    while !self.is_eof() {
      let token = self.current_token();
      let next = self.peek(1);

      // Detect shift operator pairs
      let op = match (token.kind, next.kind) {
        (TokenKind::Lt, TokenKind::Lt) => Some(BinaryOp::Shl),
        (TokenKind::Gt, TokenKind::Gt) => Some(BinaryOp::Shr),
        _ => None,
      };

      // Stop if not a shift operator
      if op.is_none() {
        break;
      }

      // Consume both characters (`<<` or `>>`)
      self.advance(engine);
      self.advance(engine);

      let rhs = self.parse_term(engine)?;

      // Reject invalid chaining with range operators after shifting
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
