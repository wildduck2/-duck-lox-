use diagnostic::{
  code::DiagnosticCode,
  diagnostic::{Diagnostic, LabelStyle, Span},
  types::error::DiagnosticError,
  DiagnosticEngine,
};
use lexer::token::{Token, TokenKind};

use crate::stmt::Stmt;

mod expr;
mod parser_utils;
mod stmt;

/// Recursive-descent parser that transforms tokens into an AST while reporting diagnostics.
pub struct Parser {
  pub tokens: Vec<Token>,
  pub ast: Vec<Stmt>,
  pub current: usize,
}

impl Parser {
  /// Creates a parser seeded with the lexer output.
  pub fn new(tokens: Vec<Token>) -> Self {
    if tokens.is_empty() {
      panic!("Parser::new: tokens is empty");
    }

    Self {
      tokens,
      ast: Vec::new(),
      current: 0,
    }
  }

  /// Parses the entire token stream, accumulating AST nodes and diagnostics.
  pub fn parse(&mut self, engine: &mut DiagnosticEngine) {
    self.parse_program(engine)
  }

  /// Returns the token at the current cursor position.
  fn current_token(&self) -> Token {
    self.tokens[self.current].clone()
  }

  /// Advances to the next token, emitting an unterminated-string diagnostic if we passed EOF.
  fn advance(&mut self, engine: &mut DiagnosticEngine) {
    if self.is_eof() {
      let current_token = self.current_token();

      let diagnostic = Diagnostic::new(
        DiagnosticCode::Error(DiagnosticError::UnterminatedString),
        "unterminated string".to_string(),
        "demo.lox".to_string(),
      )
      .with_label(
        Span::new(current_token.span.line, 1, current_token.lexeme.len() + 1),
        Some("unterminated string".to_string()),
        LabelStyle::Primary,
      );
      engine.add(diagnostic);
      return;
    }

    self.current += 1;
  }

  /// Reports whether the cursor points at the synthetic EOF token.
  fn is_eof(&self) -> bool {
    self.current == (self.tokens.len() - 1)
  }

  /// Function that consume the code until there's valid tokens to start a new expr
  pub fn synchronize(&mut self, engine: &mut DiagnosticEngine) {
    while !self.is_eof() {
      match self.current_token().kind {
        TokenKind::Semicolon => {
          self.advance(engine);
          break;
        },
        _ => {
          self.advance(engine);
        },
      }
    }
  }

  /// Expects a specific token type and provides detailed error diagnostics if not found
  fn expect(&mut self, expected: TokenKind, engine: &mut DiagnosticEngine) -> Result<Token, ()> {
    if self.is_eof() {
      self.error_expected_token_eof(expected, engine);
      return Err(());
    }

    let current = self.current_token();

    if current.kind == expected {
      self.advance(engine);
      Ok(current)
    } else {
      self.error_expected_token(expected, current, engine);
      Err(())
    }
  }

  /// Error for when we expect a token but hit EOF
  fn error_expected_token_eof(&mut self, expected: TokenKind, engine: &mut DiagnosticEngine) {
    let token = self.current_token();

    let diagnostic = Diagnostic::new(
      DiagnosticCode::Error(DiagnosticError::UnexpectedToken),
      format!("Expected '{:?}', but reached end of file", expected),
      "demo.lox".to_string(),
    )
    .with_label(
      Span::new(token.span.line + 1, 1, token.span.len),
      Some(format!(
        "Expected a primary expression, found \"{}\"",
        token.lexeme
      )),
      LabelStyle::Primary,
    );

    engine.add(diagnostic);
  }

  /// Error for when we expect a token but find something else
  fn error_expected_token(
    &mut self,
    expected: TokenKind,
    found: Token,
    engine: &mut DiagnosticEngine,
  ) {
    let current_token = self.current_token();

    let diagnostic = Diagnostic::new(
      DiagnosticCode::Error(DiagnosticError::UnexpectedToken),
      format!("Expected '{:?}', found '{}'", expected, found.lexeme),
      "demo.lox".to_string(),
    )
    .with_label(
      Span::new(current_token.span.line, 1, current_token.lexeme.len() + 1),
      Some(format!("expected '{:?}' here", expected).into()),
      LabelStyle::Primary,
    )
    .with_help(Parser::get_token_help(&expected, &found));

    engine.add(diagnostic);
  }

  /// Provides contextual help based on what was expected vs found
  fn get_token_help(expected: &TokenKind, found: &Token) -> String {
    match (expected, &found.kind) {
      (TokenKind::Semicolon, _) => "Statements must end with a semicolon".to_string(),
      (TokenKind::RightParen, TokenKind::Semicolon) => {
        "Did you forget to close the parentheses before the semicolon?".to_string()
      },
      (TokenKind::RightBrace, TokenKind::Eof) => {
        "Did you forget to close a block with '}'?".to_string()
      },
      (TokenKind::LeftParen, _) => {
        "Control flow statements require parentheses around conditions".to_string()
      },
      (TokenKind::Colon, TokenKind::Semicolon) => {
        "Ternary expressions use ':' to separate the branches".to_string()
      },
      (TokenKind::Equal, _) => "Use '=' for assignment".to_string(),
      _ => String::new(),
    }
  }

  /// Raises an unexpected-token diagnostic when the parser runs out of input mid-production.
  fn error_eof(&mut self, engine: &mut DiagnosticEngine) {
    let token = self.current_token();

    let diagnostic = Diagnostic::new(
      DiagnosticCode::Error(DiagnosticError::UnexpectedToken),
      format!("Unexpected token {:?}", token.lexeme),
      "duck.lox".to_string(),
    )
    .with_label(
      Span::new(token.span.line + 1, 1, token.span.len),
      Some(format!(
        "Expected a primary expression, found \"{}\"",
        token.lexeme
      )),
      LabelStyle::Primary,
    );

    engine.add(diagnostic);
  }
}
