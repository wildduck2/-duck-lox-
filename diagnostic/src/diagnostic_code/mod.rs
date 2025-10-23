/// Severity level of a diagnostic
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Severity {
  Error,
  Warning,
  Note,
  Help,
}

/// Unique identifier for each type of diagnostic
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DiagnosticCode {
  // Errors
  UnterminatedString,
  InvalidCharacter,
  InvalidNumber,
  UnexpectedEof,
  UnexpectedToken,
  ExpectedExpression,
  MissingClosingBrace,
  MissingClosingParen,
  MissingSemicolon,
  InvalidAssignmentTarget,
  ExpectedIdentifier,

  ContinueOutsideLoop,
  BreakOutsideLoop,
  UndeclaredVariable,
  TypeMismatch,
  DuplicateDeclaration,
  InvalidAssignment,
  InvalidOperator,
  ReturnNotInFunction,
  InvalidFunctionCall,
  WrongNumberOfArguments,
  EccededNumberOfArguments,
  CannotInferType,
  RecursiveType,
  FileNotFound,
  InvalidArguments,
  IoError,
  InvalidUnaryOperator,
  TypeError,
  DivisionByZero,
  ExpectedToken,

  // Warning
  UnusedVariable,
  UnreachableCode,
  ImplicitConversion,
}

impl DiagnosticCode {
  pub fn code(&self) -> String {
    match self {
      // Errors
      Self::UnterminatedString => "E0001".to_string(),
      Self::InvalidCharacter => "E0002".to_string(),
      Self::InvalidNumber => "E0003".to_string(),
      Self::UnexpectedEof => "E0004".to_string(),
      Self::UnexpectedToken => "E0100".to_string(),
      Self::ExpectedExpression => "E0101".to_string(),
      Self::MissingClosingBrace => "E0102".to_string(),
      Self::MissingClosingParen => "E0103".to_string(),
      Self::MissingSemicolon => "E0104".to_string(),
      Self::InvalidAssignmentTarget => "E0105".to_string(),
      Self::ExpectedIdentifier => "E0106".to_string(),
      Self::ContinueOutsideLoop => "E0200".to_string(),
      Self::BreakOutsideLoop => "E0201".to_string(),
      Self::UndeclaredVariable => "E0200".to_string(),
      Self::TypeMismatch => "E0201".to_string(),
      Self::DuplicateDeclaration => "E0202".to_string(),
      Self::InvalidAssignment => "E0203".to_string(),
      Self::InvalidOperator => "E0204".to_string(),
      Self::ReturnNotInFunction => "E0205".to_string(),
      Self::InvalidFunctionCall => "E0205".to_string(),
      Self::WrongNumberOfArguments => "E0206".to_string(),
      Self::EccededNumberOfArguments => "E0206".to_string(),
      Self::TypeError => "E0207".to_string(),
      Self::DivisionByZero => "E0208".to_string(),
      Self::CannotInferType => "E0300".to_string(),
      Self::RecursiveType => "E0301".to_string(),
      Self::FileNotFound => "E0400".to_string(),
      Self::InvalidArguments => "E0401".to_string(),
      Self::IoError => "E0402".to_string(),
      Self::InvalidUnaryOperator => "E0403".to_string(),
      Self::ExpectedToken => "E0105".to_string(), // assign a unique code or reuse MissingSemicolon code if appropriate

      // Warnings
      Self::UnusedVariable => "W0001".to_string(),
      Self::UnreachableCode => "W0002".to_string(),
      Self::ImplicitConversion => "W0003".to_string(),
    }
  }

  pub fn severity(&self) -> Severity {
    match self {
      Self::UnusedVariable | Self::UnreachableCode | Self::ImplicitConversion => Severity::Warning,
      _ => Severity::Error,
    }
  }
}
