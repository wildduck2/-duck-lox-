use diagnostic::{
  code::DiagnosticCode,
  diagnostic::{Diagnostic, LabelStyle},
  types::{error::DiagnosticError, warning::DiagnosticWarning},
  DiagnosticEngine, Span,
};
use lexer::token::{Token, TokenKind};

use crate::{
  ast::{Expr, Item},
  Parser,
};

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq)]
enum ExprContext {
  Default,           // Normal expression parsing
  IfCondition,       // Parsing condition of if statement
  MatchDiscriminant, // Parsing match expression (before arms)
  WhileCondition,    // Parsing condition of while statement
}

impl Parser {
  /// Parses the top-level production, collecting statements until EOF.
  pub fn parse_program(&mut self, engine: &mut DiagnosticEngine) {
    while !self.is_eof() {
      match self.parse_declaration(engine) {
        Ok(stmt) => {
          println!("{:?}", stmt);
          // stmt.print_tree();
          self.ast.push(stmt);
        },
        Err(_) => self.synchronize(engine),
      }
    }
  }

  /// Parses a declaration, currently delegating to statement parsing.
  fn parse_declaration(&mut self, engine: &mut DiagnosticEngine) -> Result<Item, ()> {
    match self.current_token().kind {
      // TokenKind::Let | TokenKind::Const => self.parse_variable_declaration(engine),
      // TokenKind::Fn => self.parse_fn_decl(engine),
      _ => self.parse_stmt(engine),
    }
  }

  /* -------------------------------------------------------------------------------------------- */
  /*                                         Statements                                           */
  /* -------------------------------------------------------------------------------------------- */

  /// Parses a single statement node (stubbed for future grammar branches).
  fn parse_stmt(&mut self, engine: &mut DiagnosticEngine) -> Result<Item, ()> {
    match self.current_token().kind {
      // TokenKind::If => self.parse_if_stmt(engine),
      // TokenKind::While => self.parse_whie_stmt(engine),
      // TokenKind::For => self.parse_for_stmt(engine),
      // TokenKind::Return => self.parse_return_stmt(engine),
      // TokenKind::Break => self.parse_break_stmt(engine),
      // TokenKind::Continue => self.parse_continue_stmt(engine),
      // TokenKind::LeftBrace => {
      //   let stmt = self.parse_block(engine)?;
      //   Ok(Stmt::Block(stmt))
      // },
      _ => {
        // Fallback to an expression statement when no declaration keyword is found.
        let expr = self.parse_expr_stmt(engine)?;
        let hi = self.expect(TokenKind::Semi, engine)?; // ensure the statement is terminated
        println!("{:?}", expr);

        Err(())
        // Ok(Item::Expr(expr))
      },
    }
  }

  fn parse_expr_stmt(&mut self, engine: &mut DiagnosticEngine) -> Result<Expr, ()> {
    self.parse_expr(ExprContext::Default, engine);

    Err(())
  }

  fn parse_expr(
    &mut self,
    context: ExprContext,
    engine: &mut DiagnosticEngine,
  ) -> Result<Expr, ()> {
    Err(())
  }
}
