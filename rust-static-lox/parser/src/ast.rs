// ============================================================================
// Complete Rust AST with All Features - FIXED
// ============================================================================

use diagnostic::Span;

// ----------------------------------------------------------------------------
// Top-level Items
// ----------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub enum Item {
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
pub struct ExternTypeDecl {
  pub visibility: Visibility,
  pub name: String,
  pub generics: Option<GenericParams>,
  pub span: Span,
}

// ----------------------------------------------------------------------------
// Additional Item Declarations
// ----------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct ConstDecl {
  pub visibility: Visibility,
  pub name: String,
  pub ty: Type,
  pub value: Expr,
  pub span: Span,
}

#[derive(Debug, Clone)]
pub struct StaticDecl {
  pub visibility: Visibility,
  pub name: String,
  pub ty: Type,
  pub mutability: Mutability,
  pub value: Expr,
  pub span: Span,
}

#[derive(Debug, Clone)]
pub struct TypeAliasDecl {
  pub visibility: Visibility,
  pub name: String,
  pub generics: Option<GenericParams>,
  pub where_clause: Option<WhereClause>,
  pub ty: Type,
  pub span: Span,
}

#[derive(Debug, Clone)]
pub struct ModuleDecl {
  pub visibility: Visibility,
  pub name: String,
  pub items: Option<Vec<Item>>,
  pub is_unsafe: bool,
  pub span: Span,
}

#[derive(Debug, Clone)]
pub struct UseDecl {
  pub visibility: Visibility,
  pub tree: UseTree,
  pub span: Span,
}

#[derive(Debug, Clone)]
pub enum UseTree {
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
pub struct ExternCrateDecl {
  pub visibility: Visibility,
  pub name: String,
  pub alias: Option<String>,
  pub span: Span,
}

// ----------------------------------------------------------------------------
// Macro Declarations
// ----------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct MacroDecl {
  pub name: String,
  pub rules: Vec<MacroRule>,
  pub span: Span,
}

#[derive(Debug, Clone)]
pub struct Macro2Decl {
  pub visibility: Visibility,
  pub name: String,
  pub params: Vec<MacroParam>,
  pub body: Vec<TokenTree>,
  pub span: Span,
}

#[derive(Debug, Clone)]
pub struct MacroParam {
  pub name: String,
  pub kind: MacroParamKind,
}

#[derive(Debug, Clone, PartialEq)]
pub enum MacroParamKind {
  Ident,
  Expr,
  Ty,
  Pat,
  Stmt,
  Block,
  Item,
  Meta,
  Tt,
  Path,
  Literal,
  Lifetime,
  Vis,
}

#[derive(Debug, Clone)]
pub struct MacroRule {
  pub matcher: Vec<TokenTree>,
  pub transcriber: Vec<TokenTree>,
}

#[derive(Debug, Clone)]
pub enum TokenTree {
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
pub enum Delimiter {
  Parenthesis,
  Brace,
  Bracket,
}

#[derive(Debug, Clone, PartialEq)]
pub enum RepeatKind {
  ZeroOrMore,
  OneOrMore,
  ZeroOrOne,
}

// ----------------------------------------------------------------------------
// Foreign Items
// ----------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct ForeignModDecl {
  pub abi: String,
  pub items: Vec<ForeignItem>,
  pub span: Span,
}

