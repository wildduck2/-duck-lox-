use crate::{
  ast::{Mutability, Type},
  parser_utils::ExprContext,
  DiagnosticEngine, Parser,
};
use diagnostic::{
  code::DiagnosticCode,
  diagnostic::{Diagnostic, LabelStyle},
  types::error::DiagnosticError,
};
use lexer::token::TokenKind;

impl Parser {
  pub(crate) fn parse_type(&mut self, engine: &mut DiagnosticEngine) -> Result<Type, ()> {
    let mut token = self.current_token();
    let lexeme = self.get_token_lexeme(&token);
    self.advance(engine); // consume the identifier

    match token.kind {
      TokenKind::Ident | TokenKind::KwCrate => match lexeme.as_str() {
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
        // We allow parsing generic types like `Vec<T>` and `Option<T>`
        _ => {
          self.current -= 1; // This resests the current token to the last token
                             // thence the parse_path function will consume the last token
          Ok(Type::Path(self.parse_path(true, engine)?))
        },
      },

      TokenKind::OpenParen => {
        // TODO:
        println!("debug tuple: {:?}", self.current_token().kind);
        Err(())
        // let mut params = vec![];
        // while !self.is_eof() && !matches!(self.current_token().kind, TokenKind::CloseParen) {
        //   params.push(self.parse_type(engine)?);
        //   if matches!(self.current_token().kind, TokenKind::Comma) {
        //     self.advance(engine); // consume the comma
        //   }
        // }
        // self.expect(TokenKind::CloseParen, engine)?; // consume ')'
        //
        // Ok(Type::Tuple(params))
      },

      TokenKind::OpenBracket => {
        let element = self.parse_type(engine)?;
        self.expect(TokenKind::Semi, engine)?; // consume ';'

        let size = self.parse_expression(ExprContext::Default, engine)?;
        self.expect(TokenKind::CloseBracket, engine)?; // consume ']'

        Ok(Type::Array {
          element: Box::new(element),
          size: Box::new(size),
        })
      },

      TokenKind::Star => {
        if matches!(self.current_token().kind, TokenKind::Ident) {
          // This handles the case where we have an incomplete raw pointer like `*T`
          let diagnostic = Diagnostic::new(
            DiagnosticCode::Error(DiagnosticError::InvalidPointerType),
            "Missing mutability qualifier for raw pointer type.".to_string(),
            self.source_file.path.clone(),
          )
          .with_label(
            self.current_token().span,
            Some("expected `const` or `mut` after `*`.".to_string()),
            LabelStyle::Primary,
          )
          .with_note(
            "Raw pointers in Rust must explicitly specify mutability — either `*const T` or `*mut T`."
              .to_string(),
          )
          .with_help(
            "Use `*const T` for an immutable raw pointer, or `*mut T` for a mutable one."
              .to_string(),
          );

          engine.add(diagnostic);
          return Err(());
        }

        let mutability = self.parse_mutability(engine)?;

        Ok(Type::RawPointer {
          mutability,
          inner: Box::new(self.parse_type(engine)?),
        })
      },

      TokenKind::And => {
        if matches!(self.current_token().kind, TokenKind::KwConst) {
          // This handles the case where we have a const keyword after a &
          // like `&const T`
          //        ^^^^^ Error here
          let diagnostic = Diagnostic::new(
            DiagnosticCode::Error(DiagnosticError::InvalidMutabilityInField),
            "Invalid `const` specifier in struct field declaration.".to_string(),
            self.source_file.path.clone(),
          )
          .with_label(
            self.current_token().span,
            Some("`const` is not allowed after `&` or before a field type.".to_string()),
            LabelStyle::Primary,
          )
          .with_note(
            "`const` does not apply to references. Only raw pointers support const qualifiers."
              .to_string(),
          )
          .with_help(
            "Use `*const T` for a raw const pointer, or `&T` for an immutable reference."
              .to_string(),
          );

          engine.add(diagnostic);
          return Err(());
        }

        // This will handle the case where we have a &* like `&*const T` so a reference of a raw pointer
        if matches!(self.current_token().kind, TokenKind::Star) {
          return Ok(Type::Reference {
            lifetime: None,
            mutability: Mutability::Immutable,
            inner: Box::new(self.parse_type(engine)?),
          });
        }

        let lifetime = self.parse_lifetime(engine)?;
        let mutability = self.parse_mutability(engine)?;

        Ok(Type::Reference {
          lifetime,
          mutability,
          inner: Box::new(self.parse_type(engine)?),
        })
      },

      _ if matches!(token.kind, TokenKind::KwMut | TokenKind::KwConst) => {
        // This handles the case where we have a (const|mut) keyword before a type
        // and it's not a reference or pointer
        // like `const T` or `mut T`
        token.span.merge(self.current_token().span);
        let lexeme = self.get_token_lexeme(&token);

        let diagnostic = Diagnostic::new(
          DiagnosticCode::Error(DiagnosticError::InvalidMutabilityInField),
          format!("Invalid `{:?}` specifier in struct field declaration.", lexeme),
          self.source_file.path.clone(),
        )
        .with_label(
          token.span,
          Some(format!("`{:?}` is not allowed before a field type.", lexeme)),
          LabelStyle::Primary,
        )
        .with_note("`mut` and `const` cannot modify field declarations directly.".to_string())
        .with_help("Use `&mut T` or `*mut T` for references or pointers, or make the struct binding mutable.".to_string());

        engine.add(diagnostic);
        Err(())
      },

      _ => {
        token.span.merge(self.current_token().span);
        let lexeme = self.get_token_lexeme(&token);

        let diagnostic = Diagnostic::new(
          DiagnosticCode::Error(DiagnosticError::InvalidType),
          format!("Unknown type or unexpected token `{:?}`", lexeme),
          self.source_file.path.clone(),
        )
        .with_label(
          token.span,
          Some(format!(
            "Type `{:?}` is not recognized in this context",
            lexeme
          )),
          LabelStyle::Primary,
        )
        .with_help(format!(
          r"If `{:?}` is a custom type, ensure it is declared before use. Otherwise, \ check for typos or missing imports.",
          lexeme
        ));

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
}
