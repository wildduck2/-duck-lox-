//! TODO: Bitwise operator parsing is incomplete compared to full Rust grammar.
//!
//! Missing or incomplete features:
//!
//! - **Operator precedence validation**  
//!   Current implementation relies on the surrounding parser structure.  
//!   Once all operator levels are implemented, confirm that precedence  
//!   matches Rust exactly (`&` > `^` > `|`).
//!
//! - **Diagnostics for misplaced operators**  
//!   Sequences like `a | | b`, `a ^ ^ b`, or lone operators such as  
//!   `&x` in expression position should emit errors rather than  
//!   allowing obscure parser failures later.
//!
//! - **Span merging improvements**  
//!   The produced `Expr::Binary` span currently uses only the operator’s  
//!   span. For better error messages, merge spans from left operand  
//!   through right operand.
//!
//! - **Short-circuit vs non-short-circuit clarity**  
//!   Ensure that `||` (logical OR) is not accepted here, and that bitwise  
//!   OR (`|`) is never confused with logical OR. Same for `&&` vs `&`.
//!
//! - **Contextual disambiguation for patterns**  
//!   Rust uses `|` in `match` patterns and or-patterns (`A | B`).  
//!   Expression-context `|` must not be parsed while in pattern context.  
//!   This parser currently assumes expression-only context.
//!
//! - **Support for assignment operators**  
//!   Rust supports `|=`, `^=`, and `&=`. These belong to the assignment  
//!   parsing level and should not be consumed here, but the parser  
//!   should detect and forward them to the assignment expression parser.
//!
//! - **Error recovery**  
//!   When parsing `a | b &`, the parser should produce a diagnostic and  
//!   skip tokens gracefully rather than propagating an opaque error upward.
//!
//! Grammar reminder (simplified):
//! ```text
//! bitwiseOr      → bitwiseXor ( "|"  bitwiseXor )*
//! bitwiseXor     → bitwiseAnd ( "^"  bitwiseAnd )*
//! bitwiseAnd     → shift       ( "&"  shift      )*
//! ```
//!
//! All operators in this group are **left-associative**.

use crate::{ast::Expr, Parser};
use diagnostic::DiagnosticEngine;

use crate::ast::BinaryOp;
use lexer::token::TokenKind;

impl Parser {
  /// Parses chained bitwise OR expressions (`expr | expr`).
  pub(crate) fn parse_bitwise_or(&mut self, engine: &mut DiagnosticEngine) -> Result<Expr, ()> {
    let mut lhs = self.parse_bitwise_xor(engine)?;

    'bitwise_or_find: while !self.is_eof() {
      let token = self.current_token();
      match token.kind {
        TokenKind::Or => {
          self.advance(engine); // consume the bitwise operator

          let rhs = self.parse_bitwise_xor(engine)?;

          lhs = Expr::Binary {
            op: BinaryOp::BitOr,
            left: Box::new(lhs),
            right: Box::new(rhs),
            span: token.span,
          };
        },
        _ => break 'bitwise_or_find,
      }
    }

    Ok(lhs)
  }

  /// Parses chained bitwise XOR expressions (`expr ^ expr`).
  pub(crate) fn parse_bitwise_xor(&mut self, engine: &mut DiagnosticEngine) -> Result<Expr, ()> {
    let mut lhs = self.parse_bitwise_and(engine)?;

    'bitwise_xor_find: while !self.is_eof() {
      let token = self.current_token();
      match token.kind {
        TokenKind::Caret => {
          self.advance(engine); // consume the bitwise operator

          let rhs = self.parse_bitwise_and(engine)?;

          lhs = Expr::Binary {
            op: BinaryOp::BitXor,
            left: Box::new(lhs),
            right: Box::new(rhs),
            span: token.span,
          };
        },
        _ => break 'bitwise_xor_find,
      }
    }

    Ok(lhs)
  }

  /// Parses chained bitwise AND expressions (`expr & expr`).
  pub(crate) fn parse_bitwise_and(&mut self, engine: &mut DiagnosticEngine) -> Result<Expr, ()> {
    let mut lhs = self.parse_shift(engine)?;

    'bitwise_and_find: while !self.is_eof() {
      let token = self.current_token();
      match token.kind {
        TokenKind::And => {
          self.advance(engine); // consume the bitwise operator

          let rhs = self.parse_shift(engine)?;

          lhs = Expr::Binary {
            op: BinaryOp::BitAnd,
            left: Box::new(lhs),
            right: Box::new(rhs),
            span: token.span,
          };
        },
        _ => break 'bitwise_and_find,
      }
    }

    Ok(lhs)
  }
}
