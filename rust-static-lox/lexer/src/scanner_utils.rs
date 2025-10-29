use diagnostic::{
  code::DiagnosticCode,
  diagnostic::{Diagnostic, LabelStyle, Span},
  types::error::DiagnosticError,
  DiagnosticEngine,
};

use crate::{token::TokenKind, Lexer};

impl<'a> Lexer<'a> {
  fn lex_string(&mut self) -> Option<TokenKind> {
    let current = self.advance(1); // consume the current "\""

    while let Some(char) = self.peek() {
      if char == '"' && current == '"' {
        self.advance(1); // consume the current "\""
        break;
      }
      self.advance(1);
    }

    Some(TokenKind::StringLiteral)
  }

  /// Function that matches the next char to an argument and returns true.
  pub fn lex_tokens(&mut self, c: char, engine: &mut DiagnosticEngine<'a>) -> Option<TokenKind> {
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
      ';' => Some(TokenKind::Semicolon),
      ',' => Some(TokenKind::Comma),
      '.' => Some(TokenKind::Dot),
      ':' => Some(TokenKind::Colon),
      '/' => self.lex_divide(),
      '=' => self.lex_equal(),
      '!' => self.lex_bang(),
      '<' => self.lex_less(),
      '>' => self.lex_greater(),
      '&' => self.lex_and(engine),
      '|' => self.lex_or(engine),
      '"' | '\'' | '`' => self.lex_string(),

      '?' => Some(TokenKind::Question),
      '\r' | '\t' | ' ' => {
        self.column += 1;
        None
      },
      '\n' => {
        self.line += 1;
        self.column = 1;
        None
      },
      'A'..='Z' | 'a'..='z' | '_' => self.lex_keywords(),
      '0'..='9' => self.lex_number(),

      _ => {
        let current_line = self.get_line(self.line);

        let diagnostic = Diagnostic::new(
          DiagnosticCode::Error(DiagnosticError::InvalidCharacter),
          format!("unexpected character: {}", self.get_current_lexeme()),
          "demo.lox",
        )
        .with_context_line(self.line, current_line)
        .with_label(
          Span::new(self.line, self.current, self.column + 1),
          Some("unexpected character"),
          LabelStyle::Primary,
        );

        engine.add(diagnostic);
        None // Don't emit a token, just error
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

  fn lex_and(&mut self, engine: &mut DiagnosticEngine<'a>) -> Option<TokenKind> {
    self.advance(1); // consumethe "&"

    if self.match_char(self.peek(), '&') {
      self.advance(1); // consume the '='
      return Some(TokenKind::And);
    } else {
      self.emit_error_unexpected_character(engine);
      None
    }
  }

  fn lex_or(&mut self, engine: &mut DiagnosticEngine<'a>) -> Option<TokenKind> {
    self.advance(1); // consumethe "|"

    if self.match_char(self.peek(), '|') {
      self.advance(1); // consume the '='
      return Some(TokenKind::Or);
    } else {
      self.emit_error_unexpected_character(engine);
      None
    }
  }

  fn lex_keywords(&mut self) -> Option<TokenKind> {
    self.advance(1); // consume the current char

    while let Some(char) = self.peek() {
      if !char.is_ascii_alphabetic() || char == '_' {
        break;
      }

      self.advance(1);
    }

    match self.get_current_lexeme() {
      "let" => Some(TokenKind::Let),
      "true" => Some(TokenKind::True),
      "false" => Some(TokenKind::False),
      "nil" => Some(TokenKind::Nil),
      _ => Some(TokenKind::Identifier),
    }
  }

  fn lex_number(&mut self) -> Option<TokenKind> {
    self.advance(1); // consume the current number

    while let Some(char) = self.peek() {
      if !char.is_ascii_digit() {
        break;
      }

      self.advance(1);
    }

    if self.get_current_lexeme().contains(".") {
      return Some(TokenKind::FloatLiteral);
    }

    Some(TokenKind::IntegerLiteral)
  }
}
