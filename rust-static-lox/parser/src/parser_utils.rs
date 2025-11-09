use diagnostic::{
  code::DiagnosticCode,
  diagnostic::{Diagnostic, LabelStyle},
  types::{error::DiagnosticError, warning::DiagnosticWarning},
  DiagnosticEngine, Span,
};
use lexer::token::{LiteralKind, Token, TokenKind};

use crate::{
  ast::{Expr, ExprPath, FieldAccess, Item, Stmt},
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
      match self.parse_postfix(engine) {
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

        // TODO: Check if followed by semicolon
        Ok(Stmt::Expr(expr))
      },
    }
  }

  fn parse_expr_stmt(&mut self, engine: &mut DiagnosticEngine) -> Result<Expr, ()> {
    self.parse_expression(ExprContext::Default, engine);

    Err(())
  }

  fn parse_expression(
    &mut self,
    context: ExprContext,
    engine: &mut DiagnosticEngine,
  ) -> Result<Expr, ()> {
    self.parse_postfix(engine)
  }

  /* -------------------------------------------------------------------------------------------- */
  /*                                     Call Parsing                                       */
  /* -------------------------------------------------------------------------------------------- */
  // *   callOp           → "(" callParams? ")" ;
  // *
  // *   methodCallOp     → "." pathExprSegment "(" callParams? ")" ;
  // *
  // *   fieldAccessOp    → "." IDENTIFIER ;
  // *
  // *   tupleIndexOp     → "." INTEGER ;
  // *
  // *   indexOp          → "[" expression "]" ;
  // *
  // *   awaitOp          → "." "await" ;
  // *
  // *   tryOp            → "?" ;
  // *
  // *   callParams       → expression ("," expression)* ","? ;

  fn parse_postfix(&mut self, engine: &mut DiagnosticEngine) -> Result<Expr, ()> {
    // First, parse the base expression (primary)
    let mut expr = self.parse_primary(engine)?;

    // Then, repeatedly apply postfix operators
    'l: loop {
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
        _ => break 'l,
      }
    }

    Ok(expr)
  }

  fn parse_await(&mut self, expr: Expr, engine: &mut DiagnosticEngine) -> Result<Expr, ()> {
    let mut span = expr.span();

    self.expect(TokenKind::Dot, engine)?;
    let token = self.current_token();
    self.advance(engine);
    span.merge(token.span);

    Ok(Expr::Await {
      expr: Box::new(expr),
      span,
    })
  }

  fn parse_try(&mut self, expr: Expr, engine: &mut DiagnosticEngine) -> Result<Expr, ()> {
    let mut start_span = expr.span();
    self.expect(TokenKind::Question, engine)?;
    start_span.merge(self.current_token().span);

    Ok(Expr::Try {
      expr: Box::new(expr),
      span: start_span,
    })
  }

  fn parse_index(&mut self, object: Expr, engine: &mut DiagnosticEngine) -> Result<Expr, ()> {
    let mut start_span = object.span();

    self.expect(TokenKind::OpenBracket, engine)?;
    let index = self.parse_expression(ExprContext::Default, engine)?;
    let close_bracket = self.current_token();
    self.expect(TokenKind::CloseBracket, engine)?;
    start_span.merge(close_bracket.span);

    Ok(Expr::Index {
      object: Box::new(object),
      index: Box::new(index),
      span: start_span,
    })
  }

  fn parse_field_or_method(
    &mut self,
    object: Expr,
    engine: &mut DiagnosticEngine,
  ) -> Result<Expr, ()> {
    let mut start_span = object.span();
    self.expect(TokenKind::Dot, engine)?;

    let token = self.current_token();

    // Look ahead for method call
    if let TokenKind::Ident = token.kind {
      let name = self
        .source_file
        .src
        .get(token.span.start..token.span.end)
        .unwrap()
        .to_string();

      self.advance(engine);

      // If next is '(', it's a method call
      if !self.is_eof() && self.peek(1).kind == TokenKind::OpenParen {
        self.expect(TokenKind::OpenParen, engine)?;
        let args = self.parse_call_params(engine)?;
        let close_paren = self.current_token();
        self.expect(TokenKind::CloseParen, engine)?;
        start_span.merge(close_paren.span);

        return Ok(Expr::MethodCall {
          receiver: Box::new(object),
          method: name,
          turbofish: None,
          args,
          span: start_span,
        });
      }

      // Otherwise it's a field access
      start_span.merge(token.span);
      return Ok(Expr::Field {
        object: Box::new(object),
        field: FieldAccess::Named(name),
        span: start_span,
      });
    }

    // Tuple indexing: .0, .1, etc.
    if let TokenKind::Literal {
      kind: LiteralKind::Integer { .. },
    } = token.kind
    {
      let value_str = self
        .source_file
        .src
        .get(token.span.start..token.span.end)
        .unwrap();
      let index = value_str.parse::<usize>().unwrap_or(0);
      self.advance(engine);
      start_span.merge(token.span);

      return Ok(Expr::Field {
        object: Box::new(object),
        field: FieldAccess::Unnamed(index),
        span: start_span,
      });
    }

    Err(())
  }

  fn parse_method_call(&mut self, callee: Expr, engine: &mut DiagnosticEngine) -> Result<Expr, ()> {
    let mut start_span = callee.span();

    self.expect(TokenKind::OpenParen, engine)?;
    let args = self.parse_call_params(engine)?;
    let close_paren = self.current_token();
    self.expect(TokenKind::CloseParen, engine)?;
    start_span.merge(close_paren.span);

    Ok(Expr::Call {
      callee: Box::new(callee),
      args,
      span: start_span,
    })
  }

  // fn parse_index(&mut self, object: Expr, engine: &mut DiagnosticEngine) -> Result<Expr, ()> {
  //   let start_span = object.span();
  //
  //   self.expect(TokenKind::OpenBracket, engine)?;
  //   let index = self.parse_expression(engine)?;
  //   let close_bracket = self.current_token();
  //   self.expect(TokenKind::CloseBracket, engine)?;
  //
  //   Ok(Expr::Index {
  //     object: Box::new(object),
  //     index: Box::new(index),
  //     span: start_span.merge(close_bracket.span),
  //   })
  // }

  fn parse_call_params(&mut self, engine: &mut DiagnosticEngine) -> Result<Vec<Expr>, ()> {
    let mut exprs = vec![];

    while !self.is_eof() && self.current_token().kind != TokenKind::CloseParen {
      // NOTE: move the higher precednce when you make more productions ready to aboid bugs like
      // `foo(foo(1, 2), 3)` thus it will be unable to parse the nested call
      let expr = self.parse_postfix(engine)?;
      exprs.push(expr);

      if matches!(self.current_token().kind, TokenKind::Comma) {
        self.expect(TokenKind::Comma, engine)?;
      }

      if matches!(self.current_token().kind, TokenKind::CloseParen) {
        break;
      }
    }

    Ok(exprs)
  }

  /* -------------------------------------------------------------------------------------------- */
  /*                                     Primary Parsing                                          */
  /* -------------------------------------------------------------------------------------------- */

  fn parse_primary(&mut self, engine: &mut DiagnosticEngine) -> Result<Expr, ()> {
    let token = self.current_token();

    match token.kind {
      TokenKind::Literal { kind } => self.parser_literal(engine, &token, kind),
      TokenKind::Ident => self.parser_ident(engine, &token),
      TokenKind::KwFalse | TokenKind::KwTrue => self.parser_bool(engine, &token),
      TokenKind::KwSelfValue | TokenKind::KwSuper | TokenKind::KwCrate | TokenKind::KwSelfType => {
        self.parse_keyword_ident(engine, &token)
      },
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
