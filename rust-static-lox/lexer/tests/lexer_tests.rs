#[cfg(test)]
mod lexer_tests {

  use diagnostic::{DiagnosticEngine, SourceMap};
  use lexer::{
    token::{DocStyle, TokenKind},
    Lexer,
  };

  #[test]
  fn test_whitespaces() {
    let mut engine = DiagnosticEngine::new();
    let mut source_map = SourceMap::new();

    source_map.add_wd("tests/files").unwrap();

    let mut lexer = Lexer::new(
      source_map
        .get("tests/files/whitespace.lox")
        .unwrap()
        .src
        .clone(),
    );
    lexer.scan_tokens(&mut engine);

    assert_eq!(lexer.tokens.len(), 1);
    assert_eq!(engine.has_errors(), false);
    assert_eq!(engine.has_warnings(), false);
  }

  #[test]
  fn test_comments() {
    let mut engine = DiagnosticEngine::new();
    let mut source_map = SourceMap::new();

    source_map.add_wd("tests/files").unwrap();

    let mut lexer = Lexer::new(
      source_map
        .get("tests/files/comments.lox")
        .unwrap()
        .src
        .clone(),
    );
    lexer.scan_tokens(&mut engine);

    // ensure no diagnostics
    assert!(!engine.has_errors(), "Lexer produced unexpected errors");
    assert!(!engine.has_warnings(), "Lexer produced unexpected warnings");

    // final EOF token check
    let last = lexer.tokens.last().unwrap();
    assert!(matches!(last.kind, TokenKind::Eof));
  }
}
