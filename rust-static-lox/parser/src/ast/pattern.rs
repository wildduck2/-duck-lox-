use crate::ast::*;
use diagnostic::Span;

/// A complete representation of all pattern forms supported by the language.
///
/// Patterns are used in:
/// - let bindings
/// - match arms
/// - function parameters
/// - for loops
///
/// This mirrors Rusts pattern grammar and unifies all allowed pattern constructs
/// into a single enum that can be structurally matched or transformed.
///
/// Examples:
/// ```rust
/// let (x, y) = point;
/// let [a, .., b] = slice;
/// match value {
///     Some(v) => ...
///     None => ...
/// }
/// ref mut x
/// x @ Some(y)
/// ```
///
#[derive(Debug, Clone)]
pub(crate) enum Pattern {
  /// Wildcard pattern `_` that matches any value and binds nothing.
  Wildcard { span: Span },

  /// Rest pattern `..` used in slice and struct patterns.
  Rest { span: Span },

  /// Literal pattern such as numbers, strings, booleans, or chars.
  Literal { expr: Box<Expr>, span: Span },

  /// Identifier binding pattern.
  ///
  /// This form covers all binding kinds:
  /// - `x`
  /// - `mut x`
  /// - `ref x`
  /// - `ref mut x`
  /// - `x @ subpattern`
  ///
  /// `reference` represents `ref` or `ref mut`, while `mutability`
  /// represents the mutability of the binding itself.
  Ident {
    reference: bool,
    mutability: Mutability,
    name: String,
    subpattern: Option<Box<Pattern>>,
    span: Span,
  },

  /// Path pattern selecting a type or enum variant.
  ///
  /// Examples:
  /// ```rust
  /// None
  /// Some
  /// module::Variant
  /// crate::Type
  /// ```
  Path {
    qself: Option<Box<Type>>,
    path: Path,
    span: Span,
  },

  /// Tuple pattern such as `(a, b, c)`.
  ///
  /// Attributes apply to inner elements.
  Tuple {
    attributes: Vec<Attribute>,
    patterns: Vec<Pattern>,
    span: Span,
  },

  /// Slice pattern `[before..., middle?, after...]` supporting Rust-like forms:
  ///
  /// - `[a, b, c]`
  /// - `[x, ..]`
  /// - `[a, .., b]`
  /// - `[x, y, ..rest]`
  Slice {
    before: Vec<Pattern>,
    middle: Option<Box<Pattern>>,
    after: Vec<Pattern>,
    span: Span,
  },

  /// Struct pattern binding specific fields.
  ///
  /// Examples:
  /// ```rust
  /// Point { x, y }
  /// Point { x: a, y: b }
  /// Point { x, .. }
  /// ```
  Struct {
    qself: Option<Box<Type>>,
    path: Path,
    fields: Vec<FieldPattern>,
    has_rest: bool,
    span: Span,
  },

  /// Tuple-struct patterns like `Some(x)` or `Color(r, g, b)`.
  TupleStruct {
    qself: Option<Box<Type>>,
    path: Path,
    patterns: Vec<Pattern>,
    span: Span,
  },

  /// Logical OR pattern `p1 | p2 | p3`.
  Or { patterns: Vec<Pattern>, span: Span },

  /// Range pattern such as `1..5`, `a..=z`, `..end`, or `start..`.
  Range {
    start: Option<Box<Expr>>,
    end: Option<Box<Expr>>,
    kind: RangeKind,
    span: Span,
  },

  /// Reference pattern: `&p` or `&mut p`.
  Reference {
    mutability: Mutability,
    pattern: Box<Pattern>,
    span: Span,
  },

  /// Box pattern: `box pat`.
  Box { pattern: Box<Pattern>, span: Span },

  /// Macro invocation inside patterns (e.g. `m!(...)`).
  Macro { mac: MacroInvocation },

  /// Parenthesized pattern `(pat)` used for grouping.
  Paren { pattern: Box<Pattern>, span: Span },
}

/// A single field inside a struct pattern.
///
/// Each field may be:
/// - a shorthand binding: `{ x }`
/// - a renamed field: `{ x: y }`
/// - a nested pattern: `{ p: Some(v) }`
///
/// Examples:
/// ```rust
/// { x, y: renamed, z: Some(value) }
/// ```
#[derive(Debug, Clone)]
pub(crate) struct FieldPattern {
  /// Optional attributes attached to the field.
  pub attributes: Vec<Attribute>,
  /// The field name appearing in the struct.
  pub name: String,
  /// Optional nested pattern if the field is not shorthand.
  pub pattern: Option<Pattern>,
  /// True for shorthand forms like `{ x }`.
  pub is_shorthand: bool,
}
