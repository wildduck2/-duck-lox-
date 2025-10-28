#[cfg(test)]
mod tests {
  use scanner::{lox::Lox, token::types::TokenType, Scanner};

  use super::*;

  #[test]
  fn test_identifier_and_equal() {
    let mut lox = Lox::new();
    let mut scanner = Scanner::new();
    scanner.run_file(
      String::from("tests/files/test_identifier_and_equal.duck"),
      &mut lox,
    );
    let tokens = scanner.tokens;

    assert_eq!(tokens[0].token_type, TokenType::Var);
    assert_eq!(tokens.last().unwrap().token_type, TokenType::Eof);
  }
}
