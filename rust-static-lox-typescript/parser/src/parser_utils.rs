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
  DiagnosticEngine, SourceFile, SourceMap, Span,
};
use lexer::token::{Token, TokenKind};

use crate::{
  expr::{
    ArrayElement, ArrowBody, BinaryOp, Expr, FunctionParam, ObjectProperty, Param, PropertyKey,
    Stmt, Type, TypeMember, TypeParam, UnaryOp,
  },
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
    loop {
      if self.is_eof() {
        break;
      }
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
    while !self.is_eof() {
      let typee = self.parse_type(engine)?;
      println!("{:#?}", typee);
    }

    Err(())
    // self.parse_primary(engine, context)
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
    let span = token.span;

    match token.kind {
      // handle any other token
      TokenKind::String => {
        self.advance(engine);
        Ok(Expr::String { span })
      },

      TokenKind::Number => {
        self.advance(engine);
        Ok(Expr::Number { span })
      },

      TokenKind::True => {
        self.advance(engine);
        Ok(Expr::Bool { span })
      },

      TokenKind::False => {
        self.advance(engine);
        Ok(Expr::Bool { span })
      },

      TokenKind::Null => {
        self.advance(engine);
        Ok(Expr::Null { span })
      },

      TokenKind::Undefined => {
        self.advance(engine);
        Ok(Expr::Undefined { span })
      },

      TokenKind::This => {
        self.advance(engine);
        Ok(Expr::This { span })
      },

      TokenKind::Super => {
        self.advance(engine);
        Ok(Expr::Super { span })
      },

      TokenKind::Identifier => {
        self.advance(engine);
        Ok(Expr::Identifier { span })
      },

      // TokenKind::LeftParen => {}
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
  /*                                         Arrow Function                                       */
  /* -------------------------------------------------------------------------------------------- */

  /* -------------------------------------------------------------------------------------------- */
  /*                                         Array Literal                                        */
  /* -------------------------------------------------------------------------------------------- */

  fn parse_array(&mut self, engine: &mut DiagnosticEngine) -> Result<Expr, ()> {
    let token = self.current_token();
    self.advance(engine);
    let mut elements = vec![];

    loop {
      if self.is_eof() {
        break;
      }
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
        TokenKind::Comma => self.advance(engine),
        _ => {
          let expr = self.parse_expr(engine, ExprContext::Default)?;
          elements.push(ArrayElement::Expression(expr));
        },
      }
    }

    Ok(Expr::Array {
      elements,
      span: Span::new(token.span.start, self.current_token().span.end),
    })
  }

  /* -------------------------------------------------------------------------------------------- */
  /*                                         Object Literal                                       */
  /* -------------------------------------------------------------------------------------------- */

  fn parse_object(&mut self, engine: &mut DiagnosticEngine) -> Result<Expr, ()> {
    let token = self.current_token();
    self.advance(engine);
    let mut properties = vec![];

    loop {
      if self.is_eof() {
        break;
      }
      match self.current_token().kind {
        TokenKind::RightBrace => {
          self.advance(engine);
          break;
        },
        TokenKind::DotDotDot => {
          self.advance(engine);
          let expr = self.parse_expr(engine, ExprContext::Default)?;
          properties.push(ObjectProperty::Spread { expr });
        },
        TokenKind::Comma => self.advance(engine),
        _ => {
          let key = self.parse_property_key(engine)?;
          self.expect(TokenKind::Colon, engine)?;
          let value = self.parse_expr(engine, ExprContext::Default)?;
          properties.push(ObjectProperty::Property { key, value });
        },
      }
    }

    Ok(Expr::Object {
      properties,
      span: Span::new(token.span.start, self.current_token().span.end),
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
      TokenKind::LeftBracket => {
        self.advance(engine);
        let expr = self.parse_expr(engine, ExprContext::Default)?;
        self.expect(TokenKind::RightBracket, engine)?;
        Ok(PropertyKey::Computed(Box::new(expr)))
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

  /* -------------------------------------------------------------------------------------------- */
  /*                                         Type                                                 */
  /* -------------------------------------------------------------------------------------------- */

  fn parse_type(&mut self, engine: &mut DiagnosticEngine) -> Result<Type, ()> {
    let token = self.current_token();
    let lexeme = self.source_file.src[token.span.start..token.span.end].to_string();

    let r#type = match self.current_token().kind {
      // PRIMITIVE TYPES AND LITERALS
      TokenKind::Number => {
        if lexeme == "number" {
          self.advance(engine);
          Some(Type::Number)
        } else {
          Some(Type::NumberLiteral(lexeme.parse::<f64>().unwrap()))
        }
      },

      TokenKind::String => {
        if lexeme == "string" {
          self.advance(engine);
          Some(Type::String)
        } else {
          Some(Type::StringLiteral(lexeme.clone()))
        }
      },

      TokenKind::True => {
        self.advance(engine);
        Some(Type::BooleanLiteral(true))
      },
      TokenKind::False => {
        self.advance(engine);
        Some(Type::BooleanLiteral(false))
      },

      TokenKind::Boolean => {
        self.advance(engine);
        Some(Type::Boolean)
      },

      TokenKind::Void => {
        self.advance(engine);
        Some(Type::Void)
      },

      TokenKind::Any => {
        self.advance(engine);
        Some(Type::Any)
      },

      TokenKind::Unknown => {
        self.advance(engine);
        Some(Type::Unknown)
      },

      TokenKind::Never => {
        self.advance(engine);
        Some(Type::Never)
      },

      TokenKind::Null => {
        self.advance(engine);
        Some(Type::Null)
      },

      TokenKind::Undefined => {
        self.advance(engine);
        Some(Type::Undefined)
      },

      // TODO: idk how to parse symbols
      TokenKind::Symbol => {
        self.advance(engine);
        Some(Type::Symbol)
      },

      TokenKind::BigInt => {
        self.advance(engine);
        Some(Type::BigInt)
      },

      TokenKind::LeftBracket => Some(self.parse_tuple_type(engine)?),
      TokenKind::LeftBrace => Some(self.parse_object_type(engine)?),
      TokenKind::Identifier if lexeme == "Array" => Some(self.parse_array_generic(engine)?),
      TokenKind::Identifier => {
        self.advance(engine);
        Some(Type::Reference {
          name: lexeme.to_string(),
          type_args: vec![],
        })
      },

      _ => None,
    };

    match r#type {
      Some(r#type) => {
        if self.current_token().kind == TokenKind::LeftBracket {
          self.expect(TokenKind::LeftBracket, engine)?; // consume the "["
          self.expect(TokenKind::RightBracket, engine)?; // consume the "]"

          Ok(Type::Array(Box::new(r#type)))
        } else {
          Ok(r#type)
        }
      },
      None => {
        let diagnostic = Diagnostic::new(
          DiagnosticCode::Error(DiagnosticError::UnexpectedToken),
          format!("Unexpected token {:?}", lexeme),
          self.source_file.path.clone(),
        )
        .with_label(
          token.span,
          Some(format!(
            "Expected a primary expression, found {:?}",
            lexeme.clone()
          )),
          LabelStyle::Primary,
        );

        engine.add(diagnostic);

        Err(())
      },
    }
  }

  /* -------------------------------------------------------------------------------------------- */
  /*                                         Object Type                                        */
  /* -------------------------------------------------------------------------------------------- */

  fn parse_object_type(&mut self, engine: &mut DiagnosticEngine) -> Result<Type, ()> {
    self.advance(engine);
    let mut properties = vec![];
    while self.current_token().kind != TokenKind::RightBrace {
      properties.push(self.parse_type_member(engine)?);
      if self.current_token().kind == TokenKind::Comma {
        self.expect(TokenKind::Comma, engine)?; // consume the ","
      }
    }

    self.expect(TokenKind::RightBrace, engine)?; // consume the "}"
    Ok(Type::Object {
      members: properties,
    })
  }

  /* -------------------------------------------------------------------------------------------- */
  /*                                         Type Member                                          */
  /* -------------------------------------------------------------------------------------------- */
  fn parse_type_member(&mut self, engine: &mut DiagnosticEngine) -> Result<TypeMember, ()> {
    // `[key: KeyType]: ValueType`
    if self.current_token().kind == TokenKind::LeftBracket {
      self.advance(engine);

      let key_name = self.get_key_name(engine)?;
      self.expect(TokenKind::Colon, engine)?;
      let key_type = self.parse_type(engine)?;

      self.expect(TokenKind::RightBracket, engine)?;

      let is_method = if self.current_token().kind == TokenKind::LeftParen {
        self.expect(TokenKind::LeftParen, engine)?;
        self.expect(TokenKind::RightParen, engine)?;
        true
      } else {
        false
      };

      let optional = self.is_optional(engine);

      self.expect(TokenKind::Colon, engine)?;
      let type_annotation = self.parse_type(engine)?;

      if is_method {
        return Ok(TypeMember::CallSignature {
          key_name,
          key_type,
          return_type: type_annotation,
        });
      }

      return Ok(TypeMember::IndexSignature {
        key_name,
        key_type,
        type_annotation,
        optional,
      });
    }

    // `method<T>(a: A): R`
    let key_name = self.parse_property_key(engine)?;

    if matches!(
      self.current_token().kind,
      TokenKind::Less | TokenKind::LeftParen
    ) {
      let type_params = if self.current_token().kind == TokenKind::Less {
        let type_params = self.parse_type_params(engine)?;
        type_params
      } else {
        vec![]
      };

      let params = if self.current_token().kind == TokenKind::LeftParen {
        self.parse_function_params(engine)?
      } else {
        vec![]
      };

      self.expect(TokenKind::Colon, engine)?;

      let return_type = self.parse_type(engine)?;

      return Ok(TypeMember::Method {
        key_name,
        type_params,
        params,
        return_type,
        optional: false,
      });
    }

    // `prop?: Type`
    let optional = self.is_optional(engine);
    self.expect(TokenKind::Colon, engine)?;
    let type_annotation = self.parse_type(engine)?;

    Ok(TypeMember::Property {
      key_name,
      type_annotation,
      optional,
    })
  }

  fn is_optional(&mut self, engine: &mut DiagnosticEngine) -> bool {
    if self.current_token().kind == TokenKind::Question {
      self.advance(engine);
      true
    } else {
      false
    }
  }

  fn get_key_name(&mut self, engine: &mut DiagnosticEngine) -> Result<String, ()> {
    let key = self.current_token();
    if !matches!(key.kind, TokenKind::Identifier | TokenKind::String) {
      self.error_expected_token(TokenKind::Identifier, key, engine);
      return Err(());
    }
    self.advance(engine);

    let key_name = self
      .source_file
      .src
      .get(key.span.start..key.span.end)
      .unwrap()
      .to_string();

    Ok(key_name)
  }

  /* -------------------------------------------------------------------------------------------- */
  /*                                         Type Function Params                                 */
  /* -------------------------------------------------------------------------------------------- */
  fn parse_function_params(
    &mut self,
    engine: &mut DiagnosticEngine,
  ) -> Result<Vec<FunctionParam>, ()> {
    self.advance(engine);
    let mut type_params = Vec::<FunctionParam>::new();
    loop {
      if self.is_eof() {
        break;
      }

      match self.current_token().kind {
        TokenKind::RightParen => {
          self.advance(engine);
          break;
        },
        _ => {
          let name = self.current_token();
          if !matches!(name.kind, TokenKind::Identifier | TokenKind::String) {
            self.error_expected_token(TokenKind::Identifier, name, engine);
            break;
          }

          self.advance(engine);
          let optional = self.is_optional(engine);
          self.expect(TokenKind::Colon, engine)?;
          let type_annotation = self.parse_type(engine)?;

          if self.current_token().kind == TokenKind::Comma {
            self.expect(TokenKind::Comma, engine)?; // consume the ","
          }

          type_params.push(FunctionParam {
            name: self
              .source_file
              .src
              .get(name.span.start..name.span.end)
              .unwrap()
              .to_string(),
            type_annotation,
            optional,
          });
        },
      }
    }

    Ok(type_params)
  }

  /* -------------------------------------------------------------------------------------------- */
  /*                                         Type Params                                          */
  /* -------------------------------------------------------------------------------------------- */
  fn parse_type_params(&mut self, engine: &mut DiagnosticEngine) -> Result<Vec<TypeParam>, ()> {
    self.expect(TokenKind::Less, engine)?;
    let mut type_params = Vec::<TypeParam>::new();

    loop {
      if self.is_eof() {
        break;
      }

      match self.current_token().kind {
        TokenKind::Comma => {
          self.advance(engine);
          break;
        },
        _ => {
          type_params.push(self.parse_type_param(engine)?);
          break;
        },
      }
    }

    self.expect(TokenKind::Greater, engine)?;

    Ok(type_params)
  }

  fn parse_type_param(&mut self, engine: &mut DiagnosticEngine) -> Result<TypeParam, ()> {
    // Expect an identifier (like `T`)
    let name_token = self.current_token();
    if !matches!(name_token.kind, TokenKind::Identifier) {
      self.error_expected_token(TokenKind::Identifier, name_token, engine);
      return Err(());
    }

    // Extract the name
    let name = self
      .source_file
      .src
      .get(name_token.span.start..name_token.span.end)
      .unwrap()
      .to_string();

    self.advance(engine); // consume the name (T)

    // Optional constraint: T extends bool ? string : false
    let mut constraint: Option<Type> = None;

    if self.peek_is("extends") {
      self.advance(engine); // consume "extends"
      let check_type = Type::Reference {
        name: name.clone(),
        type_args: vec![],
      };
      let extends_type = self.parse_type(engine)?;

      // Conditional case: extends bool ? string : false
      if self.current_token().kind == TokenKind::Question {
        self.expect(TokenKind::Question, engine)?;
        let true_type = self.parse_type(engine)?;
        self.expect(TokenKind::Colon, engine)?;
        let false_type = self.parse_type(engine)?;

        constraint = Some(Type::Conditional {
          check_type: Box::new(check_type),
          extends_type: Box::new(extends_type),
          true_type: Box::new(true_type),
          false_type: Box::new(false_type),
        });
      } else {
        constraint = Some(extends_type);
      }
    }

    Ok(TypeParam {
      name: Type::Reference {
        name,
        type_args: vec![],
      },
      constraint,
      default: None,
    })
  }

  /* -------------------------------------------------------------------------------------------- */
  /*                                         Array Generic                                        */
  /* -------------------------------------------------------------------------------------------- */

  fn parse_array_generic(&mut self, engine: &mut DiagnosticEngine) -> Result<Type, ()> {
    self.advance(engine);
    self.expect(TokenKind::Less, engine)?; // consume the "<"
    let r#type = self.parse_type(engine)?;
    self.expect(TokenKind::Greater, engine)?; // consume the ">"
    Ok(Type::Array(Box::new(r#type)))
  }

  /* -------------------------------------------------------------------------------------------- */
  /*                                         Tuple Type                                         */
  /* -------------------------------------------------------------------------------------------- */

  fn parse_tuple_type(&mut self, engine: &mut DiagnosticEngine) -> Result<Type, ()> {
    self.advance(engine);
    let mut types = vec![];
    while self.current_token().kind != TokenKind::RightBracket {
      let r#type = self.parse_type(engine)?;
      if self.current_token().kind == TokenKind::Comma {
        self.expect(TokenKind::Comma, engine)?; // consume the ","
      }
      types.push(r#type);
    }

    self.expect(TokenKind::RightBracket, engine)?; // consume the "]"
    Ok(Type::Tuple(types))
  }
}
