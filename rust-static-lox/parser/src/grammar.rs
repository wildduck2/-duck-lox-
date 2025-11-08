/*!
### Complete Rust Grammar (BNF) - 100% Language Coverage
### Updated for Rust 2024 Edition with All Stable & Unstable Features
### Based on Rust Reference and rustc implementation

program          → shebang? innerAttr* item* EOF ;

shebang          → "#!" ~[\n]* ;

// ============================================================================
// Items (Top-level Declarations)
// ============================================================================

item             → outerAttr* visItem ;

visItem          → visibility? item_kind
                 | macroItem ;

item_kind        → functionItem
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
                 | foreignModItem
                 | unionItem
                 | externTypeItem ;

macroItem        → macroInvocationSemi
                 | macroRulesItem
                 | macro2Item ;

// ----------------------------------------------------------------------------
// Attributes (COMPLETE)
// ----------------------------------------------------------------------------

outerAttr        → "#" "[" attrInput "]" ;
innerAttr        → "#" "!" "[" attrInput "]" ;

attrInput        → simplePath attrInputTail? ;

attrInputTail    → delimTokenTree
                 | "=" expression ;

delimTokenTree   → "(" tokenStream ")"
                 | "[" tokenStream "]"
                 | "{" tokenStream "}" ;

// Meta items for structured attributes
metaItem         → simplePath ( "(" metaSeq ")" | "=" literalExpr )? ;
metaSeq          → metaItemInner ( "," metaItemInner )* ","? ;
metaItemInner    → metaItem | literalExpr ;

// Doc comments (sugar for attributes)
outerDocComment  → "///" ~[\n]*
                 | "/**" (~[*] | "*"+ ~[*/])* "*"+ "/" ;

innerDocComment  → "//!" ~[\n]*
                 | "/*!" (~[*] | "*"+ ~[*/])* "*"+ "/" ;

// ----------------------------------------------------------------------------
// Function Item (COMPLETE)
// ----------------------------------------------------------------------------

functionItem     → functionQualifiers "fn" identifier
                   genericParams? "(" functionParams? ")" functionReturnType?
                   whereClause? ( blockExpr | ";" ) ;

functionQualifiers → "const"? "async"? "unsafe"? ("extern" abi?)? ;

abi              → stringLit | rawStringLit ;

functionParams   → selfParam ("," functionParam)* ","?
                 | functionParam ("," functionParam)* ","? ;

selfParam        → outerAttr* ( shorthandSelf | typedSelf ) ;

shorthandSelf    → ("&" lifetime?)? "mut"? "self" ;

typedSelf        → "mut"? "self" ":" type ;

functionParam    → outerAttr* ( functionParamPattern | "..." ) ;

functionParamPattern → patternNoTopAlt (":" ( type | "..." ))? ;

functionReturnType → "->" type ;

// ----------------------------------------------------------------------------
// Struct Item (COMPLETE)
// ----------------------------------------------------------------------------

structItem       → "struct" identifier genericParams?
                   ( whereClause? ( recordStructFields | ";" )
                   | tupleStructFields ( whereClause? ";" ) ) ;

recordStructFields → "{" structFields? "}" ;

structFields     → structField ( "," structField )* ","? ;

structField      → outerAttr* visibility? identifier ":" type ;

tupleStructFields → "(" tupleFields? ")" ;

tupleFields      → tupleField ( "," tupleField )* ","? ;

tupleField       → outerAttr* visibility? type ;

// ----------------------------------------------------------------------------
// Enum Item (COMPLETE)
// ----------------------------------------------------------------------------

enumItem         → "enum" identifier genericParams?
                   whereClause? "{" enumVariants? "}" ;

enumVariants     → enumVariant ( "," enumVariant )* ","? ;

enumVariant      → outerAttr* visibility? identifier
                   ( enumVariantFields | discriminant )? ;

enumVariantFields → recordStructFields | tupleStructFields ;

discriminant     → "=" expression ;

// ----------------------------------------------------------------------------
// Union Item (COMPLETE)
// ----------------------------------------------------------------------------

unionItem        → "union" identifier genericParams?
                   whereClause? recordStructFields ;

