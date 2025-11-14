//! TODO: Array expression parsing is incomplete compared with Rust's full grammar.
//!
//! Missing or incorrect behaviors:
//!
//! - **Ambiguity resolution for `[expr; expr]` vs `[expr, ...]`**  
//!   Current implementation parses the first expression, then:
//!   - if it sees `;`, treats it as a repeat array `[value; len]`  
//!   - otherwise collects comma-separated elements.  
//!     Rust has several subtle ambiguities involving slices, indexing, and macro
//!     contexts that require lookahead rules not yet implemented.
//!
//! - **Diagnostics for invalid repetition forms**  
//!   The parser should reject forms like:  
//!   - `[a;]` (missing repeat count)  
//!   - `[a;; 10]` (extra semicolons)  
//!   - `[a; 1, 2]` (mixing repetition + commas)  
//!   - `[a; b; c]` (multiple semicolons)
//!
//! - **Proper handling of trailing commas in element lists**  
//!   Rust allows:  
//!   - `[a, b, c,]`  
//!     Current version does not distinguish between trailing and required commas
//!     and may misparse repetitions or produce confusing errors.
//!
//! - **Better error recovery after malformed arrays**  
//!   Unexpected tokens between elements (e.g. `[a  b]`) should produce structured
//!   diagnostics with recovery to the closing `]`.
//!
//! - **Support for nested attributes inside arrays (if language allows)**  
//!   Rust allows things like:  
//!   `[ #[cfg(foo)] 1, 2 ]`  
//!   Current parser does not handle attributes before array elements.
//!
//! - **Ensure correct span merging and error spans**  
//!   Spans for the entire array expression and inner elements should track
//!   delimiters precisely. Current merging is too coarse in some cases.
//!
//! - **Better distinction between array expressions and array types**  
//!   In Rust, `[T; N]` is a type, not an expression.  
//!   Your parser currently handles expression arrays only; type arrays should use
//!   the type parser, not the expression parser.
//!
//! Grammar reference (Rust-like):
//! ```text
//! arrayExpr
//!     → "[" "]"
//!     | "[" expr ("," expr)* ","? "]"
//!     | "[" expr ";" expr "]"
//! ```
//!
//! This module handles basic parsing but lacks the complete Rust semantics,
//! correctness, and diagnostics for complex or ambiguous cases.

use lexer::token::{Token, TokenKind};

use crate::{ast::Expr, parser_utils::ExprContext, DiagnosticEngine, Parser};

impl Parser {
  // *   arrayExpr        → "[" arrayElements? "]" ;
  // *
  // *   arrayElements    → expression (";" expression | ("," expression)* ","?) ;
  /// Parses `[...]` array literals including repetition form `[expr; len]`.
  pub(crate) fn parse_array_expr(
    &mut self,
    token: &mut Token,
    engine: &mut DiagnosticEngine,
  ) -> Result<Expr, ()> {
    self.advance(engine); // consume the opening bracket

    let (elements, repeat) = self.parse_array_elements(engine)?;

    token.span.merge(self.current_token().span);

    Ok(Expr::Array {
      elements,
      repeat: repeat.map(Box::new),
      span: token.span,
    })
  }

  /// Parses the comma-separated elements or the `; repeat` segment inside an array literal.
  fn parse_array_elements(
    &mut self,
    engine: &mut DiagnosticEngine,
  ) -> Result<(Vec<Expr>, Option<Expr>), ()> {
    let mut repeat = None;
    let mut elements = vec![];

    // if it's empty, we're done
    if self.current_token().kind == TokenKind::CloseBracket {
      self.advance(engine); // consume the closing bracket
      return Ok((elements, repeat));
    }

    elements.push(self.parse_expression(ExprContext::Default, engine)?);

    while !self.is_eof() && !matches!(self.current_token().kind, TokenKind::CloseBracket) {
      if matches!(self.current_token().kind, TokenKind::Comma) {
        self.advance(engine); // consume the comma
      } else if matches!(self.current_token().kind, TokenKind::Semi) {
        self.advance(engine); // consume the semicolon
        repeat = Some(self.parse_expression(ExprContext::Default, engine)?);
      }

      if self.current_token().kind == TokenKind::CloseBracket {
        break;
      }

      elements.push(self.parse_expression(ExprContext::Default, engine)?);
    }

    self.expect(TokenKind::CloseBracket, engine)?;

    Ok((elements, repeat))
  }
}
