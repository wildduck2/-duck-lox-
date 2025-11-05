use diagnostic::{
  code::DiagnosticCode,
  diagnostic::{Diagnostic, LabelStyle},
  types::error::DiagnosticError,
  DiagnosticEngine, Span,
};

use crate::{
  token::{DocStyle, TokenKind},
  Lexer,
};

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
      '/' => self.lex_divide(),
      // '=' => self.lex_equal(),
      // '!' => self.lex_bang(),
      // '<' => self.lex_less(),
      // '>' => self.lex_greater(),
      // '&' => self.lex_and(engine),
      // '|' => self.lex_or(),

      // handle whitespace
      '\n' => {
        self.line += 1;
        self.column = 0;
        Some(TokenKind::Whitespace)
      },
      '\r' | '\t' | ' ' => Some(TokenKind::Whitespace),

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

  fn lex_divide(&mut self) -> Option<TokenKind> {
    if self.match_char(self.peek(), '/') {
      return self.lex_line_comment();
    } else if self.match_char(self.peek(), '*') {
      return self.lex_multi_line_comment();
    }

    Some(TokenKind::Slash)
  }

  // Consumes a single-line `//` comment and returns its token.
  fn lex_line_comment(&mut self) -> Option<TokenKind> {
    self.advance(); // consume the '/'

    let doc_style = if self.match_char(self.peek(), '/') {
      self.advance(); // consume the '/'
      Some(DocStyle::Inner)
    } else if self.match_char(self.peek(), '!') {
      self.advance(); // consume the '!'
      Some(DocStyle::Outer)
    } else {
      None
    };

    while !self.is_eof() {
      self.advance(); // consume the current char
      if self.match_char(self.peek(), '\n') {
        break;
      }
    }
    Some(TokenKind::LineComment { doc_style })
  }

  /// Consumes a block `/* ... */` comment and returns its token.
  fn lex_multi_line_comment(&mut self) -> Option<TokenKind> {
    self.advance(); // consume '/'
    self.advance(); // consume '*'

    // Detect Rust-style doc comments: /*! ... */ (Outer) or /** ... */ (Inner)
    let doc_style = match self.peek() {
      Some('!') => {
        self.advance(); // consume '!'
        Some(DocStyle::Outer)
      },
      Some('*') => {
        self.advance(); // consume second '*'
        Some(DocStyle::Inner)
      },
      _ => None,
    };

    let mut terminated = false;
    let mut depth = 1; // track nested comment depth

    while !self.is_eof() {
      let current = self.peek();
      let next = self.peek_next();

      // Handle newlines
      if current == Some('\n') {
        self.line += 1;
      }

      // Detect nested comment start "/*"
      if current == Some('/') && next == Some('*') {
        self.advance(); // consume '/'
        self.advance(); // consume '*'
        depth += 1;
        continue;
      }

      // Detect comment end "*/"
      if current == Some('*') && next == Some('/') {
        self.advance(); // consume '*'
        self.advance(); // consume '/'
        depth -= 1;

        if depth == 0 {
          terminated = true;
          break;
        }
        continue;
      }

      self.advance(); // consume any other char
    }

    Some(TokenKind::BlockComment {
      doc_style,
      terminated,
    })
  }
}
