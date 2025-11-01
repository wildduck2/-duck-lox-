use diagnostic::{
  code::DiagnosticCode,
  diagnostic::{Diagnostic, LabelStyle, Span},
  types::error::DiagnosticError,
  DiagnosticEngine,
};

use crate::{token::TokenKind, Lexer};

impl Lexer {
  /// Dispatches lexing for the current character, returning the matching token or emitting diagnostics.
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
      ';' => Some(TokenKind::Semicolon),
      ',' => Some(TokenKind::Comma),
      '.' => Some(TokenKind::Dot),
      ':' => Some(TokenKind::Colon),
      '?' => Some(TokenKind::Question),
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

      '\r' | '\t' | ' ' => None,
      '"' | '\'' | '`' => self.lex_string(engine),
      'A'..='Z' | 'a'..='z' | '_' => self.lex_keywords(),
      '0'..='9' => self.lex_number(),

      _ => {
        let diagnostic = Diagnostic::new(
          DiagnosticCode::Error(DiagnosticError::InvalidCharacter),
          format!("unexpected character: {}", self.get_current_lexeme()),
          "demo.lox".to_string(),
        )
        .with_label(
          Span::new(self.line, self.current, self.column + 1),
          Some("unexpected character".to_string()),
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
    if self.match_char(self.peek(), '=') {
      self.advance(); // consume the '='
      return Some(TokenKind::BangEqual);
    }

    Some(TokenKind::Bang)
  }

  /// Lexes greater-than comparators, upgrading to `>=` when an equals sign follows.
  fn lex_greater(&mut self) -> Option<TokenKind> {
    if self.match_char(self.peek(), '=') {
      self.advance(); // consume the '='
      return Some(TokenKind::GreaterEqual);
    }

    Some(TokenKind::GreaterEqual)
  }

  /// Lexes less-than comparators, upgrading to `<=` when an equals sign follows.
  fn lex_less(&mut self) -> Option<TokenKind> {
    if self.match_char(self.peek(), '=') {
      self.advance(); // consume the '='
      return Some(TokenKind::LessEqual);
    }

    Some(TokenKind::Less)
  }

  /// Lexes `=` and `==` tokens.
  fn lex_equal(&mut self) -> Option<TokenKind> {
    if self.match_char(self.peek(), '=') {
      self.advance(); // consume the '='
      return Some(TokenKind::EqualEqual);
    }

    Some(TokenKind::Equal)
  }

  /// Lexes `&&`, emitting a diagnostic when a second `&` is missing.
  fn lex_and(&mut self, engine: &mut DiagnosticEngine) -> Option<TokenKind> {
    if self.match_char(self.peek(), '&') {
      self.advance(); // consume the '='
      return Some(TokenKind::And);
    } else {
      self.emit_error_unexpected_character(engine);
      None
    }
  }

  /// Lexes `||`, emitting a diagnostic when a second `|` is missing.
  fn lex_or(&mut self, engine: &mut DiagnosticEngine) -> Option<TokenKind> {
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
    // Consume valid identifier characters
    while let Some(ch) = self.peek() {
      if !ch.is_ascii_alphanumeric() && ch != '_' {
        break;
      }
      self.advance();
    }

    match self.get_current_lexeme() {
      // Type & Structure Keywords
      "int" => Some(TokenKind::Int),
      "float" => Some(TokenKind::Float),
      "string" => Some(TokenKind::String),
      "bool" => Some(TokenKind::Bool),
      "void" => Some(TokenKind::Void),
      "type" => Some(TokenKind::Type),
      "struct" => Some(TokenKind::Struct),
      "trait" => Some(TokenKind::Trait),
      "impl" => Some(TokenKind::Impl),
      "interface" => Some(TokenKind::Interface),
      "enum" => Some(TokenKind::Enum),

      // Control Flow Keywords
      "if" => Some(TokenKind::If),
      "else" => Some(TokenKind::Else),
      "while" => Some(TokenKind::While),
      "for" => Some(TokenKind::For),
      "loop" => Some(TokenKind::Loop),
      "match" => Some(TokenKind::Match),
      "break" => Some(TokenKind::Break),
      "continue" => Some(TokenKind::Continue),
      "return" => Some(TokenKind::Return),
      "await" => Some(TokenKind::Await),

      // Declaration & Module Keywords
      "let" => Some(TokenKind::Let),
      "mut" => Some(TokenKind::Mut), // ✅ New: mutable variable bindings
      "const" => Some(TokenKind::Const),
      "fn" => Some(TokenKind::Fn),
      "function" => Some(TokenKind::Function),
      "import" => Some(TokenKind::Import),
      "from" => Some(TokenKind::From),
      "export" => Some(TokenKind::Export),
      "as" => Some(TokenKind::As),

      // Object & Type System Keywords
      "self" => Some(TokenKind::SelfKeyword),
      "super" => Some(TokenKind::Super),

      // Logical Operators
      "and" => Some(TokenKind::And),
      "or" => Some(TokenKind::Or),

      // Literals
      "true" => Some(TokenKind::True),
      "false" => Some(TokenKind::False),
      "nil" => Some(TokenKind::Nil),

      // Default Fallback
      _ => Some(TokenKind::Identifier),
    }
  }

  /// Parses an integer or floating-point literal.
  fn lex_number(&mut self) -> Option<TokenKind> {
    while let Some(char) = self.peek() {
      if !char.is_ascii_digit() {
        break;
      }

      self.advance();
    }

    if self.get_current_lexeme().contains(".") {
      return Some(TokenKind::Float);
    }

    Some(TokenKind::Int)
  }

  /// Parses a quoted string literal and reports unterminated strings.
  fn lex_string(&mut self, engine: &mut DiagnosticEngine) -> Option<TokenKind> {
    // The opening quote is already in the lexeme at position self.start
    let first_char = self.source.chars().nth(self.start).unwrap();

    while let Some(char) = self.peek() {
      if self.is_eof() {
        let diagnostic = Diagnostic::new(
          DiagnosticCode::Error(DiagnosticError::UnterminatedString),
          "unterminated string".to_string(),
          "demo.lox".to_string(),
        )
        .with_label(
          Span::new(self.line, 1, 1),
          Some("unterminated string".to_string()),
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
            "demo.lox".to_string(),
          )
          .with_label(
            Span::new(self.line, 1, line_content.len() + 1),
            Some("unterminated string".to_string()),
            LabelStyle::Primary,
          );
          engine.add(diagnostic);
          break;
        }

        let diagnostic = Diagnostic::new(
          DiagnosticCode::Error(DiagnosticError::UnterminatedString),
          "unterminated string".to_string(),
          "demo.lox".to_string(),
        )
        .with_label(
          Span::new(self.line, self.start + 1, self.current + 1),
          Some("unterminated string".to_string()),
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

    Some(TokenKind::String)
  }
}
