use diagnostic::{
  code::DiagnosticCode,
  diagnostic::{Diagnostic, LabelStyle, Span},
  types::error::DiagnosticError,
  DiagnosticEngine,
};

use crate::{token::TokenKind, Lexer};

impl<'a> Lexer<'a> {
  /// Function that matches the next char to an argument and returns true.
  pub fn lex_tokens(&mut self, c: char, engine: &mut DiagnosticEngine) -> Option<TokenKind> {
    match c {
      '{' => Some(TokenKind::LeftBrace),
      '}' => Some(TokenKind::RightBrace),
      '(' => Some(TokenKind::LeftParen),
      ')' => Some(TokenKind::RightParen),
      '[' => Some(TokenKind::LeftBracket),
      ']' => Some(TokenKind::RightBracket),

      '+' => Some(TokenKind::Plus),
      '-' => Some(TokenKind::Minus),
      '*' => Some(TokenKind::Star),
      '%' => Some(TokenKind::Percent),
      '^' => Some(TokenKind::Caret),
      '/' => self.lex_divide(),
      '=' => self.lex_equal(),
      '!' => self.lex_bang(),
      '<' => self.lex_less(),
      '>' => self.lex_greater(),
      '&' => self.lex_and(engine),
      '|' => self.lex_or(engine),

      ';' => Some(TokenKind::Semicolon),
      ',' => Some(TokenKind::Comma),
      '.' => Some(TokenKind::Dot),
      ':' => Some(TokenKind::Colon),
      '?' => Some(TokenKind::Question),

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

  fn lex_divide(&mut self) -> Option<TokenKind> {
    if self.match_char(self.peek(), '/') {
      return self.lex_line_comment();
    } else if self.match_char(self.peek(), '*') {
      return self.lex_multi_line_comment();
    }

    Some(TokenKind::Slash)
  }

  fn lex_line_comment(&mut self) -> Option<TokenKind> {
    self.advance(1); // consumethe "/"

    while !self.is_eof() {
      self.advance(1); // consume the current char
      if self.match_char(self.peek(), '\n') {
        self.advance(1); // consume the '\n'
        break;
      }
    }
    Some(TokenKind::SingleLineComment)
  }
  fn lex_multi_line_comment(&mut self) -> Option<TokenKind> {
    self.advance(1); // consumethe "*"

    while !self.is_eof() {
      self.advance(1); // consume the current char
      if self.match_char(self.peek(), '*') && self.match_char(self.peek(), '/') {
        self.advance(2); // consume the "*/"
        break;
      }
    }

    Some(TokenKind::MultiLineComment)
  }

  fn lex_bang(&mut self) -> Option<TokenKind> {
    self.advance(1); // consumethe "!"

    if self.match_char(self.peek(), '=') {
      self.advance(1); // consume the '='
      return Some(TokenKind::BangEqual);
    }

    Some(TokenKind::Bang)
  }

  fn lex_greater(&mut self) -> Option<TokenKind> {
    self.advance(1); // consumethe ">"

    if self.match_char(self.peek(), '=') {
      self.advance(1); // consume the '='
      return Some(TokenKind::GreaterEqual);
    }

    Some(TokenKind::GreaterEqual)
  }

  fn lex_less(&mut self) -> Option<TokenKind> {
    self.advance(1); // consumethe "<"

    if self.match_char(self.peek(), '=') {
      self.advance(1); // consume the '='
      return Some(TokenKind::LessEqual);
    }

    Some(TokenKind::Less)
  }

  fn lex_equal(&mut self) -> Option<TokenKind> {
    self.advance(1); // consumethe "="

    if self.match_char(self.peek(), '=') {
      self.advance(1); // consume the '='
      return Some(TokenKind::EqualEqual);
    }

    Some(TokenKind::Equal)
  }

  fn lex_and(&mut self, engine: &mut DiagnosticEngine) -> Option<TokenKind> {
    self.advance(1); // consumethe "&"

    if self.match_char(self.peek(), '&') {
      self.advance(1); // consume the '='
      return Some(TokenKind::And);
    } else {
      self.emit_error_unexpected_character(engine);
      None
    }
  }

  fn lex_or(&mut self, engine: &mut DiagnosticEngine) -> Option<TokenKind> {
    self.advance(1); // consumethe "|"

    if self.match_char(self.peek(), '|') {
      self.advance(1); // consume the '='
      return Some(TokenKind::Or);
    } else {
      self.emit_error_unexpected_character(engine);
      None
    }
  }
}
