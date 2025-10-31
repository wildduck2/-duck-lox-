/*
*  program        → declaration* EOF ;
*
*  declaration    → stmt ;
*
*  stmt           → expr_stmt ;
*
*  expr           → comma ;
*
*  comma          → assignment ( "," assignment )* ;
*
*  assignment     → (ternary ".")? IDENTIFIER "=" assignment
*                 | ternary ;
*
*  ternary        → logical_or ( "?" expr ":" ternary )? ;
*
*  logical_or     → logical_and ( "or" logical_and )* ;
*
*  logical_and    → equality ( "and" equality )* ;
*
*  equality       → comparison ( ( "==" | "!=" ) comparison )* ;
*
*  comparison     → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
*
*  term           → factor ( ( "+" | "-" ) factor )* ;
*
*  factor         → unary ( ( "/" | "*" | "%" ) unary )* ;
*
*  unary          → ( "-" | "!" ) unary
*                   | call;
*
*  call           → primary ( "(" arguments? ")" | "." IDENTIFIER )* ;
*
*  primary        → "true" | "false" | IDENTIFIER
*                   | STRING | FLOATING | INTEGER
*                   | "nil" | "(" expr ")" ;
*
*/

use diagnostic::{
  code::DiagnosticCode,
  diagnostic::{Diagnostic, LabelStyle, Span},
  types::error::DiagnosticError,
  DiagnosticEngine,
};
use lexer::token::TokenKind;

use crate::{expr::Expr, stmt::Stmt, Parser};

impl Parser {
  /// Parses the top-level production, collecting statements until EOF.
  pub fn parse_program(&mut self, engine: &mut DiagnosticEngine) {
    while !self.is_eof() {
      match self.parse_declaration(engine) {
        Ok(stmt) => {
          stmt.print_tree();
          self.ast.push(stmt);
        },
        Err(_) => self.synchronize(engine),
      }
    }
  }

  /// Parses a declaration, currently delegating to statement parsing.
  fn parse_declaration(&mut self, engine: &mut DiagnosticEngine) -> Result<Stmt, ()> {
    let expr = self.parse_expr(engine)?;

    Ok(Stmt::Expr(expr))
  }

  /// Parses a single statement node (stubbed for future grammar branches).
  fn parse_stmt(&mut self, engine: &mut DiagnosticEngine) -> Result<Stmt, ()> {
    Err(())
  }

  /// Parses a general expression entrypoint.
  fn parse_expr(&mut self, engine: &mut DiagnosticEngine) -> Result<Expr, ()> {
    self.parse_comma(engine)
  }

