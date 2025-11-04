#[cfg(test)]
mod lexer_tests {

  use diagnostic::DiagnosticEngine;
  use lexer::{token::TokenKind, Lexer};

  fn lex_source(source: &str) -> (Vec<TokenKind>, Vec<String>, usize) {
    let mut lexer = Lexer::new(source.to_string());
    let mut engine = DiagnosticEngine::new();
    engine.insert_source(source.to_string());

    lexer.scan_tokens(&mut engine);

    let kinds = lexer.tokens.iter().map(|token| token.kind).collect();
    let lexemes = lexer
      .tokens
      .iter()
      .map(|token| token.lexeme.clone())
      .collect();
    let error_count = engine.error_count();

    (kinds, lexemes, error_count)
  }

  #[test]
  fn lexes_simple_expression() {
    let (kinds, _lexemes, errors) = lex_source("let count = 42;");

    assert_eq!(errors, 0, "expected no diagnostics for simple statement");
    assert_eq!(
      kinds,
      vec![
        TokenKind::Let,
        TokenKind::Identifier,
        TokenKind::Equal,
        TokenKind::IntegerLiteral,
        TokenKind::Semicolon,
        TokenKind::Eof
      ]
    );
  }

  #[test]
  fn lexes_boolean_keywords() {
    let (kinds, _lexemes, errors) = lex_source("true false");

    assert_eq!(errors, 0, "boolean keywords should not emit diagnostics");
    assert_eq!(
      kinds,
      vec![
        TokenKind::TrueLiteral,
        TokenKind::FalseLiteral,
        TokenKind::Eof
      ]
    );
  }

  #[test]
  fn lexes_string_literal() {
    let (kinds, lexemes, errors) = lex_source("\"hello\"");

    assert_eq!(errors, 0, "well-formed string should not emit diagnostics");
    assert_eq!(kinds, vec![TokenKind::StringLiteral, TokenKind::Eof]);
    assert_eq!(
      lexemes[0], "\"hello\"",
      "string literal should include the surrounding quotes"
    );
    assert_eq!(lexemes[1], "", "EOF token should have an empty lexeme");
  }

  #[test]
  fn reports_invalid_character() {
    let (kinds, _lexemes, errors) = lex_source("@");

    assert_eq!(errors, 1, "invalid character should trigger a diagnostic");
    assert_eq!(
      kinds,
      vec![TokenKind::Eof],
      "only EOF token should be emitted"
    );
  }
}
