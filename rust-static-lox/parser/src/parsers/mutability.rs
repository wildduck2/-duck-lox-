use crate::ast::Mutability;
use crate::{DiagnosticEngine, Parser};
use diagnostic::{
  code::DiagnosticCode,
  diagnostic::{Diagnostic, LabelStyle},
  types::error::DiagnosticError,
};
use lexer::token::TokenKind;

impl Parser {
  /// Function that parses a mutability specifier
  /// It returns a `Mutability` enum that represents the mutability specifier
  ///
  /// for example:
  /// ```rust
  /// let mut x = 10;
  /// ```
  /// You will use this to get the mutability of a variable or binding
  pub(crate) fn parse_mutability(
    &mut self,
    engine: &mut DiagnosticEngine,
  ) -> Result<Mutability, ()> {
    let mut token = self.current_token();

    match token.kind {
      TokenKind::KwMut => {
        self.advance(engine); // consume the "mut"
        Ok(Mutability::Mutable)
      },
      TokenKind::KwConst => {
        self.advance(engine); // consume the "const"
        Ok(Mutability::Immutable)
      },
      // TODO: check for all the edge cases that kind is not allowed here
      _ if !matches!(
        token.kind,
        TokenKind::Ident | TokenKind::OpenBracket | TokenKind::Lifetime { .. } | TokenKind::And
      ) =>
      {
        token.span.merge(self.current_token().span);
        let lexeme = self.get_token_lexeme(&token);

        let diagnostic = Diagnostic::new(
          DiagnosticCode::Error(DiagnosticError::InvalidLifetime),
          format!(
            "Unexpected token `{}` while parsing mutability specifier.",
            lexeme
          ),
          self.source_file.path.clone(),
        )
        .with_label(
          token.span,
          Some(format!(
            "Expected `mut` or `const`, but found `{}` instead.",
            lexeme
          )),
          LabelStyle::Primary,
        )
        .with_note(
          "Mutability specifiers must precede variable declarations or bindings.".to_string(),
        )
        .with_help(
          "Try using `mut` for mutable bindings, or `const` for immutable constants.\n\
                Example: `mut x = 10` or `const PI = 3.14`"
            .to_string(),
        )
        .with_help(Parser::get_token_help(&token.kind, &token));

        engine.add(diagnostic);
        Err(())
      },
      _ => Ok(Mutability::Immutable),
    }
  }
}
