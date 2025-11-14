//! TODO: Missing / TODO items for full Rust-compatible multiplicative parsing:
//!
//! - **Overflow awareness for constant folding**
//!   The parser currently only builds an AST. If you later add constant
//!   propagation, you must ensure that `*`, `/`, `%` overflow checks follow
//!   Rust semantics (panic in const-eval, UB in release? etc.).
//!
//! - **Division by zero diagnostics (optional)**
//!   Rust does not diagnose this at parse time, but a language extension
//!   *could* warn when seeing something like `10 / 0` or `% 0`.
//!
//! - **Operator span merging**
//!   Right now the produced `span` reflects the operator token, not the full
//!   `lhs <op> rhs` region. Full Rust spans merge the whole expression.
//!
//! - **Missing recovery on malformed RHS**
//!   If `rhs` fails to parse, the parser aborts the entire `factor`. A more
//!   robust parser would attempt error-recovery and continue parsing additional
//!   operators instead of exiting immediately.
//!
//! - **No contextual restrictions**
//!   Rust disallows some expressions in constant patterns or match arms;
//!   multiplicative expressions may need context-sensitive restrictions in
//!   future.
//!
//! - **No MIR-level special cases**
//!   Rust treats `x * 1`, `x / 1`, or `%` patterns in certain ways during
//!   optimizations. Parser does not attempt to simplify anything.
//!
//! Grammar reminder:
//! ```text
//! factor
//!     → cast ( factorOp cast )*
//!
//! factorOp
//!     → "*" | "/" | "%"
//! ```
//!
//! Notes:
//! - Operators are **left-associative**.
//! - No precedence inversion occurs here; `parse_cast` handles higher-precedence ops.
//! - Parser does **not** consume `/=` or `*=` (assignment ops); those belong to
//!   assignment parsing.

use diagnostic::DiagnosticEngine;
use lexer::token::TokenKind;

use crate::ast::BinaryOp;
use crate::{ast::Expr, Parser};

impl Parser {
  /// Parses multiplicative expressions: star, slash, percent.
  ///
  /// Grammar (simplified):
  ///
  /// factor
  ///     -> cast ( factorOp cast )*
  ///
  /// factorOp
  ///     -> "*" | "/" | "%"
  ///
  /// Notes:
  /// - This operator group is left associative.
  /// - We repeatedly fold lhs = Binary(lhs, op, rhs).
  /// - Right associativity would be incorrect for these operators.
  pub(crate) fn parse_factor(&mut self, engine: &mut DiagnosticEngine) -> Result<Expr, ()> {
    // start with the next higher-precedence expression
    let mut lhs = self.parse_cast(engine)?;

    loop {
      let token = self.current_token();

      let op = match token.kind {
        TokenKind::Star => BinaryOp::Mul,
        TokenKind::Slash => BinaryOp::Div,
        TokenKind::Percent => BinaryOp::Mod,
        _ => break, // not a factor operator
      };

      self.advance(engine); // consume operator

      // parse the next cast-level expression (not parse_factor)
      let rhs = self.parse_cast(engine)?;

      lhs = Expr::Binary {
        op,
        left: Box::new(lhs),
        right: Box::new(rhs),
        span: token.span,
      };
    }

    Ok(lhs)
  }
}
