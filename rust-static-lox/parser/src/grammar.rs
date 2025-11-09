/*
 *
 *   Complete Rust Grammar - Recursive Descent Parsing Style
 *
 *   program          → shebang? innerAttr* item* EOF ;
 *
 *   shebang          → "#!" ~[\n]* ;
 *
 *   item             → outerAttr* visItem ;
 *
 *   visItem          → visibility? itemKind
 *                    | macroItem ;
 *
 *   itemKind         → functionItem
 *                    | structItem
 *                    | enumItem
 *                    | traitItem
 *                    | implItem
 *                    | constItem
 *                    | staticItem
 *                    | typeAliasItem
 *                    | moduleItem
 *                    | useItem
 *                    | externCrateItem
 *                    | foreignModItem
 *                    | unionItem
 *                    | externTypeItem ;
 *
 *   macroItem        → macroInvocationSemi
 *                    | macroRulesItem
 *                    | macro2Item ;
 *
 *   outerAttr        → "#" "[" attrInput "]" ;
 *
 *   innerAttr        → "#" "!" "[" attrInput "]" ;
 *
 *   attrInput        → simplePath attrInputTail? ;
 *
 *   attrInputTail    → delimTokenTree
 *                    | "=" expression ;
 *
 *   delimTokenTree   → "(" tokenStream ")"
 *                    | "[" tokenStream "]"
 *                    | "{" tokenStream "}" ;
 *
 *   metaItem         → simplePath ("(" metaSeq ")" | "=" literalExpr)? ;
 *
 *   metaSeq          → metaItemInner ("," metaItemInner)* ","? ;
 *
 *   metaItemInner    → metaItem | literalExpr ;
 *
 *   functionItem     → functionQualifiers "fn" IDENTIFIER
 *                      genericParams? "(" functionParams? ")" functionReturnType?
 *                      whereClause? (blockExpr | ";") ;
 *
 *   functionQualifiers → "const"? "async"? "unsafe"? ("extern" abi?)? ;
 *
 *   abi              → STRING | RAW_STRING ;
 *
 *   functionParams   → selfParam ("," functionParam)* ","?
 *                    | functionParam ("," functionParam)* ","? ;
 *
 *   selfParam        → outerAttr* (shorthandSelf | typedSelf) ;
 *
 *   shorthandSelf    → ("&" LIFETIME?)? "mut"? "self" ;
 *
 *   typedSelf        → "mut"? "self" ":" type ;
 *
 *   functionParam    → outerAttr* (functionParamPattern | "...") ;
 *
 *   functionParamPattern → patternNoTopAlt (":" (type | "..."))? ;
 *
 *   functionReturnType → "->" type ;
 *
 *   structItem       → "struct" IDENTIFIER genericParams?
 *                      (whereClause? (recordStructFields | ";")
 *                      | tupleStructFields (whereClause? ";")) ;
 *
 *   recordStructFields → "{" structFields? "}" ;
 *
 *   structFields     → structField ("," structField)* ","? ;
 *
 *   structField      → outerAttr* visibility? IDENTIFIER ":" type ;
 *
 *   tupleStructFields → "(" tupleFields? ")" ;
 *
 *   tupleFields      → tupleField ("," tupleField)* ","? ;
 *
 *   tupleField       → outerAttr* visibility? type ;
 *
 *   enumItem         → "enum" IDENTIFIER genericParams?
 *                      whereClause? "{" enumVariants? "}" ;
 *
 *   enumVariants     → enumVariant ("," enumVariant)* ","? ;
 *
 *   enumVariant      → outerAttr* visibility? IDENTIFIER
 *                      (enumVariantFields | discriminant)? ;
 *
 *   enumVariantFields → recordStructFields | tupleStructFields ;
 *
 *   discriminant     → "=" expression ;
 *
 *   unionItem        → "union" IDENTIFIER genericParams?
 *                      whereClause? recordStructFields ;
 *
 *   constItem        → "const" (IDENTIFIER | "_") ":" type
 *                      ("=" expression)? ";" ;
 *
 *   staticItem       → "static" "mut"? IDENTIFIER ":" type
 *                      ("=" expression)? ";" ;
 *
 *   typeAliasItem    → "type" IDENTIFIER genericParams?
 *                      (":" typeBounds)?
 *                      whereClause? ("=" type)? ";" ;
 *
 *   traitItem        → "unsafe"? "auto"? "trait" IDENTIFIER
 *                      genericParams? (":" typeParamBounds)?
 *                      whereClause? "{" innerAttr* associatedItem* "}" ;
 *
 *   implItem         → "unsafe"? "impl" genericParams?
 *                      "const"? "!"? traitPath "for" type
 *                      whereClause? "{" innerAttr* associatedItem* "}"
 *                    | "unsafe"? "impl" genericParams? type
 *                      whereClause? "{" innerAttr* inherentImplItem* "}" ;
 *
 *   traitPath        → typePath ;
 *
 *   associatedItem   → outerAttr* (macroInvocationSemi | associatedItemKind) ;
 *
 *   associatedItemKind → typeAliasItem
 *                      | constItem
 *                      | functionItem ;
 *
 *   inherentImplItem → outerAttr* (visibility? associatedItemKind | macroInvocationSemi) ;
 *
 *   externCrateItem  → "extern" "crate" crateRef asClause? ";" ;
 *
 *   crateRef         → IDENTIFIER | "self" ;
 *
 *   asClause         → "as" (IDENTIFIER | "_") ;
 *
 *   useItem          → "use" useTree ";" ;
 *
 *   useTree          → (simplePath? "::")? ("*" | useTreeList)
 *                    | simplePath ("as" (IDENTIFIER | "_"))? ;
 *
 *   useTreeList      → "{" (useTree ("," useTree)* ","?)? "}" ;
 *
 *   moduleItem       → "unsafe"? "mod" IDENTIFIER (";" | "{" innerAttr* item* "}") ;
 *
 *   foreignModItem   → "unsafe"? "extern" abi? "{" innerAttr* externalItem* "}" ;
 *
 *   externalItem     → outerAttr* (macroInvocationSemi | (visibility? externalItemKind)) ;
 *
 *   externalItemKind → "static" "mut"? IDENTIFIER ":" type ";"
 *                    | "type" IDENTIFIER genericParams? ";"
 *                    | functionItem ;
 *
 *   externTypeItem   → "extern" "type" IDENTIFIER genericParams? ";" ;
 *
 *   macroRulesItem   → "macro_rules" "!" IDENTIFIER macroRulesDef ;
 *
 *   macroRulesDef    → "(" macroRules ")" ";"
 *                    | "{" macroRules "}"
 *                    | "[" macroRules "]" ;
 *
 *   macroRules       → macroRule (";" macroRule)* ";"? ;
 *
 *   macroRule        → macroMatcher "=>" macroTranscriber ;
 *
 *   macroMatcher     → "(" macroMatch* ")"
 *                    | "[" macroMatch* "]"
 *                    | "{" macroMatch* "}" ;
 *
 *   macroMatch       → tokenExceptDelims
 *                    | macroMatcher
 *                    | "$" (IDENTIFIER ":" macroFragSpec | "(" macroMatch+ ")" macroRepSep? macroRepOp) ;
 *
 *   macroFragSpec    → "block" | "expr" | "ident" | "item" | "lifetime" | "literal"
 *                    | "meta" | "pat" | "pat_param" | "path" | "stmt" | "tt" | "ty" | "vis" ;
 *
 *   macroRepSep      → tokenExceptDelims | macroRepOp ;
 *
 *   macroRepOp       → "*" | "+" | "?" ;
 *
 *   macroTranscriber → delimTokenTree ;
 *
 *   macro2Item       → "macro" IDENTIFIER "(" macroParams? ")" delimTokenTree ;
 *
 *   macroParams      → IDENTIFIER ("," IDENTIFIER)* ","? ;
 *
 *   genericParams    → "<" (genericParam ("," genericParam)* ","?)? ">" ;
 *
 *   genericParam     → outerAttr* (lifetimeParam | typeParam | constParam) ;
 *
 *   lifetimeParam    → LIFETIME (":" lifetimeBounds)? ;
 *
 *   typeParam        → IDENTIFIER (":" typeParamBounds?)? ("=" type)? ;
 *
 *   constParam       → "const" IDENTIFIER ":" type ("=" block | "=" IDENTIFIER | "=" literalExpr)? ;
 *
 *   whereClause      → "where" (whereClauseItem ("," whereClauseItem)* ","?)? ;
 *
 *   whereClauseItem  → lifetimeWhereClauseItem
 *                    | typeBoundWhereClauseItem ;
 *
 *   lifetimeWhereClauseItem → LIFETIME ":" lifetimeBounds ;
 *
 *   typeBoundWhereClauseItem → forLifetimes? type ":" typeParamBounds? ;
 *
 *   lifetimeBounds   → (LIFETIME ("+" LIFETIME)*)? "+"? ;
 *
 *   typeParamBounds  → typeParamBound ("+" typeParamBound)* "+"? ;
 *
 *   typeParamBound   → LIFETIME
 *                    | traitBound ;
 *
 *   traitBound       → "?"? "const"? forLifetimes? typePath ;
 *
 *   forLifetimes     → "for" genericParams ;
 *
 *   genericArgs      → "<" genericArg ("," genericArg)* ","? ">"
 *                    | "::" "<" genericArg ("," genericArg)* ","? ">" ;
 *
 *   genericArg       → LIFETIME
 *                    | type
 *                    | genericArgsConst
 *                    | genericArgsBinding
 *                    | constrainedTypeParam ;
 *
 *   genericArgsConst → blockExpr
 *                    | literalExpr
 *                    | simplePathSegment ;
 *
 *   genericArgsBinding → IDENTIFIER genericArgs? "=" type ;
 *
 *   constrainedTypeParam → IDENTIFIER genericArgs? ":" typeParamBounds ;
 *
 *   visibility       → "pub" ("(" ("crate" | "self" | "super" | "in" simplePath) ")")? ;
 *
 *   type             → typeNoBounds
 *                    | implTraitType
 *                    | traitObjectType ;
 *
 *   typeNoBounds     → parenthesizedType
 *                    | implTraitTypeOneBound
 *                    | traitObjectTypeOneBound
 *                    | typePath
 *                    | tupleType
 *                    | neverType
 *                    | rawPointerType
 *                    | referenceType
 *                    | arrayType
 *                    | sliceType
 *                    | inferredType
 *                    | qualifiedPathInType
 *                    | bareFunctionType
 *                    | macroInvocation ;
 *
 *   parenthesizedType → "(" type ")" ;
 *
 *   tupleType        → "(" ")"
 *                    | "(" (type ",")+ type? ")" ;
 *
 *   neverType        → "!" ;
 *
 *   rawPointerType   → "*" ("mut" | "const") typeNoBounds ;
 *
 *   referenceType    → "&" LIFETIME? "mut"? typeNoBounds ;
 *
 *   arrayType        → "[" type ";" expression "]" ;
 *
 *   sliceType        → "[" type "]" ;
 *
 *   inferredType     → "_" ;
 *
 *   qualifiedPathInType → qualifiedPathType ("::" typePath)? ;
 *
 *   qualifiedPathType → "<" type ("as" typePath)? ">" ;
 *
 *   bareFunctionType → forLifetimes? functionTypeQualifiers "fn" "(" functionParametersMaybeNamedVariadic? ")" bareFunctionReturnType? ;
 *
 *   functionTypeQualifiers → "unsafe"? ("extern" abi?)? ;
 *
 *   functionParametersMaybeNamedVariadic → functionParameterMaybeNamed ("," functionParameterMaybeNamed)* ","?
 *                                         | functionParameterMaybeNamed ("," functionParameterMaybeNamed)* "," "..."
 *                                         | "..." ;
 *
 *   functionParameterMaybeNamed → outerAttr* ((IDENTIFIER | "_") ":")? type ;
 *
 *   bareFunctionReturnType → "->" typeNoBounds ;
 *
 *   implTraitType    → "impl" typeParamBounds ;
 *
 *   implTraitTypeOneBound → "impl" traitBound ;
 *
 *   traitObjectType  → "dyn" typeParamBounds ;
 *
 *   traitObjectTypeOneBound → "dyn" traitBound ;
 *
 *   typePath         → "::"? typePathSegment ("::" typePathSegment)* ;
 *
 *   typePathSegment  → pathIdentSegment ("::" genericArgs | genericArgs)? ;
 *
 *   simplePath       → "::"? simplePathSegment ("::" simplePathSegment)* ;
 *
 *   simplePathSegment → IDENTIFIER | "super" | "self" | "crate" | "$crate" ;
 *
 *   pathInExpression → "::"? pathExprSegment ("::" pathExprSegment)* ;
 *
 *   pathExprSegment  → pathIdentSegment ("::" genericArgs)? ;
 *
 *   pathIdentSegment → IDENTIFIER | "super" | "self" | "Self" | "crate" | "$crate" ;
 *
 *   qualifiedPathInExpression → qualifiedPathType ("::" pathExprSegment)+ ;
 *
 *   pattern          → "|"? patternNoTopAlt ("|" patternNoTopAlt)* ;
 *
 *   patternNoTopAlt  → patternWithoutRange
 *                    | rangePattern ;
 *
 *   patternWithoutRange → literalPattern
 *                       | identifierPattern
 *                       | wildcardPattern
 *                       | restPattern
 *                       | referencePattern
 *                       | structPattern
 *                       | tupleStructPattern
 *                       | tuplePattern
 *                       | groupedPattern
 *                       | slicePattern
 *                       | pathPattern
 *                       | macroInvocation ;
 *
 *   literalPattern   → "true" | "false"
 *                    | CHAR
 *                    | BYTE
 *                    | STRING
 *                    | RAW_STRING
 *                    | BYTE_STRING
 *                    | RAW_BYTE_STRING
 *                    | C_STRING
 *                    | RAW_C_STRING
 *                    | INTEGER
 *                    | FLOAT
 *                    | "-" INTEGER
 *                    | "-" FLOAT ;
 *
 *   identifierPattern → "ref"? "mut"? IDENTIFIER ("@" patternNoTopAlt)? ;
 *
 *   wildcardPattern  → "_" ;
 *
 *   restPattern      → ".." ;
 *
 *   referencePattern → ("&" | "&&") "mut"? patternWithoutRange ;
 *
 *   structPattern    → pathInExpression "{" structPatternElements? "}" ;
 *
 *   structPatternElements → structPatternFields ("," structPatternEtCetera?)?
 *                         | structPatternEtCetera ;
 *
 *   structPatternFields → structPatternField ("," structPatternField)* ;
 *
 *   structPatternField → outerAttr* (tupleIndex ":" patternNoTopAlt
 *                                    | IDENTIFIER ":" patternNoTopAlt
 *                                    | "ref"? "mut"? IDENTIFIER) ;
 *
 *   structPatternEtCetera → outerAttr* ".." ;
 *
 *   tupleStructPattern → pathInExpression "(" tuplePatternItems? ")" ;
 *
 *   tuplePattern     → "(" tuplePatternItems? ")" ;
 *
 *   tuplePatternItems → pattern ("," pattern)* ","?
 *                     | restPattern ("," pattern)+ ","?
 *                     | pattern ("," pattern)* "," restPattern ("," pattern)* ","? ;
 *
 *   groupedPattern   → "(" pattern ")" ;
 *
 *   slicePattern     → "[" slicePatternItems? "]" ;
 *
 *   slicePatternItems → pattern ("," pattern)* ","? ;
 *
 *   pathPattern      → pathInExpression
 *                    | qualifiedPathInExpression ;
 *
 *   rangePattern     → rangePatternBound ("..=" | "...") rangePatternBound
 *                    | rangeInclusiveStart
 *                    | obsoleteRangePattern ;
 *
 *   rangePatternBound → CHAR | BYTE | "-"? INTEGER | "-"? FLOAT | pathInExpression ;
 *
 *   rangeInclusiveStart → rangePatternBound "..=" ;
 *
 *   obsoleteRangePattern → rangePatternBound "..." ;
 *
 *   statement        → ";"
 *                    | item
 *                    | letStatement
 *                    | expressionStatement
 *                    | macroInvocationSemi ;
 *
 *   letStatement     → outerAttr* "let" patternNoTopAlt (":" type)? ("=" expression ("else" blockExpr)?)? ";" ;
 *
 *   expressionStatement → expressionWithoutBlock ";"
 *                       | expressionWithBlock ";"? ;
 *
 *   macroInvocationSemi → simplePath "!" delimTokenTree ";" ;
 *
 *   expression       → assignment ;
 *
 *   assignment       → (rangeExpr assignOp)* rangeExpr ;
 *
 *   assignOp         → "=" | "+=" | "-=" | "*=" | "/=" | "%=" | "&=" | "|=" | "^=" | "<<=" | ">>=" ;
 *
 *   rangeExpr        → logicalOr (rangeOp logicalOr?)? ;
 *
 *   rangeOp          → ".." | "..=" ;
 *
 *   logicalOr        → logicalAnd ("||" logicalAnd)* ;
 *
 *   logicalAnd       → comparison ("&&" comparison)* ;
 *
 *   comparison       → bitwiseOr (comparisonOp bitwiseOr)* ;
 *
 *   comparisonOp     → "==" | "!=" | "<" | "<=" | ">" | ">=" ;
 *
 *   bitwiseOr        → bitwiseXor ("|" bitwiseXor)* ;
 *
 *   bitwiseXor       → bitwiseAnd ("^" bitwiseAnd)* ;
 *
 *   bitwiseAnd       → shift ("&" shift)* ;
 *
 *   shift            → term (shiftOp term)* ;
 *
 *   shiftOp          → "<<" | ">>" ;
 *
 *   term             → factor (termOp factor)* ;
 *
 *   termOp           → "+" | "-" ;
 *
 *   factor           → cast (factorOp cast)* ;
 *
 *   factorOp         → "*" | "/" | "%" ;
 *
 *   cast             → unary ("as" typeNoBounds)* ;
 *
 *   unary            → unaryOp* postfix ;
 *
 *   unaryOp          → "-" | "!" | "*" | "&" "mut"? | "&&" "mut"? ;
 *
 *   postfix          → primary postfixOp* ;
 *
 *   postfixOp        → callOp
 *                    | methodCallOp
 *                    | fieldAccessOp
 *                    | tupleIndexOp
 *                    | indexOp
 *                    | awaitOp
 *                    | tryOp ;
 *
 *   callOp           → "(" callParams? ")" ;
 *
 *   methodCallOp     → "." pathExprSegment "(" callParams? ")" ;
 *
 *   fieldAccessOp    → "." IDENTIFIER ;
 *
 *   tupleIndexOp     → "." INTEGER ;
 *
 *   indexOp          → "[" expression "]" ;
 *
 *   awaitOp          → "." "await" ;
 *
 *   tryOp            → "?" ;
 *
 *   callParams       → expression ("," expression)* ","? ;
 *
 *   primary          → literalExpr
 *                    | pathExpr
 *                    | groupedExpr
 *                    | arrayExpr
 *                    | tupleExpr
 *                    | structExpr
 *                    | closureExpr
 *                    | blockExpr
 *                    | asyncBlockExpr
 *                    | unsafeBlockExpr
 *                    | loopExpr
 *                    | ifExpr
 *                    | ifLetExpr
 *                    | matchExpr
 *                    | continueExpr
 *                    | breakExpr
 *                    | returnExpr
 *                    | underscoreExpr
 *                    | macroInvocation ;
 *
 *   literalExpr      → CHAR
 *                    | STRING
 *                    | RAW_STRING
 *                    | BYTE
 *                    | BYTE_STRING
 *                    | RAW_BYTE_STRING
 *                    | C_STRING
 *                    | RAW_C_STRING
 *                    | INTEGER
 *                    | FLOAT
 *                    | "true"
 *                    | "false" ;
 *
 *   pathExpr         → pathInExpression
 *                    | qualifiedPathInExpression ;
 *
 *   groupedExpr      → "(" innerAttr* expression ")" ;
 *
 *   arrayExpr        → "[" arrayElements? "]" ;
 *
 *   arrayElements    → expression (";" expression | ("," expression)* ","?) ;
 *
 *   tupleExpr        → "(" innerAttr* tupleElements? ")" ;
 *
 *   tupleElements    → (expression ",")+ expression? ;
 *
 *   structExpr       → structExprStruct
 *                    | structExprTuple
 *                    | structExprUnit ;
 *
 *   structExprStruct → pathInExpression "{" (structExprFields | structBase)? "}" ;
 *
 *   structExprFields → structExprField ("," structExprField)* ("," structBase | ","?) ;
 *
 *   structExprField  → outerAttr* (IDENTIFIER | (IDENTIFIER | tupleIndex) ":" expression) ;
 *
 *   structBase       → ".." expression ;
 *
 *   structExprTuple  → pathInExpression "(" (expression ("," expression)* ","?)? ")" ;
 *
 *   structExprUnit   → pathInExpression ;
 *
 *   closureExpr      → "move"? "async"? ("||" | "|" closureParams? "|") (expression | "->" typeNoBounds blockExpr) ;
 *
 *   closureParams    → closureParam ("," closureParam)* ","? ;
 *
 *   closureParam     → outerAttr* patternNoTopAlt (":" type)? ;
 *
 *   blockExpr        → "{" innerAttr* statements? "}" ;
 *
 *   asyncBlockExpr   → "async" "move"? blockExpr ;
 *
 *   unsafeBlockExpr  → "unsafe" blockExpr ;
 *
 *   statements       → statement+ expression?
 *                    | expression ;
 *
 *   loopExpr         → infiniteLoopExpr
 *                    | predicateLoopExpr
 *                    | predicatePatternLoopExpr
 *                    | iteratorLoopExpr
 *                    | labelBlockExpr ;
 *
 *   infiniteLoopExpr → loopLabel? "loop" blockExpr ;
 *
 *   predicateLoopExpr → loopLabel? "while" expression blockExpr ;
 *
 *   predicatePatternLoopExpr → loopLabel? "while" "let" pattern "=" scrutinee blockExpr ;
 *
 *   iteratorLoopExpr → loopLabel? "for" pattern "in" expression blockExpr ;
 *
 *   loopLabel        → LIFETIME ":" ;
 *
 *   labelBlockExpr   → loopLabel blockExpr ;
 *
 *   scrutinee        → expression ;
 *
 *   ifExpr           → "if" expression blockExpr ("else" (blockExpr | ifExpr | ifLetExpr))? ;
 *
 *   ifLetExpr        → "if" "let" pattern "=" scrutinee blockExpr ("else" (blockExpr | ifExpr | ifLetExpr))? ;
 *
 *   matchExpr        → "match" scrutinee "{" innerAttr* matchArms? "}" ;
 *
 *   matchArms        → (matchArm "=>" (expressionWithoutBlock "," | expressionWithBlock ","?))* matchArm "=>" expression ","? ;
 *
 *   matchArm         → outerAttr* pattern matchArmGuard? ;
 *
 *   matchArmGuard    → "if" expression ;
 *
 *   continueExpr     → "continue" LIFETIME? ;
 *
 *   breakExpr        → "break" LIFETIME? expression? ;
 *
 *   returnExpr       → "return" expression? ;
 *
 *   underscoreExpr   → "_" ;
 *
 *   expressionWithoutBlock → outerAttr* expressionKind ;
 *
 *   expressionWithBlock → outerAttr* expressionKindWithBlock ;
 *
 *   expressionKind   → literalExpr
 *                    | pathExpr
 *                    | groupedExpr
 *                    | arrayExpr
 *                    | tupleExpr
 *                    | structExpr
 *                    | closureExpr
 *                    | continueExpr
 *                    | breakExpr
 *                    | returnExpr
 *                    | underscoreExpr
 *                    | macroInvocation ;
 *
 *   expressionKindWithBlock → blockExpr
 *                           | asyncBlockExpr
 *                           | unsafeBlockExpr
 *                           | loopExpr
 *                           | ifExpr
 *                           | ifLetExpr
 *                           | matchExpr ;
 *
 *   macroInvocation  → simplePath "!" delimTokenTree ;
 *
 *   tokenStream      → tokenTree* ;
 *
 *   tokenTree        → tokenExceptDelims
 *                    | delimTokenTree ;
 *
 *   tupleIndex       → INTEGER ;
 *
 */
