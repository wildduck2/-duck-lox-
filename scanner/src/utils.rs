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
  pub fn get_tokens(&mut self, mut lox: Lox) -> () {
    while !self.is_at_end() {
      self.start = self.current;
      let c = self.advance();

      let token = match c {
        '{' => Some(TokenType::LeftBrace),
        '}' => Some(TokenType::RightBrace),
        '(' => Some(TokenType::LeftParen),
        ')' => Some(TokenType::RightParen),

        // Mathematical operators
        '+' => Some(TokenType::Plus),
        '-' => Some(TokenType::Minus),
        '*' => Some(TokenType::Star),
        '%' => Some(TokenType::Modulus),

        // comment and the divide
        '/' => Some(TokenType::Divide),

        // Comparison
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

        '\n' => {
          self.column = 0;
          self.line += 1;
          None
        },

        '=' => {
          if self.match_char(&'=') {
            Some(TokenType::EqualEqual)
          } else {
            Some(TokenType::Equal)
          }
        },

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
              "{:?} Unexpected character: {} line: {}:{}",
              LoxError::CompileError(CompilerError::SyntaxError),
              c,
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

  /// Function that tokenize lox numbers and return `TokenType`
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

  /// Function that tokenize lox keywords and return `TokenType`
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

  /// Function that takes "token_type" and push a struct token to the `Vec<Token>`
  fn add_token(&mut self, token_type: TokenType) -> () {
    let lexeme = self.get_current_lexeme().to_string();
    let literal = self.get_literal(&token_type);

    self.tokens.push(Token {
      token_type,
      lexeme,
      literal,
      position: (self.line, self.column + 1),
    });

    ()
  }

  /// Function that gets the literal type of the token
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

  /// Function that return the next char and shift the current and column count to this char
  fn advance(&mut self) -> char {
    let char = self.peek();

    self.current += 1;
    self.column += 1;

    char.unwrap()
  }

  fn peek(&self) -> Option<char> {
    if self.is_at_end() {
      return Some('\0');
    };

    let char = self.source[(self.current as usize)..]
      .chars()
      .next()
      .unwrap();

    Some(char)
  }

  /// Function that returns the current lexelme
  fn get_current_lexeme(&self) -> &str {
    return &self.source[(self.start as usize)..(self.current as usize)];
  }

  /// Function that matches the next char to an argument and returns true
  fn match_char(&self, expected: &char) -> bool {
    if !self.is_at_end() {
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
