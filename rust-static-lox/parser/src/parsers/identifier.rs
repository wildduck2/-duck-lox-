use diagnostic::DiagnosticEngine;
use lexer::token::Token;

use crate::{ast::Expr, Parser};

impl Parser {
  /// Parses a standard identifier expression.
  ///
  /// Grammar:
  ///
  ///   identExpr ::= IDENT
  ///
  /// This rule covers ordinary identifiers such as:
  ///   foo
  ///   variable_name
  ///
  /// Notes:
  /// - This function handles normal identifiers only.
  /// - Contextual identifiers such as self, super, crate, and Self
  ///   are parsed by parse_keyword_ident.
  pub(crate) fn parser_ident(
    &mut self,
    token: &mut Token,
    engine: &mut DiagnosticEngine,
  ) -> Result<Expr, ()> {
    let name = self
      .source_file
      .src
      .get(token.span.start..token.span.end)
      .unwrap()
      .to_string();

    // consume the identifier
    self.advance(engine);
    token.span.merge(self.current_token().span);

    Ok(Expr::Ident {
      name,
      span: token.span,
    })
  }

  /// Parses contextual keyword identifiers.
  ///
  /// Grammar:
  ///
  ///   keywordIdentExpr ::= "self"
  ///                      | "super"
  ///                      | "crate"
  ///                      | "Self"
  ///
  /// Notes:
  /// - These keywords behave like identifiers in expression position.
  /// - They do not initiate type paths or module paths when used here.
  /// - The returned expression is always Expr::Ident.
  pub(crate) fn parse_keyword_ident(
    &mut self,
    token: &mut Token,
    engine: &mut DiagnosticEngine,
  ) -> Result<Expr, ()> {
    let name = self.get_token_lexeme(token);

    // consume the keyword token
    self.advance(engine);
    token.span.merge(self.current_token().span);

    Ok(Expr::Ident {
      name,
      span: token.span,
    })
  }
}
