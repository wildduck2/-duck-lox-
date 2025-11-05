/*!
### Complete Rust Grammar (BNF) - Full Language Coverage
### Matching Complete AST with All Features

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
                 | macroRulesItem
                 | macro2Item
                 | foreignModItem
                 | unionItem
                 | externTypeItem ;

// ----------------------------------------------------------------------------
// Attributes (COMPLETE)
// ----------------------------------------------------------------------------

attributes       → attribute* ;

attribute        → outerAttr | innerAttr ;

outerAttr        → "#" "[" attrContent "]" ;
innerAttr        → "#" "!" "[" attrContent "]" ;

attrContent      → metaItem | cfgAttr | cfgAttrAttr ;

// Normal meta items
metaItem         → path ( "(" metaSeq ")" | "=" literal )? ;
metaSeq          → metaItem ( "," metaItem )* ","? ;

// Cfg attributes: #[cfg(target_os = "linux")]
cfgAttr          → "cfg" "(" cfgPredicate ")" ;

cfgPredicate     → cfgPredicateAnd
                 | cfgPredicateOr
                 | cfgPredicateNot
                 | cfgPredicateValue ;

cfgPredicateAnd  → "all" "(" cfgPredicate ( "," cfgPredicate )* ")" ;
cfgPredicateOr   → "any" "(" cfgPredicate ( "," cfgPredicate )* ")" ;
cfgPredicateNot  → "not" "(" cfgPredicate ")" ;
cfgPredicateValue→ IDENTIFIER ( "=" STRING )? ;

// Cfg attr: #[cfg_attr(test, derive(Debug))]
cfgAttrAttr      → "cfg_attr" "(" cfgPredicate "," metaItem ( "," metaItem )* ")" ;

// Doc comments are parsed as attributes internally
docComment       → "///" .*
                 | "//!" .*
                 | "/**" .* "*/"
                 | "/*!" .* "*/" ;

// ----------------------------------------------------------------------------
// Function Item
// ----------------------------------------------------------------------------

functionItem     → visibility? fnQualifiers "fn" IDENTIFIER
                   genericParams? "(" parameters? ")" returnType?
                   whereClause? ( block | ";" ) ;

fnQualifiers     → "const"? "async"? "unsafe"? ( "extern" abi? )? ;
abi              → STRING ;

parameters       → parameter ( "," parameter )* ","? ;
parameter        → attributes ( pattern | "..." ) ( ":" type )? ;

returnType       → "->" type ;

// ----------------------------------------------------------------------------
// Struct Item
// ----------------------------------------------------------------------------

structItem       → visibility? "struct" IDENTIFIER genericParams?
                   whereClause? structKind ;

structKind       → "{" structFields? "}"
                 | "(" tupleFields? ")" ";"
                 | ";" ;

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

variantKind      → "{" structFields? "}"
                 | "(" tupleFields? ")" ;

discriminant     → "=" expression ;

// ----------------------------------------------------------------------------
// Trait Item
// ----------------------------------------------------------------------------

traitItem        → visibility? "unsafe"? "auto"? "trait" IDENTIFIER
                   genericParams? ( ":" supertraits )?
                   whereClause? "{" traitItems "}" ;

supertraits      → typeBound ( "+" typeBound )* "+"? ;

traitItems       → traitItem* ;
traitItem        → attributes traitMember ;

traitMember      → traitMethod
                 | traitType
                 | traitConst
                 | macroInvocation ;

traitMethod      → fnQualifiers "fn" IDENTIFIER genericParams?
                   "(" parameters? ")" returnType?
                   whereClause? ( block | ";" ) ;

traitType        → "type" IDENTIFIER genericParams?  // GATs
                   ( ":" typeBounds )? ( "=" type )? ";" ;

traitConst       → "const" IDENTIFIER ":" type ( "=" expression )? ";" ;

// ----------------------------------------------------------------------------
// Impl Block
// ----------------------------------------------------------------------------

implItem         → "unsafe"? "default"? "impl" genericParams?
                   implPolarity? traitRef? "for"? type
                   whereClause? "{" implItems "}" ;

implPolarity     → "!" ;
traitRef         → path ;

implItems        → implMember* ;
implMember       → attributes implItemKind ;

implItemKind     → implMethod
                 | implType
                 | implConst
                 | macroInvocation ;

implMethod       → visibility? functionItem ;

