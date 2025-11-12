// ============================================================================
// Complete Rust AST - FIXED (duplications removed)
// ============================================================================

pub(crate) mod path;
pub(crate) mod print;
use diagnostic::Span;
use path::*;

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
  pub visibility: Visibility,
  pub name: String,
  pub generics: Option<GenericParams>,
  pub span: Span,
}

// ----------------------------------------------------------------------------
// Additional Item Declarations
// ----------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub(crate) struct ConstDecl {
  pub visibility: Visibility,
  pub name: String,
  pub ty: Type,
  pub value: Option<Expr>,
  pub span: Span,
}

#[derive(Debug, Clone)]
pub(crate) struct StaticDecl {
  pub visibility: Visibility,
  pub name: String,
  pub ty: Type,
  pub mutability: Mutability,
  pub value: Option<Expr>,
  pub span: Span,
}

#[derive(Debug, Clone)]
pub(crate) struct TypeAliasDecl {
  pub visibility: Visibility,
  pub name: String,
  pub generics: Option<GenericParams>,
  pub bounds: Option<Vec<TypeBound>>,
  pub where_clause: Option<WhereClause>,
  pub ty: Option<Type>,
  pub span: Span,
}

#[derive(Debug, Clone)]
pub(crate) struct ModuleDecl {
  pub visibility: Visibility,
  pub name: String,
  pub items: Option<Vec<Item>>,
  pub is_unsafe: bool,
  pub span: Span,
}

#[derive(Debug, Clone)]
pub(crate) struct UseDecl {
  pub visibility: Visibility,
  pub tree: UseTree,
  pub span: Span,
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
  pub visibility: Visibility,
  pub name: String,
  pub alias: Option<String>,
  pub span: Span,
}

// ----------------------------------------------------------------------------
// Macro Declarations
// ----------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub(crate) struct MacroDecl {
  pub name: String,
  pub rules: Vec<MacroRule>,
  pub span: Span,
}

#[derive(Debug, Clone)]
pub(crate) struct Macro2Decl {
  pub visibility: Visibility,
  pub name: String,
  pub params: Vec<MacroParam>,
  pub body: Vec<TokenTree>,
  pub span: Span,
}

#[derive(Debug, Clone)]
pub(crate) struct MacroParam {
  pub name: String,
  pub kind: MacroParamKind,
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
  pub matcher: Vec<TokenTree>,
  pub transcriber: Vec<TokenTree>,
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
  pub is_unsafe: bool,
  pub abi: Option<String>,
  pub items: Vec<ForeignItem>,
  pub span: Span,
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
  pub visibility: Visibility,
  pub name: String,
  pub generics: Option<GenericParams>,
  pub fields: Vec<FieldDecl>,
  pub where_clause: Option<WhereClause>,
  pub span: Span,
}

// ----------------------------------------------------------------------------
// Function Declaration
// ----------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub(crate) struct FnDecl {
  pub visibility: Visibility,
  pub name: String,
  pub generics: Option<GenericParams>,
  pub params: Vec<Param>,
  pub return_type: Option<Type>,
  pub where_clause: Option<WhereClause>,
  pub body: Option<Vec<Stmt>>,
  pub is_async: bool,
  pub is_const: bool,
  pub is_unsafe: bool,
  pub is_extern: bool,
  pub abi: Option<String>,
  pub span: Span,
}

#[derive(Debug, Clone)]
pub(crate) struct Param {
  pub attributes: Vec<Attribute>,
  pub pattern: Pattern,
  pub type_annotation: Option<Type>,
  pub is_self: bool,
  pub is_variadic: bool,
}

// ----------------------------------------------------------------------------
// Attributes and Derives
// ----------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub(crate) struct Attribute {
  pub style: AttrStyle,
  pub kind: AttrKind,
  pub span: Span,
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
  pub attributes: Vec<Attribute>,
  pub visibility: Visibility,
  pub name: String,
  pub generics: Option<GenericParams>,
  pub kind: StructKind,
  pub where_clause: Option<WhereClause>,
  pub span: Span,
}

#[derive(Debug, Clone)]
pub(crate) enum StructKind {
  Named { fields: Vec<FieldDecl> },
  Tuple(Vec<TupleField>),
  Unit,
}

#[derive(Debug, Clone)]
pub(crate) struct TupleField {
  pub attributes: Vec<Attribute>,
  pub visibility: Visibility,
  pub ty: Type,
  pub span: Span,
}

