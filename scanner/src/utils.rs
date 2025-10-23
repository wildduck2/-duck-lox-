use diagnostic::{
  diagnostic::{Diagnostic, Label, Span},
  diagnostic_code::DiagnosticCode,
  DiagnosticEngine,
};

use crate::{
  token::{
    types::{Literal, TokenType},
    Token,
  },
  Scanner,
};

impl Scanner {
  /// Function that maps over the "lox" and returns a `Vec<Token>`.
  pub fn get_tokens(&mut self, engine: &mut DiagnosticEngine) -> () {
    while !self.is_at_end() {
      self.start = self.current;
      let c = self.advance();

      let token = match c {
        '{' => Some(TokenType::LeftBrace),
        '}' => Some(TokenType::RightBrace),
        '(' => Some(TokenType::LeftParen),
        ')' => Some(TokenType::RightParen),
        '[' => Some(TokenType::LeftBracket),
        ']' => Some(TokenType::RightBracket),

        // Mathematical operators
        //
        // According to the maximal munch rule, +++a is tokenized as '++' '+a', not '+''++a'.
        // The scanner always chooses the longest valid token, even if it leads to a syntax error later.
        '+' => {
          if self.match_char(&'+') {
            self.advance();
            Some(TokenType::PlusPlus)
          } else if self.match_char(&'=') {
            self.advance();
            Some(TokenType::PlusEqual)
          } else {
            Some(TokenType::Plus)
          }
        },

        // Note: '-' is parsed as a unary operator, not part of a number literal.
        // This keeps expressions like `-n.abs()` and `-123.abs()` consistent,
        // since method calls have higher precedence than unary minus.
        // Treating '-' as part of the literal would make `-123.abs()` behave differently.
        //
        // According to the maximal munch rule, ---a is tokenized as '--' '-a', not '-''--a'.
        // The scanner always chooses the longest valid token, even if it leads to a syntax error later.
        '-' => {
          if self.match_char(&'-') {
            self.advance();
            Some(TokenType::MinusMinus)
          } else if self.match_char(&'=') {
            self.advance();
            Some(TokenType::MinusEqual)
          } else {
            Some(TokenType::Minus)
          }
        },
        '*' => {
          if self.match_char(&'=') {
            self.advance();
            Some(TokenType::MultiplyEqual)
          } else {
            Some(TokenType::Multiply)
          }
        },

        '%' => Some(TokenType::Modulus),

        // Comment and the Divide
        '/' => Some(self.tokenize_comments(engine)),

        // Strings
        '"' | '\'' | '`' => Some(self.tokenize_strings(engine)),

        // And condition check
        '&' => {
          if self.match_char(&'&') {
            self.advance();
            Some(TokenType::And)
          } else {
            None
          }
        },

        // Ternary operators
        '?' => Some(TokenType::Question),
        ':' => Some(TokenType::Colon),
        // Or condition check
        '|' => {
          if self.match_char(&'|') {
            self.advance();
            Some(TokenType::Or)
          } else {
            None
          }
        },

        // Comparison And/Or Equality
        '>' => {
          if self.match_char(&'=') {
            self.advance();
            Some(TokenType::GreaterEqual)
          } else {
            Some(TokenType::Greater)
          }
        },
        '<' => {
          if self.match_char(&'=') {
            self.advance();
            Some(TokenType::LessEqual)
          } else {
            Some(TokenType::Less)
          }
        },

        // Equal and Strict Equal
        '=' => {
          if self.match_char(&'=') {
            self.advance();
            Some(TokenType::EqualEqual)
          } else {
            Some(TokenType::Equal)
          }
        },

        // Not Equal and Bang
        '!' => {
          if self.match_char(&'=') {
            self.advance();
            Some(TokenType::BangEqual)
          } else {
            Some(TokenType::Bang)
          }
        },

        // SemiColon line Terminator
        ';' => Some(TokenType::SemiColon),

        '.' => self.tokenize_dot(),
        ',' => Some(TokenType::Comma),

        // Ignore whitespace
        ' ' | '\r' | '\t' => None,
        // String
        'a'..='z' | 'A'..='Z' | '_' => Some(self.tokenize_keywords()),
        // Number
        '0'..='9' => Some(self.tokenize_numbers()),

        // New line
        '\n' => {
          self.column = 0;
          self.line += 1;
          None
        },

        // Default case: unrecognized characters
        _ => {
          let diagnostic = Diagnostic::new(
            DiagnosticCode::InvalidCharacter,
            format!("unexpected character: {:?}", self.get_current_lexeme()),
          )
          .with_label(Label::primary(
            Span {
              file: "input".to_string(),
              line: self.line,
              column: self.column,
              length: 1,
            },
            None,
          ));

          engine.emit(diagnostic);
          None
        },
      };

      if let Some(token_type) = token {
        self.add_token(token_type);
      };
    }

    self.tokens.push(Token {
      token_type: TokenType::Eof,
      lexeme: String::from(""),
      literal: Literal::Nil,
      position: (self.line, self.column),
    });

    ()
  }

