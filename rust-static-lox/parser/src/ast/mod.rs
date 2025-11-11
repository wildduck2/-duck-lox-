// ============================================================================
// Complete Rust AST - FIXED (duplications removed)
// ============================================================================

pub(crate) mod print_tree;

use diagnostic::Span;

// ----------------------------------------------------------------------------
// Top-level Items
// ----------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub(crate) enum Item {
  Function(FnDecl),
  Struct(StructDecl),
  Enum(EnumDecl),
  Trait(TraitDecl),
  Impl(ImplBlock),
  Const(ConstDecl),
  Static(StaticDecl),
  TypeAlias(TypeAliasDecl),
  Module(ModuleDecl),
  Use(UseDecl),
  ExternCrate(ExternCrateDecl),
  Macro(MacroDecl),
  Macro2(Macro2Decl),
  ForeignMod(ForeignModDecl),
  Union(UnionDecl),
  ExternType(ExternTypeDecl),
}

// ----------------------------------------------------------------------------
// Extern Type Declaration
// ----------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub(crate) struct ExternTypeDecl {
  pub(crate) visibility: Visibility,
  pub(crate) name: String,
  pub(crate) generics: Option<GenericParams>,
  pub(crate) span: Span,
}

// ----------------------------------------------------------------------------
// Additional Item Declarations
// ----------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub(crate) struct ConstDecl {
  pub(crate) visibility: Visibility,
  pub(crate) name: String,
  pub(crate) ty: Type,
  pub(crate) value: Option<Expr>,
  pub(crate) span: Span,
}

#[derive(Debug, Clone)]
pub(crate) struct StaticDecl {
  pub(crate) visibility: Visibility,
  pub(crate) name: String,
  pub(crate) ty: Type,
  pub(crate) mutability: Mutability,
  pub(crate) value: Option<Expr>,
  pub(crate) span: Span,
}

#[derive(Debug, Clone)]
pub(crate) struct TypeAliasDecl {
  pub(crate) visibility: Visibility,
  pub(crate) name: String,
  pub(crate) generics: Option<GenericParams>,
  pub(crate) bounds: Option<Vec<TypeBound>>,
  pub(crate) where_clause: Option<WhereClause>,
  pub(crate) ty: Option<Type>,
  pub(crate) span: Span,
}

#[derive(Debug, Clone)]
pub(crate) struct ModuleDecl {
  pub(crate) visibility: Visibility,
  pub(crate) name: String,
  pub(crate) items: Option<Vec<Item>>,
  pub(crate) is_unsafe: bool,
  pub(crate) span: Span,
}

#[derive(Debug, Clone)]
pub(crate) struct UseDecl {
  pub(crate) visibility: Visibility,
  pub(crate) tree: UseTree,
  pub(crate) span: Span,
}

#[derive(Debug, Clone)]
pub(crate) enum UseTree {
  Path {
    prefix: String,
    suffix: Box<UseTree>,
  },
  Name(String),
  Rename {
    name: String,
    alias: String,
  },
  Glob,
  List(Vec<UseTree>),
}

#[derive(Debug, Clone)]
pub(crate) struct ExternCrateDecl {
  pub(crate) visibility: Visibility,
  pub(crate) name: String,
  pub(crate) alias: Option<String>,
  pub(crate) span: Span,
}

// ----------------------------------------------------------------------------
// Macro Declarations
// ----------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub(crate) struct MacroDecl {
  pub(crate) name: String,
  pub(crate) rules: Vec<MacroRule>,
  pub(crate) span: Span,
}

#[derive(Debug, Clone)]
pub(crate) struct Macro2Decl {
  pub(crate) visibility: Visibility,
  pub(crate) name: String,
  pub(crate) params: Vec<MacroParam>,
  pub(crate) body: Vec<TokenTree>,
  pub(crate) span: Span,
}

