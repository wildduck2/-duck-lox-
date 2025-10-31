/*
*  program        → declaration* EOF ;
*
*  declaration    → stmt ;
*
*  stmt           → expr_stmt ;
*
*  expr_stmt      → expr ";" ;
*
*  term           → factor ( ( "+" | "-" ) ) factor )* ;
*
*  factor         → unary ( ( "/" | "*" | "%" ) unary )* ;
*
*  unary          → ( "-" | "!" ) unary
*                   | primary ;
*
*  primary        → "true" | "false" | IDENTIFIER
*                   | STRING | FLOATING | INEGER
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

  fn parse_declaration(&mut self, engine: &mut DiagnosticEngine) -> Result<Stmt, ()> {
    let expr = self.parse_term(engine)?;

    Ok(Stmt::Expr(expr))
  }

  fn parse_stmt(&mut self, engine: &mut DiagnosticEngine) -> Result<Stmt, ()> {
    Err(())
  }

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

  fn parse_unary(&mut self, engine: &mut DiagnosticEngine) -> Result<Expr, ()> {
    if self.is_eof() {
      self.error_eof(engine);
    }

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
      _ => self.parse_primary(engine),
    }
  }

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

      _ => {
        // make some diagnostic here
        let diagnostic = Diagnostic::new(
          DiagnosticCode::Error(DiagnosticError::UnexpectedToken),
          format!("Unexpected token \"{}\"", token.lexeme),
          "duck.lox".to_string(),
        )
        .with_label(
          Span::new(token.span.line + 1, 1, token.span.len),
          Some(format!(
            "Expected a primary expression, found \"{}\"",
            token.lexeme
          )),
          LabelStyle::Primary,
        );

        engine.add(diagnostic);

        Err(())
      },
    }
  }
}
