use diagnostic::{
  code::DiagnosticCode,
  diagnostic::{Diagnostic, LabelStyle},
  types::error::DiagnosticError,
};
use lexer::token::{Token, TokenKind};

use crate::{ast::Expr, parser_utils::ExprContext, DiagnosticEngine, Parser};

impl Parser {
  /// Parses Rust-style array literals:
  ///
  /// ```text
  /// arrayExpr
  ///     → "[" arrayElements? "]"
  ///
  /// arrayElements
  ///     → expr (";" expr | ("," expr)* ","?)
  /// ```
  ///
  /// Supports:
  /// - Comma arrays: `[a, b, c]`
  /// - One-element arrays: `[a]` or `[a,]`
  /// - Repeat arrays: `[a; n]`
  /// - Empty arrays: `[]`
  pub(crate) fn parse_array_expr(
    &mut self,
    token: &mut Token,
    engine: &mut DiagnosticEngine,
  ) -> Result<Expr, ()> {
    self.advance(engine); // consume '['

    let (elements, repeat) = self.parse_array_elements(engine)?;

    token.span.merge(self.current_token().span);

    Ok(Expr::Array {
      elements,
      repeat: repeat.map(Box::new),
      span: token.span,
    })
  }

  /// Parses:
  ///
  /// - `[value; count]` repeat arrays
  /// - `[a, b, c]` comma-separated arrays
  /// - `[a]` or `[a,]`
  /// - `[]`
  fn parse_array_elements(
    &mut self,
    engine: &mut DiagnosticEngine,
  ) -> Result<(Vec<Expr>, Option<Expr>), ()> {
    let mut elements = vec![];

    // Empty array: `[]`
    if self.current_token().kind == TokenKind::CloseBracket {
      self.advance(engine);
      return Ok((elements, None));
    }

    // First element
    elements.push(self.parse_expression(ExprContext::Default, engine)?);

    // Check for repeat array form `[value; count]`
    if matches!(self.current_token().kind, TokenKind::Semi) {
      self.advance(engine); // consume `;`

      // Reject `[value;]` — missing count
      if !self.current_token().kind.is_literal() {
        let token = self.current_token();
        let lexeme = self.get_token_lexeme(&token);

        let diagnostic = Diagnostic::new(
          DiagnosticCode::Error(DiagnosticError::UnexpectedToken),
          "invalid repeat expression".to_string(),
          self.source_file.path.clone(),
        )
        .with_label(
          token.span,
          Some(format!(
            "expected an integer literal here, found `{lexeme}`"
          )),
          LabelStyle::Primary,
        )
        .with_note(
          "repeat array syntax is: `[value; N]` where `N` is a compile-time constant integer."
            .to_string(),
        )
        .with_help("example: `[x; 4]`".to_string());

        engine.add(diagnostic);
        return Err(());
      }

      let repeat_count = self.parse_expression(ExprContext::Default, engine)?;
      self.expect(TokenKind::CloseBracket, engine)?;
      return Ok((elements, Some(repeat_count)));
    }

    // Normal comma-separated array
    while !self.is_eof() && !matches!(self.current_token().kind, TokenKind::CloseBracket) {
      if matches!(self.current_token().kind, TokenKind::Comma) {
        self.advance(engine);
      }

      if self.current_token().kind == TokenKind::CloseBracket {
        break;
      }

      elements.push(self.parse_expression(ExprContext::Default, engine)?);
    }

    self.expect(TokenKind::CloseBracket, engine)?;
    Ok((elements, None))
  }
}
