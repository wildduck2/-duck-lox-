use diagnostic::{
  code::DiagnosticCode,
  diagnostic::{Diagnostic, LabelStyle},
  types::{error::DiagnosticError, warning::DiagnosticWarning},
  DiagnosticEngine, Span,
};
use lexer::token::{LiteralKind, Token, TokenKind};

use crate::{
  ast::{BinaryOp, Expr, ExprPath, FieldAccess, Item, RangeKind, Stmt, Type, UnaryOp},
  Parser,
};

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

  /* -------------------------------------------------------------------------------------------- */
  /*                                     Item Parsing (Top Level)                                 */
  /* -------------------------------------------------------------------------------------------- */

  fn parse_item(&mut self, engine: &mut DiagnosticEngine) -> Result<Item, ()> {
    match self.current_token().kind {
      // TokenKind::Fn => self.parse_fn_decl(engine),        // Item::Function
      // TokenKind::Struct => self.parse_struct(engine),     // Item::Struct
      // TokenKind::Enum => self.parse_enum(engine),         // Item::Enum
      // TokenKind::Const => self.parse_const(engine),       // Item::Const
      // TokenKind::Static => self.parse_static(engine),     // Item::Static
      // TokenKind::Type => self.parse_type_alias(engine),   // Item::TypeAlias
      // TokenKind::Mod => self.parse_module(engine),        // Item::Module
      // TokenKind::Use => self.parse_use(engine),           // Item::Use
      // ... etc
      _ => {
        // Error: expected item at top level
        Err(())

        // self.parse_expr(ExprContext::Default, engine)
      },
    }
  }

  /* -------------------------------------------------------------------------------------------- */
  /*                                    Block & Statement Parsing                                 */
  /* -------------------------------------------------------------------------------------------- */

  // Parse the contents of a block (between { and })
  fn parse_block_contents(&mut self, engine: &mut DiagnosticEngine) -> Result<Vec<Stmt>, ()> {
    let mut stmts = Vec::new();

    // while !self.at(TokenKind::OpenBrace) && !self.is_eof() {
    //   // THIS IS WHERE YOU CALL parse_stmt
    //   let stmt = self.parse_stmt(engine)?;
    //   stmts.push(stmt);
    // }

    Ok(stmts)
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

      _ => {
        // Expression statement
        let expr = self.parse_expression(ExprContext::Default, engine)?;

        if self.current_token().kind == TokenKind::Semi {
          self.expect(TokenKind::Semi, engine)?; // check if followed by semicolon
          Ok(Stmt::Expr(expr))
        } else {
          Ok(Stmt::TailExpr(expr))
        }
      },
    }
  }

  fn parse_expr_stmt(&mut self, engine: &mut DiagnosticEngine) -> Result<Expr, ()> {
    self.parse_expression(ExprContext::Default, engine);

    Err(())
  }

  pub(crate) fn parse_expression(
    &mut self,
    context: ExprContext,
    engine: &mut DiagnosticEngine,
  ) -> Result<Expr, ()> {
    self.parse_assignment_expr(engine)
  }

  // *   expression       → assignment ;
  // *
  // *   assignment       → (rangeExpr assignOp)* rangeExpr ;
  // *
  // *   assignOp         → "=" | "+=" | "-=" | "*=" | "/=" | "%=" | "&=" | "|=" | "^=" | "<<=" | ">>=" ;

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

  pub(crate) fn parse_primary(&mut self, engine: &mut DiagnosticEngine) -> Result<Expr, ()> {
    let mut token = self.current_token();

    match token.kind {
      TokenKind::Literal { kind } => self.parser_literal(engine, &token, kind),
      TokenKind::Ident => self.parser_ident(engine, &token),
      TokenKind::KwFalse | TokenKind::KwTrue => self.parser_bool(engine, &token),
      TokenKind::KwSelfValue | TokenKind::KwSuper | TokenKind::KwCrate | TokenKind::KwSelfType => {
        self.parse_keyword_ident(engine, &token)
      },
      TokenKind::OpenParen => self.parse_grouped_expr(&mut token, engine),
      _ => {
        let lexeme = self
          .source_file
          .src
          .get(token.span.start..token.span.end)
          .unwrap();

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
}
