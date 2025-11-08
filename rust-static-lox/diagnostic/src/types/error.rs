use crate::types::Severity;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiagnosticError {
  CodeNotFound,
  InvalidArguments,
  InvalidShebang,
  InvalidCharacter,
  UnterminatedString,
  TooManyRawStrHashes,
  InvalidStringStart,
  InvalidEscape,
  UnknownPrefix,
  ReservedPrefix,
  InvalidLifetime,
}

impl DiagnosticError {
  pub fn code(&self) -> &str {
    match self {
      Self::CodeNotFound => "E0001",
      Self::InvalidArguments => "E0002",
      Self::InvalidShebang => "E0003",
      Self::InvalidCharacter => "E0004",
      Self::UnterminatedString => "E0005",
      Self::TooManyRawStrHashes => "E0006",
      Self::InvalidStringStart => "E0007",
      Self::InvalidEscape => "E0008",
      Self::UnknownPrefix => "E0009",
      Self::ReservedPrefix => "E0010",
      Self::InvalidLifetime => "E0011",
    }
  }

  pub fn severity(&self) -> Severity {
    Severity::Error
  }
}
