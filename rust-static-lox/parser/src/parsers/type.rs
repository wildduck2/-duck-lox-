use crate::ast::{path::*, Mutability, Type};
use crate::{DiagnosticEngine, Parser};
use diagnostic::code::DiagnosticCode;
use diagnostic::diagnostic::{Diagnostic, LabelStyle};
use diagnostic::types::error::DiagnosticError;
use lexer::token::TokenKind;

impl Parser {
  pub(crate) fn parse_type(&mut self, engine: &mut DiagnosticEngine) -> Result<Type, ()> {
    let mut token = self.current_token();
    let lexeme = self.get_token_lexeme(&token);
    self.advance(engine); // consume the identifier

    match token.kind {
      TokenKind::Ident => match lexeme.as_str() {
        "u8" => Ok(Type::U8),
        "u16" => Ok(Type::U16),
        "u32" => Ok(Type::U32),
        "u64" => Ok(Type::U64),
        "u128" => Ok(Type::U128),
        "usize" => Ok(Type::Usize),

        "i8" => Ok(Type::I8),
        "i16" => Ok(Type::I16),
        "i32" => Ok(Type::I32),
        "i64" => Ok(Type::I64),
        "i128" => Ok(Type::I128),
        "isize" => Ok(Type::Isize),

        "f32" => Ok(Type::F32),
        "f64" => Ok(Type::F64),
        "f128" => Ok(Type::F128),

        "char" => Ok(Type::Char),
        "str" => Ok(Type::Str),
        "string" => Ok(Type::String),

        "bool" => Ok(Type::Bool),
        // TODO: check if we need generics parsing in the path here
        _ => Ok(Type::Path(self.parse_path(true, engine)?)),
      },
      TokenKind::And => {
        let mutability = self.parse_mutability(engine)?;
        let lifetime = self.parse_lifetime(engine)?;

        Ok(Type::Reference {
          lifetime,
          mutability,
          inner: Box::new(self.parse_type(engine)?),
        })
      },

      _ => {
        // TODO: Handle the errors diagnostics here
        token.span.merge(self.current_token().span);
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

  pub(crate) fn parse_lifetime(
    &mut self,
    engine: &mut DiagnosticEngine,
  ) -> Result<Option<String>, ()> {
    if matches!(self.current_token().kind, TokenKind::Lifetime { .. })
      && matches!(self.peek_prev(0).kind, TokenKind::And)
    {
      let token = self.current_token();
      self.advance(engine); // consume the lifetime
      Ok(Some(self.get_token_lexeme(&token)))
    } else {
      Ok(None)
    }
  }

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
      _ if !matches!(token.kind, TokenKind::Ident | TokenKind::Lifetime { .. }) => {
        token.span.merge(self.current_token().span);
        let lexeme = self.get_token_lexeme(&token);

        let diagnostic = Diagnostic::new(
          DiagnosticCode::Error(DiagnosticError::UnexpectedToken),
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
