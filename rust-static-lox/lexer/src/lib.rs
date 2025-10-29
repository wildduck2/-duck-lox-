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
  /// Creates a lexer over the provided source text.
  pub fn new(source: &'a str) -> Self {
    Self {
      source,
      tokens: Vec::new(),
      start: 0,
      current: 0,
      line: 0,
      column: 0,
    }
  }

  /// Tokenizes the entire source, emitting tokens and diagnostics along the way.
  pub fn scan_tokens(&mut self, engine: &mut DiagnosticEngine<'a>) {
    while !self.is_eof() {
      self.start = self.current;
      let c = self.advance();

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

  /// Pushes a token covering the span between `start` and `current`.
  fn emit(&mut self, kind: TokenKind) {
    self.tokens.push(Token {
      kind,
      lexeme: self.get_current_lexeme(),
      span: Span::new(self.line, self.start, self.current),
    });
    self.start = self.current;
  }

  /// Returns the next character without consuming it, or `None` at end of input.
  fn peek(&self) -> Option<char> {
    if self.is_eof() {
      println!("EOF");
      return None;
    }

    let char = self.source[(self.current as usize)..]
      .chars()
      .next()
      .unwrap();

    Some(char)
  }

  /// Returns the character one position ahead of the cursor without advancing it.
  fn peek_next(&self) -> Option<char> {
    if self.is_eof() {
      return None;
    }

    self.source[((self.current + 1) as usize)..].chars().next()
  }

  /// Consumes the next character and updates the byte offset and column counters.
  fn advance(&mut self) -> char {
    let char = self.peek();

    self.current += 1;
    self.column += 1;

    match char {
      Some(c) => c,
      None => {
        panic!("Failed to advance");
      },
    }
  }

  /// Returns the current lexeme slice spanning the active token.
  fn get_current_lexeme(&self) -> &'a str {
    &self.source[self.start..self.current]
  }

  /// Returns `true` when the cursor has reached the end of the source text.
  fn is_eof(&self) -> bool {
    self.current >= self.source.len()
  }

  /// Emits a diagnostic for an unexpected character at the current cursor.
  fn emit_error_unexpected_character(&mut self, engine: &mut DiagnosticEngine<'a>) {
    let current_line = self.get_line(self.line);

    let diagnostic = Diagnostic::new(
      DiagnosticCode::Error(DiagnosticError::InvalidCharacter),
      format!("unexpected character: {}", self.get_current_lexeme()),
      "demo.lox",
    )
    .with_context_line(self.line, current_line) // ADD THIS!
    .with_label(
      Span::new(
        self.line,
        self.current,
        self.column + self.get_current_lexeme().len() - 1,
      ),
      Some("unexpected character"),
      LabelStyle::Primary,
    );

    engine.add(diagnostic);
  }

  /// Returns the source line corresponding to `line_num`, or an empty string if it is out of range.
  pub fn get_line(&self, line_num: usize) -> &'a str {
    self.source.lines().nth(line_num).unwrap_or("")
  }
}
