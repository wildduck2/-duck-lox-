use diagnostic::{
  code::DiagnosticCode,
  diagnostic::{Diagnostic, LabelStyle},
  types::error::DiagnosticError,
  DiagnosticEngine, Span,
};

use crate::{token::TokenKind, Lexer};

impl Lexer {
  /// Dispatches lexing for the current character, returning the matching token or emitting diagnostics.
  pub fn lex_tokens(&mut self, c: char, engine: &mut DiagnosticEngine) -> Option<TokenKind> {
    match c {
      // '{' => Some(TokenKind::LeftBrace),
      // '}' => Some(TokenKind::RightBrace),
      // '(' => Some(TokenKind::LeftParen),
      // ')' => Some(TokenKind::RightParen),
      // '[' => Some(TokenKind::LeftBracket),
      // ']' => Some(TokenKind::RightBracket),
      //
      // '+' => Some(TokenKind::Plus),
      // '-' => self.parse_minus(),
      // '*' => Some(TokenKind::Star),
      // '%' => Some(TokenKind::Percent),
      // '^' => Some(TokenKind::Caret),
      // ';' => Some(TokenKind::Semicolon),
      // ',' => Some(TokenKind::Comma),
      // '.' => self.lex_dot(),
      // ':' => self.parse_colon(),
      // '?' => Some(TokenKind::Question),
      // '/' => self.lex_divide(),
      // '=' => self.lex_equal(),
      // '!' => self.lex_bang(),
      // '<' => self.lex_less(),
      // '>' => self.lex_greater(),
      // '&' => self.lex_and(engine),
      // '|' => self.lex_or(),
      // '\n' => {
      //   self.line += 1;
      //   self.column = 0;
      //   None
      // },

      // '\r' | '\t' | ' ' => None,
      // '"' | '\'' | '`' => self.lex_string(engine),
      // 'A'..='Z' | 'a'..='z' | '_' => self.lex_keywords(),
      // '0'..='9' => self.lex_number(),
      _ => {
        let diagnostic = Diagnostic::new(
          DiagnosticCode::Error(DiagnosticError::InvalidCharacter),
          format!("unexpected character: {}", self.get_current_lexeme()),
          "demo.lox".to_string(),
        )
        .with_label(
          Span::new(self.current, self.column + 1),
          Some("unexpected character".to_string()),
          LabelStyle::Primary,
        );

        engine.add(diagnostic);
        None
      },
    }
  }
}
