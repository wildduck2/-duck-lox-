/*!
### Grammar (BNF-style)

program          → declaration* EOF ;

// Declarations
declaration      → varDecl
                 | constDecl
                 | fnDecl
                 | structDecl
                 | traitDecl
                 | implBlock
                 | statement ;

varDecl          → "let" "mut"? IDENTIFIER ( ":" type )? "=" expression ";" ;
constDecl        → "const" IDENTIFIER ":" type "=" expression ";" ;

fnDecl           → "fn" IDENTIFIER "(" parameters? ")" ( "->" type )? block ;
parameters       → parameter ( "," parameter )* ;
parameter        → IDENTIFIER ":" type ( "=" expression )? ;

structDecl       → "struct" IDENTIFIER "{" fields? "}" ;
fields           → field ( "," field )* ","? ;
field            → IDENTIFIER ":" type ( "=" expression )? ;

traitDecl        → "trait" IDENTIFIER "{" fnSignatures "}" ;
fnSignatures     → fnSignature* ;
fnSignature      → "fn" IDENTIFIER "(" parameters? ")" ( "->" type )? ";" ;

implBlock        → "impl" IDENTIFIER ( "for" IDENTIFIER )? "{" fnDecl* "}" ;

// Statements
statement        → exprStmt
                 | ifStmt
                 | whileStmt
                 | forStmt
                 | returnStmt
                 | breakStmt
                 | continueStmt
                 | block ;

exprStmt         → expression ";" ;
ifStmt           → "if" expression block ( "else" ( ifStmt | block ) )? ;
whileStmt        → "while" expression block ;
forStmt          → "for" IDENTIFIER "in" expression block ;
returnStmt       → "return" expression? ";" ;
breakStmt        → "break" ";" ;
continueStmt     → "continue" ";" ;
block            → "{" declaration* "}" ;

// Expressions
expression       → assignment ;

assignment       → assignable "=" assignment | ternary ;
assignable       → IDENTIFIER
                 | call "." IDENTIFIER
                 | call "[" expression "]" ;

ternary          → logicOr ( "?" expression ":" ternary )? ;
logicOr          → logicAnd ( "or" logicAnd )* ;
logicAnd         → equality ( "and" equality )* ;
equality         → comparison ( ( "==" | "!=" ) comparison )* ;
comparison       → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
term             → factor ( ( "+" | "-" ) factor )* ;
factor           → power ( ( "*" | "/" | "%" ) power )* ;
power            → unary ( "^" unary )* ;
unary            → ( "!" | "-" ) unary | call ;

call             → primary ( "(" arguments? ")"
                 | "." IDENTIFIER
                 | "[" expression "]" )* ;

arguments        → expression ( "," expression )* ","? ;

primary          → INTEGER
                 | FLOAT
                 | STRING
                 | "true"
                 | "false"
                 | "nil"
                 | IDENTIFIER
                 | "(" tupleOrGrouping ")"
                 | "[" arrayElements? "]"
                 | object
                 | lambda
                 | match ;

tupleOrGrouping  → expression ( "," expression )* ","? ;
arrayElements    → expression ( "," expression )* ","? ;

object           → IDENTIFIER "{" objectFields? "}" ;
objectFields     → IDENTIFIER ":" expression ( "," IDENTIFIER ":" expression )* ","? ;

lambda           → "fn" "(" parameters? ")" ( "->" type )? block ;

match            → "match" expression "{" matchArm* "}" ;
matchArm         → pattern ( "if" expression )? "=>" ( block | expression "," ) ;

// Patterns
pattern          → orPattern ;
orPattern        → singlePattern ( "|" singlePattern )* ;
singlePattern    → wildcardPattern
                 | literalPattern
                 | rangePattern
                 | identifierPattern
                 | tuplePattern
                 | arrayPattern
                 | structPattern
                 | tupleStructPattern
                 | pathPattern ;

wildcardPattern     → "_" ;
literalPattern      → INTEGER | FLOAT | STRING | "true" | "false" | "nil" ;
rangePattern        → expression ".." expression ;
identifierPattern   → IDENTIFIER ;
tuplePattern        → "(" pattern ( "," pattern )* ","? ")" ;
arrayPattern        → "[" pattern ( "," pattern )* ","? "]" ;
structPattern       → IDENTIFIER "{" structPatternFields? "}" ;
structPatternFields → IDENTIFIER ( ":" pattern )? ( "," IDENTIFIER ( ":" pattern )? )* ","? ;
tupleStructPattern  → IDENTIFIER "(" pattern ( "," pattern )* ","? ")" ;
pathPattern         → IDENTIFIER ( "::" IDENTIFIER )+ ( "(" pattern ( "," pattern )* ","? ")" )? ;

// Types
type             → primitiveType
                 | arrayType
                 | tupleType
                 | fnType
                 | namedType ;

primitiveType    → "int" | "float" | "string" | "bool" | "void" | "nil" ;
arrayType        → "[" type "]" ;
tupleType        → "(" type ( "," type )* ","? ")" ;
fnType           → "fn" "(" ( type ( "," type )* )? ")" ( "->" type )? ;
namedType        → IDENTIFIER ( "<" type ( "," type )* ">" )? ;

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
          stmt.print_tree();
          self.ast.push(stmt);
        },
        Err(_) => self.synchronize(engine),
      }
    }
  }

  /// Parses a declaration, currently delegating to statement parsing.
  fn parse_declaration(&mut self, engine: &mut DiagnosticEngine) -> Result<Stmt, ()> {
    match self.current_token().kind {
      TokenKind::Let | TokenKind::Const => self.parse_variable_declaration(engine),
      _ => self.parse_stmt(engine),
    }
  }

  /* -------------------------------------------------------------------------------------------- */
  /*                                         Statements                                           */
  /* -------------------------------------------------------------------------------------------- */

  /// Parses a single statement node (stubbed for future grammar branches).
  fn parse_stmt(&mut self, engine: &mut DiagnosticEngine) -> Result<Stmt, ()> {
    match self.current_token().kind {
      TokenKind::If => self.parse_if_stmt(engine),
      TokenKind::While => self.parse_whie_stmt(engine),
      _ => {
        // Fallback to an expression statement when no declaration keyword is found.
        let expr = self.parse_expr_stmt(engine)?;
        // NOTE: i need to handle the semicolon here
        Ok(Stmt::Expr(expr))
      },
    }
  }

  /* --------------------------------------------------------------------------------------------*/
  /*                                         While Statement                                     */
  /* --------------------------------------------------------------------------------------------*/

  fn parse_whie_stmt(&mut self, engine: &mut DiagnosticEngine) -> Result<Stmt, ()> {
    let token = self.current_token();
    self.advance(engine);

    let condition = self.parse_expr_stmt_with_context(engine, ExprContext::WhileCondition)?;
    let body = self.parse_block(engine)?;

    Ok(Stmt::While {
      condition: Box::new(condition),
      body,
      span: token.span,
    })
  }

  /* --------------------------------------------------------------------------------------------*/
  /*                                         If Statement                                        */
  /* --------------------------------------------------------------------------------------------*/

  fn parse_if_stmt(&mut self, engine: &mut DiagnosticEngine) -> Result<Stmt, ()> {
    let token = self.current_token();
    self.advance(engine); // consume the 'if' token
    let condition = self.parse_expr_stmt_with_context(engine, ExprContext::IfCondition)?;
    let then_branch = self.parse_block(engine)?;

    if !matches!(self.current_token().kind, TokenKind::Else) {
      return Ok(Stmt::If {
        condition: Box::new(condition),
        then_branch,
        else_branch: None,
        span: token.span,
      });
    }

    self.expect(TokenKind::Else, engine)?;

    let else_branch = if matches!(self.current_token().kind, TokenKind::If) {
      vec![self.parse_if_stmt(engine)?]
    } else {
      self.parse_block(engine)?
    };

    Ok(Stmt::If {
      condition: Box::new(condition),
      then_branch,
      else_branch: Some(else_branch),
      span: token.span,
    })
  }

  /* -------------------------------------------------------------------------------------------- */
  /*                                         Variable Declaration                                 */
  /* -------------------------------------------------------------------------------------------- */

  fn parse_variable_declaration(&mut self, engine: &mut DiagnosticEngine) -> Result<Stmt, ()> {
    let declaration_kind = self.current_token();
    self.advance(engine); // consume the declaration keyword

    let kind = if declaration_kind.kind == TokenKind::Let {
      DeclKind::Let
    } else {
      DeclKind::Const
    };

    let is_mutable = if self.current_token().kind == TokenKind::Mut {
      self.advance(engine); // consume `mut`
      true
    } else {
      false
    };

    let identifier_token = self.current_token();
    let lhs = self.parse_primary(engine, ExprContext::Default)?;
    let name = match lhs {
      #[allow(unused_variables)]
      Expr::Identifier { name, span } => {
        let is_uppercase = name
          .chars()
          .all(|c| !c.is_ascii_alphabetic() || c.is_ascii_uppercase());

        // Const bindings must be uppercase; let bindings may use any casing.
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

    // Optional explicit type annotation following the binding name.
    let type_annotation = if self.current_token().kind == TokenKind::Colon {
      self.advance(engine); // consume ':'
      let rhs = self.parse_type(engine)?;
      Some(rhs)
    } else {
      None
    };

    // Optional initializer uses the general expression context to allow any value.
    let initializer = if self.current_token().kind == TokenKind::Equal {
      self.advance(engine); // consume '='
      let rhs = self.parse_assignment(engine, ExprContext::Default)?;
      Some(rhs)
    } else {
      None
    };

    // Declarations must be terminated explicitly so recovery stays predictable.
    self.expect(TokenKind::Semicolon, engine)?; // ensure the declaration is terminated

    Ok(Stmt::Decl {
      is_mutable,
      kind,
      name,
      type_annotation,
      initializer,
      span: declaration_kind.span,
    })
  }

  /* -------------------------------------------------------------------------------------------- */
  /*                                         Expr                                                 */
  /* -------------------------------------------------------------------------------------------- */

  /// Parses a general expression entrypoint.
  // fn parse_expr_stmt(&mut self, engine: &mut DiagnosticEngine) -> Result<Expr, ()> {
  //   let expr = self.parse_comma(engine)?;
  //
  //   Ok(expr)
  // }
  fn parse_expr_stmt(&mut self, engine: &mut DiagnosticEngine) -> Result<Expr, ()> {
    self.parse_expr_stmt_with_context(engine, ExprContext::Default)
  }

  // Internal version that accepts context
  fn parse_expr_stmt_with_context(
    &mut self,
    engine: &mut DiagnosticEngine,
    context: ExprContext,
  ) -> Result<Expr, ()> {
    // All expression parsing funnels through the comma production so we only gate in one place.
    self.parse_comma(engine, context)
  }

  /* -------------------------------------------------------------------------------------------- */
  /*                                         Comma                                                */
  /* -------------------------------------------------------------------------------------------- */

  /// Parses comma-separated expressions, emitting `Expr::Binary` nodes.
  fn parse_comma(
    &mut self,
    engine: &mut DiagnosticEngine,
    context: ExprContext,
  ) -> Result<Expr, ()> {
    let token = self.current_token();
    let mut expressions = Vec::<Expr>::new();
    let lhs = self.parse_assignment(engine, context)?;
    expressions.push(lhs);

    while !self.is_eof() && matches!(self.current_token().kind, TokenKind::Comma) {
      let token = self.current_token();

      match token.kind {
        TokenKind::Comma => {
          self.advance(engine);

          let rhs = self.parse_assignment(engine, context)?;
          expressions.push(rhs);
        },
        _ => break,
      }
    }

    if expressions.len() == 1 {
      return Ok(expressions[0].clone());
    } else {
      // Multiple expressions represent a tuple literal.
      return Ok(Expr::Comma {
        expressions: expressions,
        span: token.span,
      });
    }
  }

  /* -------------------------------------------------------------------------------------------- */
  /*                                         Assignment                                            */
  /* -------------------------------------------------------------------------------------------- */

  /// Parses assignment expressions and verifies the left side is assignable.
  fn parse_assignment(
    &mut self,
    engine: &mut DiagnosticEngine,
    context: ExprContext,
  ) -> Result<Expr, ()> {
    // Carry the current expression context downward so later productions can tailor behavior.
    let token = self.current_token();
    let lhs = self.parse_ternary(engine, context)?;

    if !self.is_eof() && matches!(self.current_token().kind, TokenKind::Equal) {
      self.advance(engine); // consume the '='
      let rhs = self.parse_assignment(engine, context)?;

      match rhs.clone() {
        Expr::Integer { value: _, span }
        | Expr::Nil { span }
        | Expr::Float { value: _, span }
        | Expr::Bool { value: _, span }
        | Expr::String { value: _, span }
        | Expr::Identifier { name: _, span } => {
          // Assignment succeeds only when the left-hand side is an identifier.
          return Ok(Expr::Assign {
            target: Box::new(lhs),
            value: Box::new(rhs),
            span,
          });
        },
        _ => {
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

          // Abort this production so the caller can attempt recovery.
          return Err(());
        },
      }
    }

    Ok(lhs)
  }

  /* -------------------------------------------------------------------------------------------- */
  /*                                         Ternary                                              */
  /* -------------------------------------------------------------------------------------------- */
  /// Parses ternary expressions of the form `cond ? a : b`.
  fn parse_ternary(
    &mut self,
    engine: &mut DiagnosticEngine,
    context: ExprContext,
  ) -> Result<Expr, ()> {
    let token = self.current_token();
    let condition = self.parse_logical_or(engine, context)?;

    if !self.is_eof() && matches!(self.current_token().kind, TokenKind::Question) {
      self.advance(engine); // consume the (?)
      // The consequent is evaluated in the default context because it stands on its own.
      let then_branch = self.parse_expr_stmt_with_context(engine, ExprContext::Default)?;

      if self.is_eof() || !matches!(self.current_token().kind, TokenKind::Colon) {
        // Colon is mandatory for ternary expressions; report the omission.
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
      // Else branch inherits the current context so surrounding constructs keep their guarantees.
      let else_branch = self.parse_ternary(engine, context)?;

      // Form the ternary expression once all components have been parsed.
      return Ok(Expr::Ternary {
        condition: Box::new(condition),
        then_branch: Box::new(then_branch),
        else_branch: Box::new(else_branch),
        span: token.span,
      });
    }

    Ok(condition)
  }

  /* -------------------------------------------------------------------------------------------- */
  /*                                         Logical OR                                           */
  /* -------------------------------------------------------------------------------------------- */

  /// Parses logical OR chains (`expr or expr`).
  fn parse_logical_or(
    &mut self,
    engine: &mut DiagnosticEngine,
    context: ExprContext,
  ) -> Result<Expr, ()> {
    let mut lhs = self.parse_logical_and(engine, context)?;

    while !self.is_eof() {
      let operator = self.current_token();

      match operator.kind {
        TokenKind::Or => {
          self.advance(engine); // consume 'or'
          let rhs = self.parse_logical_and(engine, context)?;
          // Represent `lhs or rhs` as a binary operation node.
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

  /* -------------------------------------------------------------------------------------------- */
  /*                                         Logical AND                                          */
  /* -------------------------------------------------------------------------------------------- */

  /// Parses logical AND chains (`expr and expr`).
  fn parse_logical_and(
    &mut self,
    engine: &mut DiagnosticEngine,
    context: ExprContext,
  ) -> Result<Expr, ()> {
    let mut lhs = self.parse_equality(engine, context)?;

    while !self.is_eof() {
      let operator = self.current_token();

      match operator.kind {
        TokenKind::And => {
          self.advance(engine); // consume 'and'
          let rhs = self.parse_equality(engine, context)?;
          // Build a binary node for each logical conjunction.
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

  /* -------------------------------------------------------------------------------------------- */
  /*                                         Equality                                             */
  /* -------------------------------------------------------------------------------------------- */

  /// Parses equality comparisons (`==` and `!=`).
  fn parse_equality(
    &mut self,
    engine: &mut DiagnosticEngine,
    context: ExprContext,
  ) -> Result<Expr, ()> {
    let mut lhs = self.parse_comparison(engine, context)?;

    while !self.is_eof() {
      let operator = self.current_token();

      match operator.kind {
        TokenKind::EqualEqual | TokenKind::BangEqual => {
          self.advance(engine); // consume comparison operator
          let rhs = self.parse_comparison(engine, context)?;
          // Map the lexeme to the appropriate equality variant.
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

  /* -------------------------------------------------------------------------------------------- */
  /*                                         Relational                                           */
  /* -------------------------------------------------------------------------------------------- */
  /// Parses relational comparisons (`<`, `<=`, `>`, `>=`).
  fn parse_comparison(
    &mut self,
    engine: &mut DiagnosticEngine,
    context: ExprContext,
  ) -> Result<Expr, ()> {
    let mut lhs = self.parse_term(engine, context)?;

    while !self.is_eof() {
      let operator = self.current_token();

      match operator.kind {
        TokenKind::Greater | TokenKind::GreaterEqual | TokenKind::Less | TokenKind::LessEqual => {
          self.advance(engine); // consume relational operator
          let rhs = self.parse_term(engine, context)?;
          // Convert the operator lexeme into the matching binary enum variant.
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

  /* -------------------------------------------------------------------------------------------- */
  /*                                         Additive                                             */
  /* -------------------------------------------------------------------------------------------- */
  /// Parses additive expressions (`+` and `-` sequences).
  fn parse_term(
    &mut self,
    engine: &mut DiagnosticEngine,
    context: ExprContext,
  ) -> Result<Expr, ()> {
    let mut lhs = self.parse_factor(engine, context)?;

    while !self.is_eof() {
      let operator = self.current_token();

      match operator.kind {
        TokenKind::Plus | TokenKind::Minus => {
          self.advance(engine); // consume '+' or '-'
          let rhs = self.parse_factor(engine, context)?;
          // Maintain left-associativity for additive chains.
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

  /* -------------------------------------------------------------------------------------------- */
  /*                                         Multiplicative                                       */
  /* -------------------------------------------------------------------------------------------- */
  /// Parses multiplicative expressions (`*`, `/`, `%`).
  fn parse_factor(
    &mut self,
    engine: &mut DiagnosticEngine,
    context: ExprContext,
  ) -> Result<Expr, ()> {
    let mut lhs = self.parse_unary(engine, context)?;

    while !self.is_eof() {
      let operator = self.current_token();

      match operator.kind {
        TokenKind::Percent | TokenKind::Slash | TokenKind::Star => {
          self.advance(engine); // consume multiplicative operator
          let rhs = self.parse_unary(engine, context)?;
          // Promote each operator to its binary expression counterpart.
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

  /* -------------------------------------------------------------------------------------------- */
  /*                                         Prefix Unary                                         */
  /* -------------------------------------------------------------------------------------------- */
  /// Parses prefix unary operators and defers to the next precedence level.
  fn parse_unary(
    &mut self,
    engine: &mut DiagnosticEngine,
    context: ExprContext,
  ) -> Result<Expr, ()> {
    let operator = self.current_token();

    match operator.kind {
      TokenKind::Minus | TokenKind::Bang => {
        self.advance(engine); // consume unary operator
        let rhs = self.parse_unary(engine, context)?;

        // Convert the operator lexeme into the proper unary AST node.
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
      // Fall through to call/primary parsing when no unary operator is present.
      _ => self.parse_call(engine, context),
    }
  }

  /* -------------------------------------------------------------------------------------------- */
  /*                                         Call                                                 */
  /* -------------------------------------------------------------------------------------------- */
  /// Parses function calls and dotted access chains.
  fn parse_call(
    &mut self,
    engine: &mut DiagnosticEngine,
    context: ExprContext,
  ) -> Result<Expr, ()> {
    let start_token = self.current_token();
    let mut callee = self.parse_primary(engine, context)?;

    // Chain together call/member/index operations that appear on the same source line.
    while !self.is_eof()
      && start_token.kind == TokenKind::Identifier
      && start_token.span.line == self.current_token().span.line
    {
      if matches!(self.current_token().kind, TokenKind::LeftParen) {
        let token = self.current_token();
        self.advance(engine); // consume the "("

        let args = self.parser_arguments(engine)?;
        self.expect(TokenKind::RightParen, engine)?; // consume the ")"

        callee = Expr::Call {
          callee: Box::new(callee),
          args,
          span: token.span,
        };
      } else if matches!(self.current_token().kind, TokenKind::Dot) {
        // Desugar `object.field` into a member access node.
        self.advance(engine); // consume the "."
        let token = self.current_token();
        self.advance(engine); // consume the "token"

        let field = if matches!(token.kind, TokenKind::Identifier) {
          token.lexeme
        } else {
          return Err(());
        };

        callee = Expr::Member {
          object: Box::new(callee),
          field,
          span: token.span,
        };
      } else if matches!(self.current_token().kind, TokenKind::LeftBracket) {
        // Map subscripts like `foo[bar]` onto an indexing expression.
        let token = self.current_token();
        self.advance(engine); // consume '['
        let index = self.parse_expr_stmt_with_context(engine, context)?;
        self.expect(TokenKind::RightBracket, engine)?; // consume ']'

        callee = Expr::Index {
          object: Box::new(callee),
          index: Box::new(index),
          span: token.span,
        }
      } else {
        break;
      }
    }

    Ok(callee)
  }

  /* -------------------------------------------------------------------------------------------- */
  /*                                         Arguments                                            */
  /* -------------------------------------------------------------------------------------------- */
  /// Parses a comma-separated argument list for function calls.
  fn parser_arguments(&mut self, engine: &mut DiagnosticEngine) -> Result<Vec<Expr>, ()> {
    let mut args = Vec::<Expr>::new();
    while !self.is_eof() && self.current_token().kind != TokenKind::RightParen {
      let expr = self.parse_ternary(engine, ExprContext::Default)?;
      args.push(expr);

      // Allow trailing argument without a comma, otherwise expect another value.
      if self.current_token().kind != TokenKind::Comma {
        break;
      }
      self.advance(engine); // consume the ","
    }

    if args.len() >= 255 {
      // Mirror the canonical Lox limitation on function parameters.
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

    Ok(args)
  }

  /* -------------------------------------------------------------------------------------------- */
  /*                                         Primary                                              */
  /* -------------------------------------------------------------------------------------------- */

  /// Parses primary expressions: literals, identifiers, and grouped expressions.
  fn parse_primary(
    &mut self,
    engine: &mut DiagnosticEngine,
    context: ExprContext,
  ) -> Result<Expr, ()> {
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

      // handle the case where lambda is declared
      TokenKind::Fn => self.parse_lambda(engine),

      // handle the case where group is used also for tuple
      TokenKind::LeftParen => self.parse_grouping(engine),

      // handle the case where array is declared
      TokenKind::LeftBracket => self.parse_array(engine),

      // handle the case where match is declared
      TokenKind::Match => self.parse_match(engine),

      // handle the case where match is declared
      TokenKind::If => match self.parse_if_stmt(engine)? {
        Stmt::If {
          condition,
          then_branch,
          else_branch,
          span,
        } => Ok(Expr::If {
          condition,
          then_branch,
          else_branch,
          span,
        }),
        _ => Err(()),
      },

      // handle the case where the token is a keyword
      TokenKind::Identifier => {
        self.advance(engine); // consume this token

        // Only treat `identifier { ... }` as an object literal in neutral contexts.
        if context == ExprContext::Default && self.current_token().kind == TokenKind::LeftBrace {
          return self.parse_object(token, engine);
        }

        // handle the case where object is declared
        Ok(Expr::Identifier {
          name: token.lexeme,
          span: token.span,
        })
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

  /* --------------------------------------------------------------------------------------------*/
  /*                                         Object                                              */
  /* --------------------------------------------------------------------------------------------*/

  fn parse_object(&mut self, token: Token, engine: &mut DiagnosticEngine) -> Result<Expr, ()> {
    self.expect(TokenKind::LeftBrace, engine)?; // consume '{'
    let fields = self.parse_object_fields(engine)?;
    self.expect(TokenKind::RightBrace, engine)?; // consume '}'

    Ok(Expr::Object {
      type_name: token.lexeme,
      fields,
      span: token.span,
    })
  }

  fn parse_object_fields(
    &mut self,
    engine: &mut DiagnosticEngine,
  ) -> Result<Vec<(String, Expr)>, ()> {
    let mut fields = Vec::<(String, Expr)>::new();
    // Consume repeated `key: value` pairs until we reach the closing brace.
    while !self.is_eof() && !matches!(self.current_token().kind, TokenKind::RightBrace) {
      let name = self.parse_primary(engine, ExprContext::Default)?;
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

      self.expect(TokenKind::Colon, engine)?; // consume ':'
      let value = self.parse_primary(engine, ExprContext::Default)?;
      if !matches!(self.current_token().kind, TokenKind::RightBrace) {
        self.expect(TokenKind::Comma, engine)?; // consume ',' between fields
      }

      fields.push((name, value));
    }

    Ok(fields)
  }

  /* --------------------------------------------------------------------------------------------*/
  /*                                         Lambda                                              */
  /* --------------------------------------------------------------------------------------------*/

  fn parse_lambda(&mut self, engine: &mut DiagnosticEngine) -> Result<Expr, ()> {
    let token = self.current_token();
    self.advance(engine); // consume the "fn"

    self.expect(TokenKind::LeftParen, engine)?; // consume '('
    let params = self.parse_parameters(engine)?;
    self.expect(TokenKind::RightParen, engine)?; // consume ')'
    let return_type = if matches!(self.current_token().kind, TokenKind::FatArrow) {
      self.expect(TokenKind::FatArrow, engine)?; // consume the "->"
      let r_type = self.parse_type(engine)?;
      Some(r_type)
    } else {
      None
    };

    let body = self.parse_block(engine)?;

    // Compose the lambda expression from the parsed components.
    Ok(Expr::Lambda {
      params,
      return_type,
      body,
      span: token.span,
    })
  }

  /* --------------------------------------------------------------------------------------------*/
  /*                                         Block                                              */
  /* --------------------------------------------------------------------------------------------*/

  fn parse_block(&mut self, engine: &mut DiagnosticEngine) -> Result<Vec<Stmt>, ()> {
    self.expect(TokenKind::LeftBrace, engine)?; // consume '{'
    let mut stmts = Vec::<Stmt>::new();

    while !self.is_eof() && !matches!(self.current_token().kind, TokenKind::RightBrace) {
      let stmt = self.parse_declaration(engine)?;
      stmts.push(stmt);
    }
    self.expect(TokenKind::RightBrace, engine)?; // consume '}'

    Ok(stmts)
  }

  /* --------------------------------------------------------------------------------------------*/
  /*                                         Parameters                                            */
  /* --------------------------------------------------------------------------------------------*/

  fn parse_parameters(&mut self, engine: &mut DiagnosticEngine) -> Result<Vec<Param>, ()> {
    let mut params = Vec::<Param>::new();

    while !self.is_eof() && !matches!(self.current_token().kind, TokenKind::RightParen) {
      let param_name = self.parse_primary(engine, ExprContext::Default)?;
      let param_name = match param_name {
        Expr::Identifier { name, .. } => name,
        _ => {
          // Only identifiers are valid parameter names.
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
        self.expect(TokenKind::Colon, engine)?; // consume ':'
        let typee = self.parse_type(engine)?;
        Some(typee)
      } else {
        None
      };

      let default_value = if matches!(self.current_token().kind, TokenKind::Equal) {
        self.expect(TokenKind::Equal, engine)?; // consume '='
        let default_value = self.parse_assignment(engine, ExprContext::Default)?;
        Some(default_value)
      } else {
        None
      };

      if !matches!(self.current_token().kind, TokenKind::RightParen) {
        self.expect(TokenKind::Comma, engine)?; // consume ',' between parameters
      }

      params.push(Param {
        name: param_name,
        type_annotation: typee,
        default_value,
      });
    }

    Ok(params)
  }

  /* --------------------------------------------------------------------------------------------*/
  /*                                         Grouping                                            */
  /* --------------------------------------------------------------------------------------------*/

  fn parse_grouping(&mut self, engine: &mut DiagnosticEngine) -> Result<Expr, ()> {
    let token = self.expect(TokenKind::LeftParen, engine)?; // consume '('

    let mut tuple = Vec::<Expr>::new();
    let expr = self.parse_expr_stmt(engine)?;
    tuple.push(expr);

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
      // A single expression represents a grouped expression.
      return Ok(Expr::Grouping {
        expr: Box::new(tuple[0].clone()),
        span: token.span,
      });
    } else {
      // Multiple expressions represent a tuple literal.
      return Ok(Expr::Tuple {
        elements: tuple,
        span: token.span,
      });
    }
  }

  /* --------------------------------------------------------------------------------------------*/
  /*                                         Array                                                */
  /* --------------------------------------------------------------------------------------------*/

  fn parse_array(&mut self, engine: &mut DiagnosticEngine) -> Result<Expr, ()> {
    let token = self.expect(TokenKind::LeftBracket, engine)?; // consume '['

    let mut elements = Vec::<Expr>::new();
    if self.current_token().kind != TokenKind::RightBracket {
      let expr = self.parse_expr_stmt(engine)?;
      elements.push(expr);
    }

    // parse comma-separated expressions, to handle [1, 2, 3]
    if self.current_token().kind == TokenKind::Comma {
      while !self.is_eof() && self.current_token().kind != TokenKind::RightBracket {
        self.advance(engine); // consume ','
        let expr = self.parse_expr_stmt(engine)?;
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
      self.advance(engine); // consume ']'
      return Ok(Expr::Array {
        elements,
        span: token.span,
      });
    }
  }

  /* --------------------------------------------------------------------------------------------*/
  /*                                         Match Expression                                    */
  /* --------------------------------------------------------------------------------------------*/

  fn parse_match(&mut self, engine: &mut DiagnosticEngine) -> Result<Expr, ()> {
    // Track when we are inside a match expression so pattern helpers can tighten rules.
    self.is_in_match = true;
    self.expect(TokenKind::Match, engine)?; // consume the "match"
    let expr = self.parse_primary(engine, ExprContext::MatchDiscriminant)?;

    self.expect(TokenKind::LeftBrace, engine)?; // consume '{'
    let token = self.current_token();
    let arms = self.parse_arms(engine)?;

    if arms.is_empty() {
      let diagnostic = Diagnostic::new(
        DiagnosticCode::Error(DiagnosticError::EmptyMatch),
        "Empty match".to_string(),
        "duck.lox".to_string(),
      )
      .with_label(
        Span::new(token.span.line, token.span.col + 1, token.span.len),
        Some("Empty match".to_string()),
        LabelStyle::Primary,
      );

      engine.add(diagnostic);
      self.is_in_match = false;
      return Err(());
    }

    self.expect(TokenKind::RightBrace, engine)?; // consume '}'

    // Reset the match tracking flag once the expression is complete.
    self.is_in_match = false;
    Ok(Expr::Match {
      expr: Box::new(expr),
      arms,
      span: token.span,
    })
  }

  /* --------------------------------------------------------------------------------------------*/
  /*                                         Match Arms                                          */
  /* --------------------------------------------------------------------------------------------*/

  fn parse_arms(&mut self, engine: &mut DiagnosticEngine) -> Result<Vec<MatchArm>, ()> {
    let mut arms = Vec::<MatchArm>::new();

    while !self.is_eof() && !matches!(self.current_token().kind, TokenKind::RightBrace) {
      // Parse the pattern for this arm.
      let pattern = self.parse_pattern(engine)?;

      let guard = if self.current_token().kind == TokenKind::If {
        self.advance(engine); // consume 'if'
        // Guards are plain logical expressions evaluated in the default context.
        let guard = self.parse_logical_or(engine, ExprContext::Default)?;
        Some(guard)
      } else {
        None
      };

      self.expect(TokenKind::Equal, engine)?; // consume '='
      self.expect(TokenKind::Greater, engine)?; // consume '>'

      // Parse body (either block or single expression)
      let body = if self.current_token().kind == TokenKind::LeftBrace {
        self.parse_block(engine)?
      } else {
        // Single expression - wrap in statement
        // Single-expression arms are wrapped in a tiny block for uniform handling.
        let expr = self.parse_ternary(engine, ExprContext::Default)?;
        vec![Stmt::Expr(expr)]
      };

      arms.push(MatchArm {
        pattern,
        guard,
        body,
      });

      // Optional comma between arms
      if self.current_token().kind == TokenKind::Comma {
        self.advance(engine); // consume trailing comma separating arms
      }
    }

    Ok(arms)
  }

  /* -------------------------------------------------------------------------------------------- */
  /*                                         Patterns                                             */
  /* -------------------------------------------------------------------------------------------- */

  fn parse_pattern(&mut self, engine: &mut DiagnosticEngine) -> Result<Pattern, ()> {
    // Dispatch through the precedence of pattern constructs.
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

  /* -------------------------------------------------------------------------------------------- */
  /*                                         Single Pattern                                        */
  /* -------------------------------------------------------------------------------------------- */

  fn parse_single_pattern(&mut self, engine: &mut DiagnosticEngine) -> Result<Pattern, ()> {
    let token = self.current_token();
    match token.kind {
      // Wildcard: _
      TokenKind::Underscore => {
        self.advance(engine); // consume '_'
        Ok(Pattern::Wildcard)
      },

      TokenKind::Int
      | TokenKind::Float
      | TokenKind::String
      | TokenKind::True
      | TokenKind::False
      | TokenKind::Nil => self.parse_literal_pattern(token, engine),

      TokenKind::LeftParen => {
        let tuple = self.parse_tuple_pattern(engine)?;
        Ok(Pattern::Tuple(tuple))
      },

      TokenKind::Identifier => self.parse_identifier_pattern(engine),

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

  /* -------------------------------------------------------------------------------------------- */
  /*                                         Identifier Pattern                                   */
  /* -------------------------------------------------------------------------------------------- */

  fn parse_identifier_pattern(&mut self, engine: &mut DiagnosticEngine) -> Result<Pattern, ()> {
    let struct_name = self.current_token();
    self.advance(engine); // consume the identifier

    match self.current_token().kind {
      TokenKind::ColonColon => {
        self.expect(TokenKind::ColonColon, engine)?; // consume the "::"

        let path_name = self.current_token();
        self.advance(engine); // consume the identifier
        if path_name.kind != TokenKind::Identifier {
          let diagnostic = Diagnostic::new(
            DiagnosticCode::Error(DiagnosticError::UnexpectedToken),
            "Expected identifier after '::'".to_string(),
            "duck.lox".to_string(),
          )
          .with_label(
            Span::new(
              path_name.span.line + 1,
              path_name.span.col + 1,
              path_name.span.len,
            ),
            Some("expected identifier here".to_string()),
            LabelStyle::Primary,
          );
          engine.add(diagnostic);
          return Err(());
        }

        let mut paths = Vec::<String>::new();
        // Seed the fully-qualified path with namespace and variant names.
        paths.push(struct_name.lexeme);
        paths.push(path_name.lexeme);

        if self.current_token().kind == TokenKind::LeftBrace {
          let path_struct = self.parse_struct_pattern(engine)?;
          return Ok(Pattern::PathStruct {
            path: paths,
            fields: path_struct,
          });
        } else if self.current_token().kind == TokenKind::LeftParen {
          let tuple = self.parse_tuple_pattern(engine)?;
          Ok(Pattern::PathTuple {
            path: paths,
            patterns: tuple,
          })
        } else {
          let diagnostic = Diagnostic::new(
            DiagnosticCode::Error(DiagnosticError::UnexpectedToken),
            "Expected identifier after '::'".to_string(),
            "duck.lox".to_string(),
          )
          .with_label(
            Span::new(
              path_name.span.line + 1,
              path_name.span.col + 1,
              path_name.span.len,
            ),
            Some("expected identifier here".to_string()),
            LabelStyle::Primary,
          );
          engine.add(diagnostic);
          Err(())
        }
      },

      TokenKind::LeftParen => {
        let tuple = self.parse_tuple_pattern(engine)?;
        Ok(Pattern::TupleStruct {
          name: struct_name.lexeme,
          patterns: tuple,
        })
      },
      TokenKind::LeftBrace => {
        // Struct-style destructuring: `Foo { .. }`.
        let path_struct = self.parse_struct_pattern(engine)?;
        return Ok(Pattern::Struct {
          name: struct_name.lexeme,
          fields: path_struct,
        });
      },

      _ => Ok(Pattern::Identifier(struct_name.lexeme)),
    }
  }

  /* -------------------------------------------------------------------------------------------- */
  /*                                         Struct Pattern                                       */
  /* -------------------------------------------------------------------------------------------- */

  fn parse_struct_pattern(
    &mut self,
    engine: &mut DiagnosticEngine,
  ) -> Result<Vec<(String, Pattern)>, ()> {
    let mut fields = Vec::<(String, Pattern)>::new();
    self.advance(engine); // consume '{'

    while !self.is_eof() && self.current_token().kind != TokenKind::RightBrace {
      let field_name = self.current_token();
      self.advance(engine);

      let field_pattern = if self.current_token().kind == TokenKind::Colon {
        self.advance(engine); // consume ':'
        self.parse_pattern(engine)?
      } else {
        Pattern::Identifier(field_name.lexeme.clone())
      };

      fields.push((field_name.lexeme, field_pattern));

      if self.current_token().kind == TokenKind::Comma {
        self.advance(engine);
      }
    }

    self.expect(TokenKind::RightBrace, engine)?;

    return Ok(fields);
  }

  /* -------------------------------------------------------------------------------------------- */
  /*                                         Literal Pattern                                      */
  /* -------------------------------------------------------------------------------------------- */

  fn parse_literal_pattern(
    &mut self,
    token: Token,
    engine: &mut DiagnosticEngine,
  ) -> Result<Pattern, ()> {
    if self.peek().kind == TokenKind::DotDot {
      // Interpret `start..=end` as a closed range pattern.
      self.advance(engine); // consume the literal start
      let start = Expr::Integer {
        value: token.lexeme.parse().unwrap(),
        span: token.span,
      };

      self.expect(TokenKind::DotDot, engine)?; // consume the ".."
      self.expect(TokenKind::Equal, engine)?; // consume the "="
      let end = self.parse_primary(engine, ExprContext::Default)?;

      return Ok(Pattern::Range { start, end });
    }

    // Fallback to a simple literal pattern when no range operator is present.
    let expr = self.parse_primary(engine, ExprContext::Default)?;
    Ok(Pattern::Literal(expr))
  }

  /* -------------------------------------------------------------------------------------------- */
  /*                                         Tuple Pattern                                        */
  /* -------------------------------------------------------------------------------------------- */

  fn parse_tuple_pattern(&mut self, engine: &mut DiagnosticEngine) -> Result<Vec<Pattern>, ()> {
    self.expect(TokenKind::LeftParen, engine)?; // consume '('

    let mut fields = Vec::<Pattern>::new();

    while !self.is_eof() && self.current_token().kind != TokenKind::RightParen {
      if self.current_token().kind == TokenKind::Underscore {
        // Allow `_` placeholders to short-circuit parsing for the current slot.
        self.advance(engine);
        fields.push(Pattern::Wildcard);
        if self.current_token().kind == TokenKind::Comma {
          self.advance(engine);
        }
        continue;
      }

      let field = self.parse_pattern(engine)?;

      fields.push(field);

      if self.current_token().kind != TokenKind::Comma {
        break;
      }

      self.advance(engine); // consume ','
    }

    self.expect(TokenKind::RightParen, engine)?; // consume ')'
    Ok(fields)
  }

  /* -------------------------------------------------------------------------------------------- */
  /*                                         Type                                                  */
  /* -------------------------------------------------------------------------------------------- */

  fn parse_type(&mut self, engine: &mut DiagnosticEngine) -> Result<Type, ()> {
    let token = self.current_token();
    self.advance(engine); // consume the type token

    // Map token kinds to their semantic type representation.
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
      TokenKind::LeftParen => self.parse_tuple_type(engine),
      TokenKind::LeftBracket => self.parse_array_type(engine),
      TokenKind::Identifier => self.parse_named_type(token, engine),
      TokenKind::Fn => self.parse_fn_type(engine),
      _ => Err(()),
    }
  }

  /* -------------------------------------------------------------------------------------------- */
  /*                                         Fn Type                                               */
  /* -------------------------------------------------------------------------------------------- */

  fn parse_fn_type(&mut self, engine: &mut DiagnosticEngine) -> Result<Type, ()> {
    self.expect(TokenKind::LeftParen, engine)?; // consume the "fn"
    let mut params = Vec::<Type>::new();

    while !self.is_eof() && self.current_token().kind != TokenKind::RightParen {
      let param = self.parse_type(engine)?;
      params.push(param);

      if self.current_token().kind != TokenKind::Comma {
        break;
      }

      self.advance(engine); // consume the ","
    }

    self.expect(TokenKind::RightParen, engine)?; // consume the ")"
    let return_type = if matches!(self.current_token().kind, TokenKind::Minus) {
      self.expect(TokenKind::FatArrow, engine)?; // consume the "->"
      let r_type = self.parse_type(engine)?;
      Some(r_type)
    } else {
      None
    };

    Ok(Type::Function {
      params,
      return_type: Box::new(return_type.unwrap_or(Type::Void)),
    })
  }

  /* -------------------------------------------------------------------------------------------- */
  /*                                         Named Type                                           */
  /* -------------------------------------------------------------------------------------------- */

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

  /* -------------------------------------------------------------------------------------------- */
  /*                                         Array Type                                           */
  /* -------------------------------------------------------------------------------------------- */

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

  /* -------------------------------------------------------------------------------------------- */
  /*                                         Tuple Type                                           */
  /* -------------------------------------------------------------------------------------------- */

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
