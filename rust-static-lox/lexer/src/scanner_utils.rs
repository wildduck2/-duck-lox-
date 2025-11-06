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
      // punctuation and delimiters
      ';' => self.lex_semicolon(),
      ',' => self.lex_comma(),
      '.' => self.lex_dot(),
      '(' => self.lex_open_paren(),
      ')' => self.lex_close_paren(),
      '{' => self.lex_open_brace(),
      '}' => self.lex_close_brace(),
      '[' => self.lex_open_bracket(),
      ']' => self.lex_close_bracket(),
      '@' => self.lex_at(),
      '#' => self.lex_pound(engine),
      '~' => self.lex_tilde(),
      '?' => self.lex_question(),
      ':' => self.lex_colon(),
      '$' => self.lex_dollar(),

      // Assignment & Comparison
      '=' => self.lex_equal(),
      '!' => self.lex_bang(),
      '<' => self.lex_less(),
      '>' => self.lex_greater(),

      // Arithmetic
      '+' => self.lex_plus(),
      '-' => self.lex_minus(),
      '*' => self.lex_star(),
      '/' => self.lex_slash(),

      // Bitwise & Logical
      '&' => self.lex_and(),
      '|' => self.lex_or(),
      '^' => self.lex_caret(),

      // handle whitespace
      '\n' => {
        self.line += 1;
        self.column = 0;
        Some(TokenKind::Whitespace)
      },
      '\r' | '\t' | ' ' => self.lex_whitespace(),

      // String and character literals
      '"' => self.lex_string(engine),  // Regular string
      '\'' => self.lex_string(engine), // Character literal

      // Prefixed literals (need to check next char)
      'b' => self.lex_string(engine), // b"...", b'...', br"..."
      'c' => self.lex_string(engine), // c"...", cr"..."
      'r' => self.lex_string(engine), // r"...", r#"..."#

      // Numbers
      '0'..='9' => self.lex_number(),
      // Keywords
      'A'..='Z' | 'a'..='z' | '_' => self.lex_keywords(),

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

  fn lex_whitespace(&mut self) -> Option<TokenKind> {
    while let Some(c) = self.peek() {
      if c.is_whitespace() {
        self.advance();
      } else {
        break;
      }
    }
    Some(TokenKind::Whitespace)
  }
}