  fn tokenize_dot(&mut self) -> Option<TokenType> {
    if let Some(char) = self.peek() {
      if char.is_ascii_digit() {
        while let Some(char) = self.peek() {
          if !char.is_ascii_digit() {
            break;
          }
          self.advance();
        }
        return Some(TokenType::Number);
      }
    }
    Some(TokenType::Dot)
  }

  /// Function that tokenize all the string shapes
  fn tokenize_strings(&mut self, engine: &mut DiagnosticEngine) -> TokenType {
    let current_char = self.get_current_lexeme().chars().collect::<Vec<_>>()[0];

    while let Some(char) = self.peek() {
      self.advance();
      if (current_char == '\'' && char == '\'')
        || (current_char == '"' && char == '"')
        || (current_char == '`' && char == '`')
      {
        break;
      }

      if char == '\n' && current_char != '`' {
        let diagnostic = Diagnostic::new(
          DiagnosticCode::UnterminatedString,
          "wrong string syntax".to_string(),
        )
        .with_label(Label::primary(
          Span {
            file: "input".to_string(),
            line: self.line,
            column: self.start,
            length: self.get_current_lexeme().len(),
          },
          Some("newline not allowed in string".to_string()),
        ));

        engine.emit(diagnostic);
      }
    }

    TokenType::String
  }

  /// Function that tokenize lox comments and if it's not a comment it might a "division" or `None`
  fn tokenize_comments(&mut self, engine: &mut DiagnosticEngine) -> TokenType {
    if self.match_char(&'=') {
      self.advance();
      TokenType::DivideEqual
    } else if self.match_char(&'/') {
      loop {
        match self.advance() {
          char => {
            if char == '\n' {
              break;
            };
          },
        }
      }

      TokenType::Comment
    } else if self.match_char(&'*') {
      // Checking for the block comment
      while !self.is_at_end() {
        let char = self.peek().unwrap();
        if char == '*' && self.peek_next().unwrap() == '/' {
          self.advance();
          self.advance();
          break;
        }

        let char = self.advance();
        if char == '\n' {
          self.column = 0;
          self.line += 1;
        }
      }

      if self.is_at_end() {
        let diagnostic = Diagnostic::new(
          DiagnosticCode::UnterminatedString,
          "unterminated multi-line comment".to_string(),
        )
        .with_label(Label::primary(
          Span {
            file: "input".to_string(),
            line: self.line,
            column: self.column,
            length: self.get_current_lexeme().len(),
          },
          Some("reached end of file before closing comment".to_string()),
        ));

        engine.emit(diagnostic);
      }
      TokenType::Comment
    } else {
      TokenType::Divide
    }
  }

