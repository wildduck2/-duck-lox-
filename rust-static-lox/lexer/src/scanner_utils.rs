use diagnostic::{
  code::DiagnosticCode,
  diagnostic::{Diagnostic, LabelStyle, Span},
  types::error::DiagnosticError,
  DiagnosticEngine,
};

use crate::{token::TokenKind, Lexer};

impl<'a> Lexer<'a> {
  /// Dispatches lexing for the current character, returning the matching token or emitting diagnostics.
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
      '\n' => {
        self.line += 1;
        self.column = 0;
        None
      },

      '?' => Some(TokenKind::Question),
      '\r' | '\t' | ' ' => {
        self.current += 1;
        self.column += 1;
        None
      },

      '"' | '\'' | '`' => self.lex_string(engine),
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
        None
      },
    }
  }

  /// Lexes `/`, distinguishing between division tokens and comment delimiters.
  fn lex_divide(&mut self) -> Option<TokenKind> {
    if self.match_char(self.peek(), '/') {
      return self.lex_line_comment();
    } else if self.match_char(self.peek(), '*') {
      return self.lex_multi_line_comment();
    }

    Some(TokenKind::Slash)
  }

  /// Consumes a single-line `//` comment and returns its token.
  fn lex_line_comment(&mut self) -> Option<TokenKind> {
    self.advance(); // consumethe "/"

    while !self.is_eof() {
      self.advance(); // consume the current char
      if self.match_char(self.peek(), '\n') {
        self.advance(); // consume the '\n'
        break;
      }
    }
    Some(TokenKind::SingleLineComment)
  }

  /// Consumes a block `/* ... */` comment and returns its token.
  fn lex_multi_line_comment(&mut self) -> Option<TokenKind> {
    self.advance(); // consumethe "*"

    while !self.is_eof() {
      self.advance(); // consume the current char
      if self.match_char(self.peek(), '*') && self.match_char(self.peek(), '/') {
        self.advance(); // consume the "*"
        self.advance(); // consume the "/"
        break;
      }
    }

    Some(TokenKind::MultiLineComment)
  }

  /// Lexes `!` and `!=` tokens.
  fn lex_bang(&mut self) -> Option<TokenKind> {
    self.advance(); // consumethe "!"

    if self.match_char(self.peek(), '=') {
      self.advance(); // consume the '='
      return Some(TokenKind::BangEqual);
    }

    Some(TokenKind::Bang)
  }

  /// Lexes greater-than comparators, upgrading to `>=` when an equals sign follows.
  fn lex_greater(&mut self) -> Option<TokenKind> {
    self.advance(); // consumethe ">"

    if self.match_char(self.peek(), '=') {
      self.advance(); // consume the '='
      return Some(TokenKind::GreaterEqual);
    }

    Some(TokenKind::GreaterEqual)
  }

  /// Lexes less-than comparators, upgrading to `<=` when an equals sign follows.
  fn lex_less(&mut self) -> Option<TokenKind> {
    self.advance(); // consumethe "<"

    if self.match_char(self.peek(), '=') {
      self.advance(); // consume the '='
      return Some(TokenKind::LessEqual);
    }

    Some(TokenKind::Less)
  }

  /// Lexes `=` and `==` tokens.
  fn lex_equal(&mut self) -> Option<TokenKind> {
    self.advance(); // consumethe "="

    if self.match_char(self.peek(), '=') {
      self.advance(); // consume the '='
      return Some(TokenKind::EqualEqual);
    }

    Some(TokenKind::Equal)
  }

  /// Lexes `&&`, emitting a diagnostic when a second `&` is missing.
  fn lex_and(&mut self, engine: &mut DiagnosticEngine<'a>) -> Option<TokenKind> {
    self.advance(); // consumethe "&"

    if self.match_char(self.peek(), '&') {
      self.advance(); // consume the '='
      return Some(TokenKind::And);
    } else {
      self.emit_error_unexpected_character(engine);
      None
    }
  }

  /// Lexes `||`, emitting a diagnostic when a second `|` is missing.
  fn lex_or(&mut self, engine: &mut DiagnosticEngine<'a>) -> Option<TokenKind> {
    self.advance(); // consumethe "|"

    if self.match_char(self.peek(), '|') {
      self.advance(); // consume the '='
      return Some(TokenKind::Or);
    } else {
      self.emit_error_unexpected_character(engine);
      None
    }
  }

  /// Consumes an identifier or keyword, returning the proper token kind.
  fn lex_keywords(&mut self) -> Option<TokenKind> {
    self.advance(); // consume the current char

    while let Some(char) = self.peek() {
      if !char.is_ascii_alphabetic() || char == '_' {
        break;
      }

      self.advance();
    }

    match self.get_current_lexeme() {
      "let" => Some(TokenKind::Let),
      "true" => Some(TokenKind::True),
      "false" => Some(TokenKind::False),
      "nil" => Some(TokenKind::Nil),
      _ => Some(TokenKind::Identifier),
    }
  }

  /// Parses an integer or floating-point literal.
  fn lex_number(&mut self) -> Option<TokenKind> {
    self.advance(); // consume the current number

    while let Some(char) = self.peek() {
      if !char.is_ascii_digit() {
        break;
      }

      self.advance();
    }

    if self.get_current_lexeme().contains(".") {
      return Some(TokenKind::FloatLiteral);
    }

    Some(TokenKind::IntegerLiteral)
  }

  /// Parses a quoted string literal and reports unterminated strings.
  fn lex_string(&mut self, engine: &mut DiagnosticEngine<'a>) -> Option<TokenKind> {
    // The opening quote is already in the lexeme at position self.start
    let first_char = self.source.chars().nth(self.start).unwrap();

    while let Some(char) = self.peek() {
      if self.is_eof() {
        let line_content = self.get_line(self.line);

        let diagnostic = Diagnostic::new(
          DiagnosticCode::Error(DiagnosticError::UnterminatedString),
          "unterminated string".to_string(),
          "demo.lox",
        )
        .with_context_line(self.line, line_content)
        .with_label(
          Span::new(self.line, 1, 1),
          Some("unterminated string"),
          LabelStyle::Primary,
        );

        engine.add(diagnostic);

        break;
      }

      if char == '\n' && first_char != '`' {
        let line_content = self.get_line(self.line);

        if self.peek() == Some('\n') {
          let diagnostic = Diagnostic::new(
            DiagnosticCode::Error(DiagnosticError::UnterminatedString),
            "unterminated string".to_string(),
            "demo.lox",
          )
          .with_context_line(self.line, line_content)
          .with_label(
            Span::new(self.line, 1, line_content.len() + 1),
            Some("unterminated string"),
            LabelStyle::Primary,
          );
          engine.add(diagnostic);
          break;
        }

        let diagnostic = Diagnostic::new(
          DiagnosticCode::Error(DiagnosticError::UnterminatedString),
          "unterminated string".to_string(),
          "demo.lox",
        )
        .with_context_line(self.line, line_content)
        .with_label(
          Span::new(self.line, self.start + 1, self.current + 1),
          Some("unterminated string"),
          LabelStyle::Primary,
        );

        engine.add(diagnostic);

        break;
      }

      if (first_char == '\'' && char == '\'')
        || (first_char == '"' && char == '"')
        || (first_char == '`' && char == '`')
      {
        self.advance(); // consume the closing quote
        break;
      }
      self.advance();
    }

    Some(TokenKind::StringLiteral)
  }
}