#[derive(Debug, Clone)]
pub(crate) struct MacroParam {
  pub(crate) name: String,
  pub(crate) kind: MacroParamKind,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum MacroParamKind {
  Block,
  Expr,
  Ident,
  Item,
  Lifetime,
  Literal,
  Meta,
  Pat,
  PatParam,
  Path,
  Stmt,
  Tt,
  Ty,
  Vis,
}

#[derive(Debug, Clone)]
pub(crate) struct MacroRule {
  pub(crate) matcher: Vec<TokenTree>,
  pub(crate) transcriber: Vec<TokenTree>,
}

#[derive(Debug, Clone)]
pub(crate) enum TokenTree {
  Token(String),
  Delimited {
    delimiter: Delimiter,
    tokens: Vec<TokenTree>,
  },
  Repeat {
    tokens: Vec<TokenTree>,
    separator: Option<String>,
    kind: RepeatKind,
  },
  MetaVar {
    name: String,
    kind: String,
  },
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Delimiter {
  Paren,
  Brace,
  Bracket,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum RepeatKind {
  ZeroOrMore,
  OneOrMore,
  ZeroOrOne,
}

// ----------------------------------------------------------------------------
// Foreign Items
// ----------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub(crate) struct ForeignModDecl {
  pub(crate) is_unsafe: bool,
  pub(crate) abi: Option<String>,
  pub(crate) items: Vec<ForeignItem>,
  pub(crate) span: Span,
}

#[derive(Debug, Clone)]
pub(crate) enum ForeignItem {
  Function {
    visibility: Visibility,
    name: String,
    generics: Option<GenericParams>,
    params: Vec<Param>,
    return_type: Option<Type>,
    is_variadic: bool,
    span: Span,
  },
  Static {
    visibility: Visibility,
    name: String,
    ty: Type,
    mutability: Mutability,
    span: Span,
  },
  Type {
    visibility: Visibility,
    name: String,
    generics: Option<GenericParams>,
    span: Span,
  },
}

#[derive(Debug, Clone)]
pub(crate) struct UnionDecl {
  pub(crate) visibility: Visibility,
  pub(crate) name: String,
  pub(crate) generics: Option<GenericParams>,
  pub(crate) fields: Vec<FieldDecl>,
  pub(crate) where_clause: Option<WhereClause>,
  pub(crate) span: Span,
}

// ----------------------------------------------------------------------------
// Function Declaration
// ----------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub(crate) struct FnDecl {
  pub(crate) visibility: Visibility,
  pub(crate) name: String,
  pub(crate) generics: Option<GenericParams>,
  pub(crate) params: Vec<Param>,
  pub(crate) return_type: Option<Type>,
  pub(crate) where_clause: Option<WhereClause>,
  pub(crate) body: Option<Vec<Stmt>>,
  pub(crate) is_async: bool,
  pub(crate) is_const: bool,
  pub(crate) is_unsafe: bool,
  pub(crate) is_extern: bool,
  pub(crate) abi: Option<String>,
  pub(crate) span: Span,
}

#[derive(Debug, Clone)]
pub(crate) struct Param {
  pub(crate) attributes: Vec<Attribute>,
  pub(crate) pattern: Pattern,
  pub(crate) type_annotation: Option<Type>,
  pub(crate) is_self: bool,
  pub(crate) is_variadic: bool,
}

// ----------------------------------------------------------------------------
// Attributes and Derives
// ----------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub(crate) struct Attribute {
  pub(crate) style: AttrStyle,
  pub(crate) kind: AttrKind,
  pub(crate) span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum AttrStyle {
  Outer,
  Inner,
}

#[derive(Debug, Clone)]
pub(crate) enum AttrKind {
  Normal {
    path: Path,
    tokens: Vec<TokenTree>,
  },
  DocComment {
    is_inner: bool,
    content: String,
  },
  Cfg(MetaItem),
  CfgAttr {
    condition: MetaItem,
    attrs: Vec<Attribute>,
  },
}

// ----------------------------------------------------------------------------
// Path Types (UNIFIED)
// ----------------------------------------------------------------------------

/// Unified path type used throughout the AST
/// Replaces SimplePath, TypePath, and ExprPath
#[derive(Debug, Clone)]
pub(crate) struct Path {
  pub(crate) leading_colon: bool,
  pub(crate) segments: Vec<PathSegment>,
}

#[derive(Debug, Clone)]
pub(crate) struct PathSegment {
  pub(crate) kind: PathSegmentKind,
  pub(crate) args: Option<GenericArgs>,
}

impl PathSegment {
  // TODO: later on remove this will be removed do not rely on it
  pub(crate) fn new(kind: PathSegmentKind) -> Self {
    Self { kind, args: None }
  }
}

#[derive(Debug, Clone)]
pub(crate) enum PathSegmentKind {
  Ident(String),
  Super,
  Self_,
  Crate,
  DollarCrate,
  SelfType, // For `Self` as a type
}

// ----------------------------------------------------------------------------
// Meta Items
// ----------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub(crate) enum MetaItem {
  Word(String),
  NameValue(String, MetaItemValue),
  List(String, Vec<MetaItem>),
}

