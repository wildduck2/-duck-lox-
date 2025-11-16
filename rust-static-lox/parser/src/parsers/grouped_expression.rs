use diagnostic::DiagnosticEngine;
use lexer::token::{Token, TokenKind};

use crate::{ast::Expr, parser_utils::ExprContext, Parser};

impl Parser {
  /// Parses parenthesized expressions and determines whether the result is:
  ///
  /// - a unit expression: "()"
  /// - a grouped expression: "(expr)"
  /// - a tuple expression: "(a, b, c)" or "(expr,)" for a one-element tuple
  ///
  /// Grammar (simplified):
  ///
  ///   groupedExpr ::= "(" ")"
  ///                 | "(" inner ")"
  ///
  ///   inner ::= expr
  ///           | expr ","
  ///           | expr "," exprList
  ///
  ///   exprList ::= expr ("," expr)* (",")?
  ///
  /// Rules:
  /// - A trailing comma after a single element creates a tuple: (x,)
  /// - No trailing comma means it is a grouped expression: (x)
  /// - Multiple elements always form a tuple.
  ///
  /// Notes:
  /// - Attributes inside parentheses (#[] ...) are supported only in this
  ///   implementation and are not in standard Rust.
  /// - Unit expressions are recognized early: the parser checks for ")" after "(".
  ///
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

    // Track whether a trailing comma appears
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

    // Distinguish grouped vs tuple vs unit
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
          // (x) is a grouping expression
          Ok(Expr::Group {
            attributes,
            expr: Box::new(elements[0].clone()),
            span: token.span,
          })
        }
      },

      _ => {
        // Multiple elements always form a tuple
        Ok(Expr::Tuple {
          attributes,
          elements,
          span: token.span,
        })
      },
    }
  }
}
