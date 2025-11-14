use crate::Parser;
use crate::{
  ast::{r#struct::*, *},
  parser_utils::ExprContext,
};
use diagnostic::{
  code::DiagnosticCode,
  diagnostic::{Diagnostic, LabelStyle},
  types::error::DiagnosticError,
  DiagnosticEngine,
};
use lexer::token::{Token, TokenKind};

impl Parser {
  /// Parses `struct` declarations including unit, tuple, and record forms.
  pub(crate) fn parse_struct_decl(
    &mut self,
    attributes: Vec<Attribute>,
    visibility: Visibility,
    engine: &mut DiagnosticEngine,
  ) -> Result<Item, ()> {
    let mut token = self.current_token();
    self.advance(engine); // consume the struct keyword

    let name = self.get_token_lexeme(&self.current_token());
    self.advance(engine); // consume the identifier

    // Handle the case where the struct is empty AKA (Unit Struct)
    if matches!(self.current_token().kind, TokenKind::Semi) {
      // Note: that only unit | tuple structs can have a semicolon
      self.advance(engine); // consume the semicolon

      token.span.merge(self.current_token().span);
      return Ok(Item::Struct(StructDecl {
        attributes,
        visibility,
        name,
        kind: StructKind::Unit,
        generics: None,     // Unit Structs don't have generics
        where_clause: None, // Unit Structs don't have where clauses
        span: token.span,
      }));
    };

    let generics = self.parse_generic_params(&mut token, engine)?;
    let where_clause = self.parse_where_clause(engine)?;

    let kind = if matches!(self.current_token().kind, TokenKind::OpenBrace) {
      // Handles the case where we have a struct body like `struct User { ... }`
      let fields = self.parse_struct_dec_fields(engine)?;
      StructKind::Named { fields }
    } else if matches!(self.current_token().kind, TokenKind::OpenParen) {
      // Handles the case where we have a tuple struct body like `struct User(String, u8)`
      let fields = self.parse_struct_tuple_fields(engine)?;

      self.expect(TokenKind::Semi, engine)?; // consume ';'
      StructKind::Tuple(fields)
    } else {
      // This handles the case where we have a syntax error while declaring a struct body
      // like `struct User from ...`
      //                   ^^^^ Error here this should be a `(` or `{` hence we only
      //                        accept tow type of struct bodies
      let token = self.current_token();
      let lexeme = self.get_token_lexeme(&token);

      let diagnostic = Diagnostic::new(
        DiagnosticCode::Error(DiagnosticError::UnexpectedToken),
        format!("Unexpected token `{}` in this context", lexeme),
        self.source_file.path.clone(),
      )
      .with_label(
        token.span,
        Some(format!(
          "Expected a valid primary expression (like an identifier, literal, or grouped expression), but found `{}`",
          lexeme
        )),
        LabelStyle::Primary,
      )
      .with_help(format!(
        "Check for missing delimiters or misplaced symbols. For example:\n  - `foo(bar)` instead of `foo bar`\n  - `{}` is not valid here.",
        lexeme
      ));

      engine.add(diagnostic);
      return Err(());
    };

    token.span.merge(self.current_token().span);

    let hi = Item::Struct(StructDecl {
      attributes,
      visibility,
      name,
      generics,
      kind,
      where_clause,
      span: token.span,
    });
    println!("debug struct: {:?}", hi);

    Ok(hi)
  }

  /// Function that parses a list of struct field declarations
  /// It returns a vector of `FieldDecl` structs that represents a list of struct fields
  ///
  /// for example:
  /// ```rust
  /// let fields = self.parse_struct_dec_fields(engine)?;
  /// ```
  pub(crate) fn parse_struct_dec_fields(
    &mut self,
    engine: &mut DiagnosticEngine,
  ) -> Result<Vec<FieldDecl>, ()> {
    self.expect(TokenKind::OpenBrace, engine)?; // consume '{'

    let mut fields = vec![];

    while !self.is_eof() && !matches!(self.current_token().kind, TokenKind::CloseBrace) {
      fields.push(self.parse_struct_dec_field(engine)?);
    }

    self.expect(TokenKind::CloseBrace, engine)?; // consume '}'
    Ok(fields)
  }

  /// Function that parses a struct field declaration
  /// It returns a `FieldDecl` struct that represents a struct field
  ///
  /// for example:
  /// ```rust
  /// let field = self.parse_struct_dec_field(engine)?;
  /// ```
  fn parse_struct_dec_field(&mut self, engine: &mut DiagnosticEngine) -> Result<FieldDecl, ()> {
    let mut token = self.current_token();

    let attributes = if matches!(self.current_token().kind, TokenKind::Pound) {
      self.parse_attributes(engine)?
    } else {
      vec![]
    };

    let visibility = self.parse_visibility(engine)?;
    let name = self.parse_name_identifier(engine)?;

    self.expect(TokenKind::Colon, engine)?; // consume ':'
    let ty = self.parse_type(engine)?;

    if self.current_token().kind == TokenKind::Comma {
      self.advance(engine); // consume ','
    } else if !matches!(self.current_token().kind, TokenKind::CloseBrace) {
      // This cover the case where there is not a comma after the last field
      // like `struct User { name: String  age: u8 }`
      //                                 ^ Error here expected a comma
      let lexeme = self.get_token_lexeme(&self.current_token());
      let diagnostic = Diagnostic::new(
        DiagnosticCode::Error(DiagnosticError::UnexpectedToken),
        "Unexpected token between struct fields".to_string(),
        self.source_file.path.clone(),
      )
      .with_label(
        self.current_token().span,
        Some(format!(
          "Expected a comma `,` or closing brace `}}`, found `{}`",
          lexeme
        )),
        LabelStyle::Primary,
      )
      .with_help(
        "Struct fields must be separated by commas. Example: `struct User { name: String, age: u8 }`"
          .to_string(),
      );
      engine.add(diagnostic);
      return Err(());
    }

    token.span.merge(self.current_token().span);

    Ok(FieldDecl {
      attributes,
      name,
      ty,
      visibility,
      span: token.span,
    })
  }

  /// Function that parses a list of struct field declarations
  /// It returns a vector of `TupleField` structs that represents a list of struct fields
  ///
  /// for example:
  /// ```rust
  /// let fields = self.parse_struct_tuple_fields(engine)?;
  /// ```
  pub(crate) fn parse_struct_tuple_fields(
    &mut self,
    engine: &mut DiagnosticEngine,
  ) -> Result<Vec<TupleField>, ()> {
    self.expect(TokenKind::OpenParen, engine)?; // consume '('

    let mut fields = vec![];

    while !self.is_eof() && !matches!(self.current_token().kind, TokenKind::CloseParen) {
      fields.push(self.parse_struct_tuple_field(engine)?);
    }
    self.expect(TokenKind::CloseParen, engine)?; // consume ')'

    Ok(fields)
  }

  /// Function that parses a struct field declaration
  /// It returns a `TupleField` struct that represents a struct field
  ///
  /// for example:
  /// ```rust
  /// let field = self.parse_struct_tuple_field(engine)?;
  /// ```
  pub(crate) fn parse_struct_tuple_field(
    &mut self,
    engine: &mut DiagnosticEngine,
  ) -> Result<TupleField, ()> {
    let mut token = self.current_token();
    let attributes = self.parse_attributes(engine)?;
    let visibility = self.parse_visibility(engine)?;

    let ty = self.parse_type(engine)?;
    token.span.merge(self.current_token().span);

    if self.current_token().kind == TokenKind::Comma {
      self.advance(engine); // consume ','
    } else if !matches!(self.current_token().kind, TokenKind::CloseParen) {
      // This cover the case where there is not a comma after the last field
      // like `struct User(String  u8)`
      //                         ^ Error here expected a comma
      let lexeme = self.get_token_lexeme(&self.current_token());
      let diagnostic = Diagnostic::new(
        DiagnosticCode::Error(DiagnosticError::UnexpectedToken),
        "Unexpected token between struct fields".to_string(),
        self.source_file.path.clone(),
      )
      .with_label(
        token.span,
        Some(format!(
          "Expected a comma `,` or closing paren `)`, found `{}`",
          lexeme
        )),
        LabelStyle::Primary,
      )
      .with_help(
        "Struct fields must be separated by commas. Example: `struct User(String, u8)`".to_string(),
      );
      engine.add(diagnostic);
      return Err(());
    }

    Ok(TupleField {
      attributes,
      visibility,
      ty,
      span: self.current_token().span,
    })
  }

  /// Function that parses a name identifier
  /// It returns a string that represents the name identifier
  ///
  /// for example:
  /// ```rust
  /// let name = self.parse_name_identifier(engine)?;
  /// ```
  ///
  /// You will use this to get the name of a field, a struct, or a function
  pub(crate) fn parse_name_identifier(
    &mut self,
    engine: &mut DiagnosticEngine,
  ) -> Result<String, ()> {
    if matches!(self.current_token().kind, TokenKind::Ident) {
      let name = self.get_token_lexeme(&self.current_token());
      self.advance(engine); // consume the identifier
      return Ok(name);
    }

    let lexeme = self.get_token_lexeme(&self.current_token());
    let diagnostic = Diagnostic::new(
      DiagnosticCode::Error(DiagnosticError::UnexpectedToken),
      "Unexpected name identifier".to_string(),
      self.source_file.path.clone(),
    )
    .with_label(
      self.current_token().span,
      Some(format!(
        "Expected a primary expression, found \"{}\"",
        lexeme
      )),
      LabelStyle::Primary,
    )
    .with_help("Expected a valid name identifier".to_string());

    engine.add(diagnostic);

    Err(())
  }

  // DO NO CHANGE THIS YET
  //
  //
  //
  //
  //
  //
  //
  //
  //
  ////  TODO: check this when we handle primary structs
  /// Parses struct literal expressions (record / tuple forms).
  pub(crate) fn parse_struct_expr(
    &mut self,
    token: &mut Token,
    engine: &mut DiagnosticEngine,
  ) -> Result<Expr, ()> {
    println!("debug struct expr: {:?}", self.current_token().kind);
    Err(())
    // let struct_name = self.get_token_lexeme(token);
    // self.advance(engine); // consume the identifier
    // let args = self.parse_generic_args(engine)?;
    // let mut fields = vec![];
    //
    // match self.current_token().kind {
    //   TokenKind::OpenBrace => {
    //     self.expect(TokenKind::OpenBrace, engine)?; // consume '{'
    //     fields = self.parse_struct_expr_fields(engine)?;
    //     self.expect(TokenKind::CloseBrace, engine)?; // consume '}'
    //   },
    //   TokenKind::OpenParen => {
    //     self.expect(TokenKind::OpenParen, engine)?; // consume '('
    //     fields = self.parse_struct_expr_fields(engine)?;
    //     self.expect(TokenKind::CloseParen, engine)?; // consume ')'
    //   },
    //   _ => {
    //     let diagnostic = Diagnostic::new(
    //       DiagnosticCode::Error(DiagnosticError::UnexpectedToken),
    //       "Unexpected token".to_string(),
    //       self.source_file.path.clone(),
    //     )
    //     .with_label(
    //       self.current_token().span,
    //       Some("Expected a primary expression, found \"{}\"".to_string()),
    //       LabelStyle::Primary,
    //     )
    //     .with_help(Parser::get_token_help(
    //       &self.current_token().kind,
    //       &self.current_token(),
    //     ));
    //
    //     engine.add(diagnostic);
    //
    //     return Err(());
    //   },
    // };
    //
    // Ok(Expr::Struct {
    //   path: Path {
    //     leading_colon: false, // TODO: add this later
    //     // TODO: make sure to match multiple segments
    //     segments: vec![PathSegment {
    //       kind: PathSegmentKind::Ident(struct_name),
    //       args, // these are generic args
    //     }],
    //   },
    //   fields,
    //   base: None, // TODO: add this later
    //   span: token.span,
    // })
  }

  /// Parses the field list used in a struct literal.
  pub(crate) fn parse_struct_expr_fields(
    &mut self,
    engine: &mut DiagnosticEngine,
  ) -> Result<Vec<FieldInit>, ()> {
    let mut fields = Vec::<FieldInit>::new();

    while !self.is_eof()
      && !matches!(
        self.current_token().kind,
        TokenKind::CloseBrace | TokenKind::CloseParen
      )
    {
      // TODO: remove this to the whole struct dcalaration
      let attributes = if matches!(self.current_token().kind, TokenKind::Pound) {
        self.parse_attributes(engine)?
      } else {
        vec![]
      };

      let field_name = self.current_token();
      let lexme = self.get_token_lexeme(&field_name);
      self.advance(engine);

      let field_value = if self.current_token().kind == TokenKind::Colon {
        self.advance(engine); // consume ':'
        Some(self.parse_expression(ExprContext::Default, engine)?)
      } else {
        None
      };

      if self.current_token().kind == TokenKind::Comma {
        self.advance(engine); // consume ','
      }

      fields.push(FieldInit {
        attributes: vec![],
        name: lexme,
        value: field_value,
      });
    }

    Ok(fields)
  }
}
