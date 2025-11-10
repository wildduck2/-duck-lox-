use diagnostic::{
  code::DiagnosticCode,
  diagnostic::{Diagnostic, LabelStyle},
  types::{error::DiagnosticError, warning::DiagnosticWarning},
  DiagnosticEngine, Span,
};
use lexer::token::{LiteralKind, Token, TokenKind};

use crate::{
  ast::{BinaryOp, Expr, ExprPath, FieldAccess, Item, Stmt, UnaryOp},
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
    self.parse_factor(engine)
  }

  /* -------------------------------------------------------------------------------------------- */
  /*                                     Factor Parsing                                           */
  /* -------------------------------------------------------------------------------------------- */

  // *   factor           → cast (factorOp cast)* ;
  // *
  // *   factorOp         → "*" | "/" | "%" ;
  // *
  // *   cast             → unary ("as" typeNoBounds)* ;

  pub(crate) fn parse_factor(&mut self, engine: &mut DiagnosticEngine) -> Result<Expr, ()> {
    let mut lhs = self.parse_unary(engine)?;

    'factor_find: while !self.is_eof() {
      let token = self.current_token();
      match token.kind {
        TokenKind::Star | TokenKind::Slash | TokenKind::Percent => {
          self.advance(engine); // consume the factor operator
          let op = match token.kind {
            TokenKind::Star => BinaryOp::Mul,
            TokenKind::Slash => BinaryOp::Div,
            TokenKind::Percent => BinaryOp::Mod,
            _ => unreachable!(),
          };

          let rhs = self.parse_factor(engine)?;

          lhs = Expr::Binary {
            op,
            left: Box::new(lhs),
            right: Box::new(rhs),
            span: token.span,
          };
        },
        _ => break 'factor_find,
      }
    }

    Ok(lhs)
  }

  /* -------------------------------------------------------------------------------------------- */
  /*                                     Unary Parsing                                            */
  /* -------------------------------------------------------------------------------------------- */

  pub(crate) fn parse_unary(&mut self, engine: &mut DiagnosticEngine) -> Result<Expr, ()> {
    let mut token = self.current_token();

    match token.kind {
      TokenKind::Minus | TokenKind::Bang | TokenKind::Star | TokenKind::And | TokenKind::AndAnd => {
        self.advance(engine); // consume the unary operator

        // consume the 'mut' keyword and set the mutable flag
        let mutable = if self.current_token().kind == TokenKind::KwMut {
          self.advance(engine);
          true
        } else {
          false
        };

        let op = match token.kind {
          TokenKind::Minus => UnaryOp::Neg,
          TokenKind::Bang => UnaryOp::Not,
          TokenKind::Star => UnaryOp::Deref,
          TokenKind::And => UnaryOp::Ref { mutable, depth: 1 },
          TokenKind::AndAnd => UnaryOp::Ref { mutable, depth: 2 },
          _ => unreachable!(),
        };

        let rhs = self.parse_unary(engine)?;

        token.span.merge(self.current_token().span);

        Ok(Expr::Unary {
          expr: Box::new(rhs),
          op,
          span: token.span,
        })
      },
      _ => self.parse_postfix(engine),
    }
  }

  /* -------------------------------------------------------------------------------------------- */
  /*                                     Postfix Parsing                                          */
  /* -------------------------------------------------------------------------------------------- */

  pub(crate) fn parse_postfix(&mut self, engine: &mut DiagnosticEngine) -> Result<Expr, ()> {
    // First, parse the base expression (primary)
    let mut expr = self.parse_primary(engine)?;

    // Then, repeatedly apply postfix operators
    'postfix_find: loop {
      match self.current_token().kind {
        TokenKind::OpenParen => {
          expr = self.parse_method_call(expr, engine)?; // normal call
        },
        TokenKind::Dot => {
          // look ahead for .await vs .foo vs .0
          if self.peek(1).kind == TokenKind::KwAwait {
            expr = self.parse_await(expr, engine)?;
          } else {
            expr = self.parse_field_or_method(expr, engine)?;
          }
        },
        TokenKind::OpenBracket => {
          expr = self.parse_index(expr, engine)?;
        },
        TokenKind::Question => {
          expr = self.parse_try(expr, engine)?;
        },
        _ => break 'postfix_find,
      }
    }

    Ok(expr)
  }

  /* -------------------------------------------------------------------------------------------- */
  /*                                     Primary Parsing                                          */
  /* -------------------------------------------------------------------------------------------- */

  //FIX: when there is token before the "(" it glitches
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
