use crate::types::Severity;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiagnosticError {
  CodeNotFound,
  TraitNotSatisfied,
  MismatchedTypes,
  UndefinedVariable,
  BorrowCheckerViolation,
  InvalidCharacter,
  InvalidArguments,
  UnterminatedString,
  UnexpectedToken,
  MissingClosingParen,
  WrongNumberOfArguments,
  MissingClosingBracket,
  EmptyMatch,
}

impl DiagnosticError {
  pub fn code(&self) -> &str {
    match self {
      Self::CodeNotFound => "E0001",
      Self::TraitNotSatisfied => "E0277",
      Self::MismatchedTypes => "E0308",
      Self::UndefinedVariable => "E0425",
      Self::BorrowCheckerViolation => "E0502",
      Self::InvalidCharacter => "E0601",
      Self::InvalidArguments => "E0602",
      Self::UnterminatedString => "E0603",
      Self::UnexpectedToken => "E0604",
      Self::MissingClosingParen => "E0605",
      Self::WrongNumberOfArguments => "E0606",
      Self::MissingClosingBracket => "E0607",
      Self::EmptyMatch => "E0608",
    }
  }

  pub fn severity(&self) -> Severity {
    Severity::Error
  }
}