  /// Parses comma-separated expressions, emitting `Expr::Binary` nodes.
  fn parse_comma(&mut self, engine: &mut DiagnosticEngine) -> Result<Expr, ()> {
    let mut lhs = self.parse_assignment(engine)?;

    while !self.is_eof() || !matches!(self.current_token().kind, TokenKind::Comma) {
      let token = self.current_token();

      match token.kind {
        TokenKind::Comma => {
          self.advance(engine);

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

  /// Parses assignment expressions and verifies the left side is assignable.
  fn parse_assignment(&mut self, engine: &mut DiagnosticEngine) -> Result<Expr, ()> {
    let token = self.current_token();
    let lhs = self.parse_ternary(engine)?;

    if !self.is_eof() && matches!(self.current_token().kind, TokenKind::Equal) {
      self.advance(engine);
      let rhs = self.parse_assignment(engine)?;

      if let Expr::Identifier(name) = lhs {
        return Ok(Expr::Assign {
          name,
          rhs: Box::new(rhs),
        });
      } else {
        let diagnostic = Diagnostic::new(
          DiagnosticCode::Error(DiagnosticError::UnexpectedToken),
          "Expected identifier after '='".to_string(),
          "duck.lox".to_string(),
        )
        .with_label(
          Span::new(token.span.line + 1, token.span.col + 1, token.span.len),
          Some("expected identifier here".to_string()),
          LabelStyle::Primary,
        )
        .with_help("use '=' for assignment".to_string());

        engine.add(diagnostic);

        return Err(());
      }
      // NOTE: please check this later when we sclare the parser
      // } else if !self.is_eof()
      //   && !matches!(self.current_token().kind, TokenKind::Semicolon)
      //   && !matches!(self.current_token().kind, TokenKind::Comma)
      //   && !matches!(self.current_token().kind, TokenKind::LeftParen)
      // {
      //   let diagnostic = Diagnostic::new(
      //     DiagnosticCode::Error(DiagnosticError::UnexpectedToken),
      //     "Expected '=' or ';' after identifier, found '='".to_string(),
      //     "duck.lox".to_string(),
      //   )
      //   .with_label(
      //     Span::new(self.current_token().span.line + 1, 2, 2),
      //     Some("expected '=' or ';' here".to_string()),
      //     LabelStyle::Primary,
      //   )
      //   .with_help("use '=' for assignment".to_string());
      //
      //   engine.add(diagnostic);
      //
      //   return Err(());
    }

    Ok(lhs)
  }

  /// Parses ternary expressions of the form `cond ? a : b`.
  fn parse_ternary(&mut self, engine: &mut DiagnosticEngine) -> Result<Expr, ()> {
    let token = self.current_token();
    let condition = self.parse_logical_or(engine)?;

    if !self.is_eof() && matches!(self.current_token().kind, TokenKind::Question) {
      self.advance(engine); // consume the (?)
      let then_branch = self.parse_expr(engine)?;

      if self.is_eof() || !matches!(self.current_token().kind, TokenKind::Colon) {
        let current_token = self.current_token();

        let diagnostic = Diagnostic::new(
          DiagnosticCode::Error(DiagnosticError::UnexpectedToken),
          format!(
            "Expected ':' in ternary expr, found '{}'",
            current_token.lexeme
          ),
          "duck.lox".to_string(),
        )
        .with_label(
          Span::new(
            current_token.span.line + 1,
            current_token.span.col + 1,
            current_token.lexeme.len(),
          ),
          Some("expected ':' before this token".to_string()),
          LabelStyle::Primary,
        )
        .with_label(
          Span::new(
            current_token.span.line + 1,
            token.span.col + 1,
            current_token.span.len,
          ),
          Some("ternary started here".to_string()),
          LabelStyle::Secondary,
        )
        .with_help(
          "Ternary exprs require the format: condition ? then_value : else_value".to_string(),
        );

        engine.add(diagnostic);

        return Err(());
      }

      self.advance(engine); // consume the (:)
      let else_branch = self.parse_ternary(engine)?;

      return Ok(Expr::Ternary {
        condition: Box::new(condition),
        then_branch: Box::new(then_branch),
        else_branch: Box::new(else_branch),
      });
    }

    Ok(condition)
  }

  /// Parses logical OR chains (`expr or expr`).
  fn parse_logical_or(&mut self, engine: &mut DiagnosticEngine) -> Result<Expr, ()> {
    let mut lhs = self.parse_logical_and(engine)?;

    while !self.is_eof() {
      let operator = self.current_token();

      match operator.kind {
        TokenKind::Or => {
          self.advance(engine);
          let rhs = self.parse_logical_and(engine)?;
          lhs = Expr::Binary {
            operator,
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
          };
        },
        _ => break,
      }
    }

    Ok(lhs)
  }

  /// Parses logical AND chains (`expr and expr`).
  fn parse_logical_and(&mut self, engine: &mut DiagnosticEngine) -> Result<Expr, ()> {
    let mut lhs = self.parse_equality(engine)?;

    while !self.is_eof() {
      let operator = self.current_token();

      match operator.kind {
        TokenKind::And => {
          self.advance(engine);
          let rhs = self.parse_equality(engine)?;
          lhs = Expr::Binary {
            operator,
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
          };
        },
        _ => break,
      }
    }

    Ok(lhs)
  }

  /// Parses equality comparisons (`==` and `!=`).
  fn parse_equality(&mut self, engine: &mut DiagnosticEngine) -> Result<Expr, ()> {
    let mut lhs = self.parse_comparison(engine)?;

    while !self.is_eof() {
      let operator = self.current_token();

      match operator.kind {
        TokenKind::EqualEqual | TokenKind::BangEqual => {
          self.advance(engine);
          let rhs = self.parse_comparison(engine)?;
          lhs = Expr::Binary {
            operator,
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
          };
        },
        _ => break,
      }
    }

    Ok(lhs)
  }

  /// Parses relational comparisons (`<`, `<=`, `>`, `>=`).
  fn parse_comparison(&mut self, engine: &mut DiagnosticEngine) -> Result<Expr, ()> {
    let mut lhs = self.parse_term(engine)?;

    while !self.is_eof() {
      let operator = self.current_token();

      match operator.kind {
        TokenKind::Greater | TokenKind::GreaterEqual | TokenKind::Less | TokenKind::LessEqual => {
          self.advance(engine);
          let rhs = self.parse_term(engine)?;
          lhs = Expr::Binary {
            operator,
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
          };
        },
        _ => break,
      }
    }

    Ok(lhs)
  }

  /// Parses additive expressions (`+` and `-` sequences).
  fn parse_term(&mut self, engine: &mut DiagnosticEngine) -> Result<Expr, ()> {
    let mut lhs = self.parse_factor(engine)?;

    while !self.is_eof() {
      let operator = self.current_token();

      match operator.kind {
        TokenKind::Plus | TokenKind::Minus => {
          self.advance(engine);
          let rhs = self.parse_factor(engine)?;
          lhs = Expr::Binary {
            operator,
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
          };
        },
        _ => break,
      }
    }

    Ok(lhs)
  }

  /// Parses multiplicative expressions (`*`, `/`, `%`).
  fn parse_factor(&mut self, engine: &mut DiagnosticEngine) -> Result<Expr, ()> {
    let mut lhs = self.parse_unary(engine)?;

    while !self.is_eof() {
      let operator = self.current_token();

      match operator.kind {
        TokenKind::Percent | TokenKind::Slash | TokenKind::Star => {
          self.advance(engine);
          let rhs = self.parse_unary(engine)?;
          lhs = Expr::Binary {
            operator,
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
          };
        },
        _ => break,
      }
    }

    Ok(lhs)
  }

  /// Parses prefix unary operators and defers to the next precedence level.
  fn parse_unary(&mut self, engine: &mut DiagnosticEngine) -> Result<Expr, ()> {
    let operator = self.current_token();

    match operator.kind {
      TokenKind::Minus | TokenKind::Bang => {
        self.advance(engine);
        let rhs = self.parse_unary(engine)?;

        Ok(Expr::Unary {
          operator,
          rhs: Box::new(rhs),
        })
      },
      _ => self.parse_call(engine),
    }
  }

  /// Parses function calls and dotted access chains.
  fn parse_call(&mut self, engine: &mut DiagnosticEngine) -> Result<Expr, ()> {
    let primary = self.parse_primary(engine)?;

    if !self.is_eof() && matches!(self.current_token().kind, TokenKind::LeftParen) {
      self.advance(engine); // consume the "("
      let arguments = self.parser_arguments(engine)?;

      return Ok(Expr::Call {
        callee: Box::new(primary),
        paren: self.current_token(),
        arguments,
      });
    }

    Ok(primary)
  }

  /// Parses primary expressions: literals, identifiers, and grouped expressions.
  fn parse_primary(&mut self, engine: &mut DiagnosticEngine) -> Result<Expr, ()> {
    let token = self.current_token();
    match token.kind {
      // handle the case where the token is a literal
      TokenKind::StringLiteral
      | TokenKind::FloatLiteral
      | TokenKind::IntegerLiteral
      | TokenKind::NilLiteral
      | TokenKind::TrueLiteral
      | TokenKind::FalseLiteral => {
        self.advance(engine); // consume this token
        Ok(Expr::Literal(token))
      },

      // handle the case where the token is a keyword
      TokenKind::Identifier => {
        self.advance(engine); // consume this token
        Ok(Expr::Identifier(token))
      },

      // handle the case where group is used
      TokenKind::LeftParen => {
        self.advance(engine); // consume the "("

        let expr = self.parse_expr(engine)?;

        if self.is_eof() || self.current_token().kind != TokenKind::RightParen {
          let current = self.current_token();

          // For EOF, use the PREVIOUS token's end position
          let error_span = if self.is_eof() {
            let prev_token = &self.tokens[self.current - 1];
            Span {
              line: prev_token.span.line,
              col: prev_token.span.col + prev_token.lexeme.len(),
              len: 1,
            }
          } else {
            current.span.clone()
          };

          let diagnostic = Diagnostic::new(
            DiagnosticCode::Error(DiagnosticError::MissingClosingParen),
            "Expected ')' after expr".to_string(),
            "duck.lox".to_string(),
          )
          .with_label(
            Span::new(error_span.line + 1, error_span.col + 1, error_span.len),
            Some("expected ')' here".to_string()),
            LabelStyle::Primary,
          );

          engine.add(diagnostic);
          return Err(());
        }

        self.advance(engine); // consume ')'
        return Ok(Expr::Grouping(Box::new(expr)));
      },

      // handle any other token
      _ => {
        // make some diagnostic here
        let diagnostic = Diagnostic::new(
          DiagnosticCode::Error(DiagnosticError::UnexpectedToken),
          format!("Unexpected token {:?}", token.lexeme),
          "duck.lox".to_string(),
        )
        .with_label(
          Span::new(token.span.line + 1, 1, token.span.len),
          Some(format!(
            "Expected a primary expression, found {:?}",
            token.lexeme
          )),
          LabelStyle::Primary,
        );

        engine.add(diagnostic);

        Err(())
      },
    }
  }

  /// Parses a comma-separated argument list for function calls.
  fn parser_arguments(&mut self, engine: &mut DiagnosticEngine) -> Result<Vec<Expr>, ()> {
    let mut args = Vec::<Expr>::new();
    let expr = self.parse_assignment(engine)?;
    args.push(expr);

    while !self.is_eof() && matches!(self.current_token().kind, TokenKind::Comma) {
      self.advance(engine); // consume the comma

      let expr = self.parse_assignment(engine)?;
      args.push(expr);
    }

    if args.len() >= 255 {
      let diagnostic = Diagnostic::new(
        DiagnosticCode::Error(DiagnosticError::WrongNumberOfArguments),
        "Too many arguments".to_string(),
        "duck.lox".to_string(),
      )
      .with_label(
        Span::new(
          self.current_token().span.line + 1,
          self.current_token().span.col + 1,
          self.current_token().span.len,
        ),
        Some("too many arguments".to_string()),
        LabelStyle::Primary,
      );

      engine.add(diagnostic);

      return Err(());
    }

    self.expect(TokenKind::RightParen, engine)?;

    Ok(args)
  }
}
