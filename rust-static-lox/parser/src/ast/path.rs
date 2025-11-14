/* ---------------------------------------------------------------------------------------------- */
/*                                      Path Types                                                */
/* ---------------------------------------------------------------------------------------------- */

use crate::ast::GenericArgs;

/// Unified path type used throughout the AST
/// Replaces SimplePath, TypePath, and ExprPath
#[derive(Debug, Clone)]
pub struct Path {
  pub leading_colon: bool,
  pub segments: Vec<PathSegment>,
}

/// One segment of a path, optionally carrying generic arguments.
#[derive(Debug, Clone)]
pub struct PathSegment {
  pub kind: PathSegmentKind,
  pub args: Option<GenericArgs>,
}

/// Different canonical forms a path segment can represent.
#[derive(Debug, Clone)]
pub enum PathSegmentKind {
  Ident(String),
  Super,
  Self_,
  Crate,
  DollarCrate,
  SelfType, // For `Self` as a type
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