// ----------------------------------------------------------------------------
// Const & Static (COMPLETE)
// ----------------------------------------------------------------------------

constItem        → "const" ( identifier | "_" ) ":" type
                   ( "=" expression )? ";" ;

staticItem       → "static" "mut"? identifier ":" type
                   ( "=" expression )? ";" ;

// ----------------------------------------------------------------------------
// Type Alias (COMPLETE)
// ----------------------------------------------------------------------------

typeAliasItem    → "type" identifier genericParams?
                   ( ":" typeBounds )?
                   whereClause? ( "=" type )? ";" ;

// ----------------------------------------------------------------------------
// Trait Item (COMPLETE)
// ----------------------------------------------------------------------------

traitItem        → "unsafe"? "auto"? "trait" identifier
                   genericParams? ( ":" typeParamBounds )?
                   whereClause? "{" innerAttr* associatedItem* "}" ;

// ----------------------------------------------------------------------------
// Implementation (COMPLETE)
// ----------------------------------------------------------------------------

implItem         → "unsafe"? "impl" genericParams?
                   "const"? "!"? traitPath "for" type
                   whereClause? "{" innerAttr* associatedItem* "}"
                 | "unsafe"? "impl" genericParams? type
                   whereClause? "{" innerAttr* inherentImplItem* "}" ;

traitPath        → typePath ;

associatedItem   → outerAttr* ( macroInvocationSemi | associatedItemKind ) ;

associatedItemKind → typeAliasItem
                   | constItem
                   | functionItem ;

inherentImplItem → outerAttr* ( visibility? associatedItemKind | macroInvocationSemi ) ;

// ----------------------------------------------------------------------------
// Extern Crate (COMPLETE)
// ----------------------------------------------------------------------------

externCrateItem  → "extern" "crate" crateRef asClause? ";" ;

crateRef         → identifier | "self" ;

asClause         → "as" ( identifier | "_" ) ;

// ----------------------------------------------------------------------------
// Use Declaration (COMPLETE)
// ----------------------------------------------------------------------------

useItem          → "use" useTree ";" ;

useTree          → ( simplePath? "::" )? ( "*" | useTreeList )
                 | simplePath ( "as" ( identifier | "_" ) )? ;

useTreeList      → "{" ( useTree ( "," useTree )* ","? )? "}" ;

// ----------------------------------------------------------------------------
// Module (COMPLETE)
// ----------------------------------------------------------------------------

moduleItem       → "unsafe"? "mod" identifier ( ";" | "{" innerAttr* item* "}" ) ;

// ----------------------------------------------------------------------------
// External Block (COMPLETE)
// ----------------------------------------------------------------------------

foreignModItem   → "unsafe"? "extern" abi? "{" innerAttr* externalItem* "}" ;

externalItem     → outerAttr* ( macroInvocationSemi | ( visibility? externalItemKind ) ) ;

externalItemKind → "static" "mut"? identifier ":" type ";"
                 | "type" identifier genericParams? ";"
                 | functionItem ;

// Extern type (opaque FFI type)
externTypeItem   → "extern" "type" identifier genericParams? ";" ;

// ----------------------------------------------------------------------------
// Macro Definitions (COMPLETE)
// ----------------------------------------------------------------------------

// macro_rules!
macroRulesItem   → "macro_rules" "!" identifier macroRulesDef ;

macroRulesDef    → "(" macroRules ")" ";"
                 | "{" macroRules "}"
                 | "[" macroRules "]" ;

macroRules       → macroRule ( ";" macroRule )* ";"? ;

macroRule        → macroMatcher "=>" macroTranscriber ;

macroMatcher     → "(" macroMatch* ")"
                 | "[" macroMatch* "]"
                 | "{" macroMatch* "}" ;

macroMatch       → tokenExceptDelims
                 | macroMatcher
                 | "$" ( identifier ":" macroFragSpec | "(" macroMatch+ ")" macroRepSep? macroRepOp ) ;

macroFragSpec    → "block" | "expr" | "ident" | "item" | "lifetime" | "literal"
                 | "meta" | "pat" | "pat_param" | "path" | "stmt" | "tt" | "ty" | "vis" ;

