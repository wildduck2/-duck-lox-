#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Severity {
  Error,
  Warning,
  Note,
  Help,
}

pub mod error;
pub mod warning;
