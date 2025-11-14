use crate::{ast::*, Parser};

use diagnostic::{
  code::DiagnosticCode,
  diagnostic::{Diagnostic, LabelStyle},
  types::error::DiagnosticError,
  DiagnosticEngine,
};
use lexer::token::TokenKind;

impl Parser {
  /// Parses zero or more attributes in sequence, returning them in declaration order.
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

  /// Parses a single `#[...]` or `#![...]` attribute into an [`Attribute`].
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
        let offending = self.current_token();
        let lexeme = self.get_token_lexeme(&offending);
        let diagnostic = Diagnostic::new(
          DiagnosticCode::Error(DiagnosticError::UnexpectedToken),
          "Unexpected token".to_string(),
          self.source_file.path.clone(),
        )
        .with_label(
          offending.span,
          Some(format!(
            "expected `#` or `#!` to start an attribute, found `{lexeme}`"
          )),
          LabelStyle::Primary,
        )
        .with_help("Attributes must be prefixed with `#` (outer) or `#!` (inner).".to_string())
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

  /// Parses attribute metadata like `derive(Debug)` or `path = \"foo\"`.
  pub(crate) fn parse_attribute_input(
    &mut self,
    engine: &mut DiagnosticEngine,
  ) -> Result<AttrKind, ()> {
    // the trure value here to true means we expect the path to have generic args
    let path = self.parse_path(true, engine)?;
    let attr_input_tail = self.parse_attribute_input_tail(engine)?;

    Ok(AttrKind::Normal {
      path,
      tokens: attr_input_tail,
    })
  }

  /// Parses the attribute tail (`= value` or nested token trees inside delimiters).
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

  /// Recursively parses a delimited token-tree, handling nested delimiters.
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