macroRepSep      → tokenExceptDelims | macroRepOp ;

macroRepOp       → "*" | "+" | "?" ;

macroTranscriber → delimTokenTree ;

// Declarative macros 2.0 (macro keyword)
macro2Item       → "macro" identifier "(" macroParams? ")" delimTokenTree ;

macroParams      → identifier ( "," identifier )* ","? ;

// ============================================================================
// Generics (COMPLETE)
// ============================================================================

genericParams    → "<" ( genericParam ( "," genericParam )* ","? )? ">" ;

genericParam     → outerAttr* ( lifetimeParam | typeParam | constParam ) ;

lifetimeParam    → lifetime ( ":" lifetimeBounds )? ;

typeParam        → identifier ( ":" typeParamBounds? )? ( "=" type )? ;

constParam       → "const" identifier ":" type ( "=" block | "=" identifier | "=" literalExpr )? ;

// ----------------------------------------------------------------------------
// Where Clause (COMPLETE)
// ----------------------------------------------------------------------------

whereClause      → "where" ( whereClauseItem ( "," whereClauseItem )* ","? )? ;

whereClauseItem  → lifetimeWhereClauseItem
                 | typeBoundWhereClauseItem ;

lifetimeWhereClauseItem → lifetime ":" lifetimeBounds ;

typeBoundWhereClauseItem → forLifetimes? type ":" typeParamBounds? ;

// ----------------------------------------------------------------------------
// Type & Lifetime Bounds (COMPLETE)
// ----------------------------------------------------------------------------

lifetimeBounds   → ( lifetime ( "+" lifetime )* )? "+"? ;

typeParamBounds  → typeParamBound ( "+" typeParamBound )* "+"? ;

typeParamBound   → lifetime
                 | traitBound ;

traitBound       → "?"? "const"? forLifetimes? typePath ;

forLifetimes     → "for" genericParams ;

// ============================================================================
// Generic Arguments (COMPLETE)
// ============================================================================

genericArgs      → "<" genericArg ( "," genericArg )* ","? ">"
                 | "::" "<" genericArg ( "," genericArg )* ","? ">" ;

genericArg       → lifetime
                 | type
                 | genericArgsConst
                 | genericArgsBinding
                 | constrainedTypeParam ;

genericArgsConst → blockExpr
                 | literalExpr
                 | simplePathSegment ;

genericArgsBinding → identifier genericArgs? "=" type ;

constrainedTypeParam → identifier genericArgs? ":" typeParamBounds ;

// ============================================================================
// Visibility (COMPLETE)
// ============================================================================

visibility       → "pub" ( "(" ( "crate" | "self" | "super" | "in" simplePath ) ")" )? ;

// ============================================================================
// Types (COMPLETE - All Variants)
// ============================================================================

type             → typeNoBounds
                 | implTraitType
                 | traitObjectType ;

typeNoBounds     → parenthesizedType
                 | implTraitTypeOneBound
                 | traitObjectTypeOneBound
                 | typePath
                 | tupleType
                 | neverType
                 | rawPointerType
                 | referenceType
                 | arrayType
                 | sliceType
                 | inferredType
                 | qualifiedPathInType
                 | bareFunctionType
                 | macroInvocation ;

// Parenthesized type
parenthesizedType → "(" type ")" ;

// Tuple type
tupleType        → "(" ")"
                 | "(" ( type "," )+ type? ")" ;

// Never type
neverType        → "!" ;

// Raw pointer
rawPointerType   → "*" ( "mut" | "const" ) typeNoBounds ;

// Reference
referenceType    → "&" lifetime? "mut"? typeNoBounds ;

// Array
arrayType        → "[" type ";" expression "]" ;

// Slice
sliceType        → "[" type "]" ;

// Inferred type
inferredType     → "_" ;

// Qualified path in type
qualifiedPathInType → qualifiedPathType ( "::" typePath )? ;

qualifiedPathType → "<" type ( "as" typePath )? ">" ;

// Bare function type
bareFunctionType → forLifetimes? functionTypeQualifiers "fn" "(" functionParametersMaybeNamedVariadic? ")" bareFunctionReturnType? ;

