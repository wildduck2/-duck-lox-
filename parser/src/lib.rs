/*
* expression   → comma ;
* comma        → ternary ( "," ternary )* ;
* ternary      → assignment ( "?" expression ":" ternary )? ;
* assignment   → IDENTIFIER "=" assignment
*               | equality ;
* equality     → comparison ( ( "!=" | "==" ) comparison )* ;
* comparison   → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
* term         → factor ( ( "-" | "+" ) factor )* ;
* factor       → unary ( ( "/" | "*" ) unary )* ;
* unary        → ( "!" | "-" ) unary
*               | primary ;
* primary      → NUMBER | STRING | IDENTIFIER
*               | "true" | "false" | "nil"
*               | "(" expression ")" ;
*/

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

  /// Function that takes the tokens produced by the "Scanner" and returns an "AST".
  /// This is the **main entry point** that starts the parsing process.
  /// It repeatedly parses top-level expressions until the end of the token list.
  pub fn parse(&mut self, engine: &mut DiagnosticEngine) {
    // Start parsing an expression (lowest precedence is addition)
    while !self.is_eof() {
      let expression = self.parse_expression(engine);
      self.ast.push(expression);
    }

    // Print all parsed expressions as a tree (for debugging / visualization)
    for expression in self.ast.iter() {
      expression.print_tree();
    }
  }

  fn parse_expression(&mut self, engine: &mut DiagnosticEngine) -> Expr {
    return self.parse_coma(engine);
  }

  fn parse_coma(&mut self, engine: &mut DiagnosticEngine) -> Expr {
    let mut lhs = self.parse_ternary(engine);

    while !self.is_eof() {
      let token = self.get_current_token();

      match token.token_type {
        TokenType::Comma => {
          self.advance();

          let rhs = self.parse_ternary(engine);

          lhs = Expr::Binary {
            lhs: Box::new(lhs),
            operator: token,
            rhs: Box::new(rhs),
          }
        },
        _ => break,
      }
    }

    lhs
  }

  // * ternary      → assignment ( "?" expression ":" ternary )? ;
  fn parse_ternary(&mut self, engine: &mut DiagnosticEngine) -> Expr {
    let considtion = self.parse_assignment(engine);

    if !self.is_eof() && self.get_current_token().token_type == TokenType::Question {
      self.advance(); // consume the (?)

      let then_branch = self.parse_expression(engine);
      if self.is_eof() && self.get_current_token().token_type != TokenType::Colon {
        panic!("Expected ':' after then branch of ternary expression");
      }

      self.advance(); // consume the (:)
      let else_branch = self.parse_ternary(engine);
      return Expr::Ternary {
        condition: Box::new(considtion),
        then_branch: Box::new(then_branch),
        else_branch: Box::new(else_branch),
      };
    }

    considtion
  }

  fn parse_assignment(&mut self, engine: &mut DiagnosticEngine) -> Expr {
    let lhs = self.parse_equality(engine);

    if !self.is_eof() && self.get_current_token().token_type == TokenType::Equal {
      self.advance(); // consume the (=)
      let rhs = self.parse_assignment(engine);

      if let Expr::Identifier(name) = lhs {
        return Expr::Assign {
          name: name,
          value: Box::new(rhs),
        };
      } else {
        panic!("Invalid right hand assignment");
      }
    }
    lhs
  }

  fn parse_equality(&mut self, engine: &mut DiagnosticEngine) -> Expr {
    let mut lhs = self.parse_comparison(engine);
    while !self.is_eof() {
      let token = self.get_current_token();

      match token.token_type {
        TokenType::Equal | TokenType::BangEqual => {
          self.advance(); // consume (!=|==)
          let rhs = self.parse_comparison(engine);

          lhs = Expr::Binary {
            lhs: Box::new(lhs),
            operator: token,
            rhs: Box::new(rhs),
          }
        },
        _ => break,
      }
    }

    lhs
  }

  fn parse_comparison(&mut self, engine: &mut DiagnosticEngine) -> Expr {
    let mut lhs = self.parse_term(engine);
    while !self.is_eof() {
      let token = self.get_current_token();

      match token.token_type {
        TokenType::GreaterEqual | TokenType::Greater | TokenType::LessEqual | TokenType::Less => {
          self.advance(); // consume (<=|<|>|>=)
          let rhs = self.parse_term(engine);

          lhs = Expr::Binary {
            lhs: Box::new(lhs),
            operator: token,
            rhs: Box::new(rhs),
          }
        },
        _ => break,
      }
    }

    lhs
  }

  fn parse_term(&mut self, engine: &mut DiagnosticEngine) -> Expr {
    let mut lhs = self.parse_factor(engine);
    while !self.is_eof() {
      let token = self.get_current_token();

      match token.token_type {
        TokenType::Minus | TokenType::Plus => {
          self.advance(); // consume (+|-)
          let rhs = self.parse_factor(engine);

          lhs = Expr::Binary {
            lhs: Box::new(lhs),
            operator: token,
            rhs: Box::new(rhs),
          }
        },
        _ => break,
      }
    }

    lhs
  }

  fn parse_factor(&mut self, engine: &mut DiagnosticEngine) -> Expr {
    let mut lhs = self.parse_unary(engine);
    while !self.is_eof() {
      let token = self.get_current_token();

      match token.token_type {
        TokenType::Star | TokenType::Divide => {
          self.advance(); // Consume (*|/)
          let rhs = self.parse_unary(engine);
          lhs = Expr::Binary {
            lhs: Box::new(lhs),
            operator: token,
            rhs: Box::new(rhs),
          }
        },
        _ => break,
      }
    }
    lhs
  }

  fn parse_unary(&mut self, engine: &mut DiagnosticEngine) -> Expr {
    while !self.is_eof() {
      let token = self.get_current_token();
      match token.token_type {
        TokenType::Bang | TokenType::Minus => {
          self.advance();
          let rhs = self.parse_primary(engine);
          return Expr::Unary {
            operator: token,
            rhs: Box::new(rhs),
          };
        },
        _ => {
          break;
        },
      }
    }
    self.parse_primary(engine)
  }

  fn parse_primary(&mut self, engine: &mut DiagnosticEngine) -> Expr {
    if self.is_eof() {
      panic!("we are EOF");
    }

    let token = self.get_current_token();

    match token.token_type {
      TokenType::String
      | TokenType::Number
      | TokenType::Nil
      | TokenType::True
      | TokenType::False => {
        self.advance(); // consume the literal
        return Expr::Literal(token);
      },

      TokenType::LeftParen => {
        self.advance(); // Consume "("

        let expr = self.parse_expression(engine);

        if self.is_eof() || self.get_current_token().token_type != TokenType::RightParen {
          panic!("Expected ')' after expression");
        }

        self.advance(); // Consume ")"

        return Expr::Grouping(Box::new(expr));
      },

      TokenType::Identifier => {
        self.advance(); // consume the identifier
        return Expr::Identifier(token);
      },

      _ => {
        panic!("")
      },
    }
  }

  /// Function that gets the current token and return it with clone to remove the reference
  pub fn get_current_token(&mut self) -> Token {
    self.tokens[self.current].clone()
  }

  /// Function that returns Some(pointer +1) without shiting the pointer
  fn peek(&self) -> Option<&Token> {
    if self.is_eof() {
      return None;
    }
    Some(&self.tokens[self.current])
  }

  /// Function that shifts the pointer +1 and return it
  fn advance(&mut self) -> Option<&Token> {
    if self.is_eof() {
      return None;
    }

    self.current += 1;
    Some(&self.tokens[self.current])
  }

  /// Function that returns true if we are the end of the vec
  fn is_eof(&self) -> bool {
    if self.current == self.tokens.len() - 1 {
      return true;
    }

    false
  }
}
