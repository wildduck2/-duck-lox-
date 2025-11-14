//! TODO: Implement support for non short circuit boolean operators if your language adds them.
//!
//! TODO: Integrate parse_range_expr before parse_logical_or if your grammar supports ranges
//!       with lower precedence than logical or. In real Rust range expressions bind tighter
//!       than comparisons but looser than assignments and do not interfere with || and &&.
//!
//! TODO: Implement error recovery for cases like "a || || b" to match rustc style diagnostics.
//!
//! TODO: Support improvements for handling trailing operators such as "a ||" or "a &&"
//!       by emitting specific diagnostics instead of delegating to next parse stage.
//!
//! TODO: Implement constant folding or operator simplification if your frontend performs
//!       early expression normalization.
//!
//! TODO: Add parentheses based precedence tests to ensure logicalOr and logicalAnd
//!       interact correctly with unary, comparison, and bitwise operators.
//!
//! TODO: If implementing full Rust grammar, ensure that logical operators are not parsed
//!       inside position where patterns are expected (match arms, let patterns, if let).
//!
//! TODO: Ensure that short circuiting semantics are preserved in later lowering steps
//!       such as MIR-like or IR generation.

use diagnostic::DiagnosticEngine;
use lexer::token::TokenKind;

use crate::ast::BinaryOp;
use crate::{ast::Expr, Parser};

impl Parser {
  /// Parses logical OR expressions.
  ///
  /// Grammar:
  /// logicalOr
  ///     -> logicalAnd ( "||" logicalAnd )*
  ///
  /// Notes:
  /// - "||" is left associative.
  /// - Evaluation short circuits.
  /// - Left operand is parsed using parse_logical_and.
  ///
  /// Example:
  /// a || b || c
  pub(crate) fn parse_logical_or(&mut self, engine: &mut DiagnosticEngine) -> Result<Expr, ()> {
    let mut lhs = self.parse_logical_and(engine)?;

    loop {
      let token = self.current_token();
      match token.kind {
        TokenKind::OrOr => {
          self.advance(engine);

          let rhs = self.parse_logical_and(engine)?;

          lhs = Expr::Binary {
            op: BinaryOp::Or,
            left: Box::new(lhs),
            right: Box::new(rhs),
            span: token.span,
          };
        },
        _ => break,
      }
    }

    Ok(lhs)
  }

  /// Parses logical AND expressions.
  ///
  /// Grammar:
  /// logicalAnd
  ///     -> comparison ( "&&" comparison )*
  ///
  /// Notes:
  /// - "&&" is left associative.
  /// - Evaluation short circuits.
  /// - Left operand is parsed using parse_comparison.
  ///
  /// Example:
  /// a && b && c
  pub(crate) fn parse_logical_and(&mut self, engine: &mut DiagnosticEngine) -> Result<Expr, ()> {
    let mut lhs = self.parse_comparison(engine)?;

    loop {
      let token = self.current_token();
      match token.kind {
        TokenKind::AndAnd => {
          self.advance(engine);

          let rhs = self.parse_comparison(engine)?;

          lhs = Expr::Binary {
            op: BinaryOp::And,
            left: Box::new(lhs),
            right: Box::new(rhs),
            span: token.span,
          };
        },
        _ => break,
      }
    }

    Ok(lhs)
  }
}
