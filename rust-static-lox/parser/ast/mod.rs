// ----------------------------------------------------------------------------
// Top-level Items
// ----------------------------------------------------------------------------

use diagnostic::diagnostic::Span;

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
  ExternCrate(ExternCrateDecl), // extern crate foo;
  Macro(MacroDecl),             // macro_rules! foo { ... }
  ForeignMod(ForeignModDecl),   // extern "C" { ... }
  Union(UnionDecl),             // union Foo { ... }
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
  pub items: Option<Vec<Item>>, // None for mod foo; (external module)
  pub is_unsafe: bool,          // unsafe mod
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
  Name(String), // use std::collections::HashMap
  Rename {
    name: String,
    alias: String,
  }, // use foo as bar
  Glob,         // use foo::*
  List(Vec<UseTree>), // use foo::{a, b, c}
}

#[derive(Debug, Clone)]
pub struct ExternCrateDecl {
  pub visibility: Visibility,
  pub name: String,
  pub alias: Option<String>, // extern crate foo as bar;
  pub span: Span,
}

#[derive(Debug, Clone)]
pub struct MacroDecl {
  pub name: String,
  pub rules: Vec<MacroRule>,
  pub span: Span,
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
    kind: String, // ident, expr, ty, etc.
  },
}

#[derive(Debug, Clone, PartialEq)]
pub enum Delimiter {
  Parenthesis, // ( )
  Brace,       // { }
  Bracket,     // [ ]
}

#[derive(Debug, Clone, PartialEq)]
pub enum RepeatKind {
  ZeroOrMore, // *
  OneOrMore,  // +
  ZeroOrOne,  // ?
}

#[derive(Debug, Clone)]
pub struct ForeignModDecl {
  pub abi: String, // "C", "system", etc.
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
    is_variadic: bool, // for C varargs
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
  pub is_extern: bool,     // extern fn
  pub abi: Option<String>, // extern "C" fn
  pub span: Span,
}

#[derive(Debug, Clone)]
pub struct Param {
  pub attributes: Vec<Attribute>, // #[attr] on parameters
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
  pub path: ExprPath,
  pub tokens: Vec<TokenTree>,
  pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AttrStyle {
  Outer, // #[...]
  Inner, // #![...]
}

// ----------------------------------------------------------------------------
// Struct Declaration
// ----------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct StructDecl {
  pub attributes: Vec<Attribute>, // #[derive(Debug)]
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
  Tuple(Vec<TupleField>), // Now includes visibility per field
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
  pub is_auto: bool,   // auto trait
  pub is_unsafe: bool, // unsafe trait
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
  pub is_default: bool, // default impl (specialization)
  pub generics: Option<GenericParams>,
  pub polarity: ImplPolarity, // impl !Trait (negative impl)
  pub trait_ref: Option<TypePath>,
  pub self_ty: Type,
  pub items: Vec<ImplItem>,
  pub where_clause: Option<WhereClause>,
  pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ImplPolarity {
  Positive,
  Negative, // impl !Trait
}

#[derive(Debug, Clone)]
pub enum ImplItem {
  Method(FnDecl),
  Type {
    attributes: Vec<Attribute>,
    visibility: Visibility,
    name: String,
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
    default: Option<Expr>, // const N: usize = 10
  },
}

#[derive(Debug, Clone)]
pub struct TypeBound {
  pub modifier: TraitBoundModifier, // ? in ?Sized
  pub path: TypePath,
  pub generics: Option<Vec<Type>>,
  pub for_lifetimes: Option<Vec<String>>, // for<'a>
}

#[derive(Debug, Clone, PartialEq)]
pub enum TraitBoundModifier {
  None,
  Maybe,      // ?Trait
  MaybeConst, // ?const Trait
}

#[derive(Debug, Clone)]
pub struct WhereClause {
  pub predicates: Vec<WherePredicate>,
}

#[derive(Debug, Clone)]
pub enum WherePredicate {
  Type {
    for_lifetimes: Option<Vec<String>>, // for<'a>
    ty: Type,
    bounds: Vec<TypeBound>,
  },
  Lifetime {
    lifetime: String,
    bounds: Vec<String>,
  },
  Equality {
    // Associated type binding: where T::Item = String
    ty: Type,
    equals: Type,
  },
}

// ----------------------------------------------------------------------------
// Visibility
// ----------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq)]
pub enum Visibility {
  Public,
  PublicCrate,
  PublicSuper,
  PublicSelf, // pub(self)
  PublicIn(Vec<String>),
  Private,
}

// ----------------------------------------------------------------------------
// Type System
// ----------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub enum Type {
  // Primitives
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
  Str,   // str (unsized)
  Never, // !

  // Compound types
  Array {
    element: Box<Type>,
    size: Box<Expr>, // Changed to Expr for const expressions
  },
  Slice(Box<Type>),
  Tuple(Vec<Type>),

  // References
  Reference {
    lifetime: Option<String>,
    mutability: Mutability,
    inner: Box<Type>,
  },

  // Pointers
  RawPointer {
    mutability: Mutability,
    inner: Box<Type>,
  },

  // Functions
  BareFn {
    for_lifetimes: Option<Vec<String>>, // for<'a>
    safety: Safety,                     // unsafe
    abi: Option<String>,                // extern "C"
    params: Vec<Type>,
    return_type: Box<Type>,
    is_variadic: bool, // C varargs
  },

  // Named types
  Path(TypePath),

  // Qualified paths
  QPath {
    self_ty: Box<Type>,
    trait_ref: Option<TypePath>,
    name: String,
  }, // <T as Trait>::AssocType

  // Trait objects
  TraitObject {
    bounds: Vec<TypeBound>,
    lifetime: Option<String>,
    is_dyn: bool, // dyn Trait vs Trait (legacy)
  },