#[derive(Debug, Clone)]
pub enum ForeignItem {
  Function {
    visibility: Visibility,
    name: String,
    generics: Option<GenericParams>,
    params: Vec<Param>,
    return_type: Type,
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
pub struct UnionDecl {
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
pub struct FnDecl {
  pub visibility: Visibility,
  pub name: String,
  pub generics: Option<GenericParams>,
  pub params: Vec<Param>,
  pub return_type: Type,
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
pub struct Param {
  pub attributes: Vec<Attribute>,
  pub pattern: Pattern,
  pub type_annotation: Type,
  pub default_value: Option<Expr>,
}

// ----------------------------------------------------------------------------
// Attributes and Derives
// ----------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct Attribute {
  pub style: AttrStyle,
  pub kind: AttrKind,
  pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AttrStyle {
  Outer,
  Inner,
}

#[derive(Debug, Clone)]
pub enum AttrKind {
  Normal {
    path: ExprPath,
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

#[derive(Debug, Clone)]
pub enum MetaItem {
  Word(String),
  NameValue(String, MetaItemValue),
  List(String, Vec<MetaItem>),
}

#[derive(Debug, Clone)]
pub enum MetaItemValue {
  Str(String),
  Int(i128),
  Bool(bool),
}

// ----------------------------------------------------------------------------
// Struct Declaration
// ----------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct StructDecl {
  pub attributes: Vec<Attribute>,
  pub visibility: Visibility,
  pub name: String,
  pub generics: Option<GenericParams>,
  pub kind: StructKind,
  pub where_clause: Option<WhereClause>,
  pub span: Span,
}

#[derive(Debug, Clone)]
pub enum StructKind {
  Named { fields: Vec<FieldDecl> },
  Tuple(Vec<TupleField>),
  Unit,
}

#[derive(Debug, Clone)]
pub struct TupleField {
  pub attributes: Vec<Attribute>,
  pub visibility: Visibility,
  pub ty: Type,
  pub span: Span,
}

#[derive(Debug, Clone)]
pub struct FieldDecl {
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
pub struct EnumDecl {
  pub attributes: Vec<Attribute>,
  pub visibility: Visibility,
  pub name: String,
  pub generics: Option<GenericParams>,
  pub variants: Vec<EnumVariant>,
  pub where_clause: Option<WhereClause>,
  pub span: Span,
}

#[derive(Debug, Clone)]
pub struct EnumVariant {
  pub attributes: Vec<Attribute>,
  pub name: String,
  pub kind: EnumVariantKind,
  pub discriminant: Option<Expr>,
  pub span: Span,
}

#[derive(Debug, Clone)]
pub enum EnumVariantKind {
  Unit,
  Tuple(Vec<TupleField>),
  Struct(Vec<FieldDecl>),
}

// ----------------------------------------------------------------------------
// Trait Declaration
// ----------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct TraitDecl {
  pub attributes: Vec<Attribute>,
  pub visibility: Visibility,
  pub name: String,
  pub is_auto: bool,
  pub is_unsafe: bool,
  pub generics: Option<GenericParams>,
  pub supertraits: Vec<TypeBound>,
  pub items: Vec<TraitItem>,
  pub where_clause: Option<WhereClause>,
  pub span: Span,
}

#[derive(Debug, Clone)]
pub enum TraitItem {
  Method(FnDecl),
  Type {
    attributes: Vec<Attribute>,
    name: String,
    generics: Option<GenericParams>,
    bounds: Vec<TypeBound>,
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
pub struct MacroInvocation {
  pub path: ExprPath,
  pub delimiter: Delimiter,
  pub tokens: Vec<TokenTree>,
  pub span: Span,
}

// ----------------------------------------------------------------------------
// Impl Block
// ----------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct ImplBlock {
  pub attributes: Vec<Attribute>,
  pub is_unsafe: bool,
  pub is_default: bool,
  pub generics: Option<GenericParams>,
  pub polarity: ImplPolarity,
  pub trait_ref: Option<TypePath>,
  pub self_ty: Type,
  pub items: Vec<ImplItem>,
  pub where_clause: Option<WhereClause>,
  pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ImplPolarity {
  Positive,
  Negative,
}

#[derive(Debug, Clone)]
pub enum ImplItem {
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
pub struct GenericParams {
  pub params: Vec<GenericParam>,
  pub span: Span,
}

#[derive(Debug, Clone)]
pub enum GenericParam {
  Type {
    attributes: Vec<Attribute>,
    name: String,
    bounds: Vec<TypeBound>,
    default: Option<Type>,
  },
  Lifetime {
    attributes: Vec<Attribute>,
    name: String,
    bounds: Vec<String>,
  },
  Const {
    attributes: Vec<Attribute>,
    name: String,
    ty: Type,
    default: Option<Expr>,
  },
}

#[derive(Debug, Clone)]
pub struct TypeBound {
  pub modifier: TraitBoundModifier,
  pub path: TypePath,
  pub generics: Option<Vec<GenericArg>>,
  pub for_lifetimes: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TraitBoundModifier {
  None,
  Maybe,
  MaybeConst,
}

#[derive(Debug, Clone)]
pub struct WhereClause {
  pub predicates: Vec<WherePredicate>,
}

#[derive(Debug, Clone)]
pub enum WherePredicate {
  Type {
    for_lifetimes: Option<Vec<String>>,
    ty: Type,
    bounds: Vec<TypeBound>,
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
  Public,
  PublicCrate,
  PublicSuper,
  PublicSelf,
  PublicIn(ExprPath),
  Private,
}

// ----------------------------------------------------------------------------
// Type System
// ----------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub enum Type {
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
    return_type: Box<Type>,
    is_variadic: bool,
  },

  Path(TypePath),

  QPath {
    self_ty: Box<Type>,
    trait_ref: Option<TypePath>,
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
pub struct BareFnParam {
  pub name: Option<String>,
  pub ty: Type,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Safety {
  Safe,
  Unsafe,
}

#[derive(Debug, Clone)]
pub struct TypePath {
  pub leading_colon: bool,
  pub segments: Vec<PathSegment>,
}

#[derive(Debug, Clone)]
pub struct PathSegment {
  pub name: String,
  pub args: Option<GenericArgs>,
}

#[derive(Debug, Clone)]
pub enum GenericArgs {
  AngleBracketed {
    args: Vec<GenericArg>,
  },
  Parenthesized {
    inputs: Vec<Type>,
    output: Option<Type>,
  },
}

#[derive(Debug, Clone)]
pub enum GenericArg {
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
pub enum Mutability {
  Mutable,
  Immutable,
}

// ----------------------------------------------------------------------------
// Statements
// ----------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub enum Stmt {
  Expr(Expr),
  Semi(Expr),
  Let {
    attributes: Vec<Attribute>,
    pattern: Pattern,
    ty: Option<Type>,
    init: Option<Expr>,
    else_block: Option<Vec<Stmt>>,
    span: Span,
  },
  Item(Box<Item>),
  Empty,
}

// ----------------------------------------------------------------------------
// Patterns
// ----------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub enum Pattern {
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
    path: ExprPath,
    span: Span,
  },

  Tuple {
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
    path: ExprPath,
    fields: Vec<FieldPattern>,
    has_rest: bool,
    span: Span,
  },

  TupleStruct {
    qself: Option<Box<Type>>,
    path: ExprPath,
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
pub struct FieldPattern {
  pub attributes: Vec<Attribute>,
  pub name: String,
  pub pattern: Option<Pattern>,
  pub is_shorthand: bool,
}

// ----------------------------------------------------------------------------
// Match Arms
// ----------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct MatchArm {
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
pub enum Expr {
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

  Path(ExprPath),

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

  Range {
    start: Option<Box<Expr>>,
    end: Option<Box<Expr>>,
    kind: RangeKind,
    span: Span,
  },

  Array {
    elements: Vec<Expr>,
    span: Span,
  },
  ArrayRepeat {
    element: Box<Expr>,
    count: Box<Expr>,
    span: Span,
  },
  Tuple {
    elements: Vec<Expr>,
    span: Span,
  },

  Struct {
    path: ExprPath,
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
pub struct FormatArg {
  pub name: Option<String>,
  pub expr: Expr,
  pub format_spec: Option<FormatSpec>,
}

#[derive(Debug, Clone)]
pub struct FormatSpec {
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
pub enum FormatAlign {
  Left,
  Center,
  Right,
}

#[derive(Debug, Clone, PartialEq)]
pub enum FormatSign {
  Plus,
  Minus,
}

#[derive(Debug, Clone)]
pub enum FormatCount {
  Integer(usize),
  Argument(String),
  Asterisk,
}

#[derive(Debug, Clone, PartialEq)]
pub enum StrKind {
  Normal,
  Raw(usize),
  Byte,
  RawByte(usize),
  C,
  RawC(usize),
}

#[derive(Debug, Clone)]
pub enum FieldAccess {
  Named(String),
  Unnamed(usize),
}

#[derive(Debug, Clone)]
pub struct AsmOperand {
  pub kind: AsmOperandKind,
  pub constraint: String,
  pub expr: Expr,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AsmOperandKind {
  In,
  Out,
  InOut,
  SplitInOut,
  Const,
  Sym,
}

#[derive(Debug, Clone)]
pub struct ExprPath {
  pub leading_colon: bool,
  pub segments: Vec<String>,
  pub turbofish: Option<GenericArgs>,
}

#[derive(Debug, Clone)]
pub struct FieldInit {
  pub attributes: Vec<Attribute>,
  pub name: String,
  pub value: Option<Expr>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum RangeKind {
  Exclusive,
  Inclusive,
  From,
  To,
  ToInclusive,
  Full,
}

#[derive(Debug, Clone, PartialEq)]
pub enum CaptureKind {
  Default,
  Move,
}

#[derive(Debug, Clone)]
pub struct ClosureParam {
  pub attributes: Vec<Attribute>,
  pub pattern: Pattern,
  pub ty: Option<Type>,
}

// ----------------------------------------------------------------------------
// Binary Operators
// ----------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BinaryOp {
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
pub enum UnaryOp {
  Neg,
  Not,
  Deref,
}

// ----------------------------------------------------------------------------

impl Expr {
  pub fn span(&self) -> Span {
    match self {
      Expr::Integer { span, .. }
      | Expr::Float { span, .. }
      | Expr::String { span, .. }
      | Expr::Char { span, .. }
      | Expr::ByteString { span, .. }
      | Expr::Byte { span, .. }
      | Expr::Bool { span, .. }
      | Expr::Binary { span, .. }
      | Expr::Unary { span, .. }
      | Expr::Assign { span, .. }
      | Expr::AssignOp { span, .. }
      | Expr::Field { span, .. }
      | Expr::MethodCall { span, .. }
      | Expr::Call { span, .. }
      | Expr::Index { span, .. }
      | Expr::Range { span, .. }
      | Expr::Array { span, .. }
      | Expr::ArrayRepeat { span, .. }
      | Expr::Tuple { span, .. }
      | Expr::Struct { span, .. }
      | Expr::If { span, .. }
      | Expr::Match { span, .. }
      | Expr::Loop { span, .. }
      | Expr::While { span, .. }
      | Expr::For { span, .. }
      | Expr::Return { span, .. }
      | Expr::Break { span, .. }
      | Expr::Continue { span, .. }
      | Expr::Yield { span, .. }
      | Expr::Become { span, .. }
      | Expr::Closure { span, .. }
      | Expr::Block { span, .. }
      | Expr::LabeledBlock { span, .. }
      | Expr::Async { span, .. }
      | Expr::Await { span, .. }
      | Expr::Try { span, .. }
      | Expr::TryBlock { span, .. }
      | Expr::Cast { span, .. }
      | Expr::Type { span, .. }
      | Expr::Let { span, .. }
      | Expr::Unsafe { span, .. }
      | Expr::Const { span, .. }
      | Expr::InlineConst { span, .. }
      | Expr::Box { span, .. }
      | Expr::Underscore { span, .. }
      | Expr::Paren { span, .. }
      | Expr::InlineAsm { span, .. }
      | Expr::FormatString { span, .. } => *span,
      Expr::Path(_) => Span::default(),
      Expr::Macro { mac } => mac.span,
    }
  }

  pub fn print_tree(&self, prefix: &str, is_last: bool) {
    let connector = if is_last { "└─>" } else { "├─>" };

    match self {
      Expr::Integer { value, suffix, .. } => {
        println!(
          "{}{} Integer: {}{}",
          prefix,
          connector,
          value,
          suffix.as_ref().map(|s| s.as_str()).unwrap_or("")
        );
      },
      Expr::Float { value, suffix, .. } => {
        println!(
          "{}{} Float: {}{}",
          prefix,
          connector,
          value,
          suffix.as_ref().map(|s| s.as_str()).unwrap_or("")
        );
      },
      Expr::String { value, kind, .. } => {
        println!("{}{} String({:?}): \"{}\"", prefix, connector, kind, value);
      },
      Expr::Char { value, .. } => {
        println!("{}{} Char: '{}'", prefix, connector, value);
      },
      Expr::Bool { value, .. } => {
        println!("{}{} Bool: {}", prefix, connector, value);
      },
      Expr::Binary {
        left, op, right, ..
      } => {
        println!("{}{} Binary {:?}", prefix, connector, op);
        let new_prefix = format!("{}{}  ", prefix, if is_last { " " } else { "│" });
        left.print_tree(&new_prefix, false);
        right.print_tree(&new_prefix, true);
      },
      Expr::Unary { op, expr, .. } => {
        println!("{}{} Unary {:?}", prefix, connector, op);
        let new_prefix = format!("{}{}  ", prefix, if is_last { " " } else { "│" });
        expr.print_tree(&new_prefix, true);
      },
      Expr::Assign { target, value, .. } => {
        println!("{}{} Assign", prefix, connector);
        let new_prefix = format!("{}{}  ", prefix, if is_last { " " } else { "│" });
        target.print_tree(&new_prefix, false);
        value.print_tree(&new_prefix, true);
      },
      Expr::Call { callee, args, .. } => {
        println!("{}{} Call", prefix, connector);
        let new_prefix = format!("{}{}  ", prefix, if is_last { " " } else { "│" });
        callee.print_tree(&new_prefix, args.is_empty());
        for (i, arg) in args.iter().enumerate() {
          arg.print_tree(&new_prefix, i == args.len() - 1);
        }
      },
      Expr::Path(path) => {
        println!(
          "{}{} Path: {}{}",
          prefix,
          connector,
          if path.leading_colon { "::" } else { "" },
          path.segments.join("::")
        );
      },
      Expr::Block { stmts, .. } => {
        println!("{}{} Block", prefix, connector);
        let new_prefix = format!("{}{}  ", prefix, if is_last { " " } else { "│" });
        for (i, stmt) in stmts.iter().enumerate() {
          stmt.print_tree(&new_prefix, i == stmts.len() - 1);
        }
      },
      _ => {
        println!("{}{} [Other Expr]", prefix, connector);
      },
    }
  }
}

impl Stmt {
  pub fn print_tree(&self, prefix: &str, is_last: bool) {
    let connector = if is_last { "└─>" } else { "├─>" };

    match self {
      Stmt::Expr(expr) => {
        println!(
          "{}{} parse_expr()  (for expression statements)",
          prefix, connector
        );
        let new_prefix = format!("{}{}  ", prefix, if is_last { " " } else { "│" });
        expr.print_tree(&new_prefix, true);
      },
      Stmt::Semi(expr) => {
        println!("{}{} Stmt::Semi", prefix, connector);
        let new_prefix = format!("{}{}  ", prefix, if is_last { " " } else { "│" });
        expr.print_tree(&new_prefix, true);
      },
      Stmt::Let {
        pattern, ty, init, ..
      } => {
        println!(
          "{}{} parse_let_statement()  (for let statements)",
          prefix, connector
        );
        let new_prefix = format!("{}{}  ", prefix, if is_last { " " } else { "│" });
        println!("{}└─> pattern: {:?}", new_prefix, pattern);
        if let Some(ty) = ty {
          println!("{}└─> type: {:?}", new_prefix, ty);
        }
        if let Some(init) = init {
          init.print_tree(&new_prefix, true);
        }
      },
      Stmt::Item(item) => {
        println!("{}{} Stmt::Item", prefix, connector);
        let new_prefix = format!("{}{}  ", prefix, if is_last { " " } else { "│" });
        item.print_tree(&new_prefix, true);
      },
      Stmt::Empty => {
        println!("{}{} Stmt::Empty", prefix, connector);
      },
    }
  }
}

impl Item {
  pub fn print_tree(&self, prefix: &str, is_last: bool) {
    let connector = if is_last { "└─>" } else { "├─>" };

    match self {
      Item::Function(fn_decl) => {
        println!("{}{} parse_fn_decl()  (for functions)", prefix, connector);
        if let Some(body) = &fn_decl.body {
          let new_prefix = format!("{}{}  ", prefix, if is_last { " " } else { "│" });
          println!(
            "{}└─> parse_block_contents()  ← USES parse_stmt HERE",
            new_prefix
          );
          let stmt_prefix = format!("{}    ", new_prefix);
          for (i, stmt) in body.iter().enumerate() {
            stmt.print_tree(&stmt_prefix, i == body.len() - 1);
          }
        }
      },
      Item::Const(const_decl) => {
        println!("{}{} Item::Const '{}'", prefix, connector, const_decl.name);
        let new_prefix = format!("{}{}  ", prefix, if is_last { " " } else { "│" });
        const_decl.value.print_tree(&new_prefix, true);
      },
      Item::Static(static_decl) => {
        println!(
          "{}{} Item::Static '{}'",
          prefix, connector, static_decl.name
        );
        let new_prefix = format!("{}{}  ", prefix, if is_last { " " } else { "│" });
        static_decl.value.print_tree(&new_prefix, true);
      },
      Item::Struct(struct_decl) => {
        println!(
          "{}{} Item::Struct '{}'",
          prefix, connector, struct_decl.name
        );
      },
      Item::Enum(enum_decl) => {
        println!("{}{} Item::Enum '{}'", prefix, connector, enum_decl.name);
      },
      _ => {
        println!("{}{} [Other Item]", prefix, connector);
      },
    }
  }
}

// Helper function to start printing from the root
pub fn print_ast_tree(items: &[Item]) {
  println!("parse_program()");
  for (i, item) in items.iter().enumerate() {
    item.print_tree("  ", i == items.len() - 1);
  }
}
