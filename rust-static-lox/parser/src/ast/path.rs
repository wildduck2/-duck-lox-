use crate::ast::generic::*;

/// A unified representation of a path used across the entire AST.
///
/// This replaces multiple specialized forms like `SimplePath`, `TypePath`, and `ExprPath`,
/// allowing consistent handling of identifiers, modules, and type references.
///
/// Examples:
/// ```rust
/// foo::bar::Baz
/// ::std::collections::HashMap<K, V>
/// self::module::Type
/// crate::utils::function
/// ```
#[derive(Debug, Clone)]
pub struct Path {
  /// Whether the path begins with a leading `::` (e.g., `::std`).
  pub leading_colon: bool,
  /// Ordered list of segments composing the path.
  pub segments: Vec<PathSegment>,
}

/// A single segment in a path, optionally carrying generic arguments.
///
/// Each segment represents one identifier in a path and may include
/// angle-bracketed or parenthesized generic arguments.
///
/// Examples:
/// ```rust
/// Vec<T>          // segment: `Vec` with `<T>`
/// Iterator<Item>  // segment: `Iterator` with `<Item>`
/// Fn(i32) -> u8   // segment: `Fn` with `(i32) -> u8`
/// ```
#[derive(Debug, Clone)]
pub struct PathSegment {
  /// The specific kind of segment (identifier, `self`, `crate`, etc.).
  pub kind: PathSegmentKind,
  /// Optional generic arguments attached to the segment.
  pub args: Option<GenericArgs>,
}

/// Enumerates the possible forms of a path segment.
///
/// Covers all syntactic path heads and special forms that can appear in Rust.
/// Examples include plain identifiers, module qualifiers, and special keywords like `crate` or `Self`.
#[derive(Debug, Clone)]
pub enum PathSegmentKind {
  /// A normal identifier segment, e.g. `foo`, `HashMap`, or `MyType`.
  Ident(String),

  /// Refers to the parent module -`super`.
  Super,

  /// Refers to the current module - `self`.
  Self_,

  /// Refers to the current crate root - `crate`.
  Crate,

  /// Refers to the crate at `$crate` in macro expansion.
  DollarCrate,

  /// Refers to the `Self` type within an `impl` or trait definition.
  SelfType,
}

// ----------------------------------------------------------------------------
// Path Types Helper functions
// ----------------------------------------------------------------------------
impl PathSegment {
  pub fn new(kind: PathSegmentKind, args: Option<GenericArgs>) -> Self {
    Self { kind, args }
  }
}

impl Path {
  pub fn new(leading_colon: bool, segments: Vec<PathSegment>) -> Self {
    Self {
      leading_colon,
      segments,
    }
  }
}
