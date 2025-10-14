use std::panic;

use diagnostic::{
  diagnostic::{Diagnostic, Label, Span},
  diagnostic_code::DiagnosticCode,
  DiagnosticEngine,
};
use scanner::token::{
  types::{Literal, TokenType},
  Token,
};

use crate::expression::Expr;

mod expression;

pub struct Parser {
  /// The tokens preduced by the scanner
  pub tokens: Vec<Token>,
  /// The pointer to the current token we are looking at
  pub current: usize,
  /// List of expressions
  pub ast: Vec<Expr>,
}

impl Parser {
  /// Function to init a new struct
  pub fn new(tokens: Vec<Token>) -> Self {
    Self {
      tokens,
      current: 0,
      ast: vec![],
    }
  }

  /// Function that takes the tokens preduced by the "Scanner" and returns an "AST"
  pub fn parse(&mut self, engine: &mut DiagnosticEngine) {
    let expression = self.parse_expression(engine);
    self.ast.push(expression);

    if !self.is_at_the_end() {
      let expression = self.parse_expression(engine);
      self.ast.push(expression);
    }

    for expression in self.ast.iter() {
      expression.print_tree();
    }
  }

  fn parse_expression(&mut self, engine: &mut DiagnosticEngine) -> Expr {
    let mut left = self.parse_term(engine);

    while !self.is_at_the_end() {
      let token = self.get_current_token();
      match token.token_type {
        TokenType::Plus | TokenType::Minus => {
          self.advance();
          let right = self.parse_term(engine);
          left = Expr::Binary {
            left: Box::new(left),
            operator: token,
            right: Box::new(right),
          };
        },
        _ => break,
      }
    }

    return left;
  }

  fn parse_term(&mut self, engine: &mut DiagnosticEngine) -> Expr {
    let mut lhs = self.parse_factor(engine);

    while !self.is_at_the_end() {
      let token = self.get_current_token();
      match token.token_type {
        TokenType::Star | TokenType::Divide => {
          self.advance();

          let rhs = self.parse_factor(engine);

          lhs = Expr::Binary {
            left: Box::new(lhs),
            operator: token,
            right: Box::new(rhs),
          };
        },
        _ => break,
      }
    }
    return lhs;
  }

  fn parse_factor(&mut self, engine: &mut DiagnosticEngine) -> Expr {
    if self.is_at_the_end() {
      panic!("EOF");
    }

    let token = self.get_current_token();

    match token.token_type {
      TokenType::Number => {
        self.advance();
        return Expr::Literal(token);
      },
      TokenType::LeftParen | TokenType::RightParen => {
        self.advance();
        let expr = self.parse_expression(engine);
        self.advance();
        return Expr::Grouping(Box::new(expr));
      },
      _ => {
        panic!("Wrong token 1");
      },
    }
  }

  /// Function that gets the current token and return it with clone to remove the reference
  pub fn get_current_token(&mut self) -> Token {
    self.tokens[self.current].clone()
  }

  /// Function that returns Some(pointer +1) without shiting the pointer
  fn peek(&self) -> Option<&Token> {
    if self.is_at_the_end() {
      return None;
    }
    Some(&self.tokens[self.current])
  }

  /// Function that will return the last token without shifting the pointer
  fn previous(&self) -> Option<&Token> {
    if self.is_at_the_end() {
      return None;
    }
    Some(&self.tokens[self.current - 1])
  }

  /// Function that shifts the pointer +1 and return it
  fn advance(&mut self) -> Option<&Token> {
    if self.is_at_the_end() {
      return None;
    }

    self.current += 1;
    Some(&self.tokens[self.current])
  }

  /// Function that returns true if we are the end of the vec
  fn is_at_the_end(&self) -> bool {
    if self.current == self.tokens.len() - 1 {
      return true;
    }

    false
  }
}
