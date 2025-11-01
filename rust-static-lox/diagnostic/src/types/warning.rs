use crate::types::Severity;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiagnosticWarning {
  UnusedVariable,
  InvalidConstDeclaration,
}

impl DiagnosticWarning {
  pub fn code(&self) -> &str {
    match self {
      Self::UnusedVariable => "W0001",
      Self::InvalidConstDeclaration => "W0002",
    }
  }
  pub fn severity(&self) -> Severity {
    Severity::Warning
  }
}
