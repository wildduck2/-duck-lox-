use diagnostic::{
  code::DiagnosticCode,
  diagnostic::{Diagnostic, LabelStyle},
  types::error::DiagnosticError,
  DiagnosticEngine,
};
use lexer::token::{Token, TokenKind};

use crate::{
  ast::{
    AttrKind, AttrStyle, Attribute, Delimiter, Expr, Path, PathSegment, PathSegmentKind, TokenTree,
  },
  parser_utils::ExprContext,
  Parser,
};

impl Parser {
  pub(crate) fn parse_grouped_expr(
    &mut self,
    token: &mut Token,
    engine: &mut DiagnosticEngine,
  ) -> Result<Expr, ()> {
    self.advance(engine); // consume the open paren

    if matches!(self.current_token().kind, TokenKind::CloseParen) {
      self.advance(engine); // consume the close paren
      token.span.merge(self.current_token().span);
      return Ok(Expr::Unit(token.span));
    }

    let mut elements = vec![];

    let attributes = if matches!(self.current_token().kind, TokenKind::Pound) {
      self.parse_attributes(engine)?
    } else {
      vec![]
    };

    while !self.is_eof() && !matches!(self.current_token().kind, TokenKind::CloseParen) {
      if self.current_token().kind == TokenKind::Comma {
        self.advance(engine); // consume the comma
      }

      if self.current_token().kind == TokenKind::CloseParen {
        break;
      }

      elements.push(self.parse_expression(ExprContext::Default, engine)?);
    }

    self.expect(TokenKind::CloseParen, engine)?; // consume the close paren
    token.span.merge(self.current_token().span);

    if elements.len() == 1 {
      Ok(Expr::Group {
        attributes,
        expr: Box::new(elements[0].clone()),
        span: token.span,
      })
    } else {
      Ok(Expr::Tuple {
        attributes,
        elements,
        span: token.span,
      })
    }
  }

  pub(crate) fn parse_attributes(
    &mut self,
    engine: &mut DiagnosticEngine,
  ) -> Result<Vec<Attribute>, ()> {
    let mut attr = vec![];
    while self.current_token().kind == TokenKind::Pound {
      attr.push(self.parse_attribute(engine)?);
    }

    Ok(attr)
  }

  pub(crate) fn parse_attribute(&mut self, engine: &mut DiagnosticEngine) -> Result<Attribute, ()> {
    let mut token = self.current_token();

    let attr_style = match self.current_token().kind {
      TokenKind::Pound if self.peek(1).kind == TokenKind::Bang => {
        self.advance(engine); // consume the pound
        self.advance(engine); // consume the bang
        AttrStyle::Inner
      },
      TokenKind::Pound => {
        self.advance(engine); // consume the pound
        AttrStyle::Outer
      },
      _ => {
        let diagnostic = Diagnostic::new(
          DiagnosticCode::Error(DiagnosticError::UnexpectedToken),
          "Unexpected token".to_string(),
          self.source_file.path.clone(),
        )
        .with_label(
          self.current_token().span,
          Some("Expected an attribute, found \"{}\"".to_string()),
          LabelStyle::Primary,
        )
        .with_help(Parser::get_token_help(
          &self.current_token().kind,
          &self.current_token(),
        ));

        engine.add(diagnostic);

        return Err(());
      },
    };

    self.expect(TokenKind::OpenBracket, engine)?; // consume the open bracket
    let attr_input = self.parse_attribute_input(engine)?;
    self.expect(TokenKind::CloseBracket, engine)?; // consume the close bracket

    token.span.merge(self.current_token().span);

    Ok(Attribute {
      style: attr_style,
      kind: attr_input,
      span: token.span,
    })
  }

  pub(crate) fn parse_attribute_input(
    &mut self,
    engine: &mut DiagnosticEngine,
  ) -> Result<AttrKind, ()> {
    let path = self.parse_simple_path(engine)?;
    let attr_input_tail = self.parse_attribute_input_tail(engine)?;

    Ok(AttrKind::Normal {
      path,
      tokens: attr_input_tail,
    })
  }

  pub(crate) fn parse_simple_path(&mut self, engine: &mut DiagnosticEngine) -> Result<Path, ()> {
    let leading_colon = if matches!(self.current_token().kind, TokenKind::ColonColon) {
      self.advance(engine);
      true
    } else {
      false
    };

    let mut segments = vec![];
    let (segment, is_dollar_crate) = self.parse_path_segment(engine)?;
    segments.push(segment);

    while !self.is_eof()
      && !matches!(
        self.current_token().kind,
        TokenKind::CloseBracket | TokenKind::Eq | TokenKind::OpenParen
      )
    {
      self.expect(TokenKind::ColonColon, engine)?;
      let (segment, is_dollar_crate) = self.parse_path_segment(engine)?;
      if is_dollar_crate {
        let diagnostic = Diagnostic::new(
          DiagnosticCode::Error(DiagnosticError::UnexpectedToken),
          "Unexpected $crate segment in path".to_string(),
          self.source_file.path.clone(),
        )
        .with_label(
          self.current_token().span,
          Some("Expected an identifier, found $crate".to_string()),
          LabelStyle::Primary,
        );
        engine.add(diagnostic);
        return Err(());
      }
      segments.push(segment);
    }

    Ok(Path {
      leading_colon: leading_colon || is_dollar_crate,
      segments,
    })
  }

