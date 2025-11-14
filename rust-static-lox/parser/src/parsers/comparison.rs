//! TODO: / Missing items for full Rust-accurate comparison parsing:
//!
//! - **Do not allow chained comparisons semantically**
//!   Rust parses `a < b < c` as `(a < b) < c`, which is almost always
//!   a logic bug. Rust itself emits a warning (`non-minimal boolean expressions`),
//!   but this parser currently produces no diagnostic.
//!
//!   Example that should warn:
//!   a < b < c   // parsed as (a < b) < c
//!
//! - **Context-sensitive restrictions in patterns**
//!   Comparison operators are not always valid inside pattern contexts
//!   (`match`, `let PATTERN =`). Full Rust performs contextual validation
//!   after parsing. This parser accepts comparisons everywhere.
//!
//! - **Operator span merging improvements**
//!   Resulting `span` currently only covers the operator token.  
//!   Full Rust merges the left operand, operator, and right operand into a
//!   single combined span for better diagnostics.
//!
//! - **No constant evaluation or overflow behavior**
//!   If you later add constant folding, comparisons have special rules:
//!   `NaN == NaN` is always false, integer comparisons do not overflow,
//!   and Rust’s const-eval engine emits diagnostics for invalid constant
//!   expressions. This parser only builds the AST.
//!
//! - **No handling of type-ascription vs `<` ambiguity**
//!   In Rust, `<` after an expression may be a type ascription operator
//!   (deprecated), a turbofish, or the start of a generic argument list.
//!   Real Rust uses complex disambiguation. This parser treats `<` only
//!   as a comparison operator here.
//!
//! - **No recovery for malformed RHS**
//!   If `rhs` cannot be parsed, parsing aborts. A production-grade parser
//!   should attempt recovery after diagnostics.
//!
//! Grammar reminder:
//! ```text
//! comparison → bitwiseOr ( compOp bitwiseOr )*
//! compOp     → "==" | "!=" | "<" | "<=" | ">" | ">="
//! ```
//!
//! Notes:
//! - Operators are **left-associative**.
//! - The parser mirrors Rust by allowing syntactic chains like `a < b < c`,
//!   even though they are semantically meaningless.
//! - Precedence is handled by delegating to `parse_bitwise_or`.

use diagnostic::DiagnosticEngine;
use lexer::token::TokenKind;

use crate::ast::BinaryOp;
use crate::{ast::Expr, Parser};

impl Parser {
  /// Parses comparison expressions that use operators such as
  /// eqeq, ne, lt, le, gt, ge.
  ///
  /// Grammar (simplified):
  ///
  /// comparison
  ///     -> bitwiseOr ( compOp bitwiseOr )*
  ///
  /// compOp
  ///     -> "==" | "!=" | "<" | "<=" | ">" | ">="
  ///
  /// Notes:
  /// - Comparison operators are left associative.
  /// - This matches Rust style: a < b < c is parsed as (a < b) < c.
  /// - Each operand is parsed using parse_bitwise_or, which has higher precedence.
  pub(crate) fn parse_comparison(&mut self, engine: &mut DiagnosticEngine) -> Result<Expr, ()> {
    // first parse the next higher precedence level
    let mut lhs = self.parse_bitwise_or(engine)?;

    loop {
      let token = self.current_token();

      let op = match token.kind {
        TokenKind::EqEq => BinaryOp::Eq,
        TokenKind::Ne => BinaryOp::NotEq,
        TokenKind::Lt => BinaryOp::Less,
        TokenKind::Le => BinaryOp::LessEq,
        TokenKind::Gt => BinaryOp::Greater,
        TokenKind::Ge => BinaryOp::GreaterEq,
        _ => break, // no more comparison operators
      };

      self.advance(engine); // consume operator

      // parse the right side with the same precedence level beneath comparison
      let rhs = self.parse_bitwise_or(engine)?;

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
