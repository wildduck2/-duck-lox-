/*
*
* program        → declaration* EOF ;
*
* declaration    → classDecl
*                | funDecl
*                | varDecl
*                | stmt ;
*
* classDecl      → "class" IDENTIFIER "{" declaration* "}" ;
*
* funDecl        → "fun" function;
*
* function       → IDENTIFIER? "(" parameters ")" block;
*
* parameters     → IDENTIFIER ( "," IDENTIFIER )* ;
*
* varDecl        → "var" IDENTIFIER ( "=" expr )? ";" ;
*
* stmt           → expr_stmt
*                | for_stmt
*                | if_stmt
*                | return_stmt
*                | break_stmt
*                | continue_stmt
*                | print_stmt
*                | while_stmt
*                | block ;
*
* break_stmt     → "break" ";" ;
*
* continue_stmt  → "continue" ";" ;
*
* return_stmt    → "return" expr? ";" ;
*
*
* for_stmt       → "for" "(" ( varDec | expr_stmt | ";" ) expr? ";" expr? ")" stmt ;
*
* while_stmt     → "while" "(" expr ")" stmt ;
*
* if_stmt        → "if" "(" expr ")" stmt ( "else" stmt )? ;
*
* block          → "{" declaration* "}" ;
*
* expr_stmt      → expr ";" ;
*
* expr           → comma ;
*
* comma          → assignment ( "," assignment )* ;
*
* assignment     → (call ".")? IDENTIFIER "=" assignment
*                | ternary ;
*
* ternary        → logical_or ( "?" expr ":" ternary )? ;
*
* logical_or     → logical_and ( "or" logical_and )* ;
*
* logical_and    → equality ( "and" equality )* ;
*
* equality       → comparison ( ( "!=" | "==" ) comparison )* ;
*
* comparison     → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
*
* term           → factor ( ( "-" | "+" ) factor )* ;
*
* factor         → unary ( ( "/" | "*" | "%" ) unary )* ;
*
* unary          → ( "!" | "-" ) unary
*                | call ;
*
* call           → primary ( "(" arguments? ")" | "." IDENTIFIER )* ;
*
* arguments      → expr ( "," expr )* ;
*
* primary        → NUMBER | STRING | IDENTIFIER
*                | "true" | "false" | "nil"
*                | "(" expr ")" ;
*
*/

use diagnostic::{
  diagnostic::{Diagnostic, Label, Span},
  diagnostic_code::DiagnosticCode,
  DiagnosticEngine,
};
use scanner::token::{
  types::{Literal, TokenType},
  Token,
};

use crate::{expr::Expr, stmt::Stmt};

pub mod expr;
pub mod stmt;

pub struct Parser {
  /// The tokens preduced by the scanner
  pub tokens: Vec<Token>,
  /// The pointer to the current token we are looking at
  pub current: usize,
  /// List of exprs
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
}

