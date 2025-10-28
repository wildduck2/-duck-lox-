use crate::types::Severity;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiagnosticWarning {
  UnusedVariable,
}

impl DiagnosticWarning {
  pub fn code(&self) -> &str {
    match self {
      Self::UnusedVariable => "W0001",
    }
  }
  pub fn severity(&self) -> Severity {
    match self {
      Self::UnusedVariable => Severity::Warning,
    }
  }
}
