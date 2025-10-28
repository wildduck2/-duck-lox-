use diagnostic::{
  code::DiagnosticCode,
  diagnostic::{Diagnostic, LabelStyle, Span},
  types::error::DiagnosticError,
  DiagnosticEngine,
};

use crate::{token::TokenKind, Lexer};

impl<'a> Lexer<'a> {
  /// Function that matches the next char to an argument and returns true.
  pub fn match_char(&mut self, c: char, engine: &mut DiagnosticEngine) -> Option<TokenKind> {
    match c {
      '{' => Some(TokenKind::LeftBrace),
      _ => {
        let diagnostic = Diagnostic::new(
          DiagnosticCode::Error(DiagnosticError::InvalidCharacter),
          format!("unexpected character: {:?}", self.get_current_lexeme()),
          "demo.lox",
        )
        .with_label(
          Span::new(self.start, self.current),
          Some("unexpected character"),
          LabelStyle::Primary,
        );

        self.emit(TokenKind::Identifier);
        engine.add(diagnostic);

        None
      },
    }
  }
}
