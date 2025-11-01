/*!

### Grammar (BNF-style)

program        → declaration* EOF ;

declaration    → varDecl | fnDecl | structDecl | traitDecl | implBlock | statement ;

varDecl        → ("let" | "const") IDENTIFIER ( ":" type )? ( "=" expression )? ";" ;

fnDecl         → "fn" IDENTIFIER "(" parameters? ")" "->" type block ;
structDecl     → "struct" IDENTIFIER "{" fields "}" ;
traitDecl      → "trait" IDENTIFIER "{" fnSignatures "}" ;
implBlock      → "impl" IDENTIFIER ("for" IDENTIFIER)? "{" fnDecl* "}" ;

statement      → exprStmt | ifStmt | whileStmt | forStmt | returnStmt | block ;

exprStmt       → expression ";" ;
ifStmt         → "if" expression block ( "else" block )? ;
whileStmt      → "while" expression block ;
forStmt        → "for" IDENTIFIER "in" expression block ;
returnStmt     → "return" expression? ";" ;
block          → "{" declaration* "}" ;

expression     → assignment ;
assignment     → ( call "." )? IDENTIFIER "=" assignment | logicOr ;
logicOr        → logicAnd ( "or" logicAnd )* ;
logicAnd       → equality ( "and" equality )* ;
equality       → comparison ( ( "==" | "!=" ) comparison )* ;
comparison     → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
term           → factor ( ( "+" | "-" ) factor )* ;
factor         → unary ( ( "*" | "/" | "%" ) unary )* ;
unary          → ( "!" | "-" ) unary | call ;
call           → primary ( "(" arguments? ")" | "." IDENTIFIER | "[" expression "]" )* ;

primary        → INTEGER | FLOAT | STRING | "true" | "false" | "nil"
               | IDENTIFIER
               | "(" tupleOrGrouping ")"
               | array | object | lambda | match ;

tupleOrGrouping → expression ( "," expression )* ;

type           → "int" | "float" | "string" | "bool" | "void"
               | "[" type "]"
               | "(" type ( "," type )* ")" "->" type
               | IDENTIFIER
               | IDENTIFIER "<" type ( "," type )* ">" ;


*/

use diagnostic::{
  code::DiagnosticCode,
  diagnostic::{Diagnostic, LabelStyle, Span},
  types::{error::DiagnosticError, warning::DiagnosticWarning},
  DiagnosticEngine,
};
use lexer::token::{Token, TokenKind};

