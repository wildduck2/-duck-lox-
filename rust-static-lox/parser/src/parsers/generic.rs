use crate::ast::{GenericArg, GenericArgs};
use crate::{DiagnosticEngine, Parser};
use diagnostic::code::DiagnosticCode;
use diagnostic::diagnostic::{Diagnostic, LabelStyle};
use diagnostic::types::error::DiagnosticError;
use lexer::token::{Token, TokenKind};

impl Parser {
  pub(crate) fn parse_generic_args(
    &mut self,
    token: &mut Token,
    engine: &mut DiagnosticEngine,
  ) -> Result<Option<GenericArgs>, ()> {
    if matches!(self.current_token().kind, TokenKind::ColonColon) {
      self.advance(engine); // consume the "::"

      let mut args = Vec::<GenericArg>::new();

      match self.current_token().kind {
        TokenKind::Lt => {
          self.advance(engine); // consume the "<"

          while !self.is_eof() && self.current_token().kind != TokenKind::Gt {
            args.push(self.parse_generic_arg(engine)?);
          }
          self.expect(TokenKind::Gt, engine)?; // consume the ">"
        },
        _ => {
          token.span.merge(self.current_token().span);
          let diagnostic = Diagnostic::new(
            DiagnosticCode::Error(DiagnosticError::MissingClosingBracket),
            "Expected '>' after generic type".to_string(),
            "duck.lox".to_string(),
          )
          .with_label(
            token.span,
            Some("expected '>' here".to_string()),
            LabelStyle::Primary,
          );

          engine.add(diagnostic);

          return Err(());
        },
      }

      Ok(Some(GenericArgs::AngleBracketed { args }))
    } else {
      Ok(None)
    }
  }

  pub(crate) fn parse_generic_arg(
    &mut self,
    engine: &mut DiagnosticEngine,
  ) -> Result<GenericArg, ()> {
    let token = self.current_token();
    let lexeme = self.get_token_lexeme(&token);

    match token.kind {
      TokenKind::Lifetime { .. } => {
        self.advance(engine); // consume the lifetime
        Ok(GenericArg::Lifetime(lexeme))
      },
      TokenKind::Ident => Ok(GenericArg::Type(self.parse_type(engine)?)),
      _ => {
        let diagnostic = Diagnostic::new(
          DiagnosticCode::Error(DiagnosticError::UnexpectedToken),
          format!("Unexpected token {:?}", lexeme),
          "duck.lox".to_string(),
        )
        .with_label(
          token.span,
          Some(format!(
            "Expected a primary expression, found \"{}\"",
            lexeme
          )),
          LabelStyle::Primary,
        )
        .with_help(Parser::get_token_help(&token.kind, &token));

        engine.add(diagnostic);

        Err(())
      },
    }
  }
}
