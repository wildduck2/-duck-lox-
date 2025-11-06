use crate::{
  token::{DocStyle, TokenKind},
  Lexer,
};

impl Lexer {
  // Consumes a single-line `//` comment and returns its token.
  pub fn lex_line_comment(&mut self) -> Option<TokenKind> {
    self.advance(); // consume the '/'

    let doc_style = if self.match_char('/') {
      self.advance(); // consume the '/'
      Some(DocStyle::Inner)
    } else if self.match_char('!') {
      self.advance(); // consume the '!'
      Some(DocStyle::Outer)
    } else {
      None
    };

    while !self.is_eof() {
      self.advance(); // consume the current char
      if self.match_char('\n') {
        break;
      }
    }
    Some(TokenKind::LineComment { doc_style })
  }

  /// Consumes a block `/* ... */` comment and returns its token.
  pub fn lex_multi_line_comment(&mut self) -> Option<TokenKind> {
    self.advance(); // consume '/'
    self.advance(); // consume '*'

    // Detect Rust-style doc comments: /*! ... */ (Outer) or /** ... */ (Inner)
    let doc_style = match self.peek() {
      Some('!') => {
        self.advance(); // consume '!'
        Some(DocStyle::Outer)
      },
      Some('*') => {
        self.advance(); // consume second '*'
        Some(DocStyle::Inner)
      },
      _ => None,
    };

    let mut terminated = false;
    let mut depth = 1; // track nested comment depth

    while !self.is_eof() {
      let current = self.peek();
      let next = self.peek_next(1);

      // Handle newlines
      if current == Some('\n') {
        self.line += 1;
      }

      // Detect nested comment start "/*"
      if current == Some('/') && next == Some('*') {
        self.advance(); // consume '/'
        self.advance(); // consume '*'
        depth += 1;
        continue;
      }

      // Detect comment end "*/"
      if current == Some('*') && next == Some('/') {
        self.advance(); // consume '*'
        self.advance(); // consume '/'
        depth -= 1;

        if depth == 0 {
          terminated = true;
          break;
        }
        continue;
      }

      self.advance(); // consume any other char
    }

    Some(TokenKind::BlockComment {
      doc_style,
      terminated,
    })
  }
}