  pub(crate) fn parse_path_segment(
    &mut self,
    engine: &mut DiagnosticEngine,
  ) -> Result<(PathSegment, bool), ()> {
    let token = self.current_token();
    self.advance(engine);

    match token.kind {
      // TODO: eventually you will need it in some path contexts.
      // so you will replace the PathSegment::new with the full path struct
      TokenKind::KwSelfValue => Ok((PathSegment::new(PathSegmentKind::Self_), false)),
      TokenKind::KwSuper => Ok((PathSegment::new(PathSegmentKind::Super), false)),
      TokenKind::KwCrate => Ok((PathSegment::new(PathSegmentKind::Crate), false)),
      TokenKind::Ident => {
        let lexeme = self.get_token_lexeme(&token);
        Ok((PathSegment::new(PathSegmentKind::Ident(lexeme)), false))
      },
      TokenKind::Dollar if self.peek(0).kind == TokenKind::KwCrate => {
        self.advance(engine);
        Ok((PathSegment::new(PathSegmentKind::DollarCrate), true))
      },
      _ => {
        let lexeme = self.get_token_lexeme(&token);
        let diagnostic = Diagnostic::new(
          DiagnosticCode::Error(DiagnosticError::UnexpectedToken),
          "Unexpected token".to_string(),
          self.source_file.path.clone(),
        )
        .with_label(
          token.span,
          Some(format!("Expected a path segment, found {}", lexeme)),
          LabelStyle::Primary,
        );
        engine.add(diagnostic);
        Err(())
      },
    }
  }

  pub(crate) fn parse_attribute_input_tail(
    &mut self,
    engine: &mut DiagnosticEngine,
  ) -> Result<Vec<TokenTree>, ()> {
    let mut tokens = Vec::new();

    if self.current_token().kind == TokenKind::Eq {
      // consume '='
      tokens.push(TokenTree::Token("=".into()));
      self.advance(engine);

      // capture everything until ']' or ',' (depending on context)
      while !matches!(
        self.current_token().kind,
        TokenKind::CloseBracket | TokenKind::Comma | TokenKind::Eof
      ) {
        let lexeme = self.get_token_lexeme(&self.current_token());
        tokens.push(TokenTree::Token(lexeme));
        self.advance(engine);
      }

      return Ok(tokens);
    }

    // otherwise, handle delimTokenTree (parenthesized, bracketed, braced)
    if matches!(
      self.current_token().kind,
      TokenKind::OpenParen | TokenKind::OpenBracket | TokenKind::OpenBrace
    ) {
      let delimited = self.parse_delim_token_tree(engine)?;
      tokens.push(delimited);
      return Ok(tokens);
    }

    Ok(tokens)
  }

  pub(crate) fn parse_delim_token_tree(
    &mut self,
    engine: &mut DiagnosticEngine,
  ) -> Result<TokenTree, ()> {
    let open = self.current_token();

    // Handle the different delimiters kinds (parentheses, brackets, braces)
    let delimiter = match open.kind {
      TokenKind::OpenParen => Delimiter::Paren,
      TokenKind::OpenBracket => Delimiter::Bracket,
      TokenKind::OpenBrace => Delimiter::Brace,
      _ => {
        let diag = Diagnostic::new(
          DiagnosticCode::Error(DiagnosticError::UnexpectedToken),
          "Expected a delimiter start".to_string(),
          self.source_file.path.clone(),
        )
        .with_label(
          open.span,
          Some("not a delimiter start".to_string()),
          LabelStyle::Primary,
        );
        engine.add(diag);
        return Err(());
      },
    };

    self.advance(engine); // consume the open delimiter

    let mut tokens = Vec::new();

    while !self.is_eof() {
      let token = self.current_token();

      // Stop when we reach the matching close which we have determined above
      let is_close = matches!(
        (&token.kind, &delimiter),
        (TokenKind::CloseParen, Delimiter::Paren)
          | (TokenKind::CloseBracket, Delimiter::Bracket)
          | (TokenKind::CloseBrace, Delimiter::Brace)
      );

      if is_close {
        self.advance(engine); // consume the closing delimiter
        break;
      }

      // If we find a nested delimiter, recursively parse it
      if matches!(
        token.kind,
        TokenKind::OpenParen | TokenKind::OpenBracket | TokenKind::OpenBrace
      ) {
        let nested = self.parse_delim_token_tree(engine)?;
        tokens.push(nested);
        continue;
      }

      // Otherwise, it’s a plain token (identifier, literal, operator, etc.)
      let lexeme = self.get_token_lexeme(&token);
      tokens.push(TokenTree::Token(lexeme));

      self.advance(engine);
    }

    Ok(TokenTree::Delimited { delimiter, tokens })
  }
}
