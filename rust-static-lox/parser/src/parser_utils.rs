use diagnostic::{
  code::DiagnosticCode,
  diagnostic::{Diagnostic, LabelStyle},
  types::error::DiagnosticError,
  DiagnosticEngine,
};
use lexer::token::{Token, TokenKind};

use crate::{ast::*, match_and_consume, Parser};

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum ExprContext {
  /// Normal expression parsing with no contextual restrictions.
  Default,
  /// Parsing the condition of an `if` or `if let`.
  IfCondition,
  /// Parsing the scrutinee of a `match`.
  Match,
  /// Parsing the predicate portion of a `while`.
  WhileCondition,
}

impl Parser {
  /// Parses the top-level production, collecting statements until EOF.
  /// Currently this routine prints each item tree for debugging and relies on
  /// `parse_item` to decide which constructs are supported.
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

  /// Dispatches to the correct item parser after consuming attributes & visibility.
  fn parse_item(&mut self, engine: &mut DiagnosticEngine) -> Result<Item, ()> {
    let attributes = self.parse_attributes(engine)?;
    let visibility = self.parse_visibility(engine)?;

    match self.current_token().kind {
      TokenKind::KwStruct => self.parse_struct_decl(attributes, visibility, engine),
      TokenKind::KwFn => self.parse_fn_decl(attributes, visibility, engine),
      kind => {
        let lexeme = self.get_token_lexeme(&self.current_token());
        let diagnostic = Diagnostic::new(
          DiagnosticCode::Error(DiagnosticError::UnexpectedToken),
          format!("unsupported item starting with `{lexeme}`"),
          self.source_file.path.clone(),
        )
        .with_label(
          self.current_token().span,
          Some("the parser currently only understands struct declarations".to_string()),
          LabelStyle::Primary,
        )
        .with_help(format!(
          "item kind `{:?}` is not implemented yet; add support in `parse_item`",
          kind
        ));
        engine.add(diagnostic);
        Err(())
      },
    }
  }

  /// Parses a single statement node (stubbed for future grammar branches).
  /// Currently supports empty statements and expression statements.
  pub(crate) fn parse_stmt(&mut self, engine: &mut DiagnosticEngine) -> Result<Stmt, ()> {
    let attributes = self.parse_attributes(engine)?;

    match self.current_token().kind {
      TokenKind::KwLet => self.parse_let_statement(attributes, engine),
      TokenKind::Semi => {
        // Empty statement: just a semicolon
        self.advance(engine);
        Ok(Stmt::Empty)
      },

      _ => self.parse_expr_stmt(engine),
    }
  }

  /// Parses an expression statement, optionally consuming a trailing semicolon.
  fn parse_expr_stmt(&mut self, engine: &mut DiagnosticEngine) -> Result<Stmt, ()> {
    let expr = self.parse_expression(ExprContext::Default, engine)?;

    if self.current_token().kind == TokenKind::Semi {
      self.expect(TokenKind::Semi, engine)?; // check if followed by semicolon
      Ok(Stmt::Expr(expr))
    } else {
      Ok(Stmt::TailExpr(expr))
    }
  }

  /// Entry point for expression parsing. The supplied `context` controls
  /// future diagnostic wording once more productions are wired in.
  /// TODO: make sure that macros are supported in this context
  pub(crate) fn parse_expression(
    &mut self,
    context: ExprContext,
    engine: &mut DiagnosticEngine,
  ) -> Result<Expr, ()> {
    let is_unsafe = match_and_consume!(self, engine, TokenKind::KwUnsafe)?;
    let label = self.parse_label(engine)?;

    match self.current_token().kind {
      TokenKind::KwIf => self.parse_if_expression(ExprContext::IfCondition, engine),
      TokenKind::KwMatch => self.parse_match_expression(ExprContext::Match, engine),
      TokenKind::OpenBrace => self.parse_block_expression(is_unsafe, label, engine),
      _ => self.parse_assignment_expr(context, engine),
    }
  }

  fn parse_label(&mut self, engine: &mut DiagnosticEngine) -> Result<Option<String>, ()> {
    let token = self.current_token();
    if match_and_consume!(self, engine, TokenKind::Lifetime { .. })? {
      self.expect(TokenKind::Colon, engine)?;
      return Ok(Some(self.get_token_lexeme(&token)));
    }
    Ok(None)
  }

  /// Parses assignment expressions (including compound assignments) with right associativity.
  pub(crate) fn parse_assignment_expr(
    &mut self,
    context: ExprContext,
    engine: &mut DiagnosticEngine,
  ) -> Result<Expr, ()> {
    let mut lhs = self.parse_range_expr(context, engine)?;

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

          let rhs = self.parse_range_expr(context, engine)?;

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
  /// Parses literals, identifiers, grouped constructs, arrays, and struct expressions.
  /// Emits a targeted diagnostic when the current token cannot start a primary expression.
  pub(crate) fn parse_primary(
    &mut self,
    context: ExprContext,
    engine: &mut DiagnosticEngine,
  ) -> Result<Expr, ()> {
    let mut token = self.current_token();

    match token.kind {
      TokenKind::Literal { kind } => self.parser_literal(&token, kind, engine),
      TokenKind::Ident => {
        // FIX: this will use the context to determine whether to parse a struct expr or ident
        if matches!(context, ExprContext::Default) {
          println!("debug: {:?}", self.peek(1).kind);
          // return self.parse_struct_expr(&mut token, engine);
        }

        self.parser_ident(&mut token, engine)
      },
      TokenKind::KwFalse | TokenKind::KwTrue => self.parser_bool(&mut token, engine),
      TokenKind::KwSelf | TokenKind::KwSuper | TokenKind::KwCrate | TokenKind::KwSelfType => {
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

  /// Returns the substring that corresponds to `token`.
  pub(crate) fn get_token_lexeme(&mut self, token: &Token) -> String {
    self
      .source_file
      .src
      .get(token.span.start..token.span.end)
      .unwrap()
      .to_string()
  }

  /// Consumes tokens until `kind` is encountered or EOF is reached.
  /// Useful for resynchronizing after a diagnostic within delimited lists.
  pub(crate) fn advance_till_match(&mut self, engine: &mut DiagnosticEngine, kind: TokenKind) {
    while !self.is_eof() && self.current_token().kind != kind {
      self.advance(engine);
    }
  }
}
