use crate::ast::{Path, PathSegment, PathSegmentKind, Type};
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
        "int" => Ok(Type::I32),
        "float" => Ok(Type::F32),
        "string" => Ok(Type::Str),
        "bool" => Ok(Type::Bool),
        _ => Ok(Type::Path(self.parse_path(lexeme, engine)?)),
      },

      _ => {
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

  pub(crate) fn parse_path(
    &mut self,
    name: String,
    _engine: &mut DiagnosticEngine,
  ) -> Result<Path, ()> {
    Ok(Path {
      leading_colon: false,
      segments: vec![PathSegment {
        kind: PathSegmentKind::Ident(name),
        args: None,
      }],
    })
  }
}
