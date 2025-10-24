use crate::diagnostic_code::{DiagnosticCode, Severity};

/// Represents a source code location
#[derive(Debug, Clone, PartialEq)]
pub struct Span {
  pub file: String,
  pub line: usize,
  pub column: usize,
  pub length: usize,
}

/// Label for underlining specific parts of code
#[derive(Debug, Clone)]
pub struct Label {
  pub span: Span,
  pub message: Option<String>,
  pub style: LabelStyle,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LabelStyle {
  Primary,
  Secondary,
}

impl Label {
  pub fn primary(span: Span, message: Option<String>) -> Self {
    Self {
      span,
      message,
      style: LabelStyle::Primary,
    }
  }

  pub fn secondary(span: Span, message: Option<String>) -> Self {
    Self {
      span,
      message,
      style: LabelStyle::Secondary,
    }
  }
}

/// A single diagnostic message
#[derive(Debug, Clone)]
pub struct Diagnostic {
  pub code: DiagnosticCode,
  pub severity: Severity,
  pub message: String,
  pub labels: Vec<Label>,
  pub notes: Vec<String>,
  pub help: Option<String>,
}

impl Diagnostic {
  pub fn new(code: DiagnosticCode, message: String) -> Self {
    Self {
      severity: code.severity(),
      code,
      message,
      labels: Vec::new(),
      notes: Vec::new(),
      help: None,
    }
  }

  pub fn with_label(mut self, label: Label) -> Self {
    self.labels.push(label);
    self
  }

  pub fn with_note(mut self, note: String) -> Self {
    self.notes.push(note);
    self
  }

  pub fn with_help(mut self, help: String) -> Self {
    self.help = Some(help);
    self
  }
}