#[derive(Debug, Clone)]
pub(crate) struct FieldDecl {
  pub attributes: Vec<Attribute>,
  pub visibility: Visibility,
  pub name: String,
  pub ty: Type,
  pub span: Span,
}

// ----------------------------------------------------------------------------
// Enum Declaration
// ----------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub(crate) struct EnumDecl {
  pub attributes: Vec<Attribute>,
  pub visibility: Visibility,
  pub name: String,
  pub generics: Option<GenericParams>,
  pub variants: Vec<EnumVariant>,
  pub where_clause: Option<WhereClause>,
  pub span: Span,
}

#[derive(Debug, Clone)]
pub(crate) struct EnumVariant {
  pub attributes: Vec<Attribute>,
  pub visibility: Visibility,
  pub name: String,
  pub kind: EnumVariantKind,
  pub discriminant: Option<Expr>,
  pub span: Span,
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
  pub attributes: Vec<Attribute>,
  pub visibility: Visibility,
  pub name: String,
  pub is_auto: bool,
  pub is_unsafe: bool,
  pub generics: Option<GenericParams>,
  pub supertraits: Option<Vec<TypeBound>>,
  pub items: Vec<TraitItem>,
  pub where_clause: Option<WhereClause>,
  pub span: Span,
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
  pub path: Path,
  pub delimiter: Delimiter,
  pub tokens: Vec<TokenTree>,
  pub span: Span,
}

// ----------------------------------------------------------------------------
// Impl Block
// ----------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub(crate) struct ImplBlock {
  pub attributes: Vec<Attribute>,
  pub is_unsafe: bool,
  pub is_const: bool,
  pub generics: Option<GenericParams>,
  pub polarity: ImplPolarity,
  pub trait_ref: Option<Path>,
  pub self_ty: Type,
  pub items: Vec<ImplItem>,
  pub where_clause: Option<WhereClause>,
  pub span: Span,
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
  pub params: Vec<GenericParam>,
  pub span: Span,
}

/// Represents a single generic parameter in a type, function, or struct.
///
/// Examples:
/// - `T: Clone` → `Type`
/// - `'a: 'b` → `Lifetime`
/// - `const N: usize = 3` → `Const`
#[derive(Debug, Clone)]
pub(crate) enum GenericParam {
  /// A type parameter like `T: Clone` or `T = i32`.
  Type {
    attributes: Vec<Attribute>,
    name: String,
    bounds: Option<Vec<TypeBound>>,
    default: Option<Type>,
  },

  /// A lifetime parameter like `'a` or `'a: 'b`.
  Lifetime {
    attributes: Vec<Attribute>,
    name: String,
    bounds: Option<Vec<TypeBound>>,
  },

  /// A const parameter like `const N: usize = 3`.
  Const {
    attributes: Vec<Attribute>,
    name: String,
    ty: Type,
    default: Option<Expr>,
  },
}

#[derive(Debug, Clone)]
pub(crate) struct TypeBound {
  pub modifier: TraitBoundModifier,
  pub path: Path,
  pub generics: Option<Vec<GenericArg>>,
  pub for_lifetimes: Option<Vec<String>>,
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
  pub predicates: Vec<WherePredicate>,
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
pub enum Visibility {
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
  F128,
  Bool,
  Char,
  Str,
  String,
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
  pub attributes: Vec<Attribute>,
  pub name: Option<String>,
  pub ty: Type,
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
  pub attributes: Vec<Attribute>,
  pub name: String,
  pub pattern: Option<Pattern>,
  pub is_shorthand: bool,
}

// ----------------------------------------------------------------------------
// Match Arms
// ----------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub(crate) struct MatchArm {
  pub attributes: Vec<Attribute>,
  pub pattern: Pattern,
  pub guard: Option<Expr>,
  pub body: Expr,
  pub comma: bool,
  pub span: Span,
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
  pub name: Option<String>,
  pub expr: Expr,
  pub format_spec: Option<FormatSpec>,
}

#[derive(Debug, Clone)]
pub(crate) struct FormatSpec {
  pub fill: Option<char>,
  pub align: Option<FormatAlign>,
  pub sign: Option<FormatSign>,
  pub alternate: bool,
  pub zero_pad: bool,
  pub width: Option<FormatCount>,
  pub precision: Option<FormatCount>,
  pub ty: Option<String>,
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
  pub kind: AsmOperandKind,
  pub constraint: String,
  pub expr: Expr,
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
  pub attributes: Vec<Attribute>,
  pub name: String,
  pub value: Option<Expr>,
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
  pub attributes: Vec<Attribute>,
  pub pattern: Pattern,
  pub ty: Option<Type>,
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
