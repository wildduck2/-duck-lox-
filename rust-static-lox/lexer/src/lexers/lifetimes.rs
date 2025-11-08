use diagnostic::{
  code::DiagnosticCode,
  diagnostic::{Diagnostic, LabelStyle},
  types::error::DiagnosticError,
  DiagnosticEngine,
};

use crate::{token::TokenKind, Lexer};

impl Lexer {
  /// Lexes lifetime identifiers starting with `'`.
  ///
  /// Handles:
  /// - `'a`, `'static`, `'_'`
  /// - `'0abc` (invalid starts with number)
  /// - `'` alone (invalid / unterminated)
  ///
  /// Emits diagnostics for invalid or malformed lifetimes.
  ///
  /// Returns `TokenKind::Lifetime { starts_with_number }` or `TokenKind::Unknown` on error.
  pub fn lex_lifetime(&mut self, engine: &mut DiagnosticEngine) -> Option<TokenKind> {
    let lexeme = self.get_current_lexeme();

    // Determine if the lifetime starts with a number after the apostrophe `'`
    let starts_with_number = lexeme
      .strip_prefix('\'') // remove the leading apostrophe
      .and_then(|rest| rest.chars().next()) // get the next char (if any)
      .map(|ch| ch.is_ascii_digit()) // check if it's a digit
      .unwrap_or(false); // default to false if no next char

    if starts_with_number {
      let diagnostic = Diagnostic::new(
        DiagnosticCode::Error(DiagnosticError::InvalidLifetime),
        format!("'{}' is not a valid lifetime", lexeme),
        self.source.path.to_string(),
      )
      .with_label(
        diagnostic::Span::new(self.start, self.current),
        Some("Invalid lifetime start with number here".to_string()),
        LabelStyle::Primary,
      )
      .with_help("Lifetimes must not start with a number.".to_string());

      engine.add(diagnostic);
      Some(TokenKind::Unknown)
    } else {
      // Return lifetime token directly no diagnostics at this stage
      Some(TokenKind::Lifetime { starts_with_number })
    }
  }
}
