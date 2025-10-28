#[cfg(test)]
mod tests {
  use scanner::{lox::Lox, token::types::TokenType, Scanner};
  #[test]
  fn test_complex_scanner() {
    let mut lox = Lox::new();
    let mut scanner = Scanner::new();
    scanner.run_file(String::from("tests/files/test_complex.duck"), &mut lox);

    let tokens = scanner.tokens;

    // --- structure ---
    assert_eq!(tokens.first().unwrap().token_type, TokenType::Var);
    assert_eq!(tokens.last().unwrap().token_type, TokenType::Eof);

    // --- operators ---
    for op in [
      TokenType::Plus,
      TokenType::Minus,
      TokenType::Multiply,
      TokenType::Divide,
      TokenType::EqualEqual,
      TokenType::BangEqual,
      TokenType::GreaterEqual,
      TokenType::PlusPlus,
      TokenType::MinusMinus,
    ] {
      assert!(
        tokens.iter().any(|t| t.token_type == op),
        "Missing operator {:?}",
        op
      );
    }

    // --- keywords ---
    for keyword in [
      TokenType::Var,
      TokenType::Fun,
      TokenType::If,
      TokenType::Else,
      TokenType::For,
      TokenType::Return,
      TokenType::True,
      TokenType::Nil,
      TokenType::Print,
    ] {
      assert!(
        tokens.iter().any(|t| t.token_type == keyword),
        "Missing keyword {:?}",
        keyword
      );
    }

    // --- identifiers ---
    for name in ["result", "add", "flag"] {
      assert!(
        tokens.iter().any(|t| t.lexeme == name),
        "Missing identifier {:?}",
        name
      );
    }

    // --- string literal presence ---
    assert!(
      tokens.iter().any(|t| t.token_type == TokenType::String),
      "No string literal found"
    );

    println!("Complex scanner test passed with {} tokens", tokens.len());
  }
}