implType         → visibility? "type" IDENTIFIER genericParams?  // GATs
                   "=" type ";" ;

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
useTreeSuffix    → "*"
                 | "{" useTree ( "," useTree )* ","? "}"
                 | IDENTIFIER ( "as" IDENTIFIER )? ;

externCrateItem  → visibility? "extern" "crate" IDENTIFIER
                   ( "as" IDENTIFIER )? ";" ;

// macro_rules! macro
macroRulesItem   → "macro_rules" "!" IDENTIFIER "{" macroRules "}" ;
macroRules       → macroRule ( ";" macroRule )* ";"? ;
macroRule        → "(" tokenTree* ")" "=>" "{" tokenTree* "}" ;

// NEW: macro 2.0 (declarative macros v2)
macro2Item       → visibility? "macro" IDENTIFIER "(" macroParams? ")"
                   "{" tokenTree* "}" ;
macroParams      → macroParam ( "," macroParam )* ","? ;
macroParam       → "$" IDENTIFIER ":" fragmentSpecifier ;

fragmentSpecifier→ "item" | "block" | "stmt" | "pat" | "expr" | "ty"
                 | "ident" | "path" | "meta" | "tt" | "lifetime" | "vis"
                 | "literal" ;

// Foreign module
foreignModItem   → "extern" abi? "{" foreignItem* "}" ;

foreignItem      → attributes visibility? foreignMember ;

foreignMember    → foreignFunction
                 | foreignStatic
                 | foreignType ;

foreignFunction  → "fn" IDENTIFIER genericParams?
                   "(" parameters? ( "," "..." )? ")" returnType?
                   whereClause? ";" ;

foreignStatic    → "static" "mut"? IDENTIFIER ":" type ";" ;

foreignType      → "type" IDENTIFIER genericParams? ";" ;

unionItem        → visibility? "union" IDENTIFIER genericParams?
                   whereClause? "{" structFields "}" ;

// NEW: extern type (opaque FFI type)
externTypeItem   → visibility? "extern" "type" IDENTIFIER genericParams? ";" ;

// ============================================================================
// Generics System (COMPLETE)
// ============================================================================

genericParams    → "<" genericParam ( "," genericParam )* ","? ">" ;

genericParam     → attributes genericParamKind ;

genericParamKind → lifetimeParam
                 | typeParam
                 | constParam ;

lifetimeParam    → LIFETIME ( ":" lifetimeBounds )? ;

typeParam        → IDENTIFIER ( ":" typeBounds )? ( "=" type )? ;

constParam       → "const" IDENTIFIER ":" type ( "=" expression )? ;

lifetimeBounds   → LIFETIME ( "+" LIFETIME )* ;

typeBounds       → typeBound ( "+" typeBound )* "+"? ;

typeBound        → traitBoundModifier? forLifetimes? path genericArgs? ;

traitBoundModifier → "?" | "?const" ;

forLifetimes     → "for" "<" lifetimeParam ( "," lifetimeParam )* ">" ;

whereClause      → "where" wherePredicate ( "," wherePredicate )* ","? ;

wherePredicate   → forLifetimes? type ":" typeBounds
                 | LIFETIME ":" lifetimeBounds
                 | type "=" type ;

// ----------------------------------------------------------------------------
// Generic Arguments (COMPLETE)
// ----------------------------------------------------------------------------

genericArgs      → angleBracketedArgs | parenthesizedArgs ;

angleBracketedArgs → "<" genericArg ( "," genericArg )* ","? ">" ;

parenthesizedArgs  → "(" types? ")" ( "->" type )? ;

genericArg       → LIFETIME
                 | type
                 | expression
                 | IDENTIFIER genericParams? "=" type
                 | IDENTIFIER genericParams? ":" typeBounds ;

// ============================================================================
// Visibility (COMPLETE)
// ============================================================================

visibility       → "pub" visRestriction? ;

visRestriction   → "(" ( "crate" | "super" | "self" | "in" path ) ")" ;

// ============================================================================
// Types (COMPLETE)
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
                 | typeofType
                 | parenType
                 | macroType ;

primitiveType    → "i8" | "i16" | "i32" | "i64" | "i128" | "isize"
                 | "u8" | "u16" | "u32" | "u64" | "u128" | "usize"
                 | "f32" | "f64"
                 | "bool" | "char" | "str"
                 | "!" ;

referenceType    → "&" LIFETIME? "mut"? type ;

