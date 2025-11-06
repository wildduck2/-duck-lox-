use diagnostic::{DiagnosticEngine, SourceFile, Span};

use crate::token::{Token, TokenKind};

mod lexers;
mod scanner_utils;
pub mod token;

#[derive(Debug)]
pub struct Lexer {
  pub source: SourceFile,
  pub tokens: Vec<Token>,
  pub start: usize,   // Start byte offset of current token
  pub current: usize, // Current byte offset in source
  pub line: usize,    // Current line (1-indexed)
  pub column: usize,  // Current column (1-indexed)
}

impl Lexer {
  /// Creates a lexer over the provided source text.
  pub fn new(source: SourceFile) -> Self {
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
  pub fn scan_tokens(&mut self, engine: &mut DiagnosticEngine) {
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

  /// Check is the next character matches the target character.
  fn match_char(&mut self, match_char: char) -> bool {
    if let Some(char) = self.peek() {
      if char == match_char {
        self.advance();
        return true;
      }
    }
    return false;
  }

  /// Pushes a token covering the span between `start` and `current`.
  fn emit(&mut self, kind: TokenKind) {
    // ignore comments
    if kind.is_trivia() {
      return;
    }

    // println!("{:?}", self.source[self.start..self.current].to_string());
    self.tokens.push(Token {
      kind,
      span: Span {
        start: self.start,
        end: self.current,
      },
    });
    self.start = self.current;
  }

  /// Returns the next character without consuming it, or `None` at end of input.
  fn peek(&self) -> Option<char> {
    if self.is_eof() {
      return None;
    }

    let char = self.source.src[(self.current as usize)..]
      .chars()
      .next()
      .unwrap();

    Some(char)
  }

  /// Returns the character one position ahead of the cursor without advancing it.
  fn peek_next(&self, offset: usize) -> Option<char> {
    if self.is_eof() {
      return None;
    }

    self.source.src[((self.current + offset) as usize)..]
      .chars()
      .next()
  }

  /// Consumes the next character and updates the byte offset and column counters.
  fn advance(&mut self) -> char {
    if self.is_eof() {
      return '\0';
    }

    // get remaining string slice
    let remaining = &self.source.src[self.current..];
    let mut iter = remaining.char_indices();

    // the first character and its byte offset (always 0)
    let (_, ch) = iter.next().unwrap();

    // compute byte offset of next character (to move current forward)
    if let Some((next_byte_idx, _)) = iter.next() {
      self.current += next_byte_idx;
    } else {
      self.current = self.source.src.len();
    }

    // update column count
    self.column += 1;

    ch
  }

  /// Returns the current lexeme slice spanning the active token.
  fn get_current_lexeme(&self) -> &str {
    self.source.src.get(self.start..self.current).unwrap_or("")
  }

  fn get_current_offset(&self) -> usize {
    self.current
  }

  /// Returns `true` when the cursor has reached the end of the source text.
  fn is_eof(&self) -> bool {
    self.current >= self.source.src.len()
  }

  /// Returns the source line corresponding to `line_num`, or an empty string if it is out of range.
  pub fn get_line(&self, line_num: usize) -> String {
    self
      .source
      .src
      .lines()
      .nth(line_num)
      .unwrap_or("")
      .to_string()
  }
}