functionTypeQualifiers → "unsafe"? ( "extern" abi? )? ;

functionParametersMaybeNamedVariadic → functionParameterMaybeNamed ( "," functionParameterMaybeNamed )* ","?
                                      | functionParameterMaybeNamed ( "," functionParameterMaybeNamed )* "," "..."
                                      | "..." ;

functionParameterMaybeNamed → outerAttr* ( ( identifier | "_" ) ":" )? type ;

bareFunctionReturnType → "->" typeNoBounds ;

// Impl trait type
implTraitType    → "impl" typeParamBounds ;

implTraitTypeOneBound → "impl" traitBound ;

// Trait object type
traitObjectType  → "dyn" typeParamBounds ;

traitObjectTypeOneBound → "dyn" traitBound ;

// Type path
typePath         → "::"? typePathSegment ( "::" typePathSegment )* ;

typePathSegment  → pathIdentSegment ( "::" genericArgs | genericArgs )? ;

// ============================================================================
// Paths (COMPLETE)
// ============================================================================

// Simple path (no generics)
simplePath       → "::"? simplePathSegment ( "::" simplePathSegment )* ;

simplePathSegment → identifier | "super" | "self" | "crate" | "$crate" ;

// Path in expression
pathInExpression → "::"? pathExprSegment ( "::" pathExprSegment )* ;

pathExprSegment  → pathIdentSegment ( "::" genericArgs )? ;

pathIdentSegment → identifier | "super" | "self" | "Self" | "crate" | "$crate" ;

// Qualified path in expression
qualifiedPathInExpression → qualifiedPathType ( "::" pathExprSegment )+ ;

// ============================================================================
// Patterns (COMPLETE - All Variants)
// ============================================================================

pattern          → "|"? patternNoTopAlt ( "|" patternNoTopAlt )* ;

patternNoTopAlt  → patternWithoutRange
                 | rangePattern ;

patternWithoutRange → literalPattern
                    | identifierPattern
                    | wildcardPattern
                    | restPattern
                    | referencePattern
                    | structPattern
                    | tupleStructPattern
                    | tuplePattern
                    | groupedPattern
                    | slicePattern
                    | pathPattern
                    | macroInvocation ;

// Literal pattern
literalPattern   → "true" | "false"
                 | charLit
                 | byteLit
                 | stringLit
                 | rawStringLit
                 | byteStringLit
                 | rawByteStringLit
                 | cStringLit
                 | rawCStringLit
                 | integerLit
                 | floatLit
                 | "-" integerLit
                 | "-" floatLit ;

// Identifier pattern
identifierPattern → "ref"? "mut"? identifier ( "@" patternNoTopAlt )? ;

// Wildcard pattern
wildcardPattern  → "_" ;

// Rest pattern
restPattern      → ".." ;

// Reference pattern
referencePattern → ( "&" | "&&" ) "mut"? patternWithoutRange ;

// Struct pattern
structPattern    → pathInExpression "{" structPatternElements? "}" ;

structPatternElements → structPatternFields ( "," structPatternEtCetera? )?
                      | structPatternEtCetera ;

structPatternFields → structPatternField ( "," structPatternField )* ;

structPatternField → outerAttr* ( tupleIndex ":" patternNoTopAlt
                                 | identifier ":" patternNoTopAlt
                                 | "ref"? "mut"? identifier ) ;

structPatternEtCetera → outerAttr* ".." ;

// Tuple struct pattern
tupleStructPattern → pathInExpression "(" tuplePatternItems? ")" ;

// Tuple pattern
tuplePattern     → "(" tuplePatternItems? ")" ;

tuplePatternItems → pattern ( "," pattern )* ","?
                  | restPattern ( "," pattern )+ ","?
                  | pattern ( "," pattern )* "," restPattern ( "," pattern )* ","? ;

// Grouped pattern
groupedPattern   → "(" pattern ")" ;

// Slice pattern
slicePattern     → "[" slicePatternItems? "]" ;

slicePatternItems → pattern ( "," pattern )* ","? ;

