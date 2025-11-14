use diagnostic::DiagnosticEngine;
use lexer::token::Token;

use crate::{ast::Expr, Parser};

impl Parser {
  /// Parses a normal identifier expression.
  ///
  /// Grammar:
  /// identExpr
  ///     -> IDENT
  ///
  /// Examples:
  /// foo
  /// variable_name
  ///
  /// This handles regular identifiers only. Contextual
  /// keywords like self or super are handled by parse_keyword_ident.
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

  /// Parses contextual keywords as identifier expressions.
  ///
  /// Grammar:
  /// keywordIdentExpr
  ///     -> "self"
  ///     |  "super"
  ///     |  "crate"
  ///     |  "Self"
  ///
  /// Notes:
  /// These tokens behave like identifiers inside expressions.
  /// They do not begin type paths or module paths here.
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