  /// Function that tokenize lox numbers and return `TokenType`.
  fn tokenize_numbers(&mut self) -> TokenType {
    while let Some(char) = self.peek() {
      if char.is_ascii_digit() {
        self.advance();
      } else {
        if self.match_char(&'.') {
          match self.peek_next() {
            Some(char) => {
              if char.is_ascii_digit() {
                self.advance();
              } else {
                break;
              }
            },
            None => {
              break;
            },
          }
        } else {
          break;
        }
      }
    }

    TokenType::Number
  }

  /// Function that tokenize lox keywords and return `TokenType`.
  fn tokenize_keywords(&mut self) -> TokenType {
    while let Some(char) = self.peek() {
      if char.is_ascii_alphanumeric() || char == '_' {
        self.advance();
      } else {
        break;
      }
    }

    match self.get_current_lexeme() {
      "var" => TokenType::Var,
      "fun" => TokenType::Fun,
      "return" => TokenType::Return,
      "if" => TokenType::If,
      "else" => TokenType::Else,
      "for" => TokenType::For,
      "while" => TokenType::While,
      "break" => TokenType::Break,
      "continue" => TokenType::Continue,
      "class" => TokenType::Class,
      "this" => TokenType::This,
      "true" => TokenType::True,
      "false" => TokenType::False,
      "nil" => TokenType::Nil,
      "or" => TokenType::Or,
      "and" => TokenType::And,
      "super" => TokenType::Super,
      _ => TokenType::Identifier,
    }
  }

  /// Function that takes "token_type" and push a struct token to the `Vec<Token>`.
  fn add_token(&mut self, token_type: TokenType) {
    let mut lexeme = self.get_current_lexeme().to_string();
    let literal = self.get_literal(&token_type);

    match token_type {
      TokenType::Comment => {
        // println!("Comment: {}", lexeme);
        return; // don't add comment tokens
      },

      TokenType::String => {
        // Remove the quotes from the string literal
        if lexeme.len() >= 2 {
          lexeme = lexeme[1..lexeme.len() - 1].to_string();
        }
      },

      TokenType::Number => {
        // Normalize numbers like `.5` → `0.5` and `5.` → `5`
        if lexeme.ends_with('.') {
          lexeme = lexeme.trim_end_matches('.').to_string();
        } else if lexeme.starts_with('.') {
          lexeme = format!("0{}", lexeme);
        }
      },

      _ => {},
    }

    self.tokens.push(Token {
      token_type,
      lexeme,
      literal,
      position: (self.line, self.column),
    });
  }

  /// Function that gets the literal type of the token.
  fn get_literal(&self, token_type: &TokenType) -> Literal {
    match token_type {
      TokenType::String => Literal::String,
      TokenType::Number => Literal::Number,
      TokenType::True => Literal::Boolean,
      TokenType::False => Literal::Boolean,
      _ => Literal::Nil,
    }
  }

  /// Function that returns `bool` which indicate the state at the "EOF".
  fn is_at_end(&self) -> bool {
    (self.current as usize) == self.source.len()
  }

  /// Function that return the next char and shift the current and column count to this char.
  fn advance(&mut self) -> char {
    let char = self.peek();

    self.current += 1;
    self.column += 1;

    char.unwrap()
  }

  /// Function that returns the next char without advancing the pointer.
  fn peek(&self) -> Option<char> {
    if self.is_at_end() {
      return None;
    };

    let char = self.source[(self.current as usize)..]
      .chars()
      .next()
      .unwrap();

    Some(char)
  }
  fn peek_next(&self) -> Option<char> {
    if self.is_at_end() {
      return None;
    };

    let char = self.source[((self.current + 1) as usize)..]
      .chars()
      .next()
      .unwrap();

    Some(char)
  }

  /// Function that returns the current lexelme.
  fn get_current_lexeme(&self) -> &str {
    return &self.source[(self.start as usize)..(self.current as usize)];
  }

  /// Function that matches the next char to an argument and returns true.
  fn match_char(&self, expected: &char) -> bool {
    if self.is_at_end() {
      return false;
    }

    if &self.source[(self.current as usize)..]
      .chars()
      .next()
      .unwrap()
      != expected
    {
      return false;
    }

    true
  }
}
