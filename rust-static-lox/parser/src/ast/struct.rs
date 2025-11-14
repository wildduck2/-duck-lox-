use crate::ast::*;

/// A struct declaration - supports all Rust struct forms: record, tuple, and unit.
///
/// Captures the full metadata of a `struct` definition, including attributes,
/// visibility, generics, fields, and optional `where` clauses.
///
/// Examples:
/// ```rust
/// struct Point<T> { x: T, y: T }     // named fields
/// struct Color(u8, u8, u8);          // tuple fields
/// struct Marker;                     // unit struct
/// ```
#[derive(Debug, Clone)]
pub(crate) struct StructDecl {
  /// Outer attributes attached to the struct.
  pub attributes: Vec<Attribute>,
  /// The visibility specifier (`pub`, `pub(crate)`, etc.).
  pub visibility: Visibility,
  /// The struct’s name identifier.
  pub name: String,
  /// Optional generic parameters (`<'a, T, const N: usize>`).
  pub generics: Option<GenericParams>,
  /// The specific structural form of the struct (named, tuple, or unit).
  pub kind: StructKind,
  /// Optional `where` clause with additional bounds.
  pub where_clause: Option<WhereClause>,
  /// Source span covering the entire struct declaration.
  pub span: Span,
}

/// The structural form of a struct - named, tuple, or unit.
///
/// Mirrors the three possible syntactic shapes of Rust structs.
#[derive(Debug, Clone)]
pub(crate) enum StructKind {
  /// A struct with named fields, e.g.:
  /// ```rust
  /// struct Point { x: i32, y: i32 }
  /// ```
  Named { fields: Vec<FieldDecl> },

  /// A tuple struct, e.g.:
  /// ```rust
  /// struct Color(u8, u8, u8);
  /// ```
  Tuple(Vec<TupleField>),

  /// A unit struct, e.g.:
  /// ```rust
  /// struct Marker;
  /// ```
  Unit,
}

/// A single unnamed field in a tuple struct.
#[derive(Debug, Clone)]
pub(crate) struct TupleField {
  /// Outer attributes attached to the field.
  pub attributes: Vec<Attribute>,
  /// Field visibility (e.g., `pub` or private).
  pub visibility: Visibility,
  /// Field type (e.g., `u8`, `String`, etc.).
  pub ty: Type,
  /// Source span for the field.
  pub span: Span,
}

/// A single named field in a regular struct.
#[derive(Debug, Clone)]
pub(crate) struct FieldDecl {
  /// Outer attributes attached to the field.
  pub attributes: Vec<Attribute>,
  /// Field visibility (e.g., `pub`, `pub(crate)`).
  pub visibility: Visibility,
  /// Field name identifier.
  pub name: String,
  /// Field type (e.g., `i32`, `&'a str`, `Vec<T>`).
  pub ty: Type,
  /// Source span for the field.
  pub span: Span,
}
