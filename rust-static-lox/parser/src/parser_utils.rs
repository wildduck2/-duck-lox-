/*!
### Grammar (BNF-style) - TypeScript-like Language

program          → declaration* EOF ;

// Declarations
declaration      → varDecl
                 | functionDecl
                 | classDecl
                 | interfaceDecl
                 | typeAliasDecl
                 | enumDecl
                 | namespaceDecl
                 | exportDecl
                 | importDecl
                 | statement ;

varDecl          → ( "var" | "let" | "const" ) IDENTIFIER ( ":" type )? ( "=" expression )? ";" ;

functionDecl     → "async"? "function" IDENTIFIER typeParams? "(" parameters? ")" ( ":" type )? block ;
parameters       → parameter ( "," parameter )* ;
parameter        → "..."? IDENTIFIER "?" ":" type ( "=" expression )? ;

classDecl        → "abstract"? "class" IDENTIFIER typeParams? ( "extends" type )? ( "implements" typeList )? "{" classMember* "}" ;
classMember      → visibility? "static"? "readonly"? ( propertyDecl | methodDecl | constructor ) ;
propertyDecl     → IDENTIFIER "?" ":" type ( "=" expression )? ";" ;
methodDecl       → "async"? IDENTIFIER typeParams? "(" parameters? ")" ( ":" type )? block ;
constructor      → "constructor" "(" parameters? ")" block ;
visibility       → "public" | "private" | "protected" ;

interfaceDecl    → "interface" IDENTIFIER typeParams? ( "extends" typeList )? "{" interfaceMember* "}" ;
interfaceMember  → IDENTIFIER "?" ":" type ";"
                 | IDENTIFIER typeParams? "(" parameters? ")" ( ":" type )? ";" ;

typeAliasDecl    → "type" IDENTIFIER typeParams? "=" type ";" ;

enumDecl         → "enum" IDENTIFIER "{" enumMember ( "," enumMember )* ","? "}" ;
enumMember       → IDENTIFIER ( "=" ( NUMBER | STRING ) )? ;

namespaceDecl    → "namespace" IDENTIFIER "{" declaration* "}" ;

exportDecl       → "export" ( declaration | "{" exportList "}" ) ;
exportList       → IDENTIFIER ( "," IDENTIFIER )* ;

importDecl       → "import" ( IDENTIFIER | "{" importList "}" | "*" "as" IDENTIFIER ) "from" STRING ";" ;
importList       → IDENTIFIER ( "," IDENTIFIER )* ;

typeParams       → "<" typeParam ( "," typeParam )* ">" ;
typeParam        → IDENTIFIER ( "extends" type )? ( "=" type )? ;

typeList         → type ( "," type )* ;

// Statements
statement        → exprStmt
                 | ifStmt
                 | whileStmt
                 | forStmt
                 | forInStmt
                 | forOfStmt
                 | doWhileStmt
                 | switchStmt
                 | tryStmt
                 | throwStmt
                 | returnStmt
                 | breakStmt
                 | continueStmt
                 | block ;

exprStmt         → expression ";" ;
ifStmt           → "if" "(" expression ")" statement ( "else" statement )? ;
whileStmt        → "while" "(" expression ")" statement ;
doWhileStmt      → "do" statement "while" "(" expression ")" ";" ;
forStmt          → "for" "(" ( varDecl | exprStmt | ";" ) expression? ";" expression? ")" statement ;
forInStmt        → "for" "(" ( "var" | "let" | "const" ) IDENTIFIER "in" expression ")" statement ;
forOfStmt        → "for" "(" ( "var" | "let" | "const" ) IDENTIFIER "of" expression ")" statement ;
switchStmt       → "switch" "(" expression ")" "{" switchCase* defaultCase? "}" ;
switchCase       → "case" expression ":" statement* ;
defaultCase      → "default" ":" statement* ;
tryStmt          → "try" block catchClause? finallyClause? ;
catchClause      → "catch" "(" IDENTIFIER ( ":" type )? ")" block ;
finallyClause    → "finally" block ;
throwStmt        → "throw" expression ";" ;
returnStmt       → "return" expression? ";" ;
breakStmt        → "break" ";" ;
continueStmt     → "continue" ";" ;
block            → "{" statement* "}" ;

// Expressions
expression       → sequence ;

sequence         → assignment ( "," assignment )* ;
assignment       → ternary ( ( "=" | "+=" | "-=" | "*=" | "/=" | "%=" | "&&=" | "||=" | "??=" ) assignment )? ;
ternary          → logicOr ( "?" expression ":" ternary )? ;
logicOr          → logicAnd ( "||" logicAnd )* ;
logicAnd         → nullishCoalescing ( "&&" nullishCoalescing )* ;
nullishCoalescing → equality ( "??" equality )* ;
equality         → comparison ( ( "===" | "!==" | "==" | "!=" ) comparison )* ;
comparison       → shift ( ( ">" | ">=" | "<" | "<=" | "instanceof" | "in" ) shift )* ;
shift            → term ( ( "<<" | ">>" | ">>>" ) term )* ;
term             → factor ( ( "+" | "-" ) factor )* ;
factor           → exponent ( ( "*" | "/" | "%" ) exponent )* ;
exponent         → unary ( "**" unary )* ;
unary            → ( "!" | "~" | "-" | "+" | "typeof" | "void" | "delete" | "await" ) unary
                 | postfix ;
postfix          → call ( "++" | "--" )? ;

call             → primary ( "(" arguments? ")"
                 | "." IDENTIFIER
                 | "?." IDENTIFIER
                 | "[" expression "]"
                 | "!"
                 | "as" type )* ;

arguments        → argument ( "," argument )* ","? ;
argument         → "..."? expression ;

primary          → NUMBER
                 | STRING
                 | "true"
                 | "false"
                 | "null"
                 | "undefined"
                 | "this"
                 | "super"
                 | IDENTIFIER
                 | "(" expression ")"
                 | "[" arrayElements? "]"
                 | "{" objectLiteral? "}"
                 | arrowFunction
                 | "new" call
                 | "async" arrowFunction ;

arrayElements    → spreadOrExpr ( "," spreadOrExpr )* ","? ;
spreadOrExpr     → "..."? expression ;

objectLiteral    → objectProperty ( "," objectProperty )* ","? ;
objectProperty   → "..."? ( IDENTIFIER | STRING | "[" expression "]" ) ( ":" expression )? ;

arrowFunction    → ( IDENTIFIER | "(" parameters? ")" ) ( ":" type )? "=>" ( block | expression ) ;

templateString   → "`" ( STRING | "${" expression "}" )* "`" ;

// Types (TypeScript-style)
type             → unionType ;
unionType        → intersectionType ( "|" intersectionType )* ;
intersectionType → primaryType ( "&" primaryType )* ;

primaryType      → primitiveType
                 | literalType
                 | arrayType
                 | tupleType
                 | functionType
                 | objectType
                 | typeReference
                 | typeQuery
                 | conditionalType
                 | "(" type ")"
                 | type "?"
                 | type "[" "]" ;

primitiveType    → "number" | "string" | "boolean" | "void" | "any" | "unknown" | "never" | "null" | "undefined" | "symbol" | "bigint" ;
literalType      → NUMBER | STRING | "true" | "false" ;
arrayType        → type "[" "]" | "Array" "<" type ">" ;
tupleType        → "[" type ( "," type )* ","? "]" ;
functionType     → typeParams? "(" parameters? ")" "=>" type ;
objectType       → "{" typeMember ( "," | ";" typeMember )* ","? "}" ;
typeMember       → IDENTIFIER "?" ":" type
                 | IDENTIFIER typeParams? "(" parameters? ")" ":" type
                 | "[" IDENTIFIER ":" type "]" ":" type ;
typeReference    → IDENTIFIER ( "<" type ( "," type )* ">" )? ;
typeQuery        → "typeof" IDENTIFIER ;
conditionalType  → type "extends" type "?" type ":" type ;

*/