rawPointerType   → "*" ( "const" | "mut" ) type ;

arrayType        → "[" type ";" expression "]" ;

sliceType        → "[" type "]" ;

tupleType        → "(" ( type ( "," type )* ","? )? ")" ;

pathType         → "::"? pathSegment ( "::" pathSegment )* ;

pathSegment      → pathIdentSegment genericArgs? ;

pathIdentSegment → IDENTIFIER
                 | "super" | "self" | "Self" | "crate"
                 | "$crate" ;  // in macros

qPathType        → "<" type ( "as" path )? ">" "::" IDENTIFIER genericArgs? ;

traitObjectType  → "dyn"? typeBounds ( "+" LIFETIME )? ;

implTraitType    → "impl" typeBounds ;

bareFnType       → forLifetimes? fnTypeQualifiers
                   "fn" "(" bareFnParams? ")" returnType? ;

fnTypeQualifiers → "unsafe"? ( "extern" abi? )? ;

bareFnParams     → bareFnParam ( "," bareFnParam )* ","?
                 | bareFnParam ( "," bareFnParam )* "," "..." ;

bareFnParam      → attributes ( IDENTIFIER ":" )? type ;

types            → type ( "," type )* ","? ;

inferType        → "_" ;

// NEW: typeof (unstable)
typeofType       → "typeof" "(" expression ")" ;

parenType        → "(" type ")" ;

macroType        → macroInvocation ;

// ============================================================================
// Statements
// ============================================================================

statement        → ";"
                 | item
                 | letStatement
                 | expressionStatement ;

letStatement     → attributes "let" pattern ( ":" type )?
                   ( "=" expression )?
                   ( "else" block )? ";" ;

expressionStatement → expression ";"? ;

// ============================================================================
// Expressions (COMPLETE - Pratt Parsing)
// ============================================================================

expression       → assignmentExpr ;

// Assignment (right-associative)
assignmentExpr   → closureExpr ( assignOp closureExpr )* ;

assignOp         → "=" | "+=" | "-=" | "*=" | "/=" | "%="
                 | "&=" | "|=" | "^=" | "<<=" | ">>=" ;

// Closures
closureExpr      → rangeExpr
                 | "move"? "async"? "|" closureParams? "|" returnType? closureBody ;

closureParams    → closureParam ( "," closureParam )* ","? ;

closureParam     → attributes pattern ( ":" type )? ;

closureBody      → expression ;

// Range expressions
rangeExpr        → orExpr ( rangeOp orExpr? )?
                 | rangeOp orExpr? ;

rangeOp          → ".." | "..=" ;

// Logical OR
orExpr           → andExpr ( "||" andExpr )* ;

// Logical AND
andExpr          → comparisonExpr ( "&&" comparisonExpr )* ;

// Comparison
comparisonExpr   → bitwiseOrExpr ( comparisonOp bitwiseOrExpr )* ;

comparisonOp     → "==" | "!=" | "<" | "<=" | ">" | ">=" ;

// Bitwise OR
bitwiseOrExpr    → bitwiseXorExpr ( "|" bitwiseXorExpr )* ;

// Bitwise XOR
bitwiseXorExpr   → bitwiseAndExpr ( "^" bitwiseAndExpr )* ;

// Bitwise AND
bitwiseAndExpr   → shiftExpr ( "&" shiftExpr )* ;

// Shift
shiftExpr        → addExpr ( shiftOp addExpr )* ;

shiftOp          → "<<" | ">>" ;

// Addition/Subtraction
addExpr          → mulExpr ( addOp mulExpr )* ;

addOp            → "+" | "-" ;

// Multiplication/Division/Modulo
mulExpr          → castExpr ( mulOp castExpr )* ;

mulOp            → "*" | "/" | "%" ;

// Cast / Type ascription
castExpr         → unaryExpr ( "as" type | ":" type )* ;

// Unary
unaryExpr        → unaryOp* awaitExpr ;

unaryOp          → "-" | "!" | "*" | "&" "mut"? | "&&" "mut"? ;

// Await
awaitExpr        → postfixExpr ( "." "await" )* ;

// Postfix (method calls, field access, indexing, try)
postfixExpr      → primaryExpr postfixOp* ;

postfixOp        → callOp
                 | methodCallOp
                 | fieldAccessOp
                 | tupleIndexOp
                 | indexOp
                 | tryOp ;

