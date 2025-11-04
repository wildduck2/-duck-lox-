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
enumMember       → IDENTIFIER ( "=" ( INTEGER | STRING ) )? ;

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

primary          → INTEGER
                 | FLOAT
                 | STRING
                 | TEMPLATE_STRING
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
literalType      → INTEGER | STRING | "true" | "false" ;
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
  diagnostic::{Diagnostic, LabelStyle, Span},
  types::{error::DiagnosticError, warning::DiagnosticWarning},
  DiagnosticEngine,
};
use lexer::token::{Token, TokenKind};

use crate::{
  expr::{BinaryOp, Expr, Param, Stmt, Type, UnaryOp},
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
      _ => self.parse_stmt(engine),
    }
  }

  /* -------------------------------------------------------------------------------------------- */
  /*                                         Fn Declaration                                       */
  /* -------------------------------------------------------------------------------------------- */
  fn parse_fn_decl(&mut self, engine: &mut DiagnosticEngine) -> Result<Stmt, ()> {
    self.expect(TokenKind::Fn, engine)?; // consume the "fn"

    Err(())
  }

  /* -------------------------------------------------------------------------------------------- */
  /*                                         Statements                                           */
  /* -------------------------------------------------------------------------------------------- */

  /// Parses a single statement node (stubbed for future grammar branches).
  fn parse_stmt(&mut self, engine: &mut DiagnosticEngine) -> Result<Stmt, ()> {
    match self.current_token().kind {
      _ => {
        // Fallback to an expression statement when no declaration keyword is found.
        // let expr = self.parse_expr_stmt(engine)?;
        // self.expect(TokenKind::Semicolon, engine)?; // ensure the statement is terminated
        //
        // Ok(Stmt::Expr(expr))
        Err(())
      },
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
}
