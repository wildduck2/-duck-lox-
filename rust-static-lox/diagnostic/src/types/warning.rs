use crate::types::Severity;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiagnosticWarning {
  UnusedVariable,
  InvalidConstDeclaration,

  // parser
  EmptyChar,
}

impl DiagnosticWarning {
  pub fn code(&self) -> &str {
    match self {
      Self::UnusedVariable => "W0001",
      Self::InvalidConstDeclaration => "W0002",
      Self::EmptyChar => "W0003",
    }
  }
  pub fn severity(&self) -> Severity {
    Severity::Warning
  }
}
