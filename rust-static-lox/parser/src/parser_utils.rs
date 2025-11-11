use diagnostic::{
  code::DiagnosticCode,
  diagnostic::{Diagnostic, LabelStyle},
  types::error::DiagnosticError,
  DiagnosticEngine,
};
use lexer::token::{Token, TokenKind};

use crate::{ast::*, Parser};

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum ExprContext {
  Default,           // Normal expression parsing
  IfCondition,       // Parsing condition of if statement
  MatchDiscriminant, // Parsing match expression (before arms)
  WhileCondition,    // Parsing condition of while statement
}

impl Parser {
  /// Parses the top-level production, collecting statements until EOF.
  pub fn parse_program(&mut self, engine: &mut DiagnosticEngine) {
    while !self.is_eof() {
      match self.parse_stmt(engine) {
        // Returns Item, not Stmt
        Ok(item) => {
          item.print_tree("", true);
          // self.ast.push(item); // ast should be Vec<Item>
        },
        Err(_) => self.synchronize(engine),
      }
    }
  }

  /// Parses a single statement node (stubbed for future grammar branches).
  fn parse_stmt(&mut self, engine: &mut DiagnosticEngine) -> Result<Stmt, ()> {
    match self.current_token().kind {
      // TokenKind::Let => {
      //   // Parse let statement
      //   // Returns: Stmt::Let { attributes, pattern, ty, init, else_block, span }
      //   self.parse_let_statement(engine)
      // },
      TokenKind::Semi => {
        // Empty statement: just a semicolon
        self.advance(engine);
        Ok(Stmt::Empty)
      },

      _ => self.parse_expr_stmt(engine),
    }
  }

  fn parse_expr_stmt(&mut self, engine: &mut DiagnosticEngine) -> Result<Stmt, ()> {
    let expr = self.parse_expression(ExprContext::Default, engine)?;

    if self.current_token().kind == TokenKind::Semi {
      self.expect(TokenKind::Semi, engine)?; // check if followed by semicolon
      Ok(Stmt::Expr(expr))
    } else {
      Ok(Stmt::TailExpr(expr))
    }
  }

  pub(crate) fn parse_expression(
    &mut self,
    context: ExprContext,
    engine: &mut DiagnosticEngine,
  ) -> Result<Expr, ()> {
    self.parse_assignment_expr(engine)
  }

  pub(crate) fn parse_assignment_expr(
    &mut self,
    engine: &mut DiagnosticEngine,
  ) -> Result<Expr, ()> {
    let mut lhs = self.parse_range_expr(engine)?;

    while !self.is_eof() {
      let token = self.current_token();
      match self.current_token().kind {
        TokenKind::Eq
        | TokenKind::PlusEq
        | TokenKind::MinusEq
        | TokenKind::StarEq
        | TokenKind::SlashEq
        | TokenKind::PercentEq
        | TokenKind::AndEq
        | TokenKind::OrEq
        | TokenKind::CaretEq
        | TokenKind::ShiftLeftEq
        | TokenKind::ShiftRightEq => {
          self.advance(engine); // consume the assignment operator

          let rhs = self.parse_range_expr(engine)?;

          lhs = Expr::Assign {
            target: Box::new(lhs),
            value: Box::new(rhs),
            span: token.span,
          };
        },
        _ => break,
      }
    }

    Ok(lhs)
  }

  /* -------------------------------------------------------------------------------------------- */
  /*                                     Primary Parsing                                          */
  /* -------------------------------------------------------------------------------------------- */

  // *   primary          → literalExpr [x]
  // *                    | pathExpr
  // *                    | groupedExpr [x]
  // *                    | arrayExpr   [x]
  // *                    | tupleExpr   [x]
  // *                    | structExpr  [_]
  // *                    | closureExpr
  // *                    | blockExpr
  // *                    | asyncBlockExpr
  // *                    | unsafeBlockExpr
  // *                    | loopExpr
  // *                    | ifExpr
  // *                    | ifLetExpr
  // *                    | matchExpr
  // *                    | continueExpr
  // *                    | breakExpr
  // *                    | returnExpr
  // *                    | underscoreExpr
  // *                    | macroInvocation ;

  pub(crate) fn parse_primary(&mut self, engine: &mut DiagnosticEngine) -> Result<Expr, ()> {
    let mut token = self.current_token();

    match token.kind {
      TokenKind::Literal { kind } => self.parser_literal(&token, kind, engine),
      TokenKind::Ident => {
        if matches!(
          self.peek(1).kind,
          TokenKind::OpenBrace | TokenKind::OpenParen | TokenKind::ColonColon
        ) {
          return self.parse_struct_expr(&mut token, engine);
        }

        self.parser_ident(&mut token, engine)
      },
      TokenKind::KwFalse | TokenKind::KwTrue => self.parser_bool(&mut token, engine),
      TokenKind::KwSelfValue | TokenKind::KwSuper | TokenKind::KwCrate | TokenKind::KwSelfType => {
        self.parse_keyword_ident(&mut token, engine)
      },
      TokenKind::OpenParen => self.parse_grouped_expr(&mut token, engine),
      TokenKind::OpenBracket => self.parse_array_expr(&mut token, engine),
      _ => {
        let lexeme = self.get_token_lexeme(&token);

        let diagnostic = Diagnostic::new(
          DiagnosticCode::Error(DiagnosticError::UnexpectedToken),
          "Unexpected token".to_string(),
          self.source_file.path.clone(),
        )
        .with_label(
          token.span,
          Some(format!(
            "Expected a primary expression, found \"{}\"",
            lexeme
          )),
          LabelStyle::Primary,
        )
        .with_help(Parser::get_token_help(&token.kind, &token));

        engine.add(diagnostic);

        Err(())
      },
    }
  }

  /// Helper function that takes a token and returns the lexeme as a string
  pub(crate) fn get_token_lexeme(&mut self, token: &Token) -> String {
    self
      .source_file
      .src
      .get(token.span.start..token.span.end)
      .unwrap()
      .to_string()
  }
}
