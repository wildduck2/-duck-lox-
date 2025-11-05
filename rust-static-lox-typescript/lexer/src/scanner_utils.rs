use diagnostic::{
  code::DiagnosticCode,
  diagnostic::{Diagnostic, LabelStyle},
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

      '+' => {
        if self.match_char(self.peek(), '+') {
          self.advance();
          Some(TokenKind::PlusPlus)
        } else if self.match_char(self.peek(), '=') {
          self.advance();
          Some(TokenKind::PlusEqual)
        } else {
          Some(TokenKind::Plus)
        }
      },

      '-' => {
        if self.match_char(self.peek(), '-') {
          self.advance();
          Some(TokenKind::MinusMinus)
        } else if self.match_char(self.peek(), '=') {
          self.advance();
          Some(TokenKind::MinusEqual)
        } else {
          Some(TokenKind::Minus)
        }
      },

      '*' => {
        if self.match_char(self.peek(), '=') {
          self.advance();
          Some(TokenKind::StarEqual)
        } else {
          Some(TokenKind::Star)
        }
      },

      '/' => self.lex_divide(),
      '%' => {
        if self.match_char(self.peek(), '=') {
          self.advance();
          Some(TokenKind::PercentEqual)
        } else {
          Some(TokenKind::Percent)
        }
      },

      '&' => self.lex_and(engine),
      '|' => self.lex_or(),
      '^' => {
        if self.match_char(self.peek(), '=') {
          self.advance();
          Some(TokenKind::CaretEqual)
        } else {
          Some(TokenKind::Caret)
        }
      },

      '~' => Some(TokenKind::Tilde),
      '!' => self.lex_bang(),
      '=' => self.lex_equal(),
      '<' => self.lex_less(),
      '>' => self.lex_greater(),

      ';' => Some(TokenKind::Semicolon),
      ',' => Some(TokenKind::Comma),
      '.' => self.lex_dot(),
      ':' => Some(TokenKind::Colon),
      '?' => Some(TokenKind::Question),
      '`' => self.lex_string(engine),
      '"' | '\'' => self.lex_string(engine),

      '@' => Some(TokenKind::At),
      '#' => Some(TokenKind::Hash),

      '\n' => {
        self.line += 1;
        self.column = 0;
        None
      },

      '\r' | '\t' | ' ' => None,
      'A'..='Z' | 'a'..='z' | '_' => self.lex_keywords(),
      '0'..='9' => self.lex_number(),

      _ => {
        let diagnostic = Diagnostic::new(
          DiagnosticCode::Error(DiagnosticError::InvalidCharacter),
          format!("unexpected character: {}", self.get_current_lexeme()),
          "source.ts".to_string(),
        )
        .with_label(
          diagnostic::Span::new(self.start, self.current),
          Some("unexpected character".to_string()),
          LabelStyle::Primary,
        );

        engine.add(diagnostic);
        None
      },
    }
  }

  fn lex_dot(&mut self) -> Option<TokenKind> {
    if self.match_char(self.peek(), '.') {
      self.advance();
      if self.match_char(self.peek(), '.') {
        self.advance();
        return Some(TokenKind::DotDotDot);
      }
    }
    Some(TokenKind::Dot)
  }

  fn lex_divide(&mut self) -> Option<TokenKind> {
    if self.match_char(self.peek(), '/') {
      return self.lex_line_comment();
    } else if self.match_char(self.peek(), '*') {
      return self.lex_multi_line_comment();
    } else if self.match_char(self.peek(), '=') {
      self.advance();
      return Some(TokenKind::SlashEqual);
    }
    Some(TokenKind::Slash)
  }

  fn lex_line_comment(&mut self) -> Option<TokenKind> {
    while !self.is_eof() {
      if self.peek() == Some('\n') {
        break;
      }
      self.advance();
    }
    Some(TokenKind::SingleLineComment)
  }

  fn lex_multi_line_comment(&mut self) -> Option<TokenKind> {
    while !self.is_eof() {
      if self.peek() == Some('*') && self.peek_next() == Some('/') {
        self.advance();
        self.advance();
        break;
      }
      if self.peek() == Some('\n') {
        self.line += 1;
        self.column = 0;
      }
      self.advance();
    }
    Some(TokenKind::MultiLineComment)
  }

  fn lex_bang(&mut self) -> Option<TokenKind> {
    if self.match_char(self.peek(), '=') {
      self.advance();
      if self.match_char(self.peek(), '=') {
        self.advance();
        return Some(TokenKind::NotEqualEqual);
      }
      return Some(TokenKind::NotEqual);
    }
    Some(TokenKind::Bang)
  }

  fn lex_equal(&mut self) -> Option<TokenKind> {
    if self.match_char(self.peek(), '=') {
      self.advance();
      if self.match_char(self.peek(), '=') {
        self.advance();
        return Some(TokenKind::EqualEqualEqual);
      }
      return Some(TokenKind::EqualEqual);
    } else if self.match_char(self.peek(), '>') {
      self.advance();
      return Some(TokenKind::Arrow);
    }

    Some(TokenKind::Equal)
  }

  fn lex_less(&mut self) -> Option<TokenKind> {
    if self.match_char(self.peek(), '<') {
      self.advance();
      if self.match_char(self.peek(), '=') {
        self.advance();
        return Some(TokenKind::LessLessEqual);
      }
      return Some(TokenKind::LessLess);
    }
    if self.match_char(self.peek(), '=') {
      self.advance();
      return Some(TokenKind::LessEqual);
    }
    Some(TokenKind::Less)
  }

  fn lex_greater(&mut self) -> Option<TokenKind> {
    if self.match_char(self.peek(), '>') {
      self.advance();
      if self.match_char(self.peek(), '>') {
        self.advance();
        if self.match_char(self.peek(), '=') {
          self.advance();
          return Some(TokenKind::GreaterGreaterGreaterEqual);
        }
        return Some(TokenKind::GreaterGreaterGreater);
      }
      if self.match_char(self.peek(), '=') {
        self.advance();
        return Some(TokenKind::GreaterGreaterEqual);
      }
      return Some(TokenKind::GreaterGreater);
    }
    if self.match_char(self.peek(), '=') {
      self.advance();
      return Some(TokenKind::GreaterEqual);
    }
    Some(TokenKind::Greater)
  }

  fn lex_and(&mut self, engine: &mut DiagnosticEngine) -> Option<TokenKind> {
    if self.match_char(self.peek(), '&') {
      self.advance();
      return Some(TokenKind::AmpAmp);
    } else if self.match_char(self.peek(), '=') {
      self.advance();
      return Some(TokenKind::AmpEqual);
    } else {
      self.emit_error_unexpected_character(engine);
      None
    }
  }

  fn lex_or(&mut self) -> Option<TokenKind> {
    if self.match_char(self.peek(), '|') {
      self.advance();
      return Some(TokenKind::PipePipe);
    } else if self.match_char(self.peek(), '=') {
      self.advance();
      return Some(TokenKind::PipeEqual);
    } else {
      Some(TokenKind::Pipe)
    }
  }

  /// TypeScript keyword recognition
  fn lex_keywords(&mut self) -> Option<TokenKind> {
    while let Some(ch) = self.peek() {
      if !ch.is_ascii_alphanumeric() && ch != '_' {
        break;
      }
      self.advance();
    }

    match self.get_current_lexeme() {
      // Core Keywords
      "break" => Some(TokenKind::Break),
      "case" => Some(TokenKind::Case),
      "catch" => Some(TokenKind::Catch),
      "class" => Some(TokenKind::Class),
      "const" => Some(TokenKind::Const),
      "continue" => Some(TokenKind::Continue),
      "debugger" => Some(TokenKind::Debugger),
      "default" => Some(TokenKind::Default),
      "delete" => Some(TokenKind::Delete),
      "do" => Some(TokenKind::Do),
      "else" => Some(TokenKind::Else),
      "enum" => Some(TokenKind::Enum),
      "export" => Some(TokenKind::Export),
      "extends" => Some(TokenKind::Extends),
      "false" => Some(TokenKind::False),
      "finally" => Some(TokenKind::Finally),
      "for" => Some(TokenKind::For),
      "function" => Some(TokenKind::Function),
      "if" => Some(TokenKind::If),
      "import" => Some(TokenKind::Import),
      "in" => Some(TokenKind::In),
      "instanceof" => Some(TokenKind::Instanceof),
      "new" => Some(TokenKind::New),
      "null" => Some(TokenKind::Null),
      "never" => Some(TokenKind::Never),
      "unknown" => Some(TokenKind::Unknown),
      "undefined" => Some(TokenKind::Undefined),
      "bigint" => Some(TokenKind::BigInt),
      "return" => Some(TokenKind::Return),
      "super" => Some(TokenKind::Super),
      "switch" => Some(TokenKind::Switch),
      "this" => Some(TokenKind::This),
      "throw" => Some(TokenKind::Throw),
      "true" => Some(TokenKind::True),
      "try" => Some(TokenKind::Try),
      "typeof" => Some(TokenKind::Typeof),
      "var" => Some(TokenKind::Var),
      "void" => Some(TokenKind::Void),
      "while" => Some(TokenKind::While),
      "with" => Some(TokenKind::With),
      "yield" => Some(TokenKind::Yield),
      "await" => Some(TokenKind::Await),
      "as" => Some(TokenKind::As),
      "implements" => Some(TokenKind::Implements),
      "interface" => Some(TokenKind::Interface),
      "let" => Some(TokenKind::Let),
      "package" => Some(TokenKind::Package),
      "private" => Some(TokenKind::Private),
      "protected" => Some(TokenKind::Protected),
      "public" => Some(TokenKind::Public),
      "static" => Some(TokenKind::Static),
      "any" => Some(TokenKind::Any),
      "boolean" => Some(TokenKind::Boolean),
      "constructor" => Some(TokenKind::Constructor),
      "declare" => Some(TokenKind::Declare),
      "get" => Some(TokenKind::Get),
      "module" => Some(TokenKind::Module),
      "namespace" => Some(TokenKind::Namespace),
      "require" => Some(TokenKind::Require),
      "number" => Some(TokenKind::Number),
      "set" => Some(TokenKind::Set),
      "string" => Some(TokenKind::String),
      "symbol" => Some(TokenKind::Symbol),
      "type" => Some(TokenKind::Type),
      "from" => Some(TokenKind::From),
      "of" => Some(TokenKind::Of),
      _ => Some(TokenKind::Identifier),
    }
  }

  fn lex_number(&mut self) -> Option<TokenKind> {
    while let Some(c) = self.peek() {
      if c.is_ascii_digit() {
        self.advance();
      } else {
        break;
      }
    }

    if self.peek() == Some('.')
      && self
        .peek_next()
        .map(|n| n.is_ascii_digit())
        .unwrap_or(false)
    {
      self.advance();
      while let Some(c) = self.peek() {
        if c.is_ascii_digit() {
          self.advance();
        } else {
          break;
        }
      }
    }

    Some(TokenKind::Number)
  }

  fn lex_string(&mut self, engine: &mut DiagnosticEngine) -> Option<TokenKind> {
    let quote = self.source.chars().nth(self.start).unwrap();

    while let Some(ch) = self.peek() {
      if self.is_eof() {
        self.report_unterminated_string(engine);
        break;
      }

      if ch == quote {
        self.advance();
        break;
      }

      if ch == '\n' && quote != '`' {
        self.report_unterminated_string(engine);
        break;
      }

      self.advance();
    }

    Some(TokenKind::String)
  }

  fn report_unterminated_string(&mut self, engine: &mut DiagnosticEngine) {
    let diagnostic = Diagnostic::new(
      DiagnosticCode::Error(DiagnosticError::UnterminatedString),
      "unterminated string".to_string(),
      "source.ts".to_string(),
    )
    .with_label(
      diagnostic::Span::new(self.start, self.current),
      Some("unterminated string".to_string()),
      LabelStyle::Primary,
    );
    engine.add(diagnostic);
  }
}