#[derive(Debug, Clone)]
pub(crate) enum MetaItemValue {
  Str(String),
  Int(i128),
  Bool(bool),
}

// ----------------------------------------------------------------------------
// Struct Declaration
// ----------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub(crate) struct StructDecl {
  pub(crate) attributes: Vec<Attribute>,
  pub(crate) visibility: Visibility,
  pub(crate) name: String,
  pub(crate) generics: Option<GenericParams>,
  pub(crate) kind: StructKind,
  pub(crate) where_clause: Option<WhereClause>,
  pub(crate) span: Span,
}

#[derive(Debug, Clone)]
pub(crate) enum StructKind {
  Named { fields: Vec<FieldDecl> },
  Tuple(Vec<TupleField>),
  Unit,
}

#[derive(Debug, Clone)]
pub(crate) struct TupleField {
  pub(crate) attributes: Vec<Attribute>,
  pub(crate) visibility: Visibility,
  pub(crate) ty: Type,
  pub(crate) span: Span,
}

#[derive(Debug, Clone)]
pub(crate) struct FieldDecl {
  pub(crate) attributes: Vec<Attribute>,
  pub(crate) visibility: Visibility,
  pub(crate) name: String,
  pub(crate) ty: Type,
  pub(crate) span: Span,
}

// ----------------------------------------------------------------------------
// Enum Declaration
// ----------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub(crate) struct EnumDecl {
  pub(crate) attributes: Vec<Attribute>,
  pub(crate) visibility: Visibility,
  pub(crate) name: String,
  pub(crate) generics: Option<GenericParams>,
  pub(crate) variants: Vec<EnumVariant>,
  pub(crate) where_clause: Option<WhereClause>,
  pub(crate) span: Span,
}

#[derive(Debug, Clone)]
pub(crate) struct EnumVariant {
  pub(crate) attributes: Vec<Attribute>,
  pub(crate) visibility: Visibility,
  pub(crate) name: String,
  pub(crate) kind: EnumVariantKind,
  pub(crate) discriminant: Option<Expr>,
  pub(crate) span: Span,
}

#[derive(Debug, Clone)]
pub(crate) enum EnumVariantKind {
  Unit,
  Tuple(Vec<TupleField>),
  Struct(Vec<FieldDecl>),
}

// ----------------------------------------------------------------------------
// Trait Declaration
// ----------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub(crate) struct TraitDecl {
  pub(crate) attributes: Vec<Attribute>,
  pub(crate) visibility: Visibility,
  pub(crate) name: String,
  pub(crate) is_auto: bool,
  pub(crate) is_unsafe: bool,
  pub(crate) generics: Option<GenericParams>,
  pub(crate) supertraits: Option<Vec<TypeBound>>,
  pub(crate) items: Vec<TraitItem>,
  pub(crate) where_clause: Option<WhereClause>,
  pub(crate) span: Span,
}

#[derive(Debug, Clone)]
pub(crate) enum TraitItem {
  Method(FnDecl),
  Type {
    attributes: Vec<Attribute>,
    name: String,
    generics: Option<GenericParams>,
    bounds: Option<Vec<TypeBound>>,
    default: Option<Type>,
  },
  Const {
    attributes: Vec<Attribute>,
    name: String,
    ty: Type,
    default: Option<Expr>,
  },
  Macro {
    mac: MacroInvocation,
  },
}

#[derive(Debug, Clone)]
pub(crate) struct MacroInvocation {
  pub(crate) path: Path,
  pub(crate) delimiter: Delimiter,
  pub(crate) tokens: Vec<TokenTree>,
  pub(crate) span: Span,
}