use crate::{
  expr::{BinaryOp, Expr, MatchArm, Param, Pattern, UnaryOp},
  stmt::{DeclKind, Stmt, Type},
  Parser,
};

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
    self.parse_stmt(engine)
  }

  /// Parses a single statement node (stubbed for future grammar branches).
  fn parse_stmt(&mut self, engine: &mut DiagnosticEngine) -> Result<Stmt, ()> {
    match self.current_token().kind {
      TokenKind::Let | TokenKind::Const => self.parse_variable_declaration(engine),
      _ => {
        let expr = self.parse_expr(engine)?;
        Ok(Stmt::Expr(expr))
      },
    }
  }

  fn parse_variable_declaration(&mut self, engine: &mut DiagnosticEngine) -> Result<Stmt, ()> {
    let declaration_kind = self.current_token();
    self.advance(engine); // consume the "const"

    let kind = if declaration_kind.kind == TokenKind::Let {
      DeclKind::Let
    } else {
      DeclKind::Const
    };

    let is_mutable = if self.current_token().kind == TokenKind::Mut {
      self.advance(engine); // consume the "mut"
      true
    } else {
      false
    };

    let identifier_token = self.current_token();
    let lhs = self.parse_primary(engine)?;
    let name = match lhs {
      #[allow(unused_variables)]
      Expr::Identifier { name, span } => {
        let is_uppercase = name
          .chars()
          .all(|c| !c.is_ascii_alphabetic() || c.is_ascii_uppercase());

        if kind == DeclKind::Let || is_uppercase {
          name
        } else {
          let diagnostic = Diagnostic::new(
            DiagnosticCode::Warning(DiagnosticWarning::InvalidConstDeclaration),
            "const declarations must be uppercase".to_string(),
            "duck.lox".to_string(),
          )
          .with_label(
            Span::new(
              identifier_token.span.line + 1,
              identifier_token.span.col + 1,
              identifier_token.lexeme.len(),
            ),
            Some("const declarations must be uppercase".to_string()),
            LabelStyle::Primary,
          );

          engine.add(diagnostic);
          return Err(());
        }
      },
      _ => {
        let diagnostic = Diagnostic::new(
          DiagnosticCode::Error(DiagnosticError::UnexpectedToken),
          format!("Expected identifier, found '{:?}'", lhs),
          "duck.lox".to_string(),
        )
        .with_label(
          Span::new(
            declaration_kind.span.line + 1,
            declaration_kind.span.col + 1,
            declaration_kind.span.len,
          ),
          Some(format!("expected identifier here")),
          LabelStyle::Primary,
        );

        engine.add(diagnostic);
        return Err(());
      },
    };

    let type_annotation = if self.current_token().kind == TokenKind::Colon {
      self.advance(engine); // consume the ":" token
      let rhs = self.parse_type(engine)?;
      Some(rhs)
    } else {
      None
    };

    let initializer = if self.current_token().kind == TokenKind::Equal {
      self.advance(engine); // consume the "=" token
      let rhs = self.parse_assignment(engine)?;
      Some(rhs)
    } else {
      None
    };

    self.expect(TokenKind::Semicolon, engine)?; // consume the ";"

    Ok(Stmt::Decl {
      is_mutable,
      kind,
      name,
      type_annotation,
      initializer,
      span: declaration_kind.span,
    })
  }

  /// Parses a general expression entrypoint.
  fn parse_expr(&mut self, engine: &mut DiagnosticEngine) -> Result<Expr, ()> {
    self.parse_comma(engine)
  }

  /// Parses comma-separated expressions, emitting `Expr::Binary` nodes.
  fn parse_comma(&mut self, engine: &mut DiagnosticEngine) -> Result<Expr, ()> {
    let mut lhs = self.parse_assignment(engine)?;

    // while !self.is_eof() || !matches!(self.current_token().kind, TokenKind::Comma) {
    //   let token = self.current_token();
    //
    //   match token.kind {
    //     TokenKind::Comma => {
    //       self.advance(engine);
    //
    //       let rhs = self.parse_assignment(engine)?;
    //     },
    //     _ => break,
    //   }
    // }

    Ok(lhs)
  }

  /// Parses assignment expressions and verifies the left side is assignable.
  fn parse_assignment(&mut self, engine: &mut DiagnosticEngine) -> Result<Expr, ()> {
    let token = self.current_token();
    let lhs = self.parse_ternary(engine)?;

    if !self.is_eof() && matches!(self.current_token().kind, TokenKind::Equal) {
      self.advance(engine);
      let rhs = self.parse_assignment(engine)?;

      if let Expr::Identifier { name: _, span } = lhs.clone() {
        self.expect(TokenKind::Semicolon, engine)?; // consume the ";"
        return Ok(Expr::Assign {
          target: Box::new(lhs),
          value: Box::new(rhs),
          span,
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
        span: token.span,
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
            op: BinaryOp::Or,
            left: Box::new(lhs),
            right: Box::new(rhs),
            span: operator.span,
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
            op: BinaryOp::And,
            left: Box::new(lhs),
            right: Box::new(rhs),
            span: operator.span,
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
            op: match operator.lexeme.as_str() {
              "==" => BinaryOp::Eq,
              "!=" => BinaryOp::NotEq,
              _ => unreachable!(),
            },
            left: Box::new(lhs),
            right: Box::new(rhs),
            span: operator.span,
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
            op: match operator.lexeme.as_str() {
              ">" => BinaryOp::Greater,
              ">=" => BinaryOp::GreaterEq,
              "<" => BinaryOp::Less,
              "<=" => BinaryOp::LessEq,
              _ => unreachable!(),
            },
            left: Box::new(lhs),
            right: Box::new(rhs),
            span: operator.span,
          }
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
            op: match operator.lexeme.as_str() {
              "+" => BinaryOp::Add,
              "-" => BinaryOp::Sub,
              _ => unreachable!(),
            },
            left: Box::new(lhs),
            right: Box::new(rhs),
            span: operator.span,
          }
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
            op: match operator.lexeme.as_str() {
              "%" => BinaryOp::Mod,
              "/" => BinaryOp::Div,
              "*" => BinaryOp::Mul,
              _ => unreachable!(),
            },
            left: Box::new(lhs),
            right: Box::new(rhs),
            span: operator.span,
          }
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
          op: match operator.lexeme.as_str() {
            "!" => UnaryOp::Not,
            "-" => UnaryOp::Neg,
            _ => unreachable!(),
          },
          expr: Box::new(rhs),
          span: operator.span,
        })
      },
      _ => self.parse_call(engine),
    }
  }

  /// Parses function calls and dotted access chains.
  fn parse_call(&mut self, engine: &mut DiagnosticEngine) -> Result<Expr, ()> {
    let callee = self.parse_primary(engine)?;

    if !self.is_eof() && matches!(self.current_token().kind, TokenKind::LeftParen) {
      let token = self.current_token();
      self.advance(engine); // consume the "("
      let args = self.parser_arguments(engine)?;

      return Ok(Expr::Call {
        callee: Box::new(callee),
        args,
        span: token.span,
      });
    }

    Ok(callee)
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

    self.expect(TokenKind::RightParen, engine)?; // consume the ")"

    Ok(args)
  }

  /// Parses primary expressions: literals, identifiers, and grouped expressions.
  fn parse_primary(&mut self, engine: &mut DiagnosticEngine) -> Result<Expr, ()> {
    // primary        → INTEGER | FLOAT | STRING | "true" | "false" | "nil"
    //                | IDENTIFIER
    //                | "(" tupleOrGrouping ")"
    //                | array | object | lambda | match ;

    let token = self.current_token();
    match token.kind {
      // handle string literals
      TokenKind::String => {
        self.advance(engine); // consume this token
        Ok(Expr::String {
          value: token.lexeme,
          span: token.span,
        })
      },

      // handle numeric integers literal
      TokenKind::Float => {
        self.advance(engine); // consume this token
        Ok(Expr::Float {
          value: token.lexeme.parse().unwrap(),
          span: token.span,
        })
      },

      // handle numeric floats literal
      TokenKind::Int => {
        self.advance(engine); // consume this token
        Ok(Expr::Integer {
          value: token.lexeme.parse().unwrap(),
          span: token.span,
        })
      },

      // handle nil literal
      TokenKind::Nil => {
        self.advance(engine); // consume this token
        Ok(Expr::Nil { span: token.span })
      },

      // handle true literal
      TokenKind::True => {
        self.advance(engine); // consume this token
        Ok(Expr::Bool {
          value: true,
          span: token.span,
        })
      },

      // handle false literal
      TokenKind::False => {
        self.advance(engine); // consume this token
        Ok(Expr::Bool {
          value: false,
          span: token.span,
        })
      },

      // handle the case where the token is a keyword
      TokenKind::Identifier => {
        self.advance(engine); // consume this token
        if self.current_token().kind == TokenKind::LeftBrace
          && self.tokens[self.current - 2].kind != TokenKind::Match
        {
          return self.parse_object(token, engine);
        }

        // handle the case where object is declared
        Ok(Expr::Identifier {
          name: token.lexeme,
          span: token.span,
        })
      },

      // handle the case where group is used also for tuple
      TokenKind::LeftParen => self.parse_grouping(engine),

      // handle the case where array is declared
      TokenKind::LeftBracket => self.parse_array(engine),

      // handle the case where lambda is declared
      TokenKind::Fn => self.parse_lambda(engine),

      TokenKind::Match => self.parse_match(engine),

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

  fn parse_match(&mut self, engine: &mut DiagnosticEngine) -> Result<Expr, ()> {
    let token = self.expect(TokenKind::Match, engine)?; // consume the "match"

    let expr = self.parse_primary(engine)?;
    self.expect(TokenKind::LeftBrace, engine)?; // consume the "("
    let arms = self.parse_arms(engine)?;
    self.expect(TokenKind::RightBrace, engine)?; // consume the ")"

    // println!(
    //   "{:#?}",
    //   Expr::Match {
    //     expr: Box::new(expr.clone()),
    //     arms: arms.clone(),
    //     span: token.span.clone(),
    //   }
    // );

    Ok(Expr::Match {
      expr: Box::new(expr),
      arms,
      span: token.span,
    })
  }

  fn parse_arms(&mut self, engine: &mut DiagnosticEngine) -> Result<Vec<MatchArm>, ()> {
    let mut arms = Vec::<MatchArm>::new();

    while !self.is_eof() && !matches!(self.current_token().kind, TokenKind::RightBrace) {
      let pattern = self.parse_pattern(engine)?;

      let guard = if self.current_token().kind == TokenKind::If {
        self.advance(engine); // consume the "if"
        let guard = self.parse_logical_or(engine)?;
        Some(guard)
      } else {
        None
      };

      // TODO: make FatArrow a TokenKind
      self.expect(TokenKind::Equal, engine)?; // consume the "="
      self.expect(TokenKind::Greater, engine)?; // consume the ">"

      // Parse body (either block or single expression)
      let body = if self.current_token().kind == TokenKind::LeftBrace {
        self.parse_block(engine)?
      } else {
        // Single expression - wrap in statement
        let expr = self.parse_expr(engine)?;
        vec![Stmt::Expr(expr)]
      };

      arms.push(MatchArm {
        pattern,
        guard,
        body,
      });

      // Optional comma between arms
      if self.current_token().kind == TokenKind::Comma {
        self.advance(engine);
      }
    }

    Ok(arms)
  }

  fn parse_pattern(&mut self, engine: &mut DiagnosticEngine) -> Result<Pattern, ()> {
    self.parse_or_pattern(engine)
  }

  fn parse_or_pattern(&mut self, engine: &mut DiagnosticEngine) -> Result<Pattern, ()> {
    let mut patterns = vec![self.parse_single_pattern(engine)?];

    while self.current_token().kind == TokenKind::Pipe {
      self.advance(engine); // consume '|'
      patterns.push(self.parse_single_pattern(engine)?);
    }

    if patterns.len() == 1 {
      Ok(patterns.into_iter().next().unwrap())
    } else {
      Ok(Pattern::Or(patterns))
    }
  }

  fn parse_single_pattern(&mut self, engine: &mut DiagnosticEngine) -> Result<Pattern, ()> {
    let token = self.current_token();
    match token.kind {
      // Wildcard: _
      TokenKind::Underscore => {
        self.advance(engine);
        Ok(Pattern::Wildcard)
      },

      // Literals: 42, "hello", true, false
      TokenKind::Int
      | TokenKind::Float
      | TokenKind::String
      | TokenKind::True
      | TokenKind::False
      | TokenKind::Nil => {
        if self.peek().kind == TokenKind::DotDot {
          self.advance(engine); // consume the "consume the token
          let start = Expr::Integer {
            value: token.lexeme.parse().unwrap(),
            span: token.span,
          };

          self.expect(TokenKind::DotDot, engine)?; // consume the ".."
          self.expect(TokenKind::Equal, engine)?; // consume the "="
          let end = self.parse_primary(engine)?;

          return Ok(Pattern::Range { start, end });
        }

        let expr = self.parse_primary(engine)?;
        Ok(Pattern::Literal(expr))
      },

      TokenKind::LeftParen => {
        let tuple = self.parse_tuple(engine)?;
        Ok(Pattern::Literal(tuple))
      },

      // Identifier or Struct: x or Person { ... }
      TokenKind::Identifier => {
        let struct_name = self.current_token();
        self.advance(engine); // consume the "struct name"
        let mut fields = Vec::<(String, Pattern)>::new();

        let is_struct = self.current_token().kind == TokenKind::LeftBrace;

        if self.current_token().kind == TokenKind::LeftBrace
          || self.current_token().kind == TokenKind::LeftParen
        {
          self.advance(engine); // consume the "{"
          while !self.is_eof()
            && self.current_token().kind != TokenKind::RightBrace
            && self.current_token().kind != TokenKind::RightParen
          {
            let field_name = self.current_token();
            self.advance(engine);

            let field_pattern = if self.current_token().kind == TokenKind::Colon {
              self.advance(engine);
              self.parse_pattern(engine)?
            } else {
              Pattern::Identifier(field_name.lexeme.clone())
            };

            fields.push((field_name.lexeme, field_pattern));

            if self.current_token().kind == TokenKind::Comma {
              self.advance(engine);
            }
          }

          if is_struct {
            self.expect(TokenKind::RightBrace, engine)?;
            Ok(Pattern::Struct {
              name: struct_name.lexeme,
              fields,
            })
          } else {
            // TODO: make this work you idiot
            self.expect(TokenKind::RightParen, engine)?;
            Ok(Pattern::Enum {
              name: struct_name.lexeme,
              variant: variant_name.lexeme,
              patterns,
            })
          }
        } else {
          Ok(Pattern::Identifier(struct_name.lexeme))
        }
      },

      _ => {
        let diagnostic = Diagnostic::new(
          DiagnosticCode::Error(DiagnosticError::UnexpectedToken),
          format!("Unexpected token {:?}", token.lexeme),
          "duck.lox".to_string(),
        )
        .with_label(
          Span::new(token.span.line + 1, token.span.col + 1, token.span.len),
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

  fn parse_tuple(&mut self, engine: &mut DiagnosticEngine) -> Result<Expr, ()> {
    let token = self.expect(TokenKind::LeftParen, engine)?; // consume the "("

    let mut fields = Vec::<(String, Pattern)>::new();
    let is_struct = self.current_token().kind == TokenKind::LeftBrace;

    while !self.is_eof()
      && self.current_token().kind != TokenKind::RightParen
      && self.current_token().kind != TokenKind::RightBrace
    {
      if self.current_token().kind != TokenKind::Underscore {
        self.advance(engine); // consume the "_"
        continue;
      }
      let expr = if self.current_token().kind == TokenKind::Underscore {
        self.advance(engine); // consume the "_"
        Expr::Identifier {
          name: "_".to_string(),
          span: self.current_token().span,
        }
      } else {
        self.parse_expr(engine)?
      };

      fields.push((expr, None));

      if self.current_token().kind != TokenKind::Comma {
        break;
      }

      self.advance(engine); // consume the ","
    }

    if is_struct {
      self.expect(TokenKind::RightBrace, engine)?;
      Ok(Pattern::Struct {
        name: "tuple".to_string(),
        fields: elements,
      })
    } else {
      self.expect(TokenKind::RightParen, engine)?;
      Ok(Pattern::Enum {
        name: "tuple".to_string(),
        fields: elements,
      })
    }
    // self.expect(TokenKind::RightParen, engine)?; // consume the ")"
    //
    // Ok(Expr::Tuple {
    //   elements,
    //   span: self.current_token().span,
    // })
  }

  fn parse_lambda(&mut self, engine: &mut DiagnosticEngine) -> Result<Expr, ()> {
    let token = self.current_token();
    self.advance(engine); // consume the "fn"

    self.expect(TokenKind::LeftParen, engine)?; // consume the "("
    let params = self.parse_parameters(engine)?;
    self.expect(TokenKind::RightParen, engine)?; // consume the ")"
    let return_type = if matches!(self.current_token().kind, TokenKind::Minus) {
      // TODO: make FatArrow a TokenKind
      self.expect(TokenKind::Minus, engine)?; // consume the "-"
      self.expect(TokenKind::Greater, engine)?; // consume the ">"
      let r_type = self.parse_type(engine)?;
      Some(r_type)
    } else {
      None
    };
    let body = self.parse_block(engine)?;

    Ok(Expr::Lambda {
      params,
      return_type,
      body,
      span: token.span,
    })
  }

  fn parse_block(&mut self, engine: &mut DiagnosticEngine) -> Result<Vec<Stmt>, ()> {
    self.expect(TokenKind::LeftBrace, engine)?; // consume the "{"
    let mut stmts = Vec::<Stmt>::new();

    while !self.is_eof() && !matches!(self.current_token().kind, TokenKind::RightBrace) {
      let stmt = self.parse_stmt(engine)?;
      stmts.push(stmt);
    }
    self.expect(TokenKind::RightBrace, engine)?; // consume the "}"

    Ok(stmts)
  }

  fn parse_parameters(&mut self, engine: &mut DiagnosticEngine) -> Result<Vec<Param>, ()> {
    let mut params = Vec::<Param>::new();

    while !self.is_eof() && !matches!(self.current_token().kind, TokenKind::RightParen) {
      let param_name = self.parse_primary(engine)?;
      let param_name = match param_name {
        Expr::Identifier { name, .. } => name,
        _ => {
          let diagnostic = Diagnostic::new(
            DiagnosticCode::Error(DiagnosticError::UnexpectedToken),
            "Expected identifier after '('".to_string(),
            "duck.lox".to_string(),
          )
          .with_label(
            Span::new(
              self.current_token().span.line + 1,
              self.current_token().span.col + 1,
              self.current_token().span.len,
            ),
            Some("expected identifier here".to_string()),
            LabelStyle::Primary,
          );
          engine.add(diagnostic);
          return Err(());
        },
      };

      let typee = if matches!(self.current_token().kind, TokenKind::Colon) {
        self.expect(TokenKind::Colon, engine)?; // consume the ":"
        let typee = self.parse_type(engine)?;
        Some(typee)
      } else {
        None
      };

      let default_value = if matches!(self.current_token().kind, TokenKind::Equal) {
        self.expect(TokenKind::Equal, engine)?; // consume the "="
        let default_value = self.parse_assignment(engine)?;
        Some(default_value)
      } else {
        None
      };

      if !matches!(self.current_token().kind, TokenKind::RightParen) {
        self.expect(TokenKind::Comma, engine)?; // consume the ","
      }

      params.push(Param {
        name: param_name,
        type_annotation: typee,
        default_value,
      });
    }

    Ok(params)
  }

  fn parse_object(&mut self, token: Token, engine: &mut DiagnosticEngine) -> Result<Expr, ()> {
    self.expect(TokenKind::LeftBrace, engine)?; // consume the "{"

    let mut fields = Vec::<(String, Expr)>::new();

    // Parse fields
    while !self.is_eof() && !matches!(self.current_token().kind, TokenKind::RightBrace) {
      let name = self.parse_primary(engine)?;
      let name = match name {
        Expr::Identifier { name, .. } => name,
        _ => {
          let diagnostic = Diagnostic::new(
            DiagnosticCode::Error(DiagnosticError::UnexpectedToken),
            "Expected identifier after '{'".to_string(),
            "duck.lox".to_string(),
          )
          .with_label(
            Span::new(
              self.current_token().span.line + 1,
              self.current_token().span.col + 1,
              self.current_token().span.len,
            ),
            Some("expected identifier here".to_string()),
            LabelStyle::Primary,
          );
          engine.add(diagnostic);
          return Err(());
        },
      };

      self.expect(TokenKind::Colon, engine)?; // consume the ":"
      let value = self.parse_primary(engine)?;
      if !matches!(self.current_token().kind, TokenKind::RightBrace) {
        self.expect(TokenKind::Comma, engine)?; // consume the ","
      }

      fields.push((name, value));
    }
    self.expect(TokenKind::RightBrace, engine)?; // consume the "}"

    Ok(Expr::Object {
      type_name: String::from(""),
      fields,
      span: token.span,
    })
  }

  fn parse_array(&mut self, engine: &mut DiagnosticEngine) -> Result<Expr, ()> {
    let token = self.expect(TokenKind::LeftBracket, engine)?; // consume the "["

    let mut elements = Vec::<Expr>::new();
    let expr = self.parse_expr(engine)?;
    elements.push(expr);

    // parse comma-separated expressions, to handle [1, 2, 3]
    if self.current_token().kind == TokenKind::Comma {
      while !self.is_eof() && self.current_token().kind != TokenKind::RightBracket {
        self.advance(engine); // consume the ","
        let expr = self.parse_expr(engine)?;
        elements.push(expr);
      }
    }

    if self.is_eof() || self.current_token().kind != TokenKind::RightBracket {
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
        DiagnosticCode::Error(DiagnosticError::MissingClosingBracket),
        "Expected ']' after array element".to_string(),
        "duck.lox".to_string(),
      )
      .with_label(
        Span::new(error_span.line + 1, error_span.col + 1, error_span.len),
        Some("expected ']' here".to_string()),
        LabelStyle::Primary,
      );

      engine.add(diagnostic);
      return Err(());
    } else {
      self.advance(engine); // consume the "]"
      return Ok(Expr::Array {
        elements,
        span: token.span,
      });
    }
  }

  fn parse_grouping(&mut self, engine: &mut DiagnosticEngine) -> Result<Expr, ()> {
    let token = self.expect(TokenKind::LeftParen, engine)?; // consume the "("

    let mut tuple = Vec::<Expr>::new();
    let expr = self.parse_expr(engine)?;
    tuple.push(expr);

    // parse comma-separated expressions, to handle (1, 2, 3)
    if self.current_token().kind == TokenKind::Comma {
      while !self.is_eof() && self.current_token().kind != TokenKind::RightParen {
        self.advance(engine); // consume the ","
        let expr = self.parse_expr(engine)?;
        tuple.push(expr);
      }
    }

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

    if tuple.len() == 1 {
      return Ok(Expr::Grouping {
        expr: Box::new(tuple[0].clone()),
        span: token.span,
      });
    } else {
      return Ok(Expr::Tuple {
        elements: tuple,
        span: token.span,
      });
    }
  }

  fn parse_type(&mut self, engine: &mut DiagnosticEngine) -> Result<Type, ()> {
    let token = self.current_token();
    self.advance(engine); // consume the type token

    match token.kind {
      TokenKind::Int => {
        if token.lexeme == "int" {
          Ok(Type::Int)
        } else {
          Ok(Type::Named(token.lexeme))
        }
      },
      TokenKind::Float => {
        if token.lexeme == "float" {
          Ok(Type::Float)
        } else {
          Ok(Type::Named(token.lexeme))
        }
      },
      TokenKind::String => {
        if token.lexeme == "string" {
          Ok(Type::String)
        } else {
          Ok(Type::Named(token.lexeme))
        }
      },
      TokenKind::Bool => Ok(Type::Bool),
      TokenKind::True => Ok(Type::Named(token.lexeme)),
      TokenKind::False => Ok(Type::Named(token.lexeme)),
      TokenKind::Void => Ok(Type::Void),
      TokenKind::LeftBracket => self.parse_array_type(engine),
      TokenKind::LeftParen => self.parse_tuple_type(engine),
      TokenKind::Identifier => self.parse_named_type(token, engine),
      _ => Err(()),
    }
  }

  fn parse_named_type(&mut self, name: Token, engine: &mut DiagnosticEngine) -> Result<Type, ()> {
    if matches!(self.current_token().kind, TokenKind::Less) {
      self.advance(engine); // consume the "<"

      let mut types = Vec::<Type>::new();
      while !self.is_eof() && self.current_token().kind != TokenKind::Greater {
        let ty = self.parse_type(engine)?;
        types.push(ty);

        if self.current_token().kind != TokenKind::Comma {
          break;
        }

        self.advance(engine); // consume the ","
      }

      if self.current_token().kind == TokenKind::Greater {
        self.advance(engine); // consume the ">"
        return Ok(Type::Generic {
          name: name.lexeme,
          type_params: types,
        });
      } else {
        let diagnostic = Diagnostic::new(
          DiagnosticCode::Error(DiagnosticError::MissingClosingBracket),
          "Expected '>' after generic type".to_string(),
          "duck.lox".to_string(),
        )
        .with_label(
          Span::new(
            self.current_token().span.line + 1,
            self.current_token().span.col + 1,
            self.current_token().span.len,
          ),
          Some("expected '>' here".to_string()),
          LabelStyle::Primary,
        );

        engine.add(diagnostic);

        return Err(());
      }
    }

    Ok(Type::Named(name.lexeme))
  }

  fn parse_array_type(&mut self, engine: &mut DiagnosticEngine) -> Result<Type, ()> {
    let element_type = self.parse_type(engine)?;

    if self.current_token().kind == TokenKind::RightBracket {
      self.advance(engine); // consume the "]"
      return Ok(Type::Array(Box::new(element_type)));
    }

    let diagnostic = Diagnostic::new(
      DiagnosticCode::Error(DiagnosticError::MissingClosingBracket),
      "Expected ']' after array element type".to_string(),
      "duck.lox".to_string(),
    )
    .with_label(
      Span::new(
        self.current_token().span.line + 1,
        self.current_token().span.col + 1,
        self.current_token().span.len,
      ),
      Some("expected ']' here".to_string()),
      LabelStyle::Primary,
    );

    engine.add(diagnostic);

    Err(())
  }

  fn parse_tuple_type(&mut self, engine: &mut DiagnosticEngine) -> Result<Type, ()> {
    let mut types = Vec::<Type>::new();

    while !self.is_eof() && self.current_token().kind != TokenKind::RightParen {
      let ty = self.parse_type(engine)?;
      types.push(ty);

      if self.current_token().kind != TokenKind::Comma {
        break;
      }

      self.advance(engine); // consume the ","
    }

    if self.current_token().kind == TokenKind::RightParen {
      self.advance(engine); // consume the ")"
      return Ok(Type::Tuple(types));
    }

    let diagnostic = Diagnostic::new(
      DiagnosticCode::Error(DiagnosticError::MissingClosingParen),
      "Expected ')' after tuple element type".to_string(),
      "duck.lox".to_string(),
    )
    .with_label(
      Span::new(
        self.current_token().span.line + 1,
        self.current_token().span.col + 1,
        self.current_token().span.len,
      ),
      Some("expected ')' here".to_string()),
      LabelStyle::Primary,
    );

    engine.add(diagnostic);

    Err(())
  }
}
