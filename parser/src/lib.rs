/*
*
* program      → declaration* EOF ;
*
* declaration  → varDec
*               | statement ;
*
* varDec       → "var" IDENTIFIER ( "=" expression )? ";" ;
*
* statement    → expression_statement
*               | print_statement ;
*               | block ;
*
* block        → "{" declaration* "}" ;
*
* expression_statement → expression ";" ;
*
* print_statement → "print" expression ";" ;
*
* expression   → comma ;
*
* comma        → assignment ( "," assignment )* ;
*
* assignment   → IDENTIFIER "=" assignment
*               | ternary ;
*
* ternary      → equality ( "?" expression ":" ternary )? ;
*
* equality     → comparison ( ( "!=" | "==" ) comparison )* ;
*
* comparison   → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
*
* term         → factor ( ( "-" | "+" ) factor )* ;
*
* factor       → unary ( ( "/" | "*" ) unary )* ;
*
* unary        → ( "!" | "-" ) unary
*               | primary ;
*
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

use crate::{expression::Expr, statement::Stmt};

pub mod expression;
pub mod statement;

pub struct Parser {
  /// The tokens preduced by the scanner
  pub tokens: Vec<Token>,
  /// The pointer to the current token we are looking at
  pub current: usize,
  /// List of expressions
  pub ast: Vec<Stmt>,
}

impl Parser {
  /// Function to init a new struct
  pub fn new(tokens: Vec<Token>) -> Self {
    Self {
      tokens,
      current: 0,
      ast: Vec::new(),
    }
  }

  pub fn parse(&mut self, engine: &mut DiagnosticEngine) {
    while !self.is_eof() {
      match self.parse_program(engine) {
        Ok(stmt) => {
          stmt.print_tree();
          self.ast.push(stmt);
        },
        Err(_) => self.synchronize(),
      }
    }
  }

  fn parse_program(&mut self, engine: &mut DiagnosticEngine) -> Result<Stmt, ()> {
    self.parse_declaration(engine)
  }

  fn parse_declaration(&mut self, engine: &mut DiagnosticEngine) -> Result<Stmt, ()> {
    if self.is_eof() {
      self.error_eof(engine);
      return Err(());
    }

    match self.current_token().token_type {
      TokenType::Var => self.parse_var_stmt(engine),
      TokenType::Print => self.parse_print_stmt(engine),
      TokenType::LeftBrace => self.parse_block_stmt(engine),
      _ => self.parse_expression_stmt(engine),
    }
  }

  fn parse_block_stmt(&mut self, engine: &mut DiagnosticEngine) -> Result<Stmt, ()> {
    if self.is_eof() {
      self.error_eof(engine);
      return Err(());
    }

    let token = self.current_token();
    if token.token_type == TokenType::LeftBrace {
      self.advance(); // consume the {

      let mut declaration_vec: Vec<Stmt> = vec![];

      while !self.is_eof() {
        if !matches!(self.current_token().token_type, TokenType::RightBrace) {
          declaration_vec.push(self.parse_declaration(engine)?);
        } else {
          self.advance(); // consume the }
          break;
        }
      }
      return Ok(Stmt::Block(Box::new(declaration_vec)));
    } else {
      return Err(());
    }
  }

  fn parse_var_stmt(&mut self, engine: &mut DiagnosticEngine) -> Result<Stmt, ()> {
    if self.is_eof() {
      self.error_eof(engine);
      return Err(());
    }

    let token = self.current_token();
    if token.token_type == TokenType::Var {
      self.advance(); // consume the var

      // Check for identifier
      if !matches!(self.current_token().token_type, TokenType::Identifier) {
        let mut span = self.current_token().to_span();
        span.line += 1;
        span.column -= 1;
        let diagnostic = Diagnostic::new(
          DiagnosticCode::ExpectedIdentifier,
          "Expected identifier after 'var'".to_string(),
        )
        .with_label(Label::primary(
          span.clone(),
          Some("expected variable name here".to_string()),
        ))
        .with_label(Label::secondary(
          Span {
            length: 3,
            column: 0,
            ..span
          },
          Some("'var' keyword here".to_string()),
        ));

        engine.emit(diagnostic);
        return Err(());
      }

      let identifier = self.current_token();

      self.advance(); // consume the identifier

      if self.current_token().token_type == TokenType::SemiColon {
        self.advance(); // consume ;
        return Ok(Stmt::VarDec(identifier, None));
      } else if self.current_token().token_type == TokenType::Equal {
        self.advance(); // consume =
        let expr = self.parse_expression(engine)?;

        if self.current_token().token_type == TokenType::SemiColon {
          self.advance(); // consume ;
          return Ok(Stmt::VarDec(identifier, Some(expr)));
        } else {
          // Missing semicolon diagnostic
          let diagnostic = Diagnostic::new(
            DiagnosticCode::MissingSemicolon,
            "Expected ';' after variable declaration".to_string(),
          )
          .with_label(Label::primary(
            self.span_prev(),
            Some("semicolon missing here".to_string()),
          ));

          engine.emit(diagnostic);
          return Err(());
        }
      } else {
        // Expected = or ;
        let token = self.current_token();
        let mut span = token.to_span();
        span.length = 1;
        span.column = identifier.position.1;
        let len = identifier.lexeme.len();
        let diagnostic = Diagnostic::new(
          DiagnosticCode::UnexpectedToken,
          format!(
            "Expected '=' or ';' after identifier, found '{}'",
            token.lexeme
          ),
        )
        .with_label(Label::primary(
          span,
          Some("expected '=' or ';' here".to_string()),
        ))
        .with_label(Label::secondary(
          Span {
            length: len,
            column: identifier.position.1 - len,
            ..token.to_span()
          },
          // identifier.to_span(),
          Some("variable declared here".to_string()),
        ));

        engine.emit(diagnostic);
        return Err(());
      }
    } else {
      // This should never happen if parse_declaration routes correctly
      self.error_unexpected_token(engine, "expected 'var' keyword");
      return Err(());
    }
  }

  fn parse_expression_stmt(&mut self, engine: &mut DiagnosticEngine) -> Result<Stmt, ()> {
    let expr_start = self.current_token(); // Capture start
    let expr = self.parse_expression(engine)?;

    if self.is_eof() {
      self.error_eof(engine);
      return Err(());
    }

    if self.current_token().token_type == TokenType::SemiColon {
      self.advance();
      return Ok(Stmt::Expr(expr));
    } else {
      let diagnostic = Diagnostic::new(
        DiagnosticCode::MissingSemicolon,
        "Expected ';' after expression".to_string(),
      )
      .with_label(Label::primary(
        self.span_prev(),
        Some("semicolon missing here".to_string()),
      ))
      .with_label(Label::secondary(
        expr_start.to_span(),
        Some("expression started here".to_string()),
      ));

      engine.emit(diagnostic);
      Err(())
    }
  }

  // Function that handles print statement
  fn parse_print_stmt(&mut self, engine: &mut DiagnosticEngine) -> Result<Stmt, ()> {
    let token = self.current_token();

    if self.is_eof() {
      self.error_eof(engine);
      return Err(());
    }

    match token.token_type {
      TokenType::Print => {
        self.advance(); // consume print token
        let expr = self.parse_expression(engine)?;

        if self.current_token().token_type == TokenType::SemiColon {
          self.advance(); // consume ; token
          return Ok(Stmt::Print(expr));
        } else {
          let diagnostic = Diagnostic::new(
            DiagnosticCode::ExpectedToken,
            "Expected ';' after print statement".to_string(),
          )
          .with_label(Label::primary(
            self.span_prev(),
            Some("semicolon is missing here".to_string()),
          ));

          engine.emit(diagnostic);
          Err(())
        }
      },
      _ => {
        self.error_unexpected_token(engine, "in print statement, expected 'print' keyword");
        Err(())
      },
    }
  }

  /// Function that handles expression
  fn parse_expression(&mut self, engine: &mut DiagnosticEngine) -> Result<Expr, ()> {
    self.parse_comma(engine)
  }

  /// Function that handles the assignments (=)
  fn parse_assignment(&mut self, engine: &mut DiagnosticEngine) -> Result<Expr, ()> {
    let lhs = self.parse_ternary(engine)?;

    if !self.is_eof() && self.current_token().token_type == TokenType::Equal {
      self.advance();

      let rhs = self.parse_assignment(engine)?;

      if let Expr::Identifier(name) = lhs {
        return Ok(Expr::Assign {
          name: name,
          value: Box::new(rhs),
        });
      } else {
        self.error_unexpected_token(engine, "in assignment, left side must be an identifier");
        return Err(());
      }
    }

    Ok(lhs)
  }

  // Function that handles ,
  fn parse_comma(&mut self, engine: &mut DiagnosticEngine) -> Result<Expr, ()> {
    let mut lhs = self.parse_assignment(engine)?;

    while !self.is_eof() {
      let token = self.current_token();

      match token.token_type {
        TokenType::Comma => {
          self.advance(); // consume the ,

          let rhs = self.parse_assignment(engine)?;

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

  /// Function that handles the ternary (?:)
  fn parse_ternary(&mut self, engine: &mut DiagnosticEngine) -> Result<Expr, ()> {
    let condition = self.parse_equality(engine)?;

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
      _ => self.parse_primary(engine),
    }
  }

  // Function that parses the primary (String|Number|True|False|Nil)
  fn parse_primary(&mut self, engine: &mut DiagnosticEngine) -> Result<Expr, ()> {
    let token = self.current_token();
    if self.is_eof() {
      self.error_eof(engine);
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

  fn error_unexpected_token(&mut self, engine: &mut DiagnosticEngine, context: &str) {
    let mut token = self.current_token();
    token.position.0 += 1;
    token.position.1 -= 1;
    let diagnostic = Diagnostic::new(
      DiagnosticCode::UnexpectedToken,
      format!("Unexpected token '{}' {}", token.lexeme, context),
    )
    .with_label(Label::primary(
      token.to_span(),
      Some("unexpected token found here".to_string()),
    ));

    engine.emit(diagnostic);
  }

  fn error_eof(&mut self, engine: &mut DiagnosticEngine) {
    let token = self.current_token();
    let diagnostic = Diagnostic::new(
      DiagnosticCode::UnexpectedEof,
      "Unexpected end of file".to_string(),
    )
    .with_label(Label::primary(
      token.to_span(),
      Some("expected some expression".to_string()),
    ));

    engine.emit(diagnostic);
  }

  fn span_prev(&mut self) -> Span {
    if self.current > 0 {
      let token = self.current_token();
      let mut prev_token = self.tokens[self.current - 1].clone();
      prev_token.position.0 = token.position.0;
      prev_token.lexeme = token.lexeme;
      prev_token.to_span()
    } else {
      self.tokens[0].to_span()
    }
  }
}