// ----------------------------------------------------------------------------
// Impl Block
// ----------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub(crate) struct ImplBlock {
  pub(crate) attributes: Vec<Attribute>,
  pub(crate) is_unsafe: bool,
  pub(crate) is_const: bool,
  pub(crate) generics: Option<GenericParams>,
  pub(crate) polarity: ImplPolarity,
  pub(crate) trait_ref: Option<Path>,
  pub(crate) self_ty: Type,
  pub(crate) items: Vec<ImplItem>,
  pub(crate) where_clause: Option<WhereClause>,
  pub(crate) span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum ImplPolarity {
  Positive,
  Negative,
}

#[derive(Debug, Clone)]
pub(crate) enum ImplItem {
  Method(FnDecl),
  Type {
    attributes: Vec<Attribute>,
    visibility: Visibility,
    name: String,
    generics: Option<GenericParams>,
    ty: Type,
  },
  Const {
    attributes: Vec<Attribute>,
    visibility: Visibility,
    name: String,
    ty: Type,
    value: Expr,
  },
  Macro {
    mac: MacroInvocation,
  },
}

// ----------------------------------------------------------------------------
// Generics
// ----------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub(crate) struct GenericParams {
  pub(crate) params: Vec<GenericParam>,
  pub(crate) span: Span,
}

#[derive(Debug, Clone)]
pub(crate) enum GenericParam {
  Type {
    attributes: Vec<Attribute>,
    name: String,
    bounds: Option<Vec<TypeBound>>,
    default: Option<Type>,
  },
  Lifetime {
    attributes: Vec<Attribute>,
    name: String,
    bounds: Option<Vec<String>>,
  },
  Const {
    attributes: Vec<Attribute>,
    name: String,
    ty: Type,
    default: Option<Expr>,
  },
}

#[derive(Debug, Clone)]
pub(crate) struct TypeBound {
  pub(crate) modifier: TraitBoundModifier,
  pub(crate) path: Path,
  pub(crate) generics: Option<Vec<GenericArg>>,
  pub(crate) for_lifetimes: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum TraitBoundModifier {
  None,
  Maybe,
  MaybeConst,
  Const,
}

#[derive(Debug, Clone)]
pub(crate) struct WhereClause {
  pub(crate) predicates: Vec<WherePredicate>,
}

#[derive(Debug, Clone)]
pub(crate) enum WherePredicate {
  Type {
    for_lifetimes: Option<Vec<String>>,
    ty: Type,
    bounds: Option<Vec<TypeBound>>,
  },
  Lifetime {
    lifetime: String,
    bounds: Vec<String>,
  },
  Equality {
    ty: Type,
    equals: Type,
  },
}

// ----------------------------------------------------------------------------
// Visibility
// ----------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub(crate) enum Visibility {
  Lic,
  LicCrate,
  LicSuper,
  LicSelf,
  LicIn(Path),
  Private,
}

// ----------------------------------------------------------------------------
// Type System
// ----------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub(crate) enum Type {
  // Primitive types
  I8,
  I16,
  I32,
  I64,
  I128,
  Isize,
  U8,
  U16,
  U32,
  U64,
  U128,
  Usize,
  F32,
  F64,
  Bool,
  Char,
  Str,
  Never,

  SelfType,
  Unit,

  // Composite types
  Array {
    element: Box<Type>,
    size: Box<Expr>,
  },
  Slice(Box<Type>),
  Tuple(Vec<Type>),

  Reference {
    lifetime: Option<String>,
    mutability: Mutability,
    inner: Box<Type>,
  },
  RawPointer {
    mutability: Mutability,
    inner: Box<Type>,
  },

  BareFn {
    for_lifetimes: Option<Vec<String>>,
    safety: Safety,
    abi: Option<String>,
    params: Vec<BareFnParam>,
    return_type: Option<Box<Type>>,
    is_variadic: bool,
  },

  Path(Path),
  QPath {
    self_ty: Box<Type>,
    trait_ref: Option<Path>,
    name: String,
    generics: Option<Box<GenericArgs>>,
  },

  TraitObject {
    bounds: Vec<TypeBound>,
    lifetime: Option<String>,
    is_dyn: bool,
  },
  ImplTrait(Vec<TypeBound>),

  Infer,
  Paren(Box<Type>),
  Macro(Box<MacroInvocation>),
  Typeof(Box<Expr>),
}

