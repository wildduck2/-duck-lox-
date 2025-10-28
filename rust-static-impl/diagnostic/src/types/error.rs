use crate::types::Severity;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiagnosticError {
  CodeNotFound,
  TraitNotSatisfied,
  MismatchedTypes,
  UndefinedVariable,
  BorrowCheckerViolation,
}

impl DiagnosticError {
  pub fn code(&self) -> &str {
    match self {
      Self::CodeNotFound => "E0001",
      Self::TraitNotSatisfied => "E0277",
      Self::MismatchedTypes => "E0308",
      Self::UndefinedVariable => "E0425",
      Self::BorrowCheckerViolation => "E0502",
    }
  }

  pub fn severity(&self) -> Severity {
    Severity::Error
  }
}
