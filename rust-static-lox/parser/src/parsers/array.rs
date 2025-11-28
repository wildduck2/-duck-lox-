use diagnostic::{
  code::DiagnosticCode,
  diagnostic::{Diagnostic, LabelStyle},
  types::error::DiagnosticError,
};
use lexer::token::{Token, TokenKind};

use crate::{ast::Expr, parser_utils::ExprContext, DiagnosticEngine, Parser};

impl Parser {
  /// Parses Rust-style array expressions.
  ///
  /// Grammar reference (`<arrayExpr>`):
  ///
  /// ```bnf
  /// arrayExpr       ::= "[" arrayElements? "]"
  ///
  /// arrayElements   ::=
  ///        expression ";" expression           // repeat array
  ///      | expression ("," expression)* ","?   // normal array
  /// ```
  ///
  /// Supported forms:
  /// - Comma arrays: `[a, b, c]`
  /// - One-element arrays: `[a]`, `[a,]`
  /// - Repeat arrays: `[value; count]`
  /// - Empty arrays: `[]`
  ///
  /// Notes:
  /// - `value` may be any expression.
  /// - `count` must be a compile-time constant literal (lexer enforces literal form).
  /// - The parser returns:
  ///   - `elements: Vec<Expr>` for comma arrays
  ///   - `repeat: Option<Expr>` for `[value; count]`
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

  /// Parses elements inside an array expression.
  ///
  /// Grammar reference (`<arrayElements>`):
  ///
  /// ```bnf
  /// arrayElements ::=
  ///       expression ";" expression        // repeat array
  ///     | expression ("," expression)* ","?
  ///     | ε                               // empty array
  /// ```
  ///
  /// Handles three cases:
  ///
  /// 1. **Empty array**: `[]`
  /// 2. **Repeat array**: `[value; count]`
  /// 3. **Comma array**: `[a, b, c]`
  ///
  /// Notes:
  /// - A missing repeat count (`[value;]`) is an error.
  /// - Normal arrays tolerate trailing commas.
  fn parse_array_elements(
    &mut self,
    engine: &mut DiagnosticEngine,
  ) -> Result<(Vec<Expr>, Option<Expr>), ()> {
    let mut elements = vec![];

    // Case 1: Empty array: `[]`
    if self.current_token().kind == TokenKind::CloseBracket {
      self.advance(engine);
      return Ok((elements, None));
    }

    // First element
    elements.push(self.parse_expression(vec![], ExprContext::Default, engine)?);

    // Case 2: Repeat array form `[value; count]`
    if matches!(self.current_token().kind, TokenKind::Semi) {
      self.advance(engine); // consume `;`

      // Reject `[value;]` - missing count expression
      if !self.current_token().kind.is_literal() {
        let bad = self.current_token();
        let lexeme = self.get_token_lexeme(&bad);

        let diagnostic = Diagnostic::new(
          DiagnosticCode::Error(DiagnosticError::UnexpectedToken),
          "invalid repeat expression".into(),
          self.source_file.path.clone(),
        )
        .with_label(
          bad.span,
          Some(format!(
            "expected an integer literal here, found `{lexeme}`"
          )),
          LabelStyle::Primary,
        )
        .with_note(
          "repeat array syntax is: `[value; N]` where `N` is a compile-time constant integer."
            .into(),
        )
        .with_help("example: `[x; 4]`".into());

        engine.add(diagnostic);
        return Err(());
      }

      let repeat_count = self.parse_expression(vec![], ExprContext::Default, engine)?;
      self.expect(TokenKind::CloseBracket, engine)?;
      return Ok((elements, Some(repeat_count)));
    }

    // Case 3: Standard comma-separated array                                 */
    while !self.is_eof() && !matches!(self.current_token().kind, TokenKind::CloseBracket) {
      if matches!(self.current_token().kind, TokenKind::Comma) {
        self.advance(engine);
      }

      if self.current_token().kind == TokenKind::CloseBracket {
        break;
      }

      elements.push(self.parse_expression(vec![], ExprContext::Default, engine)?);
    }

    self.expect(TokenKind::CloseBracket, engine)?;
    Ok((elements, None))
  }
}