#[derive(Debug, Clone)]
pub(crate) struct BareFnParam {
  pub(crate) attributes: Vec<Attribute>,
  pub(crate) name: Option<String>,
  pub(crate) ty: Type,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Safety {
  Safe,
  Unsafe,
}

#[derive(Debug, Clone)]
pub(crate) enum GenericArgs {
  AngleBracketed {
    args: Vec<GenericArg>,
  },
  Parenthesized {
    inputs: Vec<Type>,
    output: Option<Type>,
  },
}

#[derive(Debug, Clone)]
pub(crate) enum GenericArg {
  Lifetime(String),
  Type(Type),
  Const(Expr),
  Binding {
    name: String,
    generics: Option<GenericParams>,
    ty: Type,
  },
  Constraint {
    name: String,
    generics: Option<GenericParams>,
    bounds: Vec<TypeBound>,
  },
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Mutability {
  Mutable,
  Immutable,
}

// ----------------------------------------------------------------------------
// Statements
// ----------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub(crate) enum Stmt {
  Expr(Expr),
  Semi(Expr),
  TailExpr(Expr),
  Let {
    attributes: Vec<Attribute>,
    pattern: Pattern,
    ty: Option<Type>,
    init: Option<Box<Expr>>,
    else_block: Option<Box<Expr>>,
    span: Span,
  },
  Item(Box<Item>),
  Empty,
}

// ----------------------------------------------------------------------------
// Patterns
// ----------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub(crate) enum Pattern {
  Wildcard {
    span: Span,
  },
  Rest {
    span: Span,
  },

  Literal {
    expr: Box<Expr>,
    span: Span,
  },

  Ident {
    reference: Option<Mutability>,
    mutability: Mutability,
    name: String,
    subpattern: Option<Box<Pattern>>,
    span: Span,
  },

  Path {
    qself: Option<Box<Type>>,
    path: Path,
    span: Span,
  },

  Tuple {
    attributes: Vec<Attribute>,
    patterns: Vec<Pattern>,
    span: Span,
  },

  Slice {
    before: Vec<Pattern>,
    middle: Option<Box<Pattern>>,
    after: Vec<Pattern>,
    span: Span,
  },

  Struct {
    qself: Option<Box<Type>>,
    path: Path,
    fields: Vec<FieldPattern>,
    has_rest: bool,
    span: Span,
  },

  TupleStruct {
    qself: Option<Box<Type>>,
    path: Path,
    patterns: Vec<Pattern>,
    span: Span,
  },

  Or {
    patterns: Vec<Pattern>,
    span: Span,
  },

  Range {
    start: Option<Box<Expr>>,
    end: Option<Box<Expr>>,
    kind: RangeKind,
    span: Span,
  },

  Reference {
    mutability: Mutability,
    pattern: Box<Pattern>,
    span: Span,
  },

  Box {
    pattern: Box<Pattern>,
    span: Span,
  },

  Macro {
    mac: MacroInvocation,
  },

  Paren {
    pattern: Box<Pattern>,
    span: Span,
  },
}

#[derive(Debug, Clone)]
pub(crate) struct FieldPattern {
  pub(crate) attributes: Vec<Attribute>,
  pub(crate) name: String,
  pub(crate) pattern: Option<Pattern>,
  pub(crate) is_shorthand: bool,
}

// ----------------------------------------------------------------------------
// Match Arms
// ----------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub(crate) struct MatchArm {
  pub(crate) attributes: Vec<Attribute>,
  pub(crate) pattern: Pattern,
  pub(crate) guard: Option<Expr>,
  pub(crate) body: Expr,
  pub(crate) comma: bool,
  pub(crate) span: Span,
}

