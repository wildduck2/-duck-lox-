use colored;

use crate::{
  logger::Log,
  lox::{
    types::{CompilerError, LoxError},
    Lox,
  },
  scanner::Scanner,
};

use super::token::{
  types::{Literal, TokenType},
  Position, Token,
};

impl Scanner {
  /// Scans the entire source string, producing tokens.
  ///
  /// Iterates through the source, advancing one character at a time,
  /// matching characters to token types, and pushing tokens onto `self.tokens`.
  /// Handles single-character tokens, two-character operators, whitespace, and line counting.
  /// At the end, pushes an EOF token.
  pub fn scan_tokens(&mut self, lox: &mut Lox) -> () {
    while !self.is_at_end() {
      self.start = self.current;
      let c = self.advance();

      let token_type = match c {
        '(' => Some(TokenType::LeftParen),
        ')' => Some(TokenType::RightParen),
        '{' => Some(TokenType::LeftBrace),
        '}' => Some(TokenType::RightBrace),
        ',' => Some(TokenType::Comma),
        '.' => Some(TokenType::Dot),
        '-' => Some(TokenType::Minus),
        '+' => Some(TokenType::Plus),
        '*' => Some(TokenType::Star),
        ';' => {
          if !self.match_char('\n') {
            let snippet: String = self.source[self.current..]
              .chars()
              .take_while(|&c| c != '\n')
              .collect();

            while let Some(ch) = self.peek() {
              if ch == '\n' {
                break;
              }
              self.advance();
            }

            lox.has_error = true;
            Lox::log_language(
              lox,
              Log::Error(LoxError::CompileError(CompilerError::SyntaxError)),
              &format!("Expect ';' after expression. Found ';{}' instead.", snippet),
              &format!("{}:{}", self.line, self.column),
            );
            Lox::log_language(
              lox,
              Log::Info,
              &format!(
                "Please make sure the end of your expression is followed by a single semicolon.",
              ),
              &format!("{}:{}", self.line, self.column),
            );

            self.current -= 1;
            None
          } else {
            self.current -= 1;
            Some(TokenType::Semicolon)
          }
        },

        // Handle possible two-character tokens (e.g., !=, ==, <=, >=)
        '!' => {
          if self.match_char('=') {
            Some(TokenType::BangEqual)
          } else {
            Some(TokenType::Bang)
          }
        },
        '=' => {
          if self.match_char('=') {
            Some(TokenType::EqualEqual)
          } else {
            Some(TokenType::Equal)
          }
        },
        '<' => {
          if self.match_char('=') {
            Some(TokenType::LessEqual)
          } else {
            Some(TokenType::Less)
          }
        },
        '>' => {
          if self.match_char('=') {
            Some(TokenType::GreaterEqual)
          } else {
            Some(TokenType::Greater)
          }
        },

        // Ignore whitespace characters
        ' ' | '\r' | '\t' => None,

        // Handle strings
        '"' | '\'' | '`' => {
          let mut s = String::new();
          while let Some(c) = self.peek() {
            if c == '"' || c == '\'' || c == '`' {
              self.advance();
              break;
            }
            s.push(c);
            self.advance();
          }
          Some(TokenType::String)
        },

        // Newline increments line counter
        '\n' => {
          self.line += 1;
          self.column = 0;
          None
        },

        // Handle identifiers and keywords
        'a'..='z' | 'A'..='Z' | '_' => Some(self.tokenize_identifier()),

        // Default case: unrecognized characters
        _ => {
          lox.has_error = true;
          lox.log_language(
            Log::Error(LoxError::CompileError(CompilerError::SyntaxError)),
            &format!("Unexpected character: {}", c),
            &format!("line: {}:{}", self.line, self.column + 1),
          );
          None
        },
      };

      // If a token type was matched, create and push a new token with the current lexeme
      if let Some(ttype) = token_type {
        let lexeme = self.current_lexeme().to_string();
        self.add_token(ttype, lexeme);
      }
    }

    // Add EOF token at the end of scanning
    self.add_token(TokenType::Eof, "".to_string());
  }

  fn tokenize_identifier(&mut self) -> TokenType {
    // Consume the rest of the identifier: letters, digits, or underscores
    while let Some(c) = self.peek() {
      if c.is_ascii_alphanumeric() || c == '_' {
        self.advance();
      } else {
        break;
      }
    }

    let identifier = self.current_lexeme();

    // Match keywords here; add more keywords as needed
    match identifier {
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
      // ...(e.g., "class", "this", etc.)
      _ => TokenType::Identifier,
    }
  }

  /// Returns true if the scanner has reached the end of the source input.
  ///
  /// This is based on the byte index `current` compared to the total byte length of `source`.
  fn is_at_end(&self) -> bool {
    self.current >= self.source.len()
  }

  /// Returns the next character without advancing the scanner.
  ///
  /// Returns `None` if at the end of input.
  fn peek(&self) -> Option<char> {
    if self.is_at_end() {
      None
    } else {
      Some(self.source[self.current..].chars().next().unwrap())
    }
  }

  /// Checks if the next character matches the expected character.
  ///
  /// If it matches, advances the scanner past the character and returns `true`.
  /// Otherwise, does not advance and returns `false`.
  fn match_char(&mut self, expected: char) -> bool {
    if self.is_at_end() {
      return false;
    }
    if self.source[self.current..].chars().next().unwrap() != expected {
      return false;
    }
    self.current += expected.len_utf8();
    true
  }

  /// Returns the current lexeme as a slice of the source string.
  ///
  /// The lexeme spans from the `start` byte index to the `current` byte index.
  fn current_lexeme(&mut self) -> &str {
    let ch = &self.source[self.start..self.current];
    ch
  }

  /// Helper function to add a token to the token list.
  ///
  /// Takes a vector of tokens, token type, and lexeme string, creates a new `Token`
  /// with a default `Literal::Nil` value and current line number, then pushes it.
  fn add_token(&mut self, token_type: TokenType, lexeme: String) -> () {
    let literal = Scanner::get_literal_type(&token_type);
    self.tokens.push(Token::new(
      token_type,
      lexeme,
      literal,
      self.line,
      self.column,
    ));
  }

  fn get_literal_type(token_type: &TokenType) -> Literal {
    match token_type {
      TokenType::Number => Literal::Number,
      TokenType::String => Literal::String,
      TokenType::True => Literal::Boolean,
      TokenType::False => Literal::Boolean,
      _ => Literal::Nil,
    }
  }

  /// Consumes the next character in the source and advances the scanner.
  ///
  /// Returns the character and moves the `current` byte index forward by the UTF-8 length of the character.
  fn advance(&mut self) -> char {
    let ch = self.source[self.current..].chars().next().unwrap();
    self.current += ch.len_utf8();
    self.column += 1;
    ch
  }
}