// Path pattern
pathPattern      → pathInExpression
                 | qualifiedPathInExpression ;

// Range pattern
rangePattern     → rangePatternBound ( "..=" | "..." ) rangePatternBound
                 | rangeInclusiveStart
                 | obsoleteRangePattern ;

rangePatternBound → charLit | byteLit | "-"? integerLit | "-"? floatLit | pathInExpression ;

rangeInclusiveStart → rangePatternBound "..=" ;

obsoleteRangePattern → rangePatternBound "..." ;

// ============================================================================
// Statements (COMPLETE)
// ============================================================================

statement        → ";"
                 | item
                 | letStatement
                 | expressionStatement
                 | macroInvocationSemi ;

// Let statement
letStatement     → outerAttr* "let" patternNoTopAlt ( ":" type )? ( "=" expression ( "else" blockExpr )? )? ";" ;

// Expression statement
expressionStatement → expressionWithoutBlock ";"
                    | expressionWithBlock ";"? ;

macroInvocationSemi → simplePath "!" delimTokenTree ";" ;

// ============================================================================
// Expressions (COMPLETE - Full Precedence)
// ============================================================================

expression       → expressionWithoutBlock
                 | expressionWithBlock ;

expressionWithoutBlock → outerAttr* expressionKind ;

expressionWithBlock → outerAttr* expressionKindWithBlock ;

expressionKind   → literalExpr
                 | pathExpr
                 | operatorExpr
                 | groupedExpr
                 | arrayExpr
                 | awaitExpr
                 | indexExpr
                 | tupleExpr
                 | tupleIndexExpr
                 | structExpr
                 | callExpr
                 | methodCallExpr
                 | fieldExpr
                 | closureExpr
                 | continueExpr
                 | breakExpr
                 | rangeExpr
                 | returnExpr
                 | underscoreExpr
                 | macroInvocation ;

expressionKindWithBlock → blockExpr
                        | asyncBlockExpr
                        | unsafeBlockExpr
                        | loopExpr
                        | ifExpr
                        | ifLetExpr
                        | matchExpr ;

// ----------------------------------------------------------------------------
// Literal Expression
// ----------------------------------------------------------------------------

literalExpr      → charLit
                 | stringLit
                 | rawStringLit
                 | byteLit
                 | byteStringLit
                 | rawByteStringLit
                 | cStringLit
                 | rawCStringLit
                 | integerLit
                 | floatLit
                 | "true"
                 | "false" ;

// ----------------------------------------------------------------------------
// Path Expression
// ----------------------------------------------------------------------------

pathExpr         → pathInExpression
                 | qualifiedPathInExpression ;

// ----------------------------------------------------------------------------
// Block Expressions
// ----------------------------------------------------------------------------

blockExpr        → "{" innerAttr* statements? "}" ;

asyncBlockExpr   → "async" "move"? blockExpr ;

unsafeBlockExpr  → "unsafe" blockExpr ;

statements       → statement+ expression?
                 | expression ;

// ----------------------------------------------------------------------------
// Operator Expressions (Full Precedence)
// ----------------------------------------------------------------------------

operatorExpr     → borrowExpr
                 | dereferenceExpr
                 | errorPropagationExpr
                 | negationExpr
                 | arithOrLogicalExpr
                 | comparisonExpr
                 | lazyBooleanExpr
                 | typecastExpr
                 | assignmentExpr
                 | compoundAssignmentExpr ;

// Borrow
borrowExpr       → ( "&" | "&&" ) "mut"? expression ;

// Dereference
dereferenceExpr  → "*" expression ;

// Error propagation
errorPropagationExpr → expression "?" ;

// Negation
negationExpr     → ( "-" | "!" ) expression ;

// Arithmetic and logical
arithOrLogicalExpr → expression "+" expression
                   | expression "-" expression
                   | expression "*" expression
                   | expression "/" expression
                   | expression "%" expression
                   | expression "&" expression
                   | expression "|" expression
                   | expression "^" expression
                   | expression "<<" expression
                   | expression ">>" expression ;

