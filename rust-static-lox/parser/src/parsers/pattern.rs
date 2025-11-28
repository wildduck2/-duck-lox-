use diagnostic::DiagnosticEngine;
use lexer::token::TokenKind;

use crate::{
  ast::{pattern::*, QSelfHeader},
  match_and_consume,
  parser_utils::ExprContext,
  Parser,
};

impl Parser {
  pub(crate) fn parse_pattern_with_or(
    &mut self,
    context: ExprContext,
    engine: &mut DiagnosticEngine,
  ) -> Result<Pattern, ()> {
    let mut patterns = vec![self.parse_pattern(context, engine)?];

    while matches!(self.current_token().kind, TokenKind::Or) {
      self.advance(engine); // consume the '|'
      patterns.push(self.parse_pattern(context, engine)?);
    }

    if patterns.len() == 1 {
      return Ok(patterns.pop().unwrap());
    }

    Ok(Pattern::Or {
      patterns,
      span: self.current_token().span,
    })
  }

  fn parse_pattern(
    &mut self,
    context: ExprContext,
    engine: &mut DiagnosticEngine,
  ) -> Result<Pattern, ()> {
    let reference = match_and_consume!(self, engine, TokenKind::KwRef)?;
    let mutability = self.parse_mutability(engine)?;

    let mut token = self.current_token();

    // Parse Literal patterns like 42, true, false, etc.
    if token.kind.can_be_literal() {
      return self.parse_literal_pattern(context, engine);
    }

    match token.kind {
      // Parse Tuple patterns like (x, y, ..rest)
      TokenKind::OpenParen => self.parse_tuple_pattern(context, engine),

      // Parse reference patterns like &x, &mut x, &mut ref x, &&mut x, etc.
      TokenKind::And => self.parse_reference_pattern(context, engine),

      // parse (Identifier | Path | TupleStruct | Struct) pattern
      TokenKind::Ident | TokenKind::KwCrate | TokenKind::Lt => {
        if matches!(
          self.current_token().kind,
          TokenKind::ColonColon | TokenKind::Lt
        ) | matches!(
          self.peek(1).kind,
          TokenKind::ColonColon | TokenKind::OpenParen | TokenKind::OpenBrace | TokenKind::Bang
        ) {
          let qself_header = self.parse_qself_header(engine)?;
          let mut path = self.parse_path(true, engine)?;

          let (qself, path) = match qself_header {
            Some(QSelfHeader { self_ty, trait_ref }) => match trait_ref {
              Some(mut trait_ref) => {
                trait_ref.segments.extend(path.segments);
                path.leading_colon = trait_ref.leading_colon;
                (Some(self_ty), trait_ref)
              },
              None => (Some(self_ty), path),
            },
            None => (None, path),
          };

          if match_and_consume!(self, engine, TokenKind::Bang)? {
            let mac = self.parse_macro_invocation(path, qself, engine)?;
            return Ok(Pattern::Macro { mac });
          }

          // Handle tuple struct patterns like `Some(x, y, ..rest)`
          if match_and_consume!(self, engine, TokenKind::OpenParen)? {
            let mut patterns = vec![];
            while !self.is_eof() && !matches!(self.current_token().kind, TokenKind::CloseParen) {
              patterns.push(self.parse_pattern(context, engine)?);
              match_and_consume!(self, engine, TokenKind::Comma)?;
            }
            self.expect(TokenKind::CloseParen, engine)?;

            return Ok(Pattern::TupleStruct {
              qself,
              path,
              patterns,
              span: token.span,
            });
          }

          // Handle struct patterns like `Cords { x, y, ..rest }`
          if match_and_consume!(self, engine, TokenKind::OpenBrace)? {
            let mut has_rest = false;
            let mut fields = vec![];
            while !self.is_eof() && !matches!(self.current_token().kind, TokenKind::CloseBrace) {
              if matches!(self.current_token().kind, TokenKind::DotDot) {
                has_rest = true;
                fields.push(self.parse_field_pattern(context, engine)?);
                match_and_consume!(self, engine, TokenKind::Comma)?;
                break;
              }

              fields.push(self.parse_field_pattern(context, engine)?);
              match_and_consume!(self, engine, TokenKind::Comma)?;
            }
            self.expect(TokenKind::CloseBrace, engine)?;

            return Ok(Pattern::Struct {
              qself,
              path,
              fields,
              has_rest,
              span: token.span,
            });
          }

          // Handle path patterns like foo::bar::Baz
          return Ok(Pattern::Path {
            qself: None,
            path,
            span: token.span,
          });
        }

        // Handle ident patterns like x
        let name = self.get_token_lexeme(&token);
        self.advance(engine);

        let subpattern = if match_and_consume!(self, engine, TokenKind::At)? {
          Some(self.parse_pattern(context, engine)?)
        } else {
          None
        };

        token.span.merge(self.current_token().span);

        if matches!(name.as_str(), "_") {
          return Ok(Pattern::Wildcard { span: token.span });
        }

        let pattern = Pattern::Ident {
          binding: match reference {
            true => BindingMode::ByRef(mutability),
            false => BindingMode::ByValue(mutability),
          },
          name,
          subpattern: subpattern.map(Box::new),
          span: token.span,
        };

        Ok(pattern)
      },

      // Parse a (Slice) pattern
      TokenKind::OpenBracket => self.parse_slice_pattern(context, engine),

      // Parse a (Rest) pattern
      TokenKind::DotDot => self.parse_rest_pattern(engine),

      _ => {
        self.advance(engine);
        Ok(Pattern::Wildcard { span: token.span })
      },
    }
  }

