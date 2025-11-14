//! TODO / Missing items for full Rust-accurate `as` cast parsing:
//!
//! - **Implement full type parsing.**  
//!   Right now the cast always produces `Type::U32` as a placeholder.  
//!   Replace this with a real call to `parse_type(engine)` once the type grammar
//!   is complete.
//!
//! - **Support contextual keyword casts.**  
//!   Rust allows casts to primitive types that overlap with keywords:
//!   `as bool`, `as str`, `as usize`, etc. These should resolve through the
//!   type parser, not token-by-token guessing.
//!
//! - **Check for invalid cast targets.**  
//!   Rust forbids many illegal cast combinations (e.g., casting unsized types,
//!   casting fat pointers to integers without `usize`, etc.).  
//!   This parser currently accepts anything `as` is followed by.
//!
//! - **Do not treat `as` inside type positions as a cast.**  
//!   In Rust, `Foo as Bar` inside a *type* position is not valid.  
//!   Contextual disambiguation is required once type- and pattern-context
//!   awareness is implemented.
//!
//! - **Improve span merging.**  
//!   The cast expression should merge the span of the full `lhs as T`,
//!   not just the operator. Diagnostics will be cleaner once fully merged.
//!
//! - **Emit diagnostics for malformed or missing types.**  
//!   When `as` is followed by invalid syntax, the parser should emit a helpful
//!   error instead of blindly consuming a token.
//!
//! - **Handle casts involving literals with suffixes.**  
//!   Example:  
//!       `1u8 as u16`  
//!   Once literal-suffix parsing is richer, cast rules should reflect the source
//!   type and suffix information.
//!
//! Grammar Reminder:
//! ```text
//! castExpr → unaryExpr ( "as" type )*
//! ```
//!
//! Notes:
//! - Casts are **left-associative**, matching Rust (`a as T1 as T2`) → `(a as T1) as T2`.
//! - `as` has very low precedence, just above assignment.
//! - Correct implementation requires a fully-featured type parser.

use diagnostic::DiagnosticEngine;
use lexer::token::TokenKind;

use crate::ast::Type;
use crate::{ast::Expr, Parser};

impl Parser {
  /// Parses chained `as` cast expressions.
  ///
  /// Grammar (simplified):
  ///
  /// castExpr
  ///     -> unaryExpr ( "as" type )*
  ///
  /// Notes:
  /// - Casts associate left to right.
  ///   Example: a as T1 as T2 is parsed as (a as T1) as T2.
  /// - The type parser is not fully implemented yet.
  ///   A placeholder type is used for now.
  ///
  /// Example:
  ///     x as u32
  ///     value as i64 as f32
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

          // TODO: implement real type parsing here.
          // For now, we consume the next token and use a placeholder type.
          self.advance(engine);

          token.span.merge(self.current_token().span);

          lhs = Expr::Cast {
            expr: Box::new(lhs),
            ty: Type::U32, // placeholder until the full type parser is implemented
            span: token.span,
          };
        },

        _ => break,
      }
    }

    Ok(lhs)
  }
}
