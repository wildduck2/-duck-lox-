use diagnostic::code::DiagnosticCode;
use diagnostic::diagnostic::{Diagnostic, LabelStyle};
use diagnostic::types::error::DiagnosticError;
use diagnostic::DiagnosticEngine;
use lexer::token::TokenKind;

use crate::ast::BinaryOp;
use crate::{ast::Expr, Parser};

impl Parser {
  /// Parses multiplicative expressions using the operators `*`, `/`, and `%`.
  ///
  /// Grammar:
  ///
  ///   factor ::= cast ( factorOp cast )*
  ///
  ///   factorOp ::= "*" | "/" | "%"
  ///
  /// Description:
  /// - A factor expression consists of a left operand followed by zero or more
  ///   multiplicative operators and right operands.
  /// - Each operand is parsed using `parse_cast`, the next higher-precedence rule.
  ///
  /// Associativity:
  /// - Multiplicative operators associate left to right.
  ///   Example: a * b * c parses as (a * b) * c.
  ///
  /// Error Handling:
  /// - If the right side of an operator does not start a valid expression,
  ///   an UnexpectedToken diagnostic is emitted.
  ///
  /// Examples:
  ///   x * y
  ///   a / b
  ///   n % 10
  ///   x * y / z % k
  ///
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

      if !self.current_token().kind.can_start_expression() {
        let bad = self.current_token();
        let lexeme = self.get_token_lexeme(&bad);

        let diagnostic = Diagnostic::new(
          DiagnosticCode::Error(DiagnosticError::UnexpectedToken),
          "invalid right-hand side of factor expression".to_string(),
          self.source_file.path.clone(),
        )
        .with_label(
          bad.span,
          Some(format!(
            "expected an expression after the factor operator, found `{lexeme}`"
          )),
          LabelStyle::Primary,
        )
        .with_help("factor operators must be followed by a valid expression".to_string())
        .with_note("examples: `x * y`, `x / y`, `x % y`".to_string())
        .with_note(
          "Rust parses `a * b * c` as `(a * b) * c`, which is almost always incorrect".to_string(),
        );

        engine.add(diagnostic);
        return Err(());
      }

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
