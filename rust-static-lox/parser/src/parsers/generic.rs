use crate::ast::path::{Path, PathSegment, PathSegmentKind};
use crate::ast::{
  GenericArg, GenericArgs, GenericParam, GenericParams, TraitBoundModifier, Type, TypeBound,
};
use crate::parsers::mutability;
use crate::{DiagnosticEngine, Parser};
use diagnostic::code::DiagnosticCode;
use diagnostic::diagnostic::{Diagnostic, LabelStyle};
use diagnostic::types::error::DiagnosticError;
use lexer::token::{Token, TokenKind};

impl Parser {
  /// Parses `<...>` generic parameter lists and returns `None` when absent.
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

  /// Parses a single generic parameter (type, lifetime, or const).
  pub(crate) fn parse_generic_param(
    &mut self,
    engine: &mut DiagnosticEngine,
  ) -> Result<GenericParam, ()> {
    let attributes = if matches!(self.current_token().kind, TokenKind::Pound) {
      self.parse_attributes(engine)?
    } else {
      vec![]
    };

    let token = self.current_token();

    match token.kind {
      // const generic: const N: usize = 3
      TokenKind::KwConst => {
        self.advance(engine); // consume "const"
        let name = self.parse_name_identifier(engine)?;

        self.expect(TokenKind::Colon, engine)?; // must have ":"
        let ty = self.parse_type(engine)?;

        let default = if matches!(self.current_token().kind, TokenKind::Eq) {
          self.advance(engine);
          Some(self.parse_type(engine)?)
        } else {
          None
        };

        Ok(GenericParam::Const {
          attributes,
          name,
          ty,
          default,
        })
      },

      // lifetime generic: 'a or 'a: 'b + 'c
      TokenKind::Lifetime { .. } => {
        let name = self.get_token_lexeme(&token);
        self.advance(engine);

        let bounds = if matches!(self.current_token().kind, TokenKind::Colon) {
          self.advance(engine);
          Some(self.parse_type_lifetime_bounds(engine)?)
        } else {
          None
        };

        Ok(GenericParam::Lifetime {
          attributes,
          name,
          bounds,
        })
      },

      // type generic: T, U: Bound, T = Default
      TokenKind::Ident => {
        let name = self.parse_name_identifier(engine)?;

        let bounds = if matches!(self.current_token().kind, TokenKind::Colon) {
          self.advance(engine);
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
          attributes,
          name,
          bounds,
          default,
        })
      },

      _ => {
        let lexeme = self.get_token_lexeme(&token);
        let diagnostic = Diagnostic::new(
          DiagnosticCode::Error(DiagnosticError::UnexpectedToken),
          format!("unexpected token `{}` in generic parameter list", lexeme),
          self.source_file.path.clone(),
        )
        .with_label(
          token.span,
          Some("expected a type, lifetime, or const parameter".to_string()),
          LabelStyle::Primary,
        );

        engine.add(diagnostic);
        Err(())
      },
    }
  }

  /// Parses either lifetime or trait bounds that follow a colon.
  pub(crate) fn parse_type_bounds(
    &mut self,
    engine: &mut DiagnosticEngine,
  ) -> Result<Vec<TypeBound>, ()> {
    if matches!(self.current_token().kind, TokenKind::Lifetime { .. }) {
      self.parse_type_lifetime_bounds(engine)
    } else {
      self.parse_type_path_bounds(engine)
    }
  }

  /// Parses the `+ 'a + 'b` lifetime bounds chain.
  pub(crate) fn parse_type_lifetime_bounds(
    &mut self,
    engine: &mut DiagnosticEngine,
  ) -> Result<Vec<TypeBound>, ()> {
    let mut lifetime_bounds = vec![];

    while !self.is_eof()
      && !matches!(
        self.current_token().kind,
        TokenKind::OpenBrace | TokenKind::Comma | TokenKind::KwWhere | TokenKind::Gt
      )
    {
      let lifetime = self.get_token_lexeme(&self.current_token());
      self.advance(engine); // consume the lifetime
      lifetime_bounds.push(TypeBound {
        modifier: TraitBoundModifier::None,
        path: Path {
          leading_colon: false,
          segments: vec![PathSegment {
            kind: PathSegmentKind::Ident(lifetime),
            args: None,
          }],
        },
        generics: None,
        for_lifetimes: None,
      });
    }

    Ok(lifetime_bounds)
  }

  /// Parses trait bounds (`T: Trait + ?Sized`) and their modifiers.
  pub(crate) fn parse_type_path_bounds(
    &mut self,
    engine: &mut DiagnosticEngine,
  ) -> Result<Vec<TypeBound>, ()> {
    let mut bounds: Vec<TypeBound> = vec![];
    while !self.is_eof()
      && !matches!(
        self.current_token().kind,
        TokenKind::Comma | TokenKind::Gt | TokenKind::Eq | TokenKind::CloseParen
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

      let path = self.parse_path(false, engine)?;

      // TODO: you will make this separate lifetimes from generic args
      let (generics, for_lifetimes) = if matches!(self.current_token().kind, TokenKind::Lt) {
        self.advance(engine); // consume the "<"

        // TODO: handle this later
        //if matches!(self.current_token().kind, TokenKind::Lifetime { .. }) {
        let mut lifetime = vec![];
        while !self.is_eof() && !matches!(self.current_token().kind, TokenKind::Gt) {
          lifetime.extend(self.parse_lifetime_bounds(engine)?);
          if matches!(self.current_token().kind, TokenKind::Comma) {
            self.advance(engine); // consume the comma
          }
        }
        self.expect(TokenKind::Gt, engine)?; // consume the ">"

        ((), Some(lifetime))
        // } else {
        //   ((), None)
        // }
      } else {
        ((), None)
      };

      bounds.push(TypeBound {
        modifier,
        path,
        generics: None, // TODO: handle this later
        for_lifetimes,
      });

      if matches!(self.current_token().kind, TokenKind::Plus) {
        self.advance(engine); // consume the plus
      }
    }
    Ok(bounds)
  }

  /// Parses generic arguments following `::?<...>`.
  pub(crate) fn parse_generic_args(
    &mut self,
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

  /// Parses a single generic argument (type, lifetime, const, binding, …).
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
      TokenKind::KwSelfType => {
        self.advance(engine); // consume the 'Self'
        Ok(GenericArg::Type(Type::SelfType))
      },
      TokenKind::OpenParen => {
        self.advance(engine); // consume the '('
        let mut params = vec![];
        while !self.is_eof() && !matches!(self.current_token().kind, TokenKind::CloseParen) {
          params.push(self.parse_type(engine)?);
          if matches!(self.current_token().kind, TokenKind::Comma) {
            self.advance(engine); // consume the comma
          }
        }
        self.expect(TokenKind::CloseParen, engine)?; // consume ')'

        Ok(GenericArg::Type(Type::Tuple(params)))
      },
      _ => {
        // TODO: enhance the diagnostic later on when we have a full clousure
        let diagnostic = Diagnostic::new(
          DiagnosticCode::Error(DiagnosticError::UnexpectedToken),
          format!("unexpected token `{lexeme}` in generic argument list"),
          self.source_file.path.clone(),
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