callOp           → "(" arguments? ")" ;

methodCallOp     → "." IDENTIFIER genericArgs? "(" arguments? ")" ;

fieldAccessOp    → "." IDENTIFIER ;

tupleIndexOp     → "." INTEGER ;

indexOp          → "[" expression "]" ;

tryOp            → "?" ;

arguments        → expression ( "," expression )* ","? ;

// ============================================================================
// Primary Expressions (COMPLETE)
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
                 | becomeExpr
                 | unsafeExpr
                 | constExpr
                 | inlineConstExpr
                 | asyncExpr
                 | tryBlockExpr
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

structExpr       → pathExpr "{" structExprFields? "}" ;

structExprFields → structExprField ( "," structExprField )* ","?
                   ( ".." expression )? ;

structExprField  → attributes ( IDENTIFIER | INTEGER ) ( ":" expression )? ;

// ----------------------------------------------------------------------------
// Array and Tuple
// ----------------------------------------------------------------------------

arrayExpr        → "[" arrayElements? "]" ;

arrayElements    → expression ( ";" expression | ( "," expression )* ","? ) ;

tupleExpr        → "(" tupleElements? ")" ;

tupleElements    → expression "," ( expression ( "," expression )* ","? )? ;

// ----------------------------------------------------------------------------
// Grouped and Blocks
// ----------------------------------------------------------------------------

groupedExpr      → "(" expression ")" ;

blockExpr        → attributes label? block ;

block            → "{" statement* expression? "}" ;

label            → LIFETIME ":" ;

unsafeExpr       → "unsafe" block ;

constExpr        → "const" block ;

// NEW: Inline const expression
inlineConstExpr  → "const" genericParams? block ;

asyncExpr        → attributes "async" "move"? block ;

// NEW: Try block
tryBlockExpr     → attributes "try" block ;

// ----------------------------------------------------------------------------
// Control Flow (COMPLETE)
// ----------------------------------------------------------------------------

ifExpr           → "if" expression block ( "else" ( ifExpr | block ) )? ;

matchExpr        → "match" expression "{" matchArm* "}" ;

matchArm         → attributes pattern matchArmGuard? "=>" matchArmBody ","? ;

matchArmGuard    → "if" expression ;

matchArmBody     → expression | block ;

loopExpr         → label? "loop" block ;

whileExpr        → label? "while" expression block ;

forExpr          → label? "for" pattern "in" expression block ;

returnExpr       → "return" expression? ;

breakExpr        → "break" LIFETIME? expression? ;

continueExpr     → "continue" LIFETIME? ;

yieldExpr        → "yield" expression? ;

// NEW: Become expression (tail call)
becomeExpr       → "become" expression ;

// ----------------------------------------------------------------------------
// Other Expressions
// ----------------------------------------------------------------------------

letExpr          → "let" pattern "=" expression ;

boxExpr          → "box" expression ;

underscoreExpr   → "_" ;

// ============================================================================
// Patterns (COMPLETE)
// ============================================================================

pattern          → patternNoTopAlt ( "|" patternNoTopAlt )* ;

patternNoTopAlt  → patternWithoutRange
                 | rangePattern ;

patternWithoutRange → wildcardPattern
                    | restPattern
                    | literalPattern
                    | identPattern
                    | refPattern
                    | structPattern
                    | tupleStructPattern
                    | tuplePattern
                    | slicePattern
                    | pathPattern
                    | boxPattern
                    | macroPattern
                    | groupedPattern ;

wildcardPattern  → "_" ;

restPattern      → ".." ;

literalPattern   → "-"? literal ;

identPattern     → "ref"? "mut"? IDENTIFIER ( "@" pattern )? ;

refPattern       → "&" "mut"? pattern ;

structPattern    → pathExpr "{" structPatFields? "}" ;

structPatFields  → structPatField ( "," structPatField )* ","? ( ".." )? ;

structPatField   → attributes ( INTEGER | IDENTIFIER ) ( ":" pattern )? ;

tupleStructPattern → pathExpr "(" tuplePatterns? ")" ;

tuplePattern     → "(" tuplePatterns? ")" ;

tuplePatterns    → pattern ( "," pattern )* ","? ;

slicePattern     → "[" slicePatElements? "]" ;

slicePatElements → pattern ( "," pattern )* ","? ;

pathPattern      → qPathExpr? pathExpr ;

