/*!

### Grammar (BNF-style)

program        → declaration* EOF ;

declaration    → varDecl | fnDecl | structDecl | traitDecl | implBlock | statement ;

varDecl        → "let" IDENTIFIER ":" type ( "=" expression )? ";" ;
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
               | IDENTIFIER | "(" expression ")" | array | object | lambda | match ;

type           → "int" | "float" | "string" | "bool" | "void"
               | "[" type "]"
               | "(" type ( "," type )* ")" "->" type
               | IDENTIFIER
               | IDENTIFIER "<" type ( "," type )* ">"

*/

use diagnostic::{
  code::DiagnosticCode,
  diagnostic::{Diagnostic, LabelStyle, Span},
  types::{error::DiagnosticError, warning::DiagnosticWarning},
  DiagnosticEngine,
};
use lexer::token::TokenKind;

use crate::{
  expr::{BinaryOp, Expr, UnaryOp},
  stmt::{DeclKind, Stmt, Type},
  Parser,
};

impl Parser {
  /// Parses the top-level production, collecting statements until EOF.
  pub fn parse_program(&mut self, engine: &mut DiagnosticEngine) {
    while !self.is_eof() {
      match self.parse_declaration(engine) {
        Ok(stmt) => {
          println!("stmt: {:#?}", stmt);
          // stmt.print_tree();
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

    let kind = if self.current_token().kind == TokenKind::Let {
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

        if is_uppercase {
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
      self.advance(engine); // consume the type token
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

    while !self.is_eof() || !matches!(self.current_token().kind, TokenKind::Comma) {
      let token = self.current_token();

      match token.kind {
        TokenKind::Comma => {
          self.advance(engine);

          let rhs = self.parse_assignment(engine)?;

          // lhs = Expr::Binary {
          //   lhs: Box::new(lhs),
          //   operator: token,
          //   rhs: Box::new(rhs),
          // };
        },
        _ => break,
      }
    }

    Ok(lhs)
  }

  /// Parses assignment expressions and verifies the left side is assignable.
  fn parse_assignment(&mut self, engine: &mut DiagnosticEngine) -> Result<Expr, ()> {
    let token = self.current_token();
    let lhs = self.parse_ternary(engine)?;

    if !self.is_eof() && matches!(self.current_token().kind, TokenKind::Equal) {
      self.advance(engine);
      let rhs = self.parse_assignment(engine)?;

      if let Expr::Identifier { name, span } = lhs.clone() {
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

    self.expect(TokenKind::RightParen, engine)?;

    Ok(args)
  }

  /// Parses primary expressions: literals, identifiers, and grouped expressions.
  fn parse_primary(&mut self, engine: &mut DiagnosticEngine) -> Result<Expr, ()> {
    // primary        → INTEGER | FLOAT | STRING | "true" | "false" | "nil"
    //                | IDENTIFIER | "(" expression ")" | array | object | lambda | match ;

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
        Ok(Expr::Identifier {
          name: token.lexeme,
          span: token.span,
        })
      },

      // handle the case where group is used
      TokenKind::LeftParen => {
        self.advance(engine); // consume the "("

        let expr = self.parse_expr(engine)?;

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
        return Ok(Expr::Grouping {
          expr: Box::new(expr),
          span: token.span,
        });
      },

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

  fn parse_type(&mut self, engine: &mut DiagnosticEngine) -> Result<Type, ()> {
    let token = self.current_token();
    // type           → "int" | "float" | "string" | "bool" | "void"
    //                | "[" type "]"
    //                | "(" type ( "," type )* ")" "->" type
    //                | IDENTIFIER
    //                | IDENTIFIER "<" type ( "," type )* ">"

    match token.kind {
      TokenKind::Int => Ok(Type::Int),
      TokenKind::Float => Ok(Type::Float),
      TokenKind::String => Ok(Type::String),
      TokenKind::Bool => Ok(Type::Bool),
      TokenKind::Void => Ok(Type::Void),
      // TokenKind::LeftBracket => self.parse_array_type(engine),
      // TokenKind::LeftParen => self.parse_tuple_type(engine),
      // TokenKind::Identifier => self.parse_named_type(engine),
      // TokenKind::Identifier => self.parse_generic_type(engine),
      _ => Err(()),
    }
  }
}
