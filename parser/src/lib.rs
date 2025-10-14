/*
*
*  The expressions
*
*  expression -> literal
*              | unary
*              | binary
*              | grouping;
*
*  literal    -> NUMBER
*              | STRING
*              | "true"
*              | "false"
*              | "nil";
*
*  grouping   -> "(" expression ")" ;
*
*  unary      -> ("-" | "!") expression
*
*  binary     -> expression operator expression
*
*  operator   -> "+" | "-" | "*" | "/"
*                "!=" | "==" | "<="
*                | ">=" | "<" | ">"
*
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
    while !self.is_at_the_end() {
      let expression = self.parse_addition(engine);
      self.ast.push(expression);
    }

    // Print all parsed expressions as a tree (for debugging / visualization)
    for expression in self.ast.iter() {
      expression.print_tree();
    }
  }

  /// Parses binary expressions involving addition and subtraction.
  /// Example: `a + b - c`
  ///
  /// Grammar rule:
  /// ```text
  /// expression → term ( ( "+" | "-" ) term )* ;
  /// ```
  pub fn parse_addition(&mut self, engine: &mut DiagnosticEngine) -> Expr {
    // Start by parsing the left-hand side (LHS) — a multiplication-level expression.
    let mut lhs = self.parse_multiplication(engine);

    // As long as we see "+" or "-", keep parsing and combining expressions.
    while !self.is_at_the_end() {
      let token = self.get_current_token();
      match token.token_type {
        TokenType::Plus | TokenType::Minus => {
          // Consume the operator
          self.advance();

          // Parse the right-hand side (RHS)
          let rhs = self.parse_multiplication(engine);

          // Combine LHS and RHS into a binary expression tree node
          lhs = Expr::Binary {
            lhs: Box::new(lhs),
            operator: token,
            rhs: Box::new(rhs),
          }
        },
        _ => break, // Stop when there’s no more + or -
      }
    }

    // Return the final (possibly nested) binary expression
    lhs
  }

  /// Parses binary expressions involving multiplication and division.
  /// Example: `a * b / c`
  ///
  /// Grammar rule:
  /// ```text
  /// term → factor ( ( "*" | "/" ) factor )* ;
  /// ```
  pub fn parse_multiplication(&mut self, engine: &mut DiagnosticEngine) -> Expr {
    // Start by parsing the left-hand side (LHS) — a unary-level expression.
    let mut lhs = self.parse_unary(engine);

    // As long as we see "*" or "/", keep parsing and combining expressions.
    while !self.is_at_the_end() {
      let token = self.get_current_token();
      match token.token_type {
        TokenType::Star | TokenType::Divide => {
          // Consume the operator
          self.advance();

          // Parse the right-hand side (RHS)
          let rhs = self.parse_unary(engine);

          // Combine LHS and RHS into a binary expression node
          lhs = Expr::Binary {
            lhs: Box::new(lhs),
            operator: token,
            rhs: Box::new(rhs),
          };
        },
        _ => break, // Stop when there’s no more * or /
      }
    }

    // Return the combined expression
    lhs
  }

  /// Parses unary operators like logical NOT (`!`) and negation (`-`).
  /// Example: `-x`, `!true`, or nested ones like `!!false` or `--5`.
  ///
  /// Grammar rule:
  /// ```text
  /// unary → ( "!" | "-" ) unary | primary ;
  /// ```
  pub fn parse_unary(&mut self, engine: &mut DiagnosticEngine) -> Expr {
    // If we see a unary operator, consume it and recursively parse the operand.
    while !self.is_at_the_end() {
      let token = self.get_current_token();
      match token.token_type {
        TokenType::Minus | TokenType::Bang => {
          // Consume the operator
          self.advance();

          // Recursively parse the operand (right-hand side)
          let rhs = self.parse_unary(engine);

          // Build a Unary AST node
          return Expr::Unary {
            operator: token,
            rhs: Box::new(rhs),
          };
        },
        _ => break, // If not a unary operator, delegate to primary
      }
    }

    // No unary operator found → parse as primary expression
    self.parse_primary(engine)
  }

  /// Parses literals (numbers, strings, booleans) and grouping expressions `(expr)`.
  ///
  /// Grammar rule:
  /// ```text
  /// primary → NUMBER | STRING | "true" | "false" | "(" expression ")" ;
  /// ```
  pub fn parse_primary(&mut self, engine: &mut DiagnosticEngine) -> Expr {
    if self.is_at_the_end() {}

    let token = self.get_current_token();

    match token.token_type {
      // Handle literals like: `123`, `"hello"`, `true`, `false`
      TokenType::String | TokenType::Number | TokenType::True | TokenType::False => {
        // Consume the literal token
        self.advance();

        // Return a literal expression node
        return Expr::Literal(token);
      },

      // Handle grouped expressions: `( expression )`
      TokenType::LeftParen => {
        self.advance(); // Consume the '('

        // Parse the inner expression recursively
        let expr = self.parse_addition(engine);

        // Expect and consume the closing ')'
        if self.get_current_token().token_type == TokenType::RightParen {
          self.advance(); // Consume the ')'
          return Expr::Grouping(Box::new(expr));
        } else {
          // If no closing parenthesis found, that’s a syntax error
          panic!("Expected ')' after expression");
        }
      },

      // If we get here, the token isn’t a valid starting point for an expression
      _ => {
        panic!("Unexpected token in primary expression");
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
