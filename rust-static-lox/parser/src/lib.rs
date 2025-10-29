use lexer::token::Token;

pub struct Parser<'a> {
  pub tokens: Vec<Token<'a>>,
  pub ast: Vec<String>,
}

impl<'a> Parser<'a> {
  pub fn new() -> Self {
    Self {
      tokens: Vec::new(),
      ast: Vec::new(),
    }
  }

  pub fn parse(&mut self) {}
}
