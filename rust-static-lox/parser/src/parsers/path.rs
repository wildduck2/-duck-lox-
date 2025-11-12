use diagnostic::{
  code::DiagnosticCode,
  diagnostic::{Diagnostic, LabelStyle},
  types::error::DiagnosticError,
};
use lexer::token::TokenKind;

use crate::{ast::path::*, DiagnosticEngine, Parser};

impl Parser {
  /// Function that parses a full path
  /// It takes a boolean flag that indicates if the path has generic args
  /// and returns a `Path` struct
  ///
  /// The boolean flag is for a private usage in the `parse_path_segment` function
  ///
  /// for example:
  /// ```rust
  /// let path = self.parse_path(true, engine)?;
  /// ```
  ///
  /// Most of the time you will use this function to parse a path
  /// but you can also use it to parse a path with generic args
  pub(crate) fn parse_path(
    &mut self,
    with_args: bool,
    engine: &mut DiagnosticEngine,
  ) -> Result<Path, ()> {
    // Handle leading '::' (absolute paths)
    let mut leading_colon = false;
    if matches!(self.current_token().kind, TokenKind::ColonColon) {
      leading_colon = true;
      self.advance(engine); // consume '::'
    }

    // Parse the first path segment and determine if it has a `$crate` segment
    let (first_segment, has_dollar_crate) = self.parse_path_segment(with_args, engine)?;
    let mut segments = vec![first_segment];

    // Parse additional segments separated by '::' and check for `$crate` segments again
    while !self.is_eof()
      && !matches!(
        self.current_token().kind,
        TokenKind::CloseBracket | TokenKind::Eq | TokenKind::OpenParen | TokenKind::CloseParen
      )
    {
      self.expect(TokenKind::ColonColon, engine)?;

      let (segment, is_dollar_crate) = self.parse_path_segment(with_args, engine)?;
      if is_dollar_crate {
        let diagnostic = Diagnostic::new(
          DiagnosticCode::Error(DiagnosticError::UnexpectedToken),
          "Unexpected `$crate` segment in path".to_string(),
          self.source_file.path.clone(),
        )
        .with_label(
          self.current_token().span,
          Some("`$crate` cannot appear in this position".to_string()),
          LabelStyle::Primary,
        )
        .with_help("`$crate` is only valid as the first path segment.".to_string());
        engine.add(diagnostic);
        return Err(());
      }

      segments.push(segment);
    }

    Ok(Path {
      leading_colon: leading_colon || has_dollar_crate,
      segments,
    })
  }

  /// Function that parses a single path segment
  /// It takes a boolean flag that indicates if the path segment has generic args
  /// and returns a tuple of the parsed path segment and a boolean flag that indicates
  /// if the path segment has `$crate` or not
  ///
  /// for example:
  /// ```rust
  /// let (segment, has_dollar_crate) = self.parse_path_segment(true, engine)?;
  /// ```
  ///
  /// Most of the time you will use this function to parse a path segment
  /// but you can also use it to parse a path segment with generic args
  pub(crate) fn parse_path_segment(
    &mut self,
    with_args: bool,
    engine: &mut DiagnosticEngine,
  ) -> Result<(PathSegment, bool), ()> {
    let mut token = self.current_token();
    self.advance(engine); // consume the path segment

    let args = if with_args && matches!(self.current_token().kind, TokenKind::Lt) {
      self.parse_generic_args(&mut token, engine)?
    } else if !with_args
        // This covers the case like `pub(in path::to<T>)` with a generic arg
      && (matches!(self.current_token().kind, TokenKind::Lt)
        // This covers the case like `pub(in path::to::module::<T>)` with a turbofish
        || (matches!(self.current_token().kind, TokenKind::ColonColon)
          && self.peek(1).kind == TokenKind::Lt))
    {
      let mut token = self.current_token();
      self.advance_till_match(engine, TokenKind::Gt);

      token.span.merge(self.current_token().span);
      let lexeme = self.get_token_lexeme(&token);

      let diagnostic = Diagnostic::new(
        DiagnosticCode::Error(DiagnosticError::UnexpectedToken),
        format!("Unexpected generic argument '{}'", lexeme),
        self.source_file.path.clone(),
      )
      .with_label(
        token.span,
        Some(format!("Expected path segment, found \"{}\"", lexeme)),
        LabelStyle::Primary,
      )
      .with_help("Generic arguments are only allowed in path segments.".to_string());

      engine.add(diagnostic);
      return Err(());
    } else {
      None
    };

    match token.kind {
      TokenKind::KwSelfValue => Ok((PathSegment::new(PathSegmentKind::Self_, args), false)),
      TokenKind::KwSuper => Ok((PathSegment::new(PathSegmentKind::Super, args), false)),
      TokenKind::KwCrate => Ok((PathSegment::new(PathSegmentKind::Crate, args), false)),
      TokenKind::Ident => Ok((
        PathSegment::new(PathSegmentKind::Ident(self.get_token_lexeme(&token)), args),
        false,
      )),
      TokenKind::Dollar if self.peek(0).kind == TokenKind::KwCrate => {
        self.advance(engine);
        Ok((PathSegment::new(PathSegmentKind::DollarCrate, args), true))
      },
      _ => {
        // This cover the caes like `pub(path:to::123)`
        //                                        ^^^ Error here
        // where the path segment is not an identifier or any known keyword token
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
}