// ----------------------------------------------------------------------------
// Expressions
// ----------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub(crate) enum Expr {
  Integer {
    value: i128,
    suffix: Option<String>,
    span: Span,
  },
  Float {
    value: f64,
    suffix: Option<String>,
    span: Span,
  },
  String {
    value: String,
    kind: StrKind,
    span: Span,
  },
  Char {
    value: char,
    span: Span,
  },
  ByteString {
    value: Vec<u8>,
    kind: ByteStrKind,
    span: Span,
  },
  Byte {
    value: u8,
    span: Span,
  },
  Bool {
    value: bool,
    span: Span,
  },

  Ident {
    name: String,
    span: Span,
  },

  Path(Path),

  Binary {
    left: Box<Expr>,
    op: BinaryOp,
    right: Box<Expr>,
    span: Span,
  },
  Unary {
    op: UnaryOp,
    expr: Box<Expr>,
    span: Span,
  },

  Group {
    attributes: Vec<Attribute>,
    expr: Box<Expr>,
    span: Span,
  },

  Tuple {
    attributes: Vec<Attribute>,
    elements: Vec<Expr>,
    span: Span,
  },

  Assign {
    target: Box<Expr>,
    value: Box<Expr>,
    span: Span,
  },
  AssignOp {
    target: Box<Expr>,
    op: BinaryOp,
    value: Box<Expr>,
    span: Span,
  },

  Field {
    object: Box<Expr>,
    field: FieldAccess,
    span: Span,
  },

  MethodCall {
    receiver: Box<Expr>,
    method: String,
    turbofish: Option<GenericArgs>,
    args: Vec<Expr>,
    span: Span,
  },

  Call {
    callee: Box<Expr>,
    args: Vec<Expr>,
    span: Span,
  },

  Index {
    object: Box<Expr>,
    index: Box<Expr>,
    span: Span,
  },

  Unit(Span),

  Range {
    start: Option<Box<Expr>>,
    end: Option<Box<Expr>>,
    kind: RangeKind,
    span: Span,
  },

  Array {
    elements: Vec<Expr>,
    repeat: Option<Box<Expr>>,
    span: Span,
  },
  ArrayRepeat {
    element: Box<Expr>,
    count: Box<Expr>,
    span: Span,
  },

  Struct {
    path: Path,
    fields: Vec<FieldInit>,
    base: Option<Box<Expr>>,
    span: Span,
  },

  If {
    condition: Box<Expr>,
    then_branch: Box<Expr>,
    else_branch: Option<Box<Expr>>,
    span: Span,
  },

  IfLet {
    pattern: Pattern,
    scrutinee: Box<Expr>,
    then_branch: Box<Expr>,
    else_branch: Option<Box<Expr>>,
    span: Span,
  },

  Match {
    expr: Box<Expr>,
    arms: Vec<MatchArm>,
    span: Span,
  },

  Loop {
    body: Vec<Stmt>,
    label: Option<String>,
    span: Span,
  },
  While {
    condition: Box<Expr>,
    body: Vec<Stmt>,
    label: Option<String>,
    span: Span,
  },
  WhileLet {
    pattern: Pattern,
    scrutinee: Box<Expr>,
    body: Vec<Stmt>,
    label: Option<String>,
    span: Span,
  },
  For {
    pattern: Pattern,
    iterator: Box<Expr>,
    body: Vec<Stmt>,
    label: Option<String>,
    span: Span,
  },

  Return {
    value: Option<Box<Expr>>,
    span: Span,
  },
  Break {
    label: Option<String>,
    value: Option<Box<Expr>>,
    span: Span,
  },
  Continue {
    label: Option<String>,
    span: Span,
  },
  Yield {
    value: Option<Box<Expr>>,
    span: Span,
  },

  Become {
    expr: Box<Expr>,
    span: Span,
  },

  Closure {
    capture: CaptureKind,
    is_async: bool,
    is_move: bool,
    params: Vec<ClosureParam>,
    return_type: Option<Type>,
    body: Box<Expr>,
    span: Span,
  },

  Block {
    attributes: Vec<Attribute>,
    stmts: Vec<Stmt>,
    label: Option<String>,
    is_unsafe: bool,
    span: Span,
  },

  LabeledBlock {
    label: String,
    block: Vec<Stmt>,
    span: Span,
  },

  Async {
    attributes: Vec<Attribute>,
    capture: CaptureKind,
    block: Vec<Stmt>,
    span: Span,
  },
  Await {
    expr: Box<Expr>,
    span: Span,
  },

  Try {
    expr: Box<Expr>,
    span: Span,
  },

  TryBlock {
    attributes: Vec<Attribute>,
    block: Vec<Stmt>,
    span: Span,
  },

  Cast {
    expr: Box<Expr>,
    ty: Type,
    span: Span,
  },
  Type {
    expr: Box<Expr>,
    ty: Type,
    span: Span,
  },

  Let {
    pattern: Pattern,
    expr: Box<Expr>,
    span: Span,
  },

  Unsafe {
    block: Vec<Stmt>,
    span: Span,
  },

  Const {
    block: Vec<Stmt>,
    span: Span,
  },

  InlineConst {
    generics: Option<GenericParams>,
    block: Vec<Stmt>,
    span: Span,
  },

  Box {
    expr: Box<Expr>,
    span: Span,
  },

  Underscore {
    span: Span,
  },

  Macro {
    mac: MacroInvocation,
  },

  Paren {
    expr: Box<Expr>,
    span: Span,
  },

  InlineAsm {
    template: String,
    operands: Vec<AsmOperand>,
    options: Vec<String>,
    span: Span,
  },

  FormatString {
    template: String,
    args: Vec<FormatArg>,
    span: Span,
  },
}

