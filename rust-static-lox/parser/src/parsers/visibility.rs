use diagnostic::code::DiagnosticCode;
use diagnostic::diagnostic::{Diagnostic, LabelStyle};
use diagnostic::types::error::DiagnosticError;
use diagnostic::DiagnosticEngine;
use lexer::token::TokenKind;

use crate::{ast::Visibility, Parser};

impl Parser {
  /// Parses the visibility modifiers like :
  /// `pub`, `pub(crate)`, `pub(self)`, `pub(in path::to::module)`, or `pub(in path::to::module)`
  ///
  /// Returns a `Visibility` enum variant.
  /// We use this to parse the visibility modifiers before any item declarations.
  ///
  /// for example:
  /// ```rust
  /// pub(self) struct User{
  ///   name: String,
  /// }
  /// ```
  pub(crate) fn parse_visibility(
    &mut self,
    engine: &mut DiagnosticEngine,
  ) -> Result<Visibility, ()> {
    let token = self.current_token();

    self.advance(engine); // consume the "pub"
    match token.kind {
      TokenKind::KwPub if matches!(self.current_token().kind, TokenKind::OpenParen) => {
        self.advance(engine); // consume '('

        let restriction = self.current_token();
        self.advance(engine); // consume keyword inside ()

        let visibility = match restriction.kind {
          TokenKind::KwCrate => Ok(Visibility::LicCrate),
          TokenKind::KwSelfValue => Ok(Visibility::LicSelf),
          TokenKind::KwSuper => Ok(Visibility::LicSuper),
          TokenKind::KwIn => {
            // Check if we have `pub(in crate)`, thus it's the same as `pub(crate)`
            if matches!(self.current_token().kind, TokenKind::KwCrate)
              && self.peek(1).kind == TokenKind::CloseParen
            {
              return Ok(Visibility::LicCrate);
            }

            // Handle the `pub(in path::to::module)` case
            // Notice we do not expect the path here to have generic args
            // so if any occur, we will report an error
            // for example: `pub(in path::to::<T>::module)`
            Ok(Visibility::LicIn(self.parse_path(false, engine)?))
          },
          _ => {
            let lexeme = self.get_token_lexeme(&restriction);

            let diagnostic = Diagnostic::new(
              DiagnosticCode::Error(DiagnosticError::InvalidVisibilityRestriction),
              format!("Invalid visibility restriction '{}'", lexeme),
              self.source_file.path.clone(),
            )
            .with_label(
              restriction.span,
              Some(format!(
                "Expected one of 'crate', 'self', 'super', or a valid module path, but found '{}'",
                lexeme
              )),
              LabelStyle::Primary,
            )
            .with_help("Valid forms are: 'pub(crate)', 'pub(super)', 'pub(self)', or 'pub(in path::to::module)'.".to_string())
            .with_note("Visibility restrictions limit the scope of public items.".to_string());

            engine.add(diagnostic);
            Err(())
          },
        };

        self.expect(TokenKind::CloseParen, engine)?; // consume ')'
        visibility
      },
      TokenKind::KwPub => Ok(Visibility::Lic),
      TokenKind::Ident | TokenKind::KwStruct => {
        // This sets the current token to the previous token
        // so that we can consume the identifier
        // Hence the private visibility does not have any words to consume
        self.current -= 1;
        Ok(Visibility::Private)
      },

      _ => {
        let lexeme = self.get_token_lexeme(&token);
        let diagnostic = Diagnostic::new(
          DiagnosticCode::Error(DiagnosticError::UnexpectedToken),
          format!("Unexpected token '{}'", lexeme),
          self.source_file.path.clone(),
        )
        .with_label(
          token.span,
          Some(format!("Unexpected token '{}'", lexeme)),
          LabelStyle::Primary,
        )
        .with_help(
          "Expected a visibility modifier like 'pub', 'pub(crate)', or leave it private."
            .to_string(),
        );

        engine.add(diagnostic);
        Err(())
      },
    }
  }
}
