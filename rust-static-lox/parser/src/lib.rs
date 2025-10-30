use diagnostic::{
  code::DiagnosticCode,
  diagnostic::{Diagnostic, LabelStyle, Span},
  types::error::DiagnosticError,
  DiagnosticEngine,
};
use lexer::token::{Token, TokenKind};
mod expr;
mod stmt;

pub struct Parser {
  pub tokens: Vec<Token>,
  pub ast: Vec<String>,
  pub current: usize,
}

impl Parser {
  pub fn new() -> Self {
    Self {
      tokens: Vec::new(),
      ast: Vec::new(),
      current: 0,
    }
  }

  pub fn parse(&mut self) {}

  fn current_token(&mut self) -> Token {
    self.tokens[self.current].clone()
  }

  fn advance(&mut self, engine: &mut DiagnosticEngine) {
    if self.is_eof() {
      let current_token = self.current_token();

      let diagnostic = Diagnostic::new(
        DiagnosticCode::Error(DiagnosticError::UnterminatedString),
        "unterminated string".to_string(),
        "demo.lox".to_string(),
      )
      .with_context_line(current_token.span.line, current_token.lexeme.clone())
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

  fn is_eof(&self) -> bool {
    self.current >= self.tokens.len() - 1
  }

  /// Error for when we expect a token but find something else
  fn error_expected_token(
    &mut self,
    expected: TokenKind,
    found: Token,
    engine: &mut DiagnosticEngine,
  ) {
    let current_token = self.current_token();

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
    );

    engine.add(diagnostic);
  }
}
/// Helper function to convert TokenType to a readable string

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