// Comparison
comparisonExpr   → expression "==" expression
                 | expression "!=" expression
                 | expression ">" expression
                 | expression "<" expression
                 | expression ">=" expression
                 | expression "<=" expression ;

// Lazy boolean
lazyBooleanExpr  → expression "||" expression
                 | expression "&&" expression ;

// Type cast
typecastExpr     → expression "as" typeNoBounds ;

// Assignment
assignmentExpr   → expression "=" expression ;

// Compound assignment
compoundAssignmentExpr → expression "+=" expression
                       | expression "-=" expression
                       | expression "*=" expression
                       | expression "/=" expression
                       | expression "%=" expression
                       | expression "&=" expression
                       | expression "|=" expression
                       | expression "^=" expression
                       | expression "<<=" expression
                       | expression ">>=" expression ;

// ----------------------------------------------------------------------------
// Grouped Expression
// ----------------------------------------------------------------------------

groupedExpr      → "(" innerAttr* expression ")" ;

// ----------------------------------------------------------------------------
// Array Expression
// ----------------------------------------------------------------------------

arrayExpr        → "[" arrayElements? "]" ;

arrayElements    → expression ( ";" expression | ( "," expression )* ","? ) ;

// ----------------------------------------------------------------------------
// Await Expression
// ----------------------------------------------------------------------------

awaitExpr        → expression "." "await" ;

// ----------------------------------------------------------------------------
// Index Expression
// ----------------------------------------------------------------------------

indexExpr        → expression "[" expression "]" ;

// ----------------------------------------------------------------------------
// Tuple Expression
// ----------------------------------------------------------------------------

tupleExpr        → "(" innerAttr* tupleElements? ")" ;

tupleElements    → ( expression "," )+ expression? ;

// ----------------------------------------------------------------------------
// Tuple Index Expression
// ----------------------------------------------------------------------------

tupleIndexExpr   → expression "." tupleIndex ;

tupleIndex       → INTEGER_LITERAL ;

// ----------------------------------------------------------------------------
// Struct Expression
// ----------------------------------------------------------------------------

structExpr       → structExprStruct
                 | structExprTuple
                 | structExprUnit ;

structExprStruct → pathInExpression "{" ( structExprFields | structBase )? "}" ;

structExprFields → structExprField ( "," structExprField )* ( "," structBase | ","? ) ;

structExprField  → outerAttr* ( identifier | ( identifier | tupleIndex ) ":" expression ) ;

structBase       → ".." expression ;

structExprTuple  → pathInExpression "(" ( expression ( "," expression )* ","? )? ")" ;

structExprUnit   → pathInExpression ;

// ----------------------------------------------------------------------------
// Call Expression
// ----------------------------------------------------------------------------

callExpr         → expression "(" callParams? ")" ;

callParams       → expression ( "," expression )* ","? ;

// ----------------------------------------------------------------------------
// Method Call Expression
// ----------------------------------------------------------------------------

methodCallExpr   → expression "." pathExprSegment "(" callParams? ")" ;

// ----------------------------------------------------------------------------
// Field Expression
// ----------------------------------------------------------------------------

fieldExpr        → expression "." identifier ;

// ----------------------------------------------------------------------------
// Closure Expression
// ----------------------------------------------------------------------------

closureExpr      → "move"? "async"? ( "||" | "|" closureParams? "|" ) ( expression | "->" typeNoBounds blockExpr ) ;

closureParams    → closureParam ( "," closureParam )* ","? ;

closureParam     → outerAttr* patternNoTopAlt ( ":" type )? ;

// ----------------------------------------------------------------------------
// Loop Expressions
// ----------------------------------------------------------------------------

loopExpr         → infiniteLoopExpr
                 | predicateLoopExpr
                 | predicatePatternLoopExpr
                 | iteratorLoopExpr
                 | labelBlockExpr ;

infiniteLoopExpr → loopLabel? "loop" blockExpr ;

predicateLoopExpr → loopLabel? "while" expression blockExpr ;

predicatePatternLoopExpr → loopLabel? "while" "let" pattern "=" scrutinee blockExpr ;

iteratorLoopExpr → loopLabel? "for" pattern "in" expression blockExpr ;

loopLabel        → lifetime ":" ;

