use diagnostic::{
  code::DiagnosticCode,
  diagnostic::{Diagnostic, LabelStyle},
  types::{error::DiagnosticError, warning::DiagnosticWarning},
  DiagnosticEngine, Span,
};
use lexer::token::{LiteralKind, Token, TokenKind};

use crate::{
  ast::{BinaryOp, Expr, ExprPath, FieldAccess, Item, Stmt, Type, UnaryOp},
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
    self.parse_comparison(engine)
  }

  /* -------------------------------------------------------------------------------------------- */
  /*                                     Comparison Parsing                                       */
  /* -------------------------------------------------------------------------------------------- */
  // *   comparison       → bitwiseOr (comparisonOp bitwiseOr)* ;
  // *
  // *   comparisonOp     → "==" | "!=" | "<" | "<=" | ">" | ">=" ;

  pub(crate) fn parse_comparison(&mut self, engine: &mut DiagnosticEngine) -> Result<Expr, ()> {
    let mut lhs = self.parse_bitwise_or(engine)?;
    'comparison_find: while !self.is_eof() {
      let token = self.current_token();
      match token.kind {
        TokenKind::EqEq
        | TokenKind::Ne
        | TokenKind::Lt
        | TokenKind::Le
        | TokenKind::Gt
        | TokenKind::Ge => {
          self.advance(engine); // consume the comparison operator

          let rhs = self.parse_bitwise_or(engine)?;

          lhs = Expr::Binary {
            op: match token.kind {
              TokenKind::EqEq => BinaryOp::Eq,
              TokenKind::Ne => BinaryOp::NotEq,
              TokenKind::Lt => BinaryOp::Less,
              TokenKind::Le => BinaryOp::LessEq,
              TokenKind::Gt => BinaryOp::Greater,
              TokenKind::Ge => BinaryOp::GreaterEq,
              _ => unreachable!(),
            },
            left: Box::new(lhs),
            right: Box::new(rhs),
            span: token.span,
          };
        },
        _ => break 'comparison_find,
      }
    }

    Ok(lhs)
  }

  /* -------------------------------------------------------------------------------------------- */
  /*                                     Bitwise Parsing                                           */
  /* -------------------------------------------------------------------------------------------- */

  pub(crate) fn parse_bitwise_or(&mut self, engine: &mut DiagnosticEngine) -> Result<Expr, ()> {
    let mut lhs = self.parse_bitwise_xor(engine)?;

    'bitwise_or_find: while !self.is_eof() {
      let token = self.current_token();
      match token.kind {
        TokenKind::Or => {
          self.advance(engine); // consume the bitwise operator

          let rhs = self.parse_bitwise_xor(engine)?;

          lhs = Expr::Binary {
            op: BinaryOp::BitOr,
            left: Box::new(lhs),
            right: Box::new(rhs),
            span: token.span,
          };
        },
        _ => break 'bitwise_or_find,
      }
    }

    Ok(lhs)
  }

  pub(crate) fn parse_bitwise_xor(&mut self, engine: &mut DiagnosticEngine) -> Result<Expr, ()> {
    let mut lhs = self.parse_bitwise_and(engine)?;

    'bitwise_xor_find: while !self.is_eof() {
      let token = self.current_token();
      match token.kind {
        TokenKind::Caret => {
          self.advance(engine); // consume the bitwise operator

          let rhs = self.parse_bitwise_and(engine)?;

          lhs = Expr::Binary {
            op: BinaryOp::BitXor,
            left: Box::new(lhs),
            right: Box::new(rhs),
            span: token.span,
          };
        },
        _ => break 'bitwise_xor_find,
      }
    }

    Ok(lhs)
  }

  pub(crate) fn parse_bitwise_and(&mut self, engine: &mut DiagnosticEngine) -> Result<Expr, ()> {
    let mut lhs = self.parse_shift(engine)?;

    'bitwise_and_find: while !self.is_eof() {
      let token = self.current_token();
      match token.kind {
        TokenKind::And => {
          self.advance(engine); // consume the bitwise operator

          let rhs = self.parse_shift(engine)?;

          lhs = Expr::Binary {
            op: BinaryOp::BitAnd,
            left: Box::new(lhs),
            right: Box::new(rhs),
            span: token.span,
          };
        },
        _ => break 'bitwise_and_find,
      }
    }

    Ok(lhs)
  }

  /* -------------------------------------------------------------------------------------------- */
  /*                                     Shift Parsing                                            */
  /* -------------------------------------------------------------------------------------------- */

  pub(crate) fn parse_shift(&mut self, engine: &mut DiagnosticEngine) -> Result<Expr, ()> {
    let mut lhs = self.parse_term(engine)?;

    'shift_find: while !self.is_eof() {
      let token = self.current_token();
      match token.kind {
        TokenKind::ShiftLeft | TokenKind::ShiftRight => {
          self.advance(engine); // consume the shift operator

          let op = match token.kind {
            TokenKind::ShiftLeft => BinaryOp::Shl,
            TokenKind::ShiftRight => BinaryOp::Shr,
            _ => unreachable!(),
          };

          let rhs = self.parse_term(engine)?;

          lhs = Expr::Binary {
            op,
            left: Box::new(lhs),
            right: Box::new(rhs),
            span: token.span,
          };
        },
        _ => break 'shift_find,
      }
    }

    Ok(lhs)
  }

  /* -------------------------------------------------------------------------------------------- */
  /*                                     Term Parsing                                             */
  /* -------------------------------------------------------------------------------------------- */

  pub(crate) fn parse_term(&mut self, engine: &mut DiagnosticEngine) -> Result<Expr, ()> {
    let mut lhs = self.parse_factor(engine)?;

    'term_find: while !self.is_eof() {
      let token = self.current_token();
      match token.kind {
        TokenKind::Plus | TokenKind::Minus => {
          self.advance(engine); // consume the term operator

          let op = match token.kind {
            TokenKind::Plus => BinaryOp::Add,
            TokenKind::Minus => BinaryOp::Sub,
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
        _ => break 'term_find,
      }
    }

    Ok(lhs)
  }

  /* -------------------------------------------------------------------------------------------- */
  /*                                     Factor Parsing                                           */
  /* -------------------------------------------------------------------------------------------- */

  pub(crate) fn parse_factor(&mut self, engine: &mut DiagnosticEngine) -> Result<Expr, ()> {
    let mut lhs = self.parse_cast(engine)?;

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
  /*                                     Cast Parsing                                           */
  /* -------------------------------------------------------------------------------------------- */

  pub(crate) fn parse_cast(&mut self, engine: &mut DiagnosticEngine) -> Result<Expr, ()> {
    let mut lhs = self.parse_unary(engine)?;

    'cast_find: while !self.is_eof() {
      let mut token = self.current_token();
      match token.kind {
        TokenKind::KwAs => {
          self.advance(engine); // consume the cast operator

          // let type_no_bounds = self.parse_type_no_bounds(engine)?;
          // NOTE: later one you will parse the type bounds
          self.advance(engine); // consume the type
          token.span.merge(self.current_token().span);

          lhs = Expr::Cast {
            expr: Box::new(lhs),
            ty: Type::U32,
            span: token.span,
          };
        },
        _ => break 'cast_find,
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