  fn parse_reference_pattern(
    &mut self,
    context: ExprContext,
    engine: &mut DiagnosticEngine,
  ) -> Result<Pattern, ()> {
    let mut token = self.current_token();
    let mut depth = 0;
    while matches!(self.current_token().kind, TokenKind::And) {
      depth += 1;
      self.advance(engine);
    }

    let mutability = self.parse_mutability(engine)?;
    let pattern = self.parse_pattern(context, engine)?;

    token.span.merge(self.current_token().span);
    Ok(Pattern::Reference {
      depth,
      mutability,
      pattern: Box::new(pattern),
      span: token.span,
    })
  }

  fn parse_literal_pattern(
    &mut self,
    context: ExprContext,
    engine: &mut DiagnosticEngine,
  ) -> Result<Pattern, ()> {
    let mut token = self.current_token();

    let expr = self.parse_expression(context, engine)?;
    token.span.merge(self.current_token().span);
    Ok(Pattern::Literal {
      expr: Box::new(expr),
      span: token.span,
    })
  }

  fn parse_tuple_pattern(
    &mut self,
    context: ExprContext,
    engine: &mut DiagnosticEngine,
  ) -> Result<Pattern, ()> {
    let mut token = self.current_token();
    self.advance(engine); // consume the token "("

    let attributes = if matches!(self.current_token().kind, TokenKind::Pound) {
      self.parse_attributes(engine)?
    } else {
      vec![]
    };

    let mut patterns = vec![];
    while !self.is_eof() && !matches!(self.current_token().kind, TokenKind::CloseParen) {
      patterns.push(self.parse_pattern(context, engine)?);
      match_and_consume!(self, engine, TokenKind::Comma)?;
    }
    self.expect(TokenKind::CloseParen, engine)?;

    token.span.merge(self.current_token().span);

    Ok(Pattern::Tuple {
      patterns,
      attributes,
      span: token.span,
    })
  }

  fn parse_slice_pattern(
    &mut self,
    context: ExprContext,
    engine: &mut DiagnosticEngine,
  ) -> Result<Pattern, ()> {
    let mut token = self.current_token();
    self.advance(engine); // consume the token "["
    let mut before = vec![];
    let mut after = vec![];
    let mut middle: Option<Box<Pattern>> = None;

    // Empty slice: []
    if matches!(self.current_token().kind, TokenKind::CloseBracket) {
      let close = self.current_token().span;
      self.advance(engine);
      return Ok(Pattern::Slice {
        before,
        middle,
        after,
        span: *token.span.merge(close),
      });
    }

    // Parse patterns until we hit ']'
    let mut seen_middle = false;

    while !self.is_eof() && !matches!(self.current_token().kind, TokenKind::CloseBracket) {
      if self.current_token().kind == TokenKind::DotDot {
        if seen_middle {
          println!("Unexpected token \"..\"");
          return Err(());
        }

        middle = Some(Box::new(self.parse_pattern(context, engine)?));

        seen_middle = true;
        continue;
      }

      if !seen_middle {
        before.push(self.parse_pattern(context, engine)?);
      } else {
        after.push(self.parse_pattern(context, engine)?);
      }

      match_and_consume!(self, engine, TokenKind::Comma)?;

      if self.current_token().kind == TokenKind::CloseBracket {
        break;
      }
    }

    // Expect closing bracket
    self.expect(TokenKind::CloseBracket, engine)?;
    let close = self.current_token().span;

    Ok(Pattern::Slice {
      before,
      middle,
      after,
      span: *token.span.merge(close),
    })
  }

  fn parse_rest_pattern(&mut self, engine: &mut DiagnosticEngine) -> Result<Pattern, ()> {
    let span = self.current_token().span;
    self.advance(engine); // consume the token ".."
    let name = if matches!(self.current_token().kind, TokenKind::Ident) {
      let na_ = self.get_token_lexeme(&self.current_token());
      self.advance(engine);
      Some(na_)
    } else {
      None
    };

    Ok(Pattern::Rest { name, span })
  }

  fn parse_field_pattern(
    &mut self,
    context: ExprContext,
    engine: &mut DiagnosticEngine,
  ) -> Result<FieldPattern, ()> {
    let attributes = if matches!(self.current_token().kind, TokenKind::Pound) {
      self.parse_attributes(engine)?
    } else {
      vec![]
    };

    if matches!(self.current_token().kind, TokenKind::KwRef) {
      let mut name = String::from("");

      let mut pointer = 0;
      while !self.is_eof()
        && !matches!(
          self.peek(pointer).kind,
          TokenKind::Colon | TokenKind::CloseBrace | TokenKind::Comma
        )
      {
        let token = self.peek(pointer);
        if token.kind == TokenKind::Ident {
          let na_ = self.get_token_lexeme(&token);
          name = na_;
        }
        pointer += 1;
      }

      let pattern = Some(self.parse_pattern(context, engine)?);

      return Ok(FieldPattern {
        attributes,
        name,
        pattern,
        is_shorthand: true,
      });
    }

    if matches!(self.current_token().kind, TokenKind::DotDot) {
      let pattern = Some(self.parse_pattern(context, engine)?);
      return Ok(FieldPattern {
        attributes,
        name: "".to_string(),
        pattern,
        is_shorthand: true,
      });
    }

    let name = self.parse_name_identifier(engine)?;

    let (pattern, is_shorthand) = if match_and_consume!(self, engine, TokenKind::Colon)? {
      (Some(self.parse_pattern(context, engine)?), false)
    } else {
      (None, true)
    };

    Ok(FieldPattern {
      attributes,
      name,
      pattern,
      is_shorthand,
    })
  }
}