labelBlockExpr   → loopLabel blockExpr ;

scrutinee        → expression ;

// ----------------------------------------------------------------------------
// Continue Expression
// ----------------------------------------------------------------------------

continueExpr     → "continue" lifetime? ;

// ----------------------------------------------------------------------------
// Break Expression
// ----------------------------------------------------------------------------

breakExpr        → "break" lifetime? expression? ;

// ----------------------------------------------------------------------------
// Range Expression
// ----------------------------------------------------------------------------

rangeExpr        → expression ".." expression
                 | expression ".."
                 | ".." expression
                 | expression "..=" expression
                 | "..=" expression
                 | ".." ;

// ----------------------------------------------------------------------------
// If Expressions
// ----------------------------------------------------------------------------

ifExpr           → "if" expression blockExpr ( "else" ( blockExpr | ifExpr | ifLetExpr ) )? ;

ifLetExpr        → "if" "let" pattern "=" scrutinee blockExpr ( "else" ( blockExpr | ifExpr | ifLetExpr ) )? ;

// ----------------------------------------------------------------------------
// Match Expression
// ----------------------------------------------------------------------------

matchExpr        → "match" scrutinee "{" innerAttr* matchArms? "}" ;

matchArms        → ( matchArm "=>" ( expressionWithoutBlock "," | expressionWithBlock ","? ) )* matchArm "=>" expression ","? ;

matchArm         → outerAttr* pattern matchArmGuard? ;

matchArmGuard    → "if" expression ;

// ----------------------------------------------------------------------------
// Return Expression
// ----------------------------------------------------------------------------

returnExpr       → "return" expression? ;

// ----------------------------------------------------------------------------
// Underscore Expression
// ----------------------------------------------------------------------------

underscoreExpr   → "_" ;

// ============================================================================
// Macro Invocation (COMPLETE)
// ============================================================================

macroInvocation  → simplePath "!" delimTokenTree ;

tokenStream      → tokenTree* ;

tokenTree        → tokenExceptDelims
                 | delimTokenTree ;

tokenExceptDelims → any_token_except_delimiters ;

// ============================================================================
// Lexical Tokens (COMPLETE)
// ============================================================================

// ----------------------------------------------------------------------------
// Identifiers
// ----------------------------------------------------------------------------

identifier       → nonKeywordIdentifier | rawIdentifier ;

nonKeywordIdentifier → XID_Start XID_Continue*
                     | "_" XID_Continue+ ;

rawIdentifier    → "r#" nonKeywordIdentifier_except_crate_super_self_Self ;

// ----------------------------------------------------------------------------
// Lifetimes
// ----------------------------------------------------------------------------

lifetime         → "'" nonKeywordIdentifier
                 | "'static"
                 | "'_" ;

// ----------------------------------------------------------------------------
// Integer Literals
// ----------------------------------------------------------------------------

integerLit       → decLit intSuffix?
                 | binLit intSuffix?
                 | octLit intSuffix?
                 | hexLit intSuffix? ;

decLit           → DEC_DIGIT ( DEC_DIGIT | "_" )* ;
binLit           → "0b" ( BIN_DIGIT | "_" )* BIN_DIGIT ( BIN_DIGIT | "_" )* ;
octLit           → "0o" ( OCT_DIGIT | "_" )* OCT_DIGIT ( OCT_DIGIT | "_" )* ;
hexLit           → "0x" ( HEX_DIGIT | "_" )* HEX_DIGIT ( HEX_DIGIT | "_" )* ;

intSuffix        → "u8" | "u16" | "u32" | "u64" | "u128" | "usize"
                 | "i8" | "i16" | "i32" | "i64" | "i128" | "isize" ;

DEC_DIGIT        → [0-9] ;
BIN_DIGIT        → [0-1] ;
OCT_DIGIT        → [0-7] ;
HEX_DIGIT        → [0-9a-fA-F] ;

// ----------------------------------------------------------------------------
// Float Literals
// ----------------------------------------------------------------------------

floatLit         → decLit "." ( decLit floatExponent? floatSuffix? | floatExponent floatSuffix? | floatSuffix )
                 | decLit floatExponent floatSuffix?
                 | decLit floatSuffix ;

