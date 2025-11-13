use diagnostic::{
  code::DiagnosticCode,
  diagnostic::{Diagnostic, LabelStyle},
  types::error::DiagnosticError,
  DiagnosticEngine,
};
use lexer::token::TokenKind;

use crate::{
  ast::{WhereClause, WherePredicate},
  Parser,
};

impl Parser {
  /// Function that parses a where clause
  /// It returns a `Option<WhereClause>` enum that represents a where clause
  ///
  /// for example:
  /// ```rust
  /// let clause = self.parse_where_clause(engine)?;
  /// ```
  ///
  /// You will use this to get the where clause of a struct declaration
  /// like `struct User { name: String, age: u8 } where T: Clone + PartialEq`
  pub(crate) fn parse_where_clause(
    &mut self,
    engine: &mut DiagnosticEngine,
  ) -> Result<Option<WhereClause>, ()> {
    if matches!(self.current_token().kind, TokenKind::KwWhere) {
      self.advance(engine); // consume the where keyword

      let mut predicates = vec![];

      while !self.is_eof() && !matches!(self.current_token().kind, TokenKind::OpenBrace) {
        predicates.push(self.parse_type_predicate(engine)?);

        if matches!(self.current_token().kind, TokenKind::Comma) {
          self.advance(engine); // consume the semicolon
        }
      }

      return Ok(Some(WhereClause { predicates }));
    }

    Ok(None)
  }

  /// Function that parses a type predicate
  /// It returns a `WherePredicate` enum that represents a type predicate
  ///
  /// for example:
  /// ```rust
  /// let predicate = self.parse_type_predicate(engine)?;
  /// ```
  ///
  /// You will use this to get the predicates of a where clause
  /// like `T: Clone + PartialEq`
  pub(crate) fn parse_type_predicate(
    &mut self,
    engine: &mut DiagnosticEngine,
  ) -> Result<WherePredicate, ()> {
    let token = self.current_token();

    if matches!(token.kind, TokenKind::Lifetime { .. }) {
      // This handle the case where we have lifetime bound in the where clause
      // like `where 'a: 'b + 'c`
      let lifetime = self.get_token_lexeme(&token);
      self.advance(engine); // consume the lifetime

      let lifetime_bounds = if matches!(self.current_token().kind, TokenKind::Colon) {
        self.advance(engine); // consume the ":"

        self.parse_lifetime_bounds(engine)?
      } else {
        vec![]
      };

      return Ok(WherePredicate::Lifetime {
        lifetime,
        bounds: lifetime_bounds,
      });
    }

    // If it's not a lifetime, then it must be a type bound like `T: Clone` so we continue parsing
    let ty = self.parse_type(engine)?;

    match self.current_token().kind {
      TokenKind::Colon => {
        self.advance(engine); // consume the ":"
        let bounds = if !matches!(self.current_token().kind, TokenKind::Lifetime { .. }) {
          Some(self.parse_type_bounds(engine)?)
        } else {
          None
        };

        let for_lifetimes = if matches!(self.current_token().kind, TokenKind::Lifetime { .. }) {
          Some(self.parse_lifetime_bounds(engine)?)
        } else {
          None
        };

        Ok(WherePredicate::Type {
          for_lifetimes,
          ty,
          bounds,
        })
      },

      TokenKind::Eq => {
        self.advance(engine); // consume the "="

        Ok(WherePredicate::Equality {
          ty,
          equals: self.parse_type(engine)?,
        })
      },

      _ => {
        let lexeme = self.get_token_lexeme(&self.current_token());

        let diagnostic = Diagnostic::new(
          DiagnosticCode::Error(DiagnosticError::InvalidWherePredicate),
          format!("unexpected token `{}` in where-clause predicate", lexeme),
          self.source_file.path.clone(),
        )
        .with_label(
          token.span,
          Some("expected `:`, `=`, or a valid bound here".to_string()),
          LabelStyle::Primary,
        )
        .with_note(
          "a predicate must be one of: `T: Bound`, `'a: 'b`, or `T::Assoc = Type`".to_string(),
        );

        engine.add(diagnostic);
        Err(())
      },
    }
  }
}
