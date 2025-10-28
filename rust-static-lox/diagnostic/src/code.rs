use crate::types::{error::DiagnosticError, warning::DiagnosticWarning, Severity};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiagnosticCode {
  Error(DiagnosticError),
  Warning(DiagnosticWarning),
}

impl DiagnosticCode {
  pub fn code(&self) -> &str {
    match self {
      Self::Error(error) => error.code(),
      Self::Warning(warning) => warning.code(),
    }
  }

  pub fn severity(&self) -> Severity {
    match self {
      Self::Error(error) => error.severity(),
      Self::Warning(warning) => warning.severity(),
    }
  }
}
