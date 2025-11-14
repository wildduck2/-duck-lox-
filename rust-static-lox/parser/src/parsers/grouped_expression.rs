//!~TODO:
//! MISSING BEHAVIOR (added):
//! - Must check for trailing comma to distinguish `(x,)` from `(x)`
//!   → `(x,)` is a tuple; `(x)` is a grouping.
use diagnostic::DiagnosticEngine;
use lexer::token::{Token, TokenKind};

use crate::{ast::Expr, parser_utils::ExprContext, Parser};

impl Parser {
  /// Parses parenthesized expressions and distinguishes between:
  /// - unit expressions: "()"
  /// - grouped expressions: "(expr)"
  /// - tuple expressions: "(a, b, c)" or "(expr,)"  // one-element tuple
  ///
  /// Full Rust-like grammar:
  ///
  /// ```
  /// groupedExpr
  ///     : "(" ")"
  ///     | "(" inner ")"
  ///
  /// inner
  ///     : expr                           // grouping
  ///     | expr ","                       // one-element tuple
  ///     | expr "," exprList              // multi-element tuple
  ///
  /// exprList
  ///     : expr ("," expr)* (",")?
  /// ```
  ///
  /// Notes:
  /// - A trailing comma forces tuple construction.
  /// - Attributes inside parentheses are allowed only in your language extension.
  pub(crate) fn parse_grouped_expr(
    &mut self,
    token: &mut Token,
    engine: &mut DiagnosticEngine,
  ) -> Result<Expr, ()> {
    // consume "("
    self.advance(engine);

    // Handle the unit expression "()"
    if matches!(self.current_token().kind, TokenKind::CloseParen) {
      self.advance(engine); // consume ")"
      token.span.merge(self.current_token().span);
      return Ok(Expr::Unit(token.span));
    }

    // Collect elements inside the parenthesis
    let mut elements = vec![];

    // Parse optional inside-attributes (#[] ...)
    let attributes = if matches!(self.current_token().kind, TokenKind::Pound) {
      self.parse_attributes(engine)?
    } else {
      vec![]
    };

    // Track if a trailing comma is present
    let mut trailing_comma = false;

    // Parse expressions until ")"
    while !self.is_eof() && !matches!(self.current_token().kind, TokenKind::CloseParen) {
      // Skip stray leading commas
      if matches!(self.current_token().kind, TokenKind::Comma) {
        trailing_comma = true;
        self.advance(engine);
        continue;
      }

      elements.push(self.parse_expression(ExprContext::Default, engine)?);

      if matches!(self.current_token().kind, TokenKind::Comma) {
        trailing_comma = true;
        self.advance(engine); // consume ","
      } else {
        trailing_comma = false;
        break;
      }
    }

    // consume ")"
    self.expect(TokenKind::CloseParen, engine)?;
    token.span.merge(self.current_token().span);

    // Decide between group or tuple
    match elements.len() {
      0 => unreachable!("zero elements already handled as unit"),

      1 => {
        if trailing_comma {
          // (x,) is a tuple
          Ok(Expr::Tuple {
            attributes,
            elements,
            span: token.span,
          })
        } else {
          // (x) is a grouping
          Ok(Expr::Group {
            attributes,
            expr: Box::new(elements[0].clone()),
            span: token.span,
          })
        }
      },

      _ => Ok(Expr::Tuple {
        attributes,
        elements,
        span: token.span,
      }),
    }
  }
}