// ----------------------------------------------------------------------------
// Expression Supporting Types
// ----------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub(crate) struct FormatArg {
  pub(crate) name: Option<String>,
  pub(crate) expr: Expr,
  pub(crate) format_spec: Option<FormatSpec>,
}

#[derive(Debug, Clone)]
pub(crate) struct FormatSpec {
  pub(crate) fill: Option<char>,
  pub(crate) align: Option<FormatAlign>,
  pub(crate) sign: Option<FormatSign>,
  pub(crate) alternate: bool,
  pub(crate) zero_pad: bool,
  pub(crate) width: Option<FormatCount>,
  pub(crate) precision: Option<FormatCount>,
  pub(crate) ty: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum FormatAlign {
  Left,
  Center,
  Right,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum FormatSign {
  Plus,
  Minus,
}

#[derive(Debug, Clone)]
pub(crate) enum FormatCount {
  Integer(usize),
  Argument(String),
  Asterisk,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum StrKind {
  Normal,
  Raw(usize),
  C,
  RawC(usize),
  Byte,
  RawByte(usize),
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum ByteStrKind {
  Normal,
  Raw(usize),
}

#[derive(Debug, Clone)]
pub(crate) enum FieldAccess {
  Named(String),
  Unnamed(usize),
}

#[derive(Debug, Clone)]
pub(crate) struct AsmOperand {
  pub(crate) kind: AsmOperandKind,
  pub(crate) constraint: String,
  pub(crate) expr: Expr,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum AsmOperandKind {
  In,
  Out,
  InOut,
  SplitInOut,
  Const,
  Sym,
}

#[derive(Debug, Clone)]
pub(crate) struct FieldInit {
  pub(crate) attributes: Vec<Attribute>,
  pub(crate) name: String,
  pub(crate) value: Option<Expr>,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum RangeKind {
  Exclusive,
  Inclusive,
  From,
  To,
  ToInclusive,
  Full,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum CaptureKind {
  Default,
  Move,
}

#[derive(Debug, Clone)]
pub(crate) struct ClosureParam {
  pub(crate) attributes: Vec<Attribute>,
  pub(crate) pattern: Pattern,
  pub(crate) ty: Option<Type>,
}

// ----------------------------------------------------------------------------
// Binary Operators
// ----------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum BinaryOp {
  Add,
  Sub,
  Mul,
  Div,
  Mod,
  BitAnd,
  BitOr,
  BitXor,
  Shl,
  Shr,
  Eq,
  NotEq,
  Less,
  LessEq,
  Greater,
  GreaterEq,
  And,
  Or,
}

// ----------------------------------------------------------------------------
// Unary Operators
// ----------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum UnaryOp {
  Neg,
  Not,
  Deref,
  Ref { mutable: bool, depth: usize },
}