qPathExpr        → "<" type ( "as" path )? ">" "::" ;

rangePattern     → rangePatternBound ( ".." | "..=" ) rangePatternBound
                 | ".." | "..=" rangePatternBound ;

rangePatternBound → literal | pathExpr ;

boxPattern       → "box" pattern ;

macroPattern     → macroInvocation ;

groupedPattern   → "(" pattern ")" ;

// ============================================================================
// Macros (COMPLETE)
// ============================================================================

macroInvocation  → path "!" delimiter ;

delimiter        → "(" tokenTree* ")"
                 | "[" tokenTree* "]"
                 | "{" tokenTree* "}" ;

tokenTree        → tokenTreeDelimited
                 | tokenTreeRepeat
                 | tokenTreeMetaVar
                 | TOKEN ;

tokenTreeDelimited → "(" tokenTree* ")"
                   | "[" tokenTree* "]"
                   | "{" tokenTree* "}" ;

tokenTreeRepeat  → "$" "(" tokenTree+ ")" tokenTreeSep? tokenTreeRepOp ;

tokenTreeSep     → TOKEN_NOT_DELIMITER ;

tokenTreeRepOp   → "*" | "+" | "?" ;

tokenTreeMetaVar → "$" IDENTIFIER ( ":" fragmentSpecifier )? ;

// ============================================================================
// Common Productions
// ============================================================================

path             → "::"? pathSegment ( "::" pathSegment )* ;

// ============================================================================
// Inline Assembly (COMPLETE)
// ============================================================================

inlineAsmExpr    → "asm" "!" "(" asmTemplate asmOptions ")" ;

asmTemplate      → STRING ( "," asmOperand )* ","? ;

asmOperand       → asmOperandKind "(" asmConstraint ")" expression ;

asmOperandKind   → "in" | "out" | "inout" | "lateout"
                 | "inlateout" | "const" | "sym" ;

asmConstraint    → STRING ;

asmOptions       → ( "," asmOption )* ;

asmOption        → "options" "(" asmOptionList ")" ;

asmOptionList    → IDENTIFIER ( "," IDENTIFIER )* ;

// ============================================================================
// Lexical Tokens
// ============================================================================

IDENTIFIER       → XID_Start XID_Continue*
                 | "_" XID_Continue+ ;

LIFETIME         → "'" IDENTIFIER ;

INTEGER          → DEC_LITERAL
                 | BIN_LITERAL
                 | OCT_LITERAL
                 | HEX_LITERAL ;

DEC_LITERAL      → [0-9] [0-9_]* ;
BIN_LITERAL      → "0b" [01] [01_]* ;
OCT_LITERAL      → "0o" [0-7] [0-7_]* ;
HEX_LITERAL      → "0x" [0-9a-fA-F] [0-9a-fA-F_]* ;

FLOAT            → DEC_LITERAL "." DEC_LITERAL? FLOAT_EXPONENT?
                 | DEC_LITERAL FLOAT_EXPONENT ;

FLOAT_EXPONENT   → [eE] [+-]? DEC_LITERAL ;

STRING           → "\"" ( STRING_CONTENT | ESCAPE )* "\"" ;
STRING_CONTENT   → ~["\\] ;

RAW_STRING       → "r" RAW_STRING_CONTENT ;
RAW_STRING_CONTENT → "#"* "\"" ~["]* "\"" "#"* ;

BYTE_STRING      → "b\"" ( BYTE_CONTENT | ESCAPE )* "\"" ;
BYTE_CONTENT     → ASCII ~["\\] ;

RAW_BYTE_STRING  → "br" RAW_STRING_CONTENT ;

CHAR             → "'" ( CHAR_CONTENT | ESCAPE ) "'" ;
CHAR_CONTENT     → ~['\\] ;

BYTE             → "b'" ( BYTE_CONTENT | ESCAPE ) "'" ;

ESCAPE           → "\\" ( ["\\nrt0]
                        | "x" [0-9a-fA-F]{2}
                        | "u{" [0-9a-fA-F]{1,6} "}" ) ;

// Comments
LINE_COMMENT     → "//" ~[\n]* ;
BLOCK_COMMENT    → "/*" ( ~[*] | "*" ~[/] )* "*/" ;

// Whitespace
WHITESPACE       → [ \t\n\r]+ ;

// Token (for macros)
TOKEN            → any_token_not_delimiter ;

*/
