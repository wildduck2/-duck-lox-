/*!
### Complete Rust Grammar (BNF) - Matching Full AST

program          → item* EOF ;

// ============================================================================
// Items (Top-level Declarations)
// ============================================================================

item             → attributes outerItem ;

outerItem        → functionItem
                 | structItem
                 | enumItem
                 | traitItem
                 | implItem
                 | constItem
                 | staticItem
                 | typeAliasItem
                 | moduleItem
                 | useItem
                 | externCrateItem
                 | macroItem
                 | foreignModItem
                 | unionItem ;

attributes       → attribute* ;
attribute        → "#" "[" attrContent "]"           // outer: #[...]
                 | "#" "!" "[" attrContent "]" ;     // inner: #![...]
attrContent      → path ( "(" tokenTree* ")" | "=" literal )? ;

// ----------------------------------------------------------------------------
// Function Item
// ----------------------------------------------------------------------------

functionItem     → visibility? fnQualifiers "fn" IDENTIFIER
                   genericParams? "(" parameters? ")" returnType?
                   whereClause? ( block | ";" ) ;

fnQualifiers     → "const"? "async"? "unsafe"? ( "extern" abi? )? ;
abi              → STRING ;  // "C", "system", etc.

parameters       → parameter ( "," parameter )* ","? ;
parameter        → attributes pattern ":" type ;

returnType       → "->" type ;

// ----------------------------------------------------------------------------
// Struct Item
// ----------------------------------------------------------------------------

structItem       → visibility? "struct" IDENTIFIER genericParams?
                   whereClause? structKind ;

structKind       → "{" structFields? "}"              // Named fields
                 | "(" tupleFields? ")" ";"           // Tuple struct
                 | ";" ;                              // Unit struct

structFields     → structField ( "," structField )* ","? ;
structField      → attributes visibility? IDENTIFIER ":" type ;

tupleFields      → tupleField ( "," tupleField )* ","? ;
tupleField       → attributes visibility? type ;

// ----------------------------------------------------------------------------
// Enum Item
// ----------------------------------------------------------------------------

enumItem         → visibility? "enum" IDENTIFIER genericParams?
                   whereClause? "{" enumVariants? "}" ;

enumVariants     → enumVariant ( "," enumVariant )* ","? ;
enumVariant      → attributes IDENTIFIER variantKind? discriminant? ;

variantKind      → "{" structFields? "}"              // Struct variant
                 | "(" tupleFields? ")" ;             // Tuple variant

discriminant     → "=" expression ;

// ----------------------------------------------------------------------------
// Trait Item
// ----------------------------------------------------------------------------

traitItem        → visibility? "unsafe"? "auto"? "trait" IDENTIFIER
                   genericParams? ( ":" supertraits )?
                   whereClause? "{" traitItems? "}" ;

supertraits      → typeBound ( "+" typeBound )* ;

traitItems       → traitItem* ;
traitItem        → attributes traitMember ;

traitMember      → traitMethod
                 | traitType
                 | traitConst
                 | macroInvocation ;

traitMethod      → fnQualifiers "fn" IDENTIFIER genericParams?
                   "(" parameters? ")" returnType?
                   whereClause? ( block | ";" ) ;

traitType        → "type" IDENTIFIER ( ":" typeBounds )? ( "=" type )? ";" ;

traitConst       → "const" IDENTIFIER ":" type ( "=" expression )? ";" ;

// ----------------------------------------------------------------------------
// Impl Block
// ----------------------------------------------------------------------------

implItem         → "unsafe"? "default"? "impl" genericParams?
                   implPolarity? traitRef? "for"? type
                   whereClause? "{" implItems? "}" ;

implPolarity     → "!" ;  // negative impl
traitRef         → path ;

implItems        → implMember* ;
implMember       → attributes implItem ;

implItem         → implMethod
                 | implType
                 | implConst
                 | macroInvocation ;

implMethod       → visibility? functionItem ;
implType         → visibility? "type" IDENTIFIER "=" type ";" ;
implConst        → visibility? "const" IDENTIFIER ":" type "=" expression ";" ;

// ----------------------------------------------------------------------------
// Other Items
// ----------------------------------------------------------------------------

constItem        → visibility? "const" IDENTIFIER ":" type "=" expression ";" ;

staticItem       → visibility? "static" "mut"? IDENTIFIER ":" type "=" expression ";" ;

typeAliasItem    → visibility? "type" IDENTIFIER genericParams?
                   whereClause? "=" type ";" ;

moduleItem       → visibility? "unsafe"? "mod" IDENTIFIER
                   ( "{" item* "}" | ";" ) ;

useItem          → visibility? "use" useTree ";" ;

useTree          → ( path? "::" )? useTreeSuffix ;
useTreeSuffix    → "*"                                    // glob
                 | "{" useTree ( "," useTree )* ","? "}"  // list
                 | IDENTIFIER ( "as" IDENTIFIER )? ;      // name/rename

externCrateItem  → visibility? "extern" "crate" IDENTIFIER
                   ( "as" IDENTIFIER )? ";" ;

macroItem        → "macro_rules" "!" IDENTIFIER "{" macroRules "}" ;
macroRules       → macroRule ( ";" macroRule )* ";"? ;
macroRule        → "(" tokenTree* ")" "=>" "{" tokenTree* "}" ;

foreignModItem   → "extern" abi "{" foreignItem* "}" ;
foreignItem      → attributes visibility? foreignMember ;
foreignMember    → foreignFunction | foreignStatic | foreignType ;

foreignFunction  → "fn" IDENTIFIER genericParams?
                   "(" parameters? ( "," "..." )? ")" returnType? ";" ;
foreignStatic    → "static" "mut"? IDENTIFIER ":" type ";" ;
foreignType      → "type" IDENTIFIER genericParams? ";" ;

unionItem        → visibility? "union" IDENTIFIER genericParams?
                   whereClause? "{" structFields "}" ;

// ============================================================================
// Generics System
// ============================================================================

genericParams    → "<" genericParam ( "," genericParam )* ","? ">" ;

genericParam     → attributes genericParamKind ;
genericParamKind → lifetimeParam | typeParam | constParam ;

lifetimeParam    → LIFETIME ( ":" lifetimeBounds )? ;
typeParam        → IDENTIFIER ( ":" typeBounds )? ( "=" type )? ;
constParam       → "const" IDENTIFIER ":" type ( "=" expression )? ;

lifetimeBounds   → LIFETIME ( "+" LIFETIME )* ;
typeBounds       → typeBound ( "+" typeBound )* ;

typeBound        → "?"? "const"? forLifetimes? path genericArgs? ;
forLifetimes     → "for" "<" LIFETIME ( "," LIFETIME )* ">" ;

whereClause      → "where" wherePredicate ( "," wherePredicate )* ","? ;
wherePredicate   → forLifetimes? type ":" typeBounds
                 | LIFETIME ":" lifetimeBounds
                 | type "=" type ;  // associated type equality

// ----------------------------------------------------------------------------
// Generic Arguments
// ----------------------------------------------------------------------------

genericArgs      → "<" genericArg ( "," genericArg )* ","? ">"
                 | "(" types? ")" returnType? ;  // Fn(A, B) -> C

genericArg       → LIFETIME
                 | type
                 | expression  // const generics
                 | IDENTIFIER "=" type        // binding
                 | IDENTIFIER ":" typeBounds ; // constraint

// ============================================================================
// Visibility
// ============================================================================

visibility       → "pub" ( "(" visRestriction ")" )? ;
visRestriction   → "crate" | "super" | "self" | "in" path ;

// ============================================================================
// Types
// ============================================================================

type             → primitiveType
                 | referenceType
                 | rawPointerType
                 | arrayType
                 | sliceType
                 | tupleType
                 | pathType
                 | qPathType
                 | traitObjectType
                 | implTraitType
                 | bareFnType
                 | inferType
                 | parenType
                 | macroType ;

primitiveType    → "i8" | "i16" | "i32" | "i64" | "i128" | "isize"
                 | "u8" | "u16" | "u32" | "u64" | "u128" | "usize"
                 | "f32" | "f64"
                 | "bool" | "char" | "str"
                 | "!" ;  // never type

referenceType    → "&" LIFETIME? "mut"? type ;
rawPointerType   → "*" ( "const" | "mut" ) type ;
arrayType        → "[" type ";" expression "]" ;
sliceType        → "[" type "]" ;
tupleType        → "(" ( type ( "," type )* ","? )? ")" ;

pathType         → "::"? pathSegment ( "::" pathSegment )* ;
pathSegment      → IDENTIFIER genericArgs? ;

qPathType        → "<" type ( "as" path )? ">" "::" IDENTIFIER ;

traitObjectType  → "dyn"? typeBounds ( "+" LIFETIME )? ;
implTraitType    → "impl" typeBounds ;

bareFnType       → forLifetimes? fnQualifiers
                   "fn" "(" types? ( "," "..." )? ")" returnType? ;

types            → type ( "," type )* ","? ;

inferType        → "_" ;
parenType        → "(" type ")" ;
macroType        → macroInvocation ;

// ============================================================================
// Statements
// ============================================================================

statement        → ";"                              // empty
                 | item
                 | letStatement
                 | expressionStatement ;

letStatement     → attributes "let" pattern ( ":" type )?
                   ( "=" expression )?
                   ( "else" block )? ";" ;  // let-else

expressionStatement → expression ";"? ;  // optional semicolon

// ============================================================================
// Expressions (Pratt Parsing / Precedence Climbing)
// ============================================================================

expression       → assignmentExpr ;

// Assignment (right-associative)
assignmentExpr   → closureExpr ( ( "=" | "+=" | "-=" | "*=" | "/=" | "%="
                                 | "&=" | "|=" | "^=" | "<<=" | ">>=" ) assignmentExpr )? ;

// Closures
closureExpr      → rangeExpr
                 | "move"? "async"? "|" closureParams "|" returnType? closureBody ;

closureParams    → closureParam ( "," closureParam )* ","? ;
closureParam     → attributes pattern ( ":" type )? ;
closureBody      → expression | block ;

// Range expressions
rangeExpr        → orExpr ( ( ".." | "..=" ) orExpr? )? ;

// Logical OR (left-associative)
orExpr           → andExpr ( "||" andExpr )* ;

// Logical AND (left-associative)
andExpr          → comparisonExpr ( "&&" comparisonExpr )* ;

// Comparison (non-associative, but we allow chaining)
comparisonExpr   → bitwiseOrExpr ( comparisonOp bitwiseOrExpr )* ;
comparisonOp     → "==" | "!=" | "<" | "<=" | ">" | ">=" ;

// Bitwise OR
bitwiseOrExpr    → bitwiseXorExpr ( "|" bitwiseXorExpr )* ;

// Bitwise XOR
bitwiseXorExpr   → bitwiseAndExpr ( "^" bitwiseAndExpr )* ;

// Bitwise AND
bitwiseAndExpr   → shiftExpr ( "&" shiftExpr )* ;

// Shift
shiftExpr        → addExpr ( ( "<<" | ">>" ) addExpr )* ;

// Addition/Subtraction
addExpr          → mulExpr ( ( "+" | "-" ) mulExpr )* ;

// Multiplication/Division/Modulo
mulExpr          → castExpr ( ( "*" | "/" | "%" ) castExpr )* ;

// Cast / Type ascription
castExpr         → unaryExpr ( "as" type )* ;

// Unary
unaryExpr        → ( "-" | "!" | "*" | "&" "mut"? | "&&" "mut"? ) unaryExpr
                 | awaitExpr ;

// Await
awaitExpr        → postfixExpr ( "." "await" )* ;

// Postfix (method calls, field access, indexing, try)
postfixExpr      → primaryExpr postfixOp* ;

postfixOp        → "(" arguments? ")"                    // call
                 | "." IDENTIFIER genericArgs? "(" arguments? ")"  // method
                 | "." IDENTIFIER                        // field
                 | "." INTEGER                           // tuple field
                 | "[" expression "]"                    // index
                 | "?" ;                                 // try

arguments        → expression ( "," expression )* ","? ;

// ============================================================================
// Primary Expressions
// ============================================================================

primaryExpr      → literal
                 | pathExpr
                 | structExpr
                 | arrayExpr
                 | tupleExpr
                 | groupedExpr
                 | blockExpr
                 | ifExpr
                 | matchExpr
                 | loopExpr
                 | whileExpr
                 | forExpr
                 | returnExpr
                 | breakExpr
                 | continueExpr
                 | yieldExpr
                 | unsafeExpr
                 | constExpr
                 | asyncExpr
                 | letExpr
                 | boxExpr
                 | underscoreExpr
                 | macroInvocation ;

// ----------------------------------------------------------------------------
// Literals
// ----------------------------------------------------------------------------

literal          → INTEGER intSuffix?
                 | FLOAT floatSuffix?
                 | STRING
                 | RAW_STRING
                 | BYTE_STRING
                 | RAW_BYTE_STRING
                 | CHAR
                 | BYTE
                 | "true"
                 | "false" ;

intSuffix        → "i8" | "i16" | "i32" | "i64" | "i128" | "isize"
                 | "u8" | "u16" | "u32" | "u64" | "u128" | "usize" ;

floatSuffix      → "f32" | "f64" ;

// ----------------------------------------------------------------------------
// Path Expression
// ----------------------------------------------------------------------------

pathExpr         → "::"? pathSegment ( "::" pathSegment )* ;

// ----------------------------------------------------------------------------
// Struct Expression
// ----------------------------------------------------------------------------

structExpr       → pathExpr "{" structFields? "}" ;
structFields     → structField ( "," structField )* ","? ( ".." expression )? ;
structField      → attributes IDENTIFIER ( ":" expression )? ;

// ----------------------------------------------------------------------------
// Array and Tuple
// ----------------------------------------------------------------------------

arrayExpr        → "[" arrayElements? "]" ;
arrayElements    → expression ( "," expression )* ","?
                 | expression ";" expression ;  // [val; count]

tupleExpr        → "(" ( expression ( "," expression )* ","? )? ")" ;

// ----------------------------------------------------------------------------
// Grouped and Blocks
// ----------------------------------------------------------------------------

groupedExpr      → "(" expression ")" ;

blockExpr        → attributes label? "{" statement* expression? "}" ;
label            → LIFETIME ":" ;  // 'label:

unsafeExpr       → "unsafe" block ;
constExpr        → "const" block ;
asyncExpr        → attributes "async" "move"? block ;

// ----------------------------------------------------------------------------
// Control Flow
// ----------------------------------------------------------------------------

ifExpr           → "if" expression block ( "else" ( ifExpr | block ) )? ;

matchExpr        → "match" expression "{" matchArm* "}" ;
matchArm         → attributes pattern ( "if" expression )? "=>"
                   ( expression ","? | block ","? ) ;

loopExpr         → label? "loop" block ;
whileExpr        → label? "while" expression block ;
forExpr          → label? "for" pattern "in" expression block ;

returnExpr       → "return" expression? ;
breakExpr        → "break" LIFETIME? expression? ;
continueExpr     → "continue" LIFETIME? ;
yieldExpr        → "yield" expression? ;

// ----------------------------------------------------------------------------
// Other Expressions
// ----------------------------------------------------------------------------

letExpr          → "let" pattern "=" expression ;
boxExpr          → "box" expression ;
underscoreExpr   → "_" ;

// ============================================================================
// Patterns
// ============================================================================

pattern          → "|"? patternNoOr ( "|" patternNoOr )* ;

patternNoOr      → wildcardPattern
                 | restPattern
                 | literalPattern
                 | identPattern
                 | refPattern
                 | structPattern
                 | tupleStructPattern
                 | tuplePattern
                 | slicePattern
                 | pathPattern
                 | rangePattern
                 | boxPattern
                 | macroPattern
                 | groupedPattern ;

wildcardPattern  → "_" ;
restPattern      → ".." ;
literalPattern   → literal | "-" literal ;

identPattern     → "ref"? "mut"? IDENTIFIER ( "@" pattern )? ;
refPattern       → "&" "mut"? pattern ;

structPattern    → pathExpr "{" structPatFields? "}" ;
structPatFields  → structPatField ( "," structPatField )* ","? ".."? ;
structPatField   → attributes ( INTEGER | IDENTIFIER ) ( ":" pattern )? ;

tupleStructPattern → pathExpr "(" patterns? ")" ;
tuplePattern     → "(" patterns? ")" ;
slicePattern     → "[" patterns? "]" ;

patterns         → pattern ( "," pattern )* ","? ;

pathPattern      → "::"? pathSegment ( "::" pathSegment )* ;

rangePattern     → literal ".."? "="? literal ;

boxPattern       → "box" pattern ;
macroPattern     → macroInvocation ;
groupedPattern   → "(" pattern ")" ;

// ============================================================================
// Macros
// ============================================================================

macroInvocation  → path "!" tokenTree ;

tokenTree        → "(" tokenTree* ")"
                 | "[" tokenTree* "]"
                 | "{" tokenTree* "}"
                 | TOKEN ;

// ============================================================================
// Common Productions
// ============================================================================

path             → "::"? pathSegment ( "::" pathSegment )* ;

block            → "{" statement* expression? "}" ;

// ============================================================================
// Lexical Tokens
// ============================================================================

IDENTIFIER       → [a-zA-Z_][a-zA-Z0-9_]* ;
LIFETIME         → "'" IDENTIFIER ;
INTEGER          → [0-9]+ | "0x"[0-9a-fA-F]+ | "0o"[0-7]+ | "0b"[01]+ ;
FLOAT            → [0-9]+ "." [0-9]+ ( "e" [+-]? [0-9]+ )? ;
STRING           → "\"" ( ESCAPE | ~["\\] )* "\"" ;
RAW_STRING       → "r" "#"* "\"" ~["]* "\"" "#"* ;
BYTE_STRING      → "b\"" ( ESCAPE | ~["\\] )* "\"" ;
RAW_BYTE_STRING  → "br" "#"* "\"" ~["]* "\"" "#"* ;
CHAR             → "'" ( ESCAPE | ~['\\] ) "'" ;
BYTE             → "b'" ( ESCAPE | ~['\\] ) "'" ;
ESCAPE           → "\\" ( ["\\nrt0] | "x"[0-9a-fA-F]{2} | "u{" [0-9a-fA-F]+ "}" ) ;
TOKEN            → any non-delimiter token ;

*/
