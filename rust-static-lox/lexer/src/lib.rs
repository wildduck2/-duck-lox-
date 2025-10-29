use diagnostic::{
  code::DiagnosticCode,
  diagnostic::{Diagnostic, LabelStyle, Span},
  types::error::DiagnosticError,
  DiagnosticEngine,
};

use crate::token::{Token, TokenKind};

mod scanner_utils;
pub mod token;

#[derive(Debug)]
pub struct Lexer<'a> {
  pub source: &'a str,
  pub tokens: Vec<Token<'a>>,
  pub start: usize,   // Start byte offset of current token
  pub current: usize, // Current byte offset in source
  pub line: usize,    // Current line (1-indexed)
  pub column: usize,  // Current column (1-indexed)
}

impl<'a> Lexer<'a> {
  /// Function that creates a new instance of the lexer.
  pub fn new(source: &'a str) -> Self {
    Self {
      source,
      tokens: Vec::new(),
      start: 0,
      current: 0,
      line: 1,
      column: 1,
    }
  }

  /// Function that scans the tokens.
  pub fn scan_tokens(&mut self, engine: &mut DiagnosticEngine) {
    while !self.is_eof() {
      self.start = self.current;
      let c = self.advance(1);

      let token = self.lex_tokens(c, engine);

      if let Some(token_type) = token {
        self.emit(token_type);
      };
    }

    self.emit(TokenKind::Eof);
  }

  fn match_char(&mut self, char: Option<char>, match_char: char) -> bool {
    if let Some(char) = char {
      if char == match_char {
        return true;
      }
    }
    return false;
  }

  /// Function that emits the token.
  fn emit(&mut self, kind: TokenKind) {
    self.tokens.push(Token {
      kind,
      lexeme: self.get_current_lexeme(),
      span: Span::new(self.start, self.current),
    });
    self.start = self.current;
  }

  /// Function that returns the next char without advancing the pointer.
  fn peek(&self) -> Option<char> {
    if self.is_eof() {
      return None;
    }

    self.source[(self.current as usize)..].chars().next()
  }

  /// Function that returns the (next + 1) char and shift the current and column count to this char.
  fn peek_next(&self) -> Option<char> {
    if self.is_eof() {
      return None;
    }

    self.source[((self.current + 1) as usize)..].chars().next()
  }

  /// Function that returns the next char without advancing the pointer.
  fn advance(&mut self, n: u32) -> char {
    let mut char: Option<char> = None;

    for _i in 0..n {
      char = self.peek();
      self.current += 1;
      self.column += 1;
    }

    match char {
      Some(c) => c,
      None => {
        panic!("Failed to advance");
      },
    }
  }

  /// Function that returns the current lexelme.
  fn get_current_lexeme(&self) -> &'a str {
    &self.source[self.start..self.current]
  }

  /// Function that matches the next char to an argument and returns true.
  fn is_eof(&self) -> bool {
    self.current >= self.source.len()
  }

  fn emit_error_unexpected_character(&mut self, engine: &mut DiagnosticEngine) {
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

    engine.add(diagnostic);
  }
}