impl Parser {
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
      TokenType::Fun => self.parse_fun_stmt(engine),
      TokenType::Class => self.parse_class_stmt(engine),
      _ => self.parse_stmt(engine),
    }
  }

  fn parse_class_stmt(&mut self, engine: &mut DiagnosticEngine) -> Result<Stmt, ()> {
    self.expect(TokenType::Class, engine)?;
    let name = self.parse_primary(engine)?;

    self.expect(TokenType::LeftBrace, engine)?;

    let mut methods = vec![];
    let mut static_methods = vec![];

    while !self.is_eof() && !matches!(self.current_token().token_type, TokenType::RightBrace) {
      let is_static = self.current_token().lexeme == "static";
      if is_static {
        self.advance();
      }

      let method_name = self.parse_primary(engine)?;

      if !matches!(self.current_token().token_type, TokenType::LeftParen) {
        break;
      }
      self.advance(); // consume the "("

      let params = if matches!(self.current_token().token_type, TokenType::RightParen) {
        vec![]
      } else {
        self.parse_parameters(engine)?
      };
      self.advance(); // consume the ")"

      let body = self.parse_block_stmt(engine)?;

      let method = Stmt::Fun(method_name, params, Box::new(body));
      if is_static {
        static_methods.push(method);
      } else {
        methods.push(method);
      }
    }
    self.expect(TokenType::RightBrace, engine)?;

    Ok(Stmt::Class(
      name,
      Box::new(methods),
      Box::new(static_methods),
    ))
  }

  fn parse_fun_stmt(&mut self, engine: &mut DiagnosticEngine) -> Result<Stmt, ()> {
    self.expect(TokenType::Fun, engine)?;
    let fn_name = if !matches!(self.current_token().token_type, TokenType::LeftParen) {
      Some(self.parse_primary(engine)?)
    } else {
      None
    };

    self.advance(); // consume the "("
    let params = if matches!(self.current_token().token_type, TokenType::RightParen) {
      vec![]
    } else {
      let params = self.parse_parameters(engine)?;
      params
    };

    self.advance(); // consume the ")"
    let body = self.parse_block_stmt(engine)?;

    match fn_name {
      Some(name) => Ok(Stmt::Fun(name, params, Box::new(body))),

      None => {
        let uuid = uuid::Uuid::now_v7();
        Ok(Stmt::Fun(
          Expr::Identifier(Token {
            token_type: TokenType::Identifier,
            lexeme: uuid.to_string().split_once('-').unwrap().0.to_string(),
            literal: Literal::Nil,
            position: (0, 0),
          }),
          params,
          Box::new(body),
        ))
      },
    }
  }

  fn parse_parameters(&mut self, engine: &mut DiagnosticEngine) -> Result<Vec<Expr>, ()> {
    let mut args = vec![];

    fn check_iditifer(expr: &Expr, parser: &mut Parser, engine: &mut DiagnosticEngine) -> bool {
      let check = matches!(expr, Expr::Identifier(_));

      if !check {
        let token = parser.tokens[parser.current - 2].clone();
        let diag = Diagnostic::new(
          DiagnosticCode::UnexpectedToken,
          "Unexpected parameter".to_string(),
        )
        .with_label(Label::primary(
          token.to_span(),
          Some("Unexpected Parameter".to_string()),
        ));

        engine.emit(diag);
      }

      check
    }

    // Parse first argument
    let expr = self.parse_primary(engine)?;
    check_iditifer(&expr, self, engine);
    args.push(expr);

    if args.len() >= 255 {
      let diagnostic = Diagnostic::new(
        DiagnosticCode::WrongNumberOfArguments,
        "Too many arguments".to_string(),
      )
      .with_label(Label::primary(
        self.current_token().to_span(),
        Some("too many arguments".to_string()),
      ));
      engine.emit(diagnostic);

      return Err(());
    }

    // Parse remaining arguments separated by commas
    while !self.is_eof() && self.matches_token(TokenType::Comma) {
      self.advance(); // consume ","

      // Check for trailing comma: foo(1, 2, )
      if self.matches_token(TokenType::RightParen) {
        // Could emit a warning here about trailing comma
        break;
      }

      let expr = self.parse_primary(engine)?;
      check_iditifer(&expr, self, engine);
      args.push(expr);
    }

    Ok(args)
  }

  fn parse_stmt(&mut self, engine: &mut DiagnosticEngine) -> Result<Stmt, ()> {
    match self.current_token().token_type {
      TokenType::For => self.parse_for_stmt(engine),
      TokenType::Break => self.parse_break_stmt(engine),
      TokenType::Continue => self.parse_continue_stmt(engine),
      TokenType::If => self.parse_if_stmt(engine),
      TokenType::LeftBrace => self.parse_block_stmt(engine),
      TokenType::Return => self.parse_return_stmt(engine),
      TokenType::While => self.parse_while_stmt(engine),
      _ => self.parse_expr_stmt(engine),
    }
  }

  fn parse_break_stmt(&mut self, engine: &mut DiagnosticEngine) -> Result<Stmt, ()> {
    let token = self.current_token();
    self.expect(TokenType::Break, engine)?;
    self.expect(TokenType::SemiColon, engine)?;

    // Ok(Stmt::Block(Box::new(vec![Stmt::Break(
    //   self.current_token(),
    // )])))

    Ok(Stmt::Break(token))
  }

  fn parse_continue_stmt(&mut self, engine: &mut DiagnosticEngine) -> Result<Stmt, ()> {
    let token = self.current_token();
    self.expect(TokenType::Continue, engine)?;
    self.expect(TokenType::SemiColon, engine)?;

    // Ok(Stmt::Block(Box::new(vec![Stmt::Continue(
    //   self.current_token(),
    // )])))

    Ok(Stmt::Continue(token))
  }

  fn parse_return_stmt(&mut self, engine: &mut DiagnosticEngine) -> Result<Stmt, ()> {
    let token = self.current_token();
    self.expect(TokenType::Return, engine)?;

    if matches!(self.current_token().token_type, TokenType::SemiColon) {
      return Ok(Stmt::Return(self.current_token(), None));
    }

    let value = self.parse_expr(engine)?;

    if !matches!(self.current_token().token_type, TokenType::SemiColon) {
      let diagnostic = Diagnostic::new(
        DiagnosticCode::MissingSemicolon,
        "Expected ';' after return value".to_string(),
      )
      .with_label(Label::primary(
        self.current_token().to_span(),
        Some("semicolon missing here".to_string()),
      ));

      engine.emit(diagnostic);
      return Err(());
    }

    self.advance();
    Ok(Stmt::Return(token, Some(value)))
  }

  fn parse_var_stmt(&mut self, engine: &mut DiagnosticEngine) -> Result<Stmt, ()> {
    self.expect(TokenType::Var, engine)?;

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

    if matches!(self.current_token().token_type, TokenType::SemiColon) {
      self.advance(); // consume ;
      return Ok(Stmt::VarDecl(identifier, None));
    } else if matches!(self.current_token().token_type, TokenType::Equal) {
      self.advance(); // consume =
                      // TODO: parse the caller in the declaration

      let mut is_function = false;
      let expr;

      if matches!(self.current_token().token_type, TokenType::Identifier)
        && self.tokens[self.current + 1].token_type == TokenType::LeftParen
      {
        let callee = self.parse_call(engine)?;
        expr = callee;
      } else if matches!(self.current_token().token_type, TokenType::Fun) {
        is_function = true;
        let fun = self.parse_fun_stmt(engine)?;

        if let Stmt::Fun(name, _, _) = &fun {
          expr = name.clone();
        } else {
          return Err(());
        }
        self.ast.push(fun);
      } else {
        expr = self.parse_expr(engine)?;
      }

      if matches!(self.current_token().token_type, TokenType::SemiColon) || is_function {
        if !is_function {
          self.advance(); // consume ;
        }
        return Ok(Stmt::VarDecl(identifier, Some(expr)));
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
  }

  fn parse_for_stmt(&mut self, engine: &mut DiagnosticEngine) -> Result<Stmt, ()> {
    self.expect(TokenType::For, engine)?;
    self.expect(TokenType::LeftParen, engine)?;

    // Parse initializer
    let initializer = if self.matches_token(TokenType::SemiColon) {
      self.advance();
      None
    } else if self.matches_token(TokenType::Var) {
      Some(self.parse_declaration(engine)?)
    } else {
      Some(self.parse_expr_stmt(engine)?)
    };

    // Parse condition
    let condition = if !self.matches_token(TokenType::SemiColon) {
      let expr = self.parse_expr(engine)?;
      self.expect(TokenType::SemiColon, engine)?;
      Some(expr)
    } else {
      self.advance();
      None
    };

    // Parse increment
    let increment = if !self.matches_token(TokenType::RightParen) {
      let expr = self.parse_expr(engine)?;
      self.expect(TokenType::RightParen, engine)?;
      Some(expr)
    } else {
      self.advance();
      None
    };

    // Parse body
    let mut body = self.parse_stmt(engine)?;

    // Desugar: add increment to body
    if let Some(inc) = increment {
      body = Stmt::Block(Box::new(vec![body, Stmt::Expr(inc)]));
    }

    // Desugar: wrap in while loop
    let condition_expr = condition.unwrap_or(Expr::Literal(Token::new(
      TokenType::True,
      "true".to_string(),
      Literal::Boolean,
      (0, 0),
    )));
    body = Stmt::While(Box::new(condition_expr), Box::new(body));

    // Desugar: add initializer
    if let Some(init) = initializer {
      Ok(Stmt::Block(Box::new(vec![init, body])))
    } else {
      Ok(body)
    }
  }

  fn parse_while_stmt(&mut self, engine: &mut DiagnosticEngine) -> Result<Stmt, ()> {
    self.expect(TokenType::While, engine)?;
    self.expect(TokenType::LeftParen, engine)?;
    let condition = self.parse_expr(engine)?;
    self.expect(TokenType::RightParen, engine)?;
    let stmt = self.parse_stmt(engine)?;

    Ok(Stmt::While(Box::new(condition), Box::new(stmt)))
  }

  fn parse_if_stmt(&mut self, engine: &mut DiagnosticEngine) -> Result<Stmt, ()> {
    self.expect(TokenType::If, engine)?;
    self.expect(TokenType::LeftParen, engine)?;
    let expr = self.parse_expr(engine)?;
    self.expect(TokenType::RightParen, engine)?;

    let stmt = match self.parse_stmt(engine)? {
      Stmt::Block(block) => Stmt::Block(block),
      stmt => Stmt::Block(Box::new(vec![stmt])),
    };

    if !self.matches_token(TokenType::Else) {
      return Ok(Stmt::If(Box::new(expr), Box::new(stmt), None));
    }

    self.advance();

    // Handle else-if chain
    let else_branch = if self.matches_token(TokenType::If) {
      self.parse_if_stmt(engine)?
    } else {
      self.parse_stmt(engine)?
    };

    Ok(Stmt::If(
      Box::new(expr),
      Box::new(stmt),
      Some(Box::new(else_branch)),
    ))
  }

  fn parse_block_stmt(&mut self, engine: &mut DiagnosticEngine) -> Result<Stmt, ()> {
    self.expect(TokenType::LeftBrace, engine)?;
    let mut declarations = Vec::new();

    while !self.is_eof() && !self.matches_token(TokenType::RightBrace) {
      declarations.push(self.parse_declaration(engine)?);
    }

    self.expect(TokenType::RightBrace, engine)?;
    Ok(Stmt::Block(Box::new(declarations)))
  }

  fn parse_expr_stmt(&mut self, engine: &mut DiagnosticEngine) -> Result<Stmt, ()> {
    let expr = self.parse_expr(engine)?;
    self.expect(TokenType::SemiColon, engine)?;
    Ok(Stmt::Expr(expr))
  }

  // Helper method to check if current token matches a type
  fn matches_token(&self, token_type: TokenType) -> bool {
    !self.is_eof() && self.tokens[self.current].token_type == token_type
  }
}

impl Parser {
  /// Function that handles expr
  fn parse_expr(&mut self, engine: &mut DiagnosticEngine) -> Result<Expr, ()> {
    self.parse_comma(engine)
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

  /// Function that handles the assignments (=)
  fn parse_assignment(&mut self, engine: &mut DiagnosticEngine) -> Result<Expr, ()> {
    let lhs = self.parse_ternary(engine)?;

    if !self.is_eof() && matches!(self.current_token().token_type, TokenType::Equal) {
      self.advance();

      let rhs = self.parse_assignment(engine)?;

      if let Expr::Identifier(name) = lhs {
        return Ok(Expr::Assign {
          name: name,
          value: Box::new(rhs),
        });
      } else if let Expr::Get { object, name } = lhs {
        return Ok(Expr::Set {
          name,
          object,
          value: Box::new(rhs),
        });
      } else {
        self.error_unexpected_token(engine, "in assignment, left side must be an identifier");
        return Err(());
      }
    }

    Ok(lhs)
  }

  /// Function that handles the ternary (?:)
  fn parse_ternary(&mut self, engine: &mut DiagnosticEngine) -> Result<Expr, ()> {
    let condition = self.parse_logic_or(engine)?;

    if !self.is_eof() && matches!(self.current_token().token_type, TokenType::Question) {
      let question_token = self.current_token();
      self.advance(); // consume the (?)

      let then_branch = self.parse_expr(engine)?;

      if self.is_eof() || !matches!(self.current_token().token_type, TokenType::Colon) {
        let current_token = self.current_token();

        let error = Diagnostic::new(
          DiagnosticCode::UnexpectedToken,
          format!(
            "Expected ':' in ternary expr, found '{}'",
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
          "Ternary exprs require the format: condition ? then_value : else_value".to_string(),
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

  fn parse_logic_or(&mut self, engine: &mut DiagnosticEngine) -> Result<Expr, ()> {
    let mut lhs = self.parse_logic_and(engine)?;

    while !self.is_eof() && matches!(self.current_token().token_type, TokenType::Or) {
      let token = self.current_token();
      self.advance(); // consume the &&
      let rhs = self.parse_logic_and(engine)?;
      lhs = Expr::Binary {
        lhs: Box::new(lhs),
        operator: token,
        rhs: Box::new(rhs),
      }
    }

    Ok(lhs)
  }

  fn parse_logic_and(&mut self, engine: &mut DiagnosticEngine) -> Result<Expr, ()> {
    let mut lhs = self.parse_equality(engine)?;

    while !self.is_eof() && matches!(self.current_token().token_type, TokenType::And) {
      let token = self.current_token();
      self.advance(); // consume the &&
      let rhs = self.parse_equality(engine)?;
      lhs = Expr::Binary {
        lhs: Box::new(lhs),
        operator: token,
        rhs: Box::new(rhs),
      }
    }

    Ok(lhs)
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
        TokenType::Divide | TokenType::Multiply | TokenType::Modulus => {
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

  /// Parse unary: ( "!" | "-" ) unary | call

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
      _ => self.parse_call(engine), // Changed from parse_primary
    }
  }

  /// Parse call: primary ( "(" arguments? ")" )*
  fn parse_call(&mut self, engine: &mut DiagnosticEngine) -> Result<Expr, ()> {
    // Start with a primary expression (the callee)
    let mut expr = self.parse_primary(engine)?;

    // Loop to handle chained calls: foo()()()
    while !self.is_eof() {
      match self.current_token().token_type {
        // TokenType::LeftParen => {
        //   self.advance(); // consume '('
        //
        //   // Parse arguments (if any)
        //   let args = if !self.matches_token(TokenType::RightParen) {
        //     self.parse_arguments(engine)?
        //   } else {
        //     Vec::new() // No arguments
        //   };
        //
        //   let token = self.current_token();
        //   self.expect(TokenType::RightParen, engine)?;
        //
        //   // Wrap in a Call expression
        //   callee = Expr::Call {
        //     callee: Box::new(callee),
        //     paren: token, // The ')'
        //     arguments: args,
        //   };
        // },
        TokenType::LeftParen => {
          // Handle function/method call: foo(), foo.bar(), etc.
          self.advance(); // consume '('

          let mut args = Vec::new();
          if !matches!(self.current_token().token_type, TokenType::RightParen) {
            args = self.parse_arguments(engine)?;
          }

          let paren = self.current_token(); // For error tracking
          self.expect(TokenType::RightParen, engine)?;

          expr = Expr::Call {
            callee: Box::new(expr),
            paren,
            arguments: args,
          };
        },

        TokenType::Dot => {
          self.advance(); // consume the "."
          let name = self.current_token();
          if name.token_type != TokenType::Identifier {
            eprintln!("Expected property name after '.'");
            return Err(());
          }
          self.advance(); // consume identifier

          expr = Expr::Get {
            object: Box::new(expr),
            name: name,
          };
        },
        _ => break, // No more calls
      }
    }

    Ok(expr)
  }

  /// Parse arguments: expr ( "," expr )*
  fn parse_arguments(&mut self, engine: &mut DiagnosticEngine) -> Result<Vec<Expr>, ()> {
    let mut args = vec![];

    // Parse first argument
    args.push(self.parse_assignment(engine)?);

    if args.len() >= 255 {
      let diagnostic = Diagnostic::new(
        DiagnosticCode::WrongNumberOfArguments,
        "Too many arguments".to_string(),
      )
      .with_label(Label::primary(
        self.current_token().to_span(),
        Some("too many arguments".to_string()),
      ));
      engine.emit(diagnostic);

      return Err(());
    }

    // Parse remaining arguments separated by commas
    while !self.is_eof() && self.matches_token(TokenType::Comma) {
      self.advance(); // consume ","

      // Check for trailing comma: foo(1, 2, )
      if self.matches_token(TokenType::RightParen) {
        // Could emit a warning here about trailing comma
        break;
      }

      args.push(self.parse_assignment(engine)?);
    }

    Ok(args)
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

      TokenType::This => {
        self.advance();
        return Ok(Expr::This(token));
      },

      TokenType::Identifier => {
        self.advance();
        return Ok(Expr::Identifier(token));
      },

      TokenType::LeftParen => {
        let opening_paren_token = self.current_token();
        self.advance(); // consume '('

        let expr = self.parse_expr(engine)?;

        if self.is_eof() || self.current_token().token_type != TokenType::RightParen {
          let current = self.current_token();

          // For EOF, use the PREVIOUS token's end position
          let error_span = if self.is_eof() {
            let prev_token = &self.tokens[self.current - 1];
            Span {
              // TODO: add the real file name
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
            "Expected ')' after expr".to_string(),
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
        self.check_double_semicolon(engine);
        Err(())
      },

      TokenType::Fun => {
        let fun = self.parse_fun_stmt(engine)?;
        let token;
        if let Stmt::Fun(name, _, _) = &fun {
          token = name.clone();
        } else {
          return Err(());
        }

        self.ast.push(fun);
        if let Expr::Identifier(name) = token {
          return Ok(Expr::Identifier(name));
        } else {
          return Err(());
        }
      },

      _ => {
        let mut token = self.current_token();
        token.position.1 = 0;
        let diagnostic = Diagnostic::new(
          DiagnosticCode::ExpectedExpression,
          "Expected expr".to_string(),
        )
        .with_label(Label::primary(
          token.to_span(),
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

  /// Function that consume the code until there's valid tokens to start a new expr
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

  fn check_double_semicolon(&mut self, engine: &mut DiagnosticEngine) {
    if !self.is_eof() && matches!(self.current_token().token_type, TokenType::SemiColon) {
      let mut token = self.current_token();
      token.position.0 += 1;
      token.position.1 -= 1;

      let diagnostic = Diagnostic::new(
        DiagnosticCode::UnexpectedToken,
        "Unexpected extra semicolon".to_string(),
      )
      .with_label(Label::primary(
        token.to_span(),
        Some("remove this extra semicolon".to_string()),
      ));
      engine.emit(diagnostic);
      self.advance(); // skip the extra semicolon
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
      Some("expected some expr".to_string()),
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

impl Parser {
  /// Expects a specific token type and provides detailed error diagnostics if not found
  fn expect(&mut self, expected: TokenType, engine: &mut DiagnosticEngine) -> Result<Token, ()> {
    if self.is_eof() {
      self.error_expected_token_eof(expected, engine);
      return Err(());
    }

    let current = self.current_token();

    if current.token_type == expected {
      self.advance();
      Ok(current)
    } else {
      self.error_expected_token(expected, current, engine);
      Err(())
    }
  }

  /// Error for when we expect a token but hit EOF
  fn error_expected_token_eof(&mut self, expected: TokenType, engine: &mut DiagnosticEngine) {
    let token = self.current_token();
    let last_token = &self.tokens[self.current - 1];

    let error_span = Span {
      file: last_token.to_span().file.clone(),
      line: token.position.0,
      column: token.position.1,
      length: 1,
    };

    let diagnostic = Diagnostic::new(
      DiagnosticCode::UnexpectedEof,
      format!(
        "Expected '{}', but reached end of file",
        expected.to_string()
      ),
    )
    .with_label(Label::primary(
      error_span,
      Some(format!("expected '{}' here", expected.to_string())),
    ))
    .with_label(Label::secondary(
      last_token.to_span(),
      Some("after this token".to_string()),
    ));

    engine.emit(diagnostic);
  }

  /// Error for when we expect a token but find something else
  fn error_expected_token(&self, expected: TokenType, found: Token, engine: &mut DiagnosticEngine) {
    let diagnostic = Diagnostic::new(
      DiagnosticCode::UnexpectedToken,
      format!(
        "Expected '{}', found '{}'",
        &expected.to_string(),
        found.lexeme
      ),
    )
    .with_label(Label::primary(
      found.to_span(),
      Some(format!("expected '{}' here", &expected.to_string())),
    ))
    .with_help(get_token_help(&expected, &found));

    engine.emit(diagnostic);
  }
}
/// Helper function to convert TokenType to a readable string

/// Provides contextual help based on what was expected vs found
fn get_token_help(expected: &TokenType, found: &Token) -> String {
  match (expected, &found.token_type) {
    (TokenType::SemiColon, _) => "Statements must end with a semicolon".to_string(),
    (TokenType::RightParen, TokenType::SemiColon) => {
      "Did you forget to close the parentheses before the semicolon?".to_string()
    },
    (TokenType::RightBrace, TokenType::Eof) => {
      "Did you forget to close a block with '}'?".to_string()
    },
    (TokenType::LeftParen, _) => {
      "Control flow statements require parentheses around conditions".to_string()
    },
    (TokenType::Colon, TokenType::SemiColon) => {
      "Ternary expressions use ':' to separate the branches".to_string()
    },
    (TokenType::Equal, _) => "Use '=' for assignment".to_string(),
    _ => String::new(),
  }
}
