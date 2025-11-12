use crate::ast::{
  GenericArg, GenericArgs, GenericParam, GenericParams, TraitBoundModifier, TypeBound,
};
use crate::{DiagnosticEngine, Parser};
use diagnostic::code::DiagnosticCode;
use diagnostic::diagnostic::{Diagnostic, LabelStyle};
use diagnostic::types::error::DiagnosticError;
use lexer::token::{Token, TokenKind};

impl Parser {
  pub(crate) fn parse_generic_params(
    &mut self,
    token: &mut Token,
    engine: &mut DiagnosticEngine,
  ) -> Result<Option<GenericParams>, ()> {
    let mut params = Vec::<GenericParam>::new();

    self.expect(TokenKind::Lt, engine)?; // consume the "<"
    while !self.is_eof() && self.current_token().kind != TokenKind::Gt {
      params.push(self.parse_generic_param(engine)?);

      if matches!(self.current_token().kind, TokenKind::Comma) {
        self.advance(engine); // consume the comma
      }
    }
    self.expect(TokenKind::Gt, engine)?; // consume the ">"

    token.span.merge(self.current_token().span);
    Ok(Some(GenericParams {
      params,
      span: token.span,
    }))
  }

  pub(crate) fn parse_generic_param(
    &mut self,
    engine: &mut DiagnosticEngine,
  ) -> Result<GenericParam, ()> {
    let token = self.current_token();
    let name = self.get_token_lexeme(&token);
    self.advance(engine); // consume the identifier

    match token.kind {
      TokenKind::Ident => {
        let bounds = if self.current_token().kind == TokenKind::Colon {
          self.advance(engine); // consume the ":"
          Some(self.parse_type_bounds(engine)?)
        } else {
          None
        };

        let default = if matches!(self.current_token().kind, TokenKind::Eq) {
          self.advance(engine);
          Some(self.parse_type(engine)?)
        } else {
          None
        };

        Ok(GenericParam::Type {
          attributes: vec![],
          name,
          bounds,
          default,
        })
      },
      TokenKind::Lifetime { .. } => {
        // TODO: handle the lifetime bounds like `for<'a: 'b>`
        Ok(GenericParam::Lifetime {
          attributes: vec![],
          name,
          bounds: None,
        })
      },
      _ => Err(()),
    }
  }

  pub(crate) fn parse_type_bounds(
    &mut self,
    engine: &mut DiagnosticEngine,
  ) -> Result<Vec<TypeBound>, ()> {
    let mut bounds: Vec<TypeBound> = vec![];

    while !self.is_eof()
      && !matches!(
        self.current_token().kind,
        TokenKind::Comma | TokenKind::Gt | TokenKind::Eq
      )
    {
      let modifier = if matches!(self.current_token().kind, TokenKind::Question) {
        // (e.g., `?Clone`)
        self.advance(engine); // consume the "?"
        TraitBoundModifier::Maybe
      } else if matches!(self.current_token().kind, TokenKind::KwConst) {
        // (e.g., `const Clone`)
        self.advance(engine); // consume the "const"
        TraitBoundModifier::Const
      } else if matches!(self.current_token().kind, TokenKind::KwConst)
        && matches!(self.peek(1).kind, TokenKind::Question)
      {
        self.advance(engine); // consume the "const"
        self.advance(engine); // consume the "?"
                              // (e.g., `const ?Clone`)
        TraitBoundModifier::MaybeConst
      } else {
        TraitBoundModifier::None
      };

      // TODO: check if we need generics parsing in the path here
      let path = self.parse_path(true, engine)?;
      self.advance(engine); // consume the identifier

      bounds.push(TypeBound {
        modifier,
        path,
        generics: None,
        for_lifetimes: None,
      });

      if matches!(self.current_token().kind, TokenKind::Plus) {
        self.advance(engine); // consume the plus
      }
    }

    Ok(bounds)
  }

  pub(crate) fn parse_generic_args(
    &mut self,
    token: &mut Token,
    engine: &mut DiagnosticEngine,
  ) -> Result<Option<GenericArgs>, ()> {
    if matches!(self.current_token().kind, TokenKind::ColonColon) {
      self.advance(engine); // consume the "::"
    }

    let mut args = Vec::<GenericArg>::new();

    self.expect(TokenKind::Lt, engine)?; // consume the "<"
    while !self.is_eof() && self.current_token().kind != TokenKind::Gt {
      args.push(self.parse_generic_arg(engine)?);

      if matches!(self.current_token().kind, TokenKind::Comma) {
        self.advance(engine); // consume the comma
      }
    }
    self.expect(TokenKind::Gt, engine)?; // consume the ">"

    Ok(Some(GenericArgs::AngleBracketed { args }))
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