use std::fmt::Arguments;

use diagnostic::{
  code::DiagnosticCode,
  diagnostic::{Diagnostic, LabelStyle},
  source_map,
  types::{error::DiagnosticError, warning::DiagnosticWarning},
  DiagnosticEngine, SourceFile, SourceMap,
};
use lexer::token::{Token, TokenKind};

use crate::{
  expr::{ArrayElement, BinaryOp, Expr, ObjectProperty, Param, PropertyKey, Stmt, Type, UnaryOp},
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
          stmt.print_tree(&self.source_file);
          self.ast.push(stmt);
        },
        Err(_) => self.synchronize(engine),
      }
    }
  }

  /// Parses a declaration, currently delegating to statement parsing.
  fn parse_declaration(&mut self, engine: &mut DiagnosticEngine) -> Result<Stmt, ()> {
    match self.current_token().kind {
      _ => self.parse_stmt(engine),
    }
  }

  /* -------------------------------------------------------------------------------------------- */
  /*                                         Fn Declaration                                       */
  /* -------------------------------------------------------------------------------------------- */
  fn parse_fn_decl(&mut self, engine: &mut DiagnosticEngine) -> Result<Stmt, ()> {
    // self.expect(TokenKind::Fn, engine)?; // consume the "fn"

    Err(())
  }

  /* -------------------------------------------------------------------------------------------- */
  /*                                         Statements                                           */
  /* -------------------------------------------------------------------------------------------- */

  /// Parses a single statement node (stubbed for future grammar branches).
  fn parse_stmt(&mut self, engine: &mut DiagnosticEngine) -> Result<Stmt, ()> {
    match self.current_token().kind {
      _ => self.parse_expr_stmt(engine),
    }
  }

  /* -------------------------------------------------------------------------------------------- */
  /*                                         Break Statement                                       */
  /* -------------------------------------------------------------------------------------------- */

  fn parse_break_stmt(&mut self, engine: &mut DiagnosticEngine) -> Result<Stmt, ()> {
    let token = self.current_token();
    self.advance(engine); // consume the break keyword
    self.expect(TokenKind::Semicolon, engine)?; // ensure the statement is terminated

    Ok(Stmt::Break { span: token.span })
  }

  /* -------------------------------------------------------------------------------------------- */
  /*                                         Continue Statement                                    */
  /* -------------------------------------------------------------------------------------------- */

  fn parse_continue_stmt(&mut self, engine: &mut DiagnosticEngine) -> Result<Stmt, ()> {
    let token = self.current_token();
    self.advance(engine); // consume the continue keyword
    self.expect(TokenKind::Semicolon, engine)?; // ensure the statement is terminated

    Ok(Stmt::Continue { span: token.span })
  }

  /* -------------------------------------------------------------------------------------------- */
  /*                                    Expression Statement                                      */
  /* -------------------------------------------------------------------------------------------- */
  fn parse_expr_stmt(&mut self, engine: &mut DiagnosticEngine) -> Result<Stmt, ()> {
    let expr = self.parse_expr(engine, ExprContext::Default)?;
    self.expect(TokenKind::Semicolon, engine)?; // ensure the statement is terminated

    Ok(Stmt::Expr(expr))
  }

  /* -------------------------------------------------------------------------------------------- */
  /*                                    Expression Statement                                      */
  /* -------------------------------------------------------------------------------------------- */

  fn parse_expr(
    &mut self,
    engine: &mut DiagnosticEngine,
    context: ExprContext,
  ) -> Result<Expr, ()> {
    self.parse_primary(engine, context)
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
      // handle any other token
      TokenKind::String => {
        self.advance(engine);
        Ok(Expr::String { span: token.span })
      },

      TokenKind::Number => {
        self.advance(engine);
        Ok(Expr::Number { span: token.span })
      },

      TokenKind::True => {
        self.advance(engine);
        Ok(Expr::Bool { span: token.span })
      },

      TokenKind::False => {
        self.advance(engine);
        Ok(Expr::Bool { span: token.span })
      },

      TokenKind::Null => {
        self.advance(engine);
        Ok(Expr::Null { span: token.span })
      },

      TokenKind::Undefined => {
        self.advance(engine);
        Ok(Expr::Undefined { span: token.span })
      },

      TokenKind::This => {
        self.advance(engine);
        Ok(Expr::This { span: token.span })
      },

      TokenKind::Super => {
        self.advance(engine);
        Ok(Expr::Super { span: token.span })
      },

      TokenKind::Identifier => {
        self.advance(engine);
        Ok(Expr::Identifier { span: token.span })
      },

      TokenKind::LeftParen => {
        self.advance(engine);
        let expr = self.parse_expr(engine, ExprContext::Default)?;
        self.expect(TokenKind::RightParen, engine)?;
        Ok(expr)
      },

      TokenKind::LeftBracket => self.parse_array(engine),
      TokenKind::LeftBrace => self.parse_object(engine),

      _ => {
        let lexeme = self
          .source_file
          .src
          .get(token.span.start..token.span.end)
          .unwrap();

        // make some diagnostic here
        let diagnostic = Diagnostic::new(
          DiagnosticCode::Error(DiagnosticError::UnexpectedToken),
          format!("Unexpected token {:?}", lexeme),
          self.source_file.path.clone(),
        )
        .with_label(
          token.span,
          Some(format!("Expected a primary expression, found {:?}", lexeme)),
          LabelStyle::Primary,
        );

        engine.add(diagnostic);

        Err(())
      },
    }
  }

  /* -------------------------------------------------------------------------------------------- */
  /*                                         Array Literal                                        */
  /* -------------------------------------------------------------------------------------------- */

  fn parse_array(&mut self, engine: &mut DiagnosticEngine) -> Result<Expr, ()> {
    let token = self.current_token();
    self.advance(engine);
    let mut elements = vec![];

    while !self.is_eof() {
      match self.current_token().kind {
        TokenKind::RightBracket => {
          self.advance(engine);
          break;
        },
        TokenKind::DotDotDot => {
          self.advance(engine);
          let expr = self.parse_expr(engine, ExprContext::Default)?;
          elements.push(ArrayElement::Spread(expr));
        },
        _ => {
          let expr = self.parse_expr(engine, ExprContext::Default)?;
          elements.push(ArrayElement::Expression(expr));

          if self.current_token().kind == TokenKind::Comma {
            self.expect(TokenKind::Comma, engine)?;
          }
        },
      }
    }

    Ok(Expr::Array {
      elements,
      span: token.span,
    })
  }

  /* -------------------------------------------------------------------------------------------- */
  /*                                         Object Literal                                        */
  /* -------------------------------------------------------------------------------------------- */

  fn parse_object(&mut self, engine: &mut DiagnosticEngine) -> Result<Expr, ()> {
    let token = self.current_token();
    self.advance(engine);
    let mut properties = vec![];

    while !self.is_eof() {
      match self.current_token().kind {
        TokenKind::RightBrace => {
          self.advance(engine);
          break;
        },
        _ => {
          let key = self.parse_property_key(engine)?;
          self.expect(TokenKind::Colon, engine)?;
          let value = self.parse_expr(engine, ExprContext::Default)?;
          properties.push(ObjectProperty::Property { key, value });

          if self.current_token().kind == TokenKind::Comma {
            self.expect(TokenKind::Comma, engine)?;
          }
        },
      }
    }

    Ok(Expr::Object {
      properties,
      span: token.span,
    })
  }

  /* -------------------------------------------------------------------------------------------- */
  /*                                         Property Key                                         */
  /* -------------------------------------------------------------------------------------------- */

  fn parse_property_key(&mut self, engine: &mut DiagnosticEngine) -> Result<PropertyKey, ()> {
    let lexeme = self.source_file.src.clone();
    let lexeme = lexeme
      .get(self.current_token().span.start..self.current_token().span.end)
      .unwrap();

    match self.current_token().kind {
      TokenKind::Identifier => {
        self.advance(engine);
        Ok(PropertyKey::Identifier(lexeme.to_string()))
      },
      TokenKind::String => {
        self.advance(engine);
        Ok(PropertyKey::String(lexeme.to_string()))
      },
      TokenKind::Number => {
        self.advance(engine);
        Ok(PropertyKey::Number(lexeme.parse::<f64>().unwrap()))
      },
      _ => {
        let diagnostic = Diagnostic::new(
          DiagnosticCode::Error(DiagnosticError::UnexpectedToken),
          "Unexpected token".to_string(),
          self.source_file.path.clone(),
        )
        .with_label(
          self.current_token().span,
          Some("Unexpected token".to_string()),
          LabelStyle::Primary,
        );

        engine.add(diagnostic);
        Err(())
      },
    }
  }
}
