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

use diagnostic::{
  diagnostic::{Diagnostic, Label, Span},
  diagnostic_code::DiagnosticCode,
  DiagnosticEngine,
};
use scanner::token::{types::TokenType, Token};

use crate::expression::Expr;

pub mod expression;

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

  pub fn parse(&mut self, engine: &mut DiagnosticEngine) {
    while !self.is_eof() {
      match self.parse_expression(engine) {
        Ok(expr) => {
          expr.print_tree();
          self.ast.push(expr);
        },
        Err(_) => self.synchronize(),
      }
    }
  }

  fn parse_expression(&mut self, engine: &mut DiagnosticEngine) -> Result<Expr, ()> {
    self.parse_comma(engine)
  }

  // * expression   → comma ;
  // * comma        → ternary ( "," ternary )* ;

  fn parse_comma(&mut self, engine: &mut DiagnosticEngine) -> Result<Expr, ()> {
    let mut lhs = self.parse_ternary(engine)?;

    while !self.is_eof() {
      let token = self.current_token();

      match token.token_type {
        TokenType::Comma => {
          self.advance();

          let rhs = self.parse_ternary(engine)?;

          lhs = Expr::Binary {
            lhs: Box::new(lhs),
            operator: token,
            rhs: Box::new(rhs),
          };
        },
        _ => break,
      }
    }

    Ok(lhs)
  }

  /// Function that handles the assignments (=)
  fn parse_ternary(&mut self, engine: &mut DiagnosticEngine) -> Result<Expr, ()> {
    let condition = self.parse_assignment(engine)?;

    if !self.is_eof() && self.current_token().token_type == TokenType::Question {
      let question_token = self.current_token();
      self.advance(); // consume the (?)

      let then_branch = self.parse_expression(engine)?;

      if self.is_eof() || self.current_token().token_type != TokenType::Colon {
        let current_token = self.current_token();

        let error = Diagnostic::new(
          DiagnosticCode::UnexpectedToken,
          format!(
            "Expected ':' in ternary expression, found '{}'",
            current_token.lexeme
          ),
        )
        .with_label(Label::primary(
          current_token.to_span(),
          Some("expected ':' before this token".to_string()),
        ))
        .with_label(Label::secondary(
          Token::to_span_with_token(question_token),
          Some("ternary started here".to_string()),
        ))
        .with_help(
          "Ternary expressions require the format: condition ? then_value : else_value".to_string(),
        );

        engine.emit(error);
        return Err(());
      }

      self.advance(); // consume the (:)
      let else_branch = self.parse_ternary(engine)?;

      return Ok(Expr::Ternary {
        condition: Box::new(condition),
        then_branch: Box::new(then_branch),
        else_branch: Box::new(else_branch),
      });
    }

    Ok(condition)
  }

  /// Function that handles the assignments (=)
  fn parse_assignment(&mut self, engine: &mut DiagnosticEngine) -> Result<Expr, ()> {
    let lhs = self.parse_equality(engine)?;

    if !self.is_eof() && self.current_token().token_type == TokenType::Equal {
      self.advance();

      let rhs = self.parse_assignment(engine)?;

      if let Expr::Identifier(name) = lhs {
        return Ok(Expr::Assign {
          name: name,
          value: Box::new(rhs),
        });
      } else {
        return Err(());
      }
    }

    Ok(lhs)
  }

  /// Function that handles the terms (==|!=)
  fn parse_equality(&mut self, engine: &mut DiagnosticEngine) -> Result<Expr, ()> {
    let mut lhs = self.parse_comparison(engine)?;

    while !self.is_eof() {
      let token = self.current_token();

      match token.token_type {
        TokenType::EqualEqual | TokenType::BangEqual => {
          self.advance();

          let rhs = self.parse_comparison(engine)?;

          lhs = Expr::Binary {
            lhs: Box::new(lhs),
            operator: token,
            rhs: Box::new(rhs),
          };
        },
        _ => break,
      }
    }

    Ok(lhs)
  }

  /// Function that handles the terms (<|<=|>=|>)
  fn parse_comparison(&mut self, engine: &mut DiagnosticEngine) -> Result<Expr, ()> {
    let mut lhs = self.parse_term(engine)?;

    while !self.is_eof() {
      let token = self.current_token();

      match token.token_type {
        TokenType::Greater | TokenType::GreaterEqual | TokenType::Less | TokenType::LessEqual => {
          self.advance();

          let rhs = self.parse_term(engine)?;

          lhs = Expr::Binary {
            lhs: Box::new(lhs),
            operator: token,
            rhs: Box::new(rhs),
          };
        },
        _ => break,
      }
    }

    Ok(lhs)
  }

  /// Function that handles the terms (+|-)
  fn parse_term(&mut self, engine: &mut DiagnosticEngine) -> Result<Expr, ()> {
    let mut lhs = self.parse_factor(engine)?;

    while !self.is_eof() {
      let token = self.current_token();

      match token.token_type {
        TokenType::Minus | TokenType::Plus => {
          self.advance();

          let rhs = self.parse_factor(engine)?;

          lhs = Expr::Binary {
            lhs: Box::new(lhs),
            operator: token,
            rhs: Box::new(rhs),
          };
        },
        _ => break,
      }
    }

    Ok(lhs)
  }

  /// Function that handles the factors (*|/)
  fn parse_factor(&mut self, engine: &mut DiagnosticEngine) -> Result<Expr, ()> {
    let mut lhs = self.parse_unary(engine)?;

    while !self.is_eof() {
      let token = self.current_token();

      match token.token_type {
        TokenType::Divide | TokenType::Multiply => {
          self.advance();

          let rhs = self.parse_unary(engine)?;

          lhs = Expr::Binary {
            lhs: Box::new(lhs),
            operator: token,
            rhs: Box::new(rhs),
          };
        },
        _ => break,
      }
    }

    Ok(lhs)
  }

  /// Function that parses the unary operators (!|-)
  fn parse_unary(&mut self, engine: &mut DiagnosticEngine) -> Result<Expr, ()> {
    let token = self.current_token();

    match token.token_type {
      TokenType::Bang | TokenType::Minus => {
        self.advance();
        let rhs = self.parse_unary(engine)?;

        return Ok(Expr::Unary {
          operator: token,
          rhs: Box::new(rhs),
        });
      },
      _ => {},
    }

    self.parse_primary(engine)
  }

  // Function that parses the primary (String|Number|True|False|Nil)
  fn parse_primary(&mut self, engine: &mut DiagnosticEngine) -> Result<Expr, ()> {
    let token = self.current_token();
    if self.is_eof() {
      let diagnostic = Diagnostic::new(
        DiagnosticCode::UnexpectedEof,
        "Unexpected end of file".to_string(),
      )
      .with_label(Label::primary(
        token.to_span(),
        Some("expected some expression".to_string()),
      ));

      engine.emit(diagnostic);

      return Err(());
    }

    match token.token_type {
      TokenType::String
      | TokenType::Number
      | TokenType::True
      | TokenType::False
      | TokenType::Nil => {
        self.advance();
        return Ok(Expr::Literal(token));
      },

      TokenType::Identifier => {
        self.advance();
        return Ok(Expr::Identifier(token));
      },

      TokenType::LeftParen => {
        let opening_paren_token = self.current_token();
        self.advance(); // consume '('

        let expr = self.parse_expression(engine)?;

        if self.is_eof() || self.current_token().token_type != TokenType::RightParen {
          let current = self.current_token();

          // For EOF, use the PREVIOUS token's end position
          let error_span = if self.is_eof() {
            let prev_token = &self.tokens[self.current - 1];
            Span {
              file: "asdfa".to_string(),
              line: prev_token.position.0,
              column: prev_token.position.1 + prev_token.lexeme.len(),
              length: 1,
            }
          } else {
            current.to_span()
          };

          let diagnostic = Diagnostic::new(
            DiagnosticCode::MissingClosingParen,
            "Expected ')' after expression".to_string(),
          )
          .with_label(Label::primary(
            error_span,
            Some("expected ')' here".to_string()),
          ))
          .with_label(Label::secondary(
            opening_paren_token.to_span(),
            Some("to match this '('".to_string()),
          ));

          engine.emit(diagnostic);
          return Err(());
        }

        self.advance(); // consume ')'
        return Ok(Expr::Grouping(Box::new(expr)));
      },

      TokenType::SemiColon => {
        self.advance();
        Err(())
      },

      _ => {
        let diagnostic = Diagnostic::new(
          DiagnosticCode::ExpectedExpression,
          "Expected expression".to_string(),
        )
        .with_label(Label::primary(
          self.current_token().to_span(),
          Some(format!("unexpected token '{}'", token.lexeme)),
        ));
        engine.emit(diagnostic);

        return Err(());
      },
    }
  }

  ///  Function that moves the pointer one step
  fn advance(&mut self) {
    if !self.is_eof() {
      self.current += 1;
    }
  }

  /// Function that gets the currnete token
  fn current_token(&mut self) -> Token {
    self.tokens[self.current].clone()
  }

  /// Function that returns bool indicating the EOF state
  fn is_eof(&self) -> bool {
    self.current == (self.tokens.len() - 1)
  }

  /// Function that consume the code until there's valid tokens to start a new expression
  fn synchronize(&mut self) {
    self.advance();

    while !self.is_eof() {
      let token = self.current_token();
      match token.token_type {
        TokenType::SemiColon => {
          self.advance();
          break;
        },
        _ => self.advance(),
      }
    }
  }
}
