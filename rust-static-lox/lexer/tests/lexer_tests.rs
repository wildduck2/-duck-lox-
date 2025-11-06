#[cfg(test)]
mod lexer_tests {
  use diagnostic::{DiagnosticEngine, SourceMap};
  use lexer::{
    token::{Base, LiteralKind, TokenKind},
    Lexer,
  };

  fn lex_test_file(file: &str) -> (Lexer, DiagnosticEngine) {
    let mut engine = DiagnosticEngine::new();
    let mut source_map = SourceMap::new();

    source_map.add_wd("tests/files").unwrap();
    let mut lexer = Lexer::new(source_map.get(file).unwrap().src.clone());
    lexer.scan_tokens(&mut engine);
    (lexer, engine)
  }

  #[test]
  fn test_whitespaces() {
    let (lexer, engine) = lex_test_file("tests/files/whitespace.lox");

    assert_eq!(lexer.tokens.len(), 1);
    assert!(!engine.has_errors(), "Lexer produced unexpected errors");
    assert!(!engine.has_warnings(), "Lexer produced unexpected warnings");
  }

  #[test]
  fn test_comments() {
    let (lexer, engine) = lex_test_file("tests/files/comments.lox");

    assert!(!engine.has_errors(), "Lexer produced unexpected errors");
    assert!(!engine.has_warnings(), "Lexer produced unexpected warnings");

    let last = lexer.tokens.last().unwrap();
    assert!(matches!(last.kind, TokenKind::Eof));
  }

  #[test]
  fn test_numbers() {
    let (lexer, engine) = lex_test_file("tests/files/numbers.lox");

    assert!(!engine.has_errors(), "Lexer produced unexpected errors");
    assert!(!engine.has_warnings(), "Lexer produced unexpected warnings");

    let tokens = &lexer.tokens;

    // ✅ EOF
    let last = tokens.last().unwrap();
    assert!(matches!(last.kind, TokenKind::Eof));

    // ✅ Base checks
    assert!(
      tokens.iter().any(|t| matches!(
        t.kind,
        TokenKind::Literal {
          kind: LiteralKind::Int {
            base: Base::Binary,
            ..
          },
          ..
        }
      )),
      "Missing binary literal"
    );

    assert!(
      tokens.iter().any(|t| matches!(
        t.kind,
        TokenKind::Literal {
          kind: LiteralKind::Int {
            base: Base::Octal,
            ..
          },
          ..
        }
      )),
      "Missing octal literal"
    );

    assert!(
      tokens.iter().any(|t| matches!(
        t.kind,
        TokenKind::Literal {
          kind: LiteralKind::Int {
            base: Base::Hexadecimal,
            ..
          },
          ..
        }
      )),
      "Missing hex literal"
    );

    assert!(
      tokens.iter().any(|t| matches!(
        t.kind,
        TokenKind::Literal {
          kind: LiteralKind::Float {
            base: Base::Decimal,
            ..
          },
          ..
        }
      )),
      "Missing float literal"
    );

    // ✅ Empty int detection
    assert!(
      tokens.iter().any(|t| matches!(
        t.kind,
        TokenKind::Literal {
          kind: LiteralKind::Int {
            base: Base::Hexadecimal,
            empty_int: true
          },
          ..
        }
      )),
      "Empty 0x not marked correctly"
    );

    // ✅ Empty exponent detection
    assert!(
      tokens.iter().any(|t| matches!(
        t.kind,
        TokenKind::Literal {
          kind: LiteralKind::Float {
            empty_exponent: true,
            ..
          },
          ..
        }
      )),
      "Malformed exponents not detected"
    );

    // ✅ Suffix split check (Float followed by Ident)
    let mut found_suffix_pair = false;
    for win in tokens.windows(2) {
      if matches!(
        win[0].kind,
        TokenKind::Literal {
          kind: LiteralKind::Float { .. },
          ..
        }
      ) && matches!(win[1].kind, TokenKind::Ident)
      {
        found_suffix_pair = true;
        break;
      }
    }
    assert!(
      found_suffix_pair,
      "Float + suffix identifier pair (like 1.0f32) not found"
    );
  }
}
