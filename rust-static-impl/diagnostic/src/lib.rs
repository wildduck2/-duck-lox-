use crate::{
  code::DiagnosticCode,
  diagnostic::{Diagnostic, Span},
};

pub mod code;
pub mod diagnostic;
pub mod types;

#[derive(Debug, Default)]
pub struct DiagnosticEngine<'a> {
  source: &'a str,
  diagnostics: Vec<Diagnostic<'a>>,
  error_count: usize,
  warning_count: usize,
}

impl<'a> DiagnosticEngine<'a> {
  pub fn new() -> Self {
    Self::default()
  }

  pub fn insert_source(&mut self, source: &'a str) {
    self.source = source;
  }

  pub fn add(&mut self, diagnostic: Diagnostic<'a>) {
    self.diagnostics.push(diagnostic);
  }

  pub fn print_diagnostics(&self) {
    for diagnostic in &self.diagnostics {
      diagnostic.print();
    }
  }

  pub fn error_count(&self) -> usize {
    self.error_count
  }

  pub fn warning_count(&self) -> usize {
    self.warning_count
  }
}
