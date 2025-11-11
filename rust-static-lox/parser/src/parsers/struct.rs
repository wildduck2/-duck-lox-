use crate::ast::{Expr, FieldInit, Path, PathSegment, PathSegmentKind};
use crate::parser_utils::ExprContext;
use crate::{DiagnosticEngine, Parser};
use diagnostic::code::DiagnosticCode;
use diagnostic::diagnostic::{Diagnostic, LabelStyle};
use diagnostic::types::error::DiagnosticError;
use lexer::token::{Token, TokenKind};

impl Parser {
  //   structExpr       → structExprStruct
  //                    | structExprTuple
  //                    | structExprUnit ;
  //
  //   structExprStruct → pathInExpression "{" (structExprFields | structBase)? "}" ;
  //
  //   structExprFields → structExprField ("," structExprField)* ("," structBase | ","?) ;
  //
  //   structExprField  → outerAttr* (IDENTIFIER | (IDENTIFIER | tupleIndex) ":" expression) ;
  //
  //   structBase       → ".." expression ;
  //
  //   structExprTuple  → pathInExpression "(" (expression ("," expression)* ","?)? ")" ;
  //
  //   structExprUnit   → pathInExpression ;
  //
  //   pathInExpression → "::"? pathExprSegment ("::" pathExprSegment)* ;
  //
  //   pathExprSegment  → pathIdentSegment ("::" genericArgs)? ;
  //
  //   pathIdentSegment → IDENTIFIER | "super" | "self" | "Self" | "crate" | "$crate" ;
  //

  pub(crate) fn parse_struct_expr(
    &mut self,
    token: &mut Token,
    engine: &mut DiagnosticEngine,
  ) -> Result<Expr, ()> {
    let mut token = self.current_token();
    let struct_name = self.get_token_lexeme(&token);
    self.advance(engine); // consume the identifier
    let args = self.parse_generic_args(&mut token, engine)?;
    let mut fields = vec![];

    match self.current_token().kind {
      TokenKind::OpenBrace => {
        self.expect(TokenKind::OpenBrace, engine)?; // consume '{'
        fields = self.parse_struct_expr_fields(engine)?;
        self.expect(TokenKind::CloseBrace, engine)?; // consume '}'
      },
      TokenKind::OpenParen => {
        self.expect(TokenKind::OpenParen, engine)?; // consume '('
        fields = self.parse_struct_expr_fields(engine)?;
        self.expect(TokenKind::CloseParen, engine)?; // consume ')'
      },
      _ => {
        let diagnostic = Diagnostic::new(
          DiagnosticCode::Error(DiagnosticError::UnexpectedToken),
          "Unexpected token".to_string(),
          self.source_file.path.clone(),
        )
        .with_label(
          self.current_token().span,
          Some("Expected a primary expression, found \"{}\"".to_string()),
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

    Ok(Expr::Struct {
      // TODO: fix this type later one
      path: Path {
        leading_colon: false,
        segments: vec![PathSegment {
          kind: PathSegmentKind::Ident(struct_name),
          args, // these are generic args
        }],
      },
      fields,
      base: None,
      span: token.span,
    })
  }

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
        // TODO: parse attributes per field
        attributes: vec![],
        name: lexme,
        value: field_value,
      });
    }

    Ok(fields)
  }
}