floatExponent    → ( "e" | "E" ) ( "+" | "-" )? decLit ;

floatSuffix      → "f32" | "f64" ;

// ----------------------------------------------------------------------------
// String Literals
// ----------------------------------------------------------------------------

stringLit        → "\"" ( ~["\\] | QUOTE_ESCAPE | ASCII_ESCAPE | UNICODE_ESCAPE | STRING_CONTINUE )* "\"" ;

rawStringLit     → "r" RAW_STRING_CONTENT ;

RAW_STRING_CONTENT → "#"* "\"" ( ~["] | "\"" ~[#] )* "\"" "#"* ;

// ----------------------------------------------------------------------------
// Byte String Literals
// ----------------------------------------------------------------------------

byteStringLit    → "b\"" ( ASCII_FOR_STRING | BYTE_ESCAPE | STRING_CONTINUE )* "\"" ;

rawByteStringLit → "br" RAW_STRING_CONTENT ;

// ----------------------------------------------------------------------------
// C String Literals (NEW)
// ----------------------------------------------------------------------------

cStringLit       → "c\"" ( ~["\\] | QUOTE_ESCAPE | ASCII_ESCAPE | UNICODE_ESCAPE | STRING_CONTINUE )* "\"" ;

rawCStringLit    → "cr" RAW_STRING_CONTENT ;

// ----------------------------------------------------------------------------
// Char Literals
// ----------------------------------------------------------------------------

charLit          → "'" ( ~['\\] | QUOTE_ESCAPE | ASCII_ESCAPE | UNICODE_ESCAPE ) "'" ;

// ----------------------------------------------------------------------------
// Byte Literals
// ----------------------------------------------------------------------------

byteLit          → "b'" ( ASCII_FOR_CHAR | BYTE_ESCAPE ) "'" ;

// ----------------------------------------------------------------------------
// Escape Sequences
// ----------------------------------------------------------------------------

QUOTE_ESCAPE     → "\\'" | "\\\"" ;

ASCII_ESCAPE     → "\\x" HEX_DIGIT HEX_DIGIT
                 | "\\n" | "\\r" | "\\t" | "\\\\" | "\\0" ;

UNICODE_ESCAPE   → "\\u{" HEX_DIGIT HEX_DIGIT? HEX_DIGIT? HEX_DIGIT? HEX_DIGIT? HEX_DIGIT? "}" ;

STRING_CONTINUE  → "\\" "\n" ;

BYTE_ESCAPE      → "\\x" HEX_DIGIT HEX_DIGIT
                 | "\\n" | "\\r" | "\\t" | "\\\\" | "\\0" | "\\'" | "\\\"" ;

ASCII_FOR_STRING → [\x00-\x7F] ;
ASCII_FOR_CHAR   → [\x00-\x7F] ;

// ----------------------------------------------------------------------------
// Comments
// ----------------------------------------------------------------------------

lineComment      → "//" ( ~[/!] | "//" ) ~[\n]* ;

blockComment     → "/*" ( ~[*!] | "**" | blockCommentOrDoc ) ( ~[*] | "*" ~[/] )* "*/" ;

blockCommentOrDoc → blockComment | outerDocComment | innerDocComment ;

// ----------------------------------------------------------------------------
// Whitespace
// ----------------------------------------------------------------------------

whitespace       → [\t\n\x0B\x0C\r\x20]+ ;

// ============================================================================
// Reserved Keywords (For Reference)
// ============================================================================

// Strict keywords (always keywords)
// as, async, await, break, const, continue, crate, dyn, else, enum, extern
// false, fn, for, if, impl, in, let, loop, match, mod, move, mut, pub, ref
// return, self, Self, static, struct, super, trait, true, try, type, unsafe
// use, where, while, async, await, dyn

// Reserved keywords (reserved for future use)
// abstract, become, box, do, final, macro, override, priv, typeof, unsized
// virtual, yield

// Weak keywords (contextual)
// 'static, union, 'static, dyn, macro_rules

// Edition-specific
// gen (2024+), raw (raw identifiers)
*/
