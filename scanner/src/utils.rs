use logger::Logger;

use crate::{
  lox::{CompilerError, Lox, LoxError},
  token::{
    types::{Literal, TokenType},
    Token,
  },
  Scanner,
};

impl Scanner {
  /// Function that maps over the "lox" and returns a `Vec<Token>`.
  pub fn get_tokens(&mut self, mut lox: &mut Lox) -> () {
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
        '+' => Some(TokenType::Plus),
        '-' => Some(TokenType::Minus),
        '*' => Some(TokenType::Star),
        '%' => Some(TokenType::Modulus),

        // Comment and the Divide
        '/' => Some(self.tokenize_comments(&mut lox)),

        // Strings
        '"' | '\'' | '`' => Some(self.tokenize_strings(lox)),

        // New line
        '\n' => {
          self.column = 0;
          self.line += 1;
          None
        },

        // And condition check
        '&' => {
          if self.match_char(&'&') {
            Some(TokenType::Or)
          } else {
            None
          }
        },

        // Or condition check
        '|' => {
          if self.match_char(&'|') {
            Some(TokenType::Or)
          } else {
            None
          }
        },

        // Comparison And/Or Equality
        '>' => {
          if self.match_char(&'=') {
            Some(TokenType::GreaterEqual)
          } else {
            Some(TokenType::Greater)
          }
        },
        '<' => {
          if self.match_char(&'=') {
            Some(TokenType::LessEqual)
          } else {
            Some(TokenType::Less)
          }
        },

        // Equal and Strict Equal
        '=' => {
          if self.match_char(&'=') {
            Some(TokenType::EqualEqual)
          } else {
            Some(TokenType::Equal)
          }
        },

        // Not Equal
        '!' => {
          if self.match_char(&'=') {
            Some(TokenType::EqualEqual)
          } else {
            None
          }
        },

        // SemiColon line Terminator
        ';' => self.tokenize_semicolon(lox),

        '.' => self.tokenize_dot(lox),

        // Ignore whitespace
        ' ' | '\r' | '\t' => None,
        // String
        'a'..='z' | 'A'..='Z' | '_' => Some(self.tokenize_keywords()),
        // Number
        '1'..='9' => Some(self.tokenize_numbers()),

        // Default case: unrecognized characters
        _ => {
          lox.has_error = true;

          Logger::log(
            logger::LogType::Error(&format!(
              "[{:?}] Unexpected character: {:?} [{}:{}]",
              LoxError::CompileError(CompilerError::SyntaxError),
              self.get_current_lexeme(),
              self.line,
              self.column + 1
            )),
            0,
          );

          None
        },
      };

      if let Some(token_type) = token {
        self.add_token(token_type);
      };
    }

    ()
  }

  // Function that tokenize dot
  fn tokenize_dot(&mut self, lox: &mut Lox) -> Option<TokenType> {
    while let Some(char) = self.peek() {
      if char == '(' {
        // Check for the second pair or the function execution
        if self.match_char(&')') {
          break;
        }
        // NOTE: later you have to make some token type for function execution.
        return Some(TokenType::Fun);
        // Function Execution
      } else if char == ' ' {
        let snippet: String = self.source[((self.current - 1) as usize)..]
          .chars()
          .take_while(|&c| c != '\n')
          .collect();

        // Throw Error wrong syntax
        Logger::log(
          logger::LogType::Error(&format!(
            "[{:?}] Unexpected character: {:?} [{}:{}]",
            LoxError::CompileError(CompilerError::SyntaxError),
            format!("{}{}", self.get_current_lexeme(), snippet),
            self.line,
            self.column + 1
          )),
          0,
        );
      }
    }
    Some(TokenType::Dot)
  }

  // Function that tokenize the semi colon
  fn tokenize_semicolon(&self, lox: &mut Lox) -> Option<TokenType> {
    if self.match_char(&'\n')
      && self.tokens.len() > 0
      && !self.tokens[self.tokens.len() - 1].lexeme.ends_with(';')
    {
      Some(TokenType::SemiColon)
    } else {
      lox.has_error = true;
      let snippet: String = self.source[(self.current as usize)..]
        .chars()
        .take_while(|&c| c != '\n')
        .collect();

      Logger::log(
        logger::LogType::Error(&format!(
          "[{:?}] Unexpected character: {:?} [{}:{}]",
          LoxError::CompileError(CompilerError::SyntaxError),
          format!("{}{}", self.get_current_lexeme(), snippet),
          self.line,
          self.column + 1
        )),
        0,
      );
      Logger::log(
              logger::LogType::Info(&format!(
                "[{:?}] Please make sure the end of your expression is followed by a single semicolon. [{}:{}]",
                LoxError::CompileError(CompilerError::SyntaxError),
                self.line,
                self.column + 1
              )),
              0,
            );

      None
    }
  }

  /// Function that tokenize all the string shapes
  fn tokenize_strings(&mut self, lox: &mut Lox) -> TokenType {
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
        lox.has_error = true;
        Logger::log(
          logger::LogType::Error(&format!(
            "[{:?}] wrong string syntax: {:?} [{}:{}]",
            LoxError::CompileError(CompilerError::SyntaxError),
            self.get_current_lexeme(),
            self.line,
            self.column + 1
          )),
          0,
        );
      }
    }

    TokenType::String
  }

  /// Function that tokenize lox comments and if it's not a comment it might a "division" or `None`
  fn tokenize_comments(&mut self, lox: &mut Lox) -> TokenType {
    if self.match_char(&'/') {
      while let Some(char) = self.peek() {
        // Check for line comment
        if char == '\n' {
          break;
        }
        self.advance();
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
        // Unterminated multi-line comment
        lox.has_error = true;
        Logger::log(
          logger::LogType::Error(&format!(
            "[{:?}] Unterminated multi-line comment: {:?} [{}:{}]",
            LoxError::CompileError(CompilerError::SyntaxError),
            self.get_current_lexeme(),
            self.line,
            self.column + 1,
          )),
          0,
        );
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
          self.advance();
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
      "print" => TokenType::Print,
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
        println!("Comment: {}", lexeme);
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