  // Implementation trait
  ImplTrait(Vec<TypeBound>),

  // Type inference placeholder
  Infer,

  // Parenthesized type
  Paren(Box<Type>), // (T)

  // Macro invocation in type position
  Macro(MacroInvocation),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Safety {
  Safe,
  Unsafe,
}

#[derive(Debug, Clone)]
pub struct TypePath {
  pub leading_colon: bool, // ::std::vec::Vec
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
    output: Option<Type>, // Fn(A, B) -> C
  },
}

#[derive(Debug, Clone)]
pub enum GenericArg {
  Lifetime(String),
  Type(Type),
  Const(Expr),
  Binding {
    // AssocType = Foo
    name: String,
    ty: Type,
  },
  Constraint {
    // AssocType: Bound
    name: String,
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
    else_block: Option<Vec<Stmt>>, // let-else: let Some(x) = y else { return };
    span: Span,
  },
  Item(Box<Item>),
  Empty, // ;
}

// ----------------------------------------------------------------------------
// Expressions
// ----------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub enum Expr {
  // Literals
  Integer {
    value: i128, // Changed to i128 for full range
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
    kind: StrKind, // regular, raw, byte
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

  // Paths
  Path(ExprPath),

  // Operations
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

  // Assignment
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

  // Field access
  Field {
    object: Box<Expr>,
    field: FieldAccess,
    span: Span,
  },

  // Method call
  MethodCall {
    receiver: Box<Expr>,
    method: String,
    turbofish: Option<GenericArgs>,
    args: Vec<Expr>,
    span: Span,
  },

  // Function call
  Call {
    callee: Box<Expr>,
    args: Vec<Expr>,
    span: Span,
  },

  // Indexing
  Index {
    object: Box<Expr>,
    index: Box<Expr>,
    span: Span,
  },

  // Range expressions
  Range {
    start: Option<Box<Expr>>,
    end: Option<Box<Expr>>,
    kind: RangeKind,
    span: Span,
  },

  // Collections
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

  // Struct literal
  Struct {
    path: ExprPath,
    fields: Vec<FieldInit>,
    base: Option<Box<Expr>>,
    span: Span,
  },

  // Control flow
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

  // Returns and breaks
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
    // for generators
    value: Option<Box<Expr>>,
    span: Span,
  },

  // Closures
  Closure {
    capture: CaptureKind,
    is_async: bool, // async closures
    is_move: bool,  // explicit move
    params: Vec<ClosureParam>,
    return_type: Option<Type>,
    body: Box<Expr>,
    span: Span,
  },

  // Block
  Block {
    attributes: Vec<Attribute>,
    stmts: Vec<Stmt>,
    label: Option<String>,
    is_unsafe: bool,
    span: Span,
  },

  // Async/await
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

  // Try operator
  Try {
    expr: Box<Expr>,
    span: Span,
  },

  // Type operations
  Cast {
    expr: Box<Expr>,
    ty: Type,
    span: Span,
  },
  Type {
    // Type ascription (unstable)
    expr: Box<Expr>,
    ty: Type,
    span: Span,
  },

  // Let expression (unstable)
  Let {
    pattern: Pattern,
    expr: Box<Expr>,
    span: Span,
  },

  // Unsafe
  Unsafe {
    block: Vec<Stmt>,
    span: Span,
  },

  // Const block
  Const {
    block: Vec<Stmt>,
    span: Span,
  },

  // Box expression (placement new)
  Box {
    expr: Box<Expr>,
    span: Span,
  },

  // Underscore expression (type inference in expressions)
  Underscore {
    span: Span,
  },

  // Macro invocation
  Macro {
    mac: MacroInvocation,
  },

  // Grouped expression (parentheses)
  Paren {
    expr: Box<Expr>,
    span: Span,
  },

  // Inline assembly
  InlineAsm {
    template: String,
    operands: Vec<AsmOperand>,
    options: Vec<String>,
    span: Span,
  },
}

#[derive(Debug, Clone, PartialEq)]
pub enum StrKind {
  Normal,
  Raw(usize),     // r#"..."# (number of hashes)
  Byte,           // b"..."
  RawByte(usize), // br#"..."#
}

#[derive(Debug, Clone)]
pub enum FieldAccess {
  Named(String),  // .field
  Unnamed(usize), // .0 (tuple field)
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
  pub turbofish: Option<GenericArgs>, // For paths with generics
}

#[derive(Debug, Clone)]
pub struct FieldInit {
  pub attributes: Vec<Attribute>,
  pub name: String,
  pub value: Option<Expr>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum RangeKind {
  Exclusive,   // start..end
  Inclusive,   // start..=end
  From,        // start..
  To,          // ..end
  ToInclusive, // ..=end
  Full,        // ..
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
  // Arithmetic
  Add,
  Sub,
  Mul,
  Div,
  Mod,

  // Bitwise
  BitAnd,
  BitOr,
  BitXor,
  Shl,
  Shr,

  // Comparison
  Eq,
  NotEq,
  Less,
  LessEq,
  Greater,
  GreaterEq,

  // Logical
  And,
  Or,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UnaryOp {
  Neg,   // -
  Not,   // !
  Deref, // * (moved from separate node)
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
    reference: Option<Mutability>, // ref or ref mut
    mutability: Mutability,
    name: String,
    subpattern: Option<Box<Pattern>>,
    span: Span,
  },

  Path {
    qself: Option<Box<Type>>, // <T as Trait>::CONST
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
  pub is_shorthand: bool, // true for { x } vs { x: x }
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
  pub comma: bool, // trailing comma affects whether body needs braces
  pub span: Span,
}
