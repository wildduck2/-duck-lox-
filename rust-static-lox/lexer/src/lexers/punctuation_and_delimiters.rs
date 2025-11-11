//! Lexers for punctuation and delimiter tokens.
//!
//! Handles single-character tokens like semicolons, commas, brackets,
//! and multi-character sequences like `::` and `..=`.

use crate::{token::TokenKind, Lexer};
use diagnostic::DiagnosticEngine;

impl Lexer {
  /// Lexes a semicolon (`;`).
  ///
  /// Used to terminate statements and separate items.
  ///
  /// # Returns
  ///
  /// `Some(TokenKind::Semi)`
  pub fn lex_semicolon(&mut self) -> Option<TokenKind> {
    Some(TokenKind::Semi)
  }

  /// Lexes a comma (`,`).
  ///
  /// Used to separate items in lists, function arguments, etc.
  ///
  /// # Returns
  ///
  /// `Some(TokenKind::Comma)`
  pub fn lex_comma(&mut self) -> Option<TokenKind> {
    Some(TokenKind::Comma)
  }

  /// Lexes a dot (`.`) or dot sequences (`..`, `..=`).
  ///
  /// Handles:
  /// - `.` - Member access or decimal point
  /// - `..` - Range operator
  /// - `..=` - Inclusive range operator
  ///
  /// # Returns
  ///
  /// `Some(TokenKind::Dot)`, `Some(TokenKind::DotDot)`, or `Some(TokenKind::DotDotEq)`
  pub fn lex_dot(&mut self) -> Option<TokenKind> {
    if self.match_char('.') {
      if self.match_char('=') {
        return Some(TokenKind::DotDotEq);
      }

      return Some(TokenKind::DotDot);
    }

    Some(TokenKind::Dot)
  }

  /// Lexes an open parenthesis (`(`).
  ///
  /// Used for grouping expressions, function calls, and tuples.
  ///
  /// # Returns
  ///
  /// `Some(TokenKind::OpenParen)`
  pub fn lex_open_paren(&mut self) -> Option<TokenKind> {
    Some(TokenKind::OpenParen)
  }

  /// Lexes a close parenthesis (`)`).
  ///
  /// Closes grouping expressions, function calls, and tuples.
  ///
  /// # Returns
  ///
  /// `Some(TokenKind::CloseParen)`
  pub fn lex_close_paren(&mut self) -> Option<TokenKind> {
    Some(TokenKind::CloseParen)
  }

  /// Lexes an open brace (`{`).
  ///
  /// Used for blocks, struct literals, and match arms.
  ///
  /// # Returns
  ///
  /// `Some(TokenKind::OpenBrace)`
  pub fn lex_open_brace(&mut self) -> Option<TokenKind> {
    Some(TokenKind::OpenBrace)
  }

  /// Lexes a close brace (`}`).
  ///
  /// Closes blocks, struct literals, and match arms.
  ///
  /// # Returns
  ///
  /// `Some(TokenKind::CloseBrace)`
  pub fn lex_close_brace(&mut self) -> Option<TokenKind> {
    Some(TokenKind::CloseBrace)
  }

  /// Lexes an open bracket (`[`).
  ///
  /// Used for array literals and indexing expressions.
  ///
  /// # Returns
  ///
  /// `Some(TokenKind::OpenBracket)`
  pub fn lex_open_bracket(&mut self) -> Option<TokenKind> {
    Some(TokenKind::OpenBracket)
  }

  /// Lexes a close bracket (`]`).
  ///
  /// Closes array literals and indexing expressions.
  ///
  /// # Returns
  ///
  /// `Some(TokenKind::CloseBracket)`
  pub fn lex_close_bracket(&mut self) -> Option<TokenKind> {
    Some(TokenKind::CloseBracket)
  }

  /// Lexes an at symbol (`@`).
  ///
  /// Used for pattern bindings and attributes.
  ///
  /// # Returns
  ///
  /// `Some(TokenKind::At)`
  pub fn lex_at(&mut self) -> Option<TokenKind> {
    Some(TokenKind::At)
  }

  /// Lexes a pound symbol (`#`) or shebang (`#!`).
  ///
  /// Handles:
  /// - `#` - Attribute or pound token
  /// - `#!` - Shebang line (only valid at file start)
  ///
  /// # Arguments
  ///
  /// * `engine` - Diagnostic engine for error reporting
  ///
  /// # Returns
  ///
  /// `Some(TokenKind::Pound)` or `Some(TokenKind::Shebang)`, or `None` for invalid shebang
  pub fn lex_pound(&mut self, engine: &mut DiagnosticEngine) -> Option<TokenKind> {
    //FIX: the shebang must be at the start of the file
    if self.column == 0 && self.match_char('!') {
      return self.lex_shebang(engine);
    }

    Some(TokenKind::Pound)
  }

  /// Lexes a tilde symbol (`~`).
  ///
  /// Used for bitwise NOT operations.
  ///
  /// # Returns
  ///
  /// `Some(TokenKind::Tilde)`
  pub fn lex_tilde(&mut self) -> Option<TokenKind> {
    Some(TokenKind::Tilde)
  }

  /// Lexes a question mark (`?`).
  ///
  /// Used for optional types and error propagation.
  ///
  /// # Returns
  ///
  /// `Some(TokenKind::Question)`
  pub fn lex_question(&mut self) -> Option<TokenKind> {
    Some(TokenKind::Question)
  }

  /// Lexes a colon (`:`) or path separator (`::`).
  ///
  /// Handles:
  /// - `:` - Type annotations, match patterns
  /// - `::` - Path separator (e.g., `std::io`)
  ///
  /// # Returns
  ///
  /// `Some(TokenKind::Colon)` or `Some(TokenKind::ColonColon)`
  pub fn lex_colon(&mut self) -> Option<TokenKind> {
    if self.match_char(':') {
      return Some(TokenKind::ColonColon);
    }

    Some(TokenKind::Colon)
  }

  /// Lexes a dollar symbol (`$`).
  ///
  /// Used in macros and template contexts.
  ///
  /// # Returns
  ///
  /// `Some(TokenKind::Dollar)`
  pub fn lex_dollar(&mut self) -> Option<TokenKind> {
    Some(TokenKind::Dollar)
  }
}
