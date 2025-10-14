pub mod diagnostic;
pub mod diagnostic_code;
pub mod formatter;

use colored::*;

use crate::{diagnostic::Diagnostic, diagnostic_code::Severity, formatter::DiagnosticFormatter};

/// Collector for all diagnostics during compilation
#[derive(Debug, Default)]
pub struct DiagnosticEngine {
  diagnostics: Vec<Diagnostic>,
  error_count: usize,
  warning_count: usize,
}

impl DiagnosticEngine {
  pub fn new() -> Self {
    Self::default()
  }

  pub fn emit(&mut self, diagnostic: Diagnostic) {
    match diagnostic.severity {
      Severity::Error => self.error_count += 1,
      Severity::Warning => self.warning_count += 1,
      _ => {},
    }
    self.diagnostics.push(diagnostic);
  }

  pub fn has_errors(&self) -> bool {
    self.error_count > 0
  }

  pub fn error_count(&self) -> usize {
    self.error_count
  }

  pub fn warning_count(&self) -> usize {
    self.warning_count
  }

  /// Print all diagnostics with colors to stdout
  pub fn print_all(&self, source_code: &str) {
    for diagnostic in &self.diagnostics {
      let formatter = DiagnosticFormatter::new(diagnostic, source_code);
      print!("{}", formatter.format());
    }

    self.print_summary();
  }

  /// Get all diagnostics as plain text (for file logging)
  pub fn format_all_plain(&self, source_code: &str) -> String {
    let mut output = String::new();

    for diagnostic in &self.diagnostics {
      let formatter = DiagnosticFormatter::new(diagnostic, source_code);
      output.push_str(&formatter.format_plain());
      output.push_str("\n");
    }

    output.push_str(&self.format_summary_plain());
    output
  }

  fn print_summary(&self) {
    if self.error_count > 0 || self.warning_count > 0 {
      println!();

      if self.has_errors() {
        println!(
          "{}: could not compile due to {} previous {}{}",
          "error".red().bold(),
          self.error_count.to_string().red().bold(),
          if self.error_count == 1 {
            "error"
          } else {
            "errors"
          },
          if self.warning_count > 0 {
            format!(
              "; {} {} emitted",
              self.warning_count.to_string().yellow().bold(),
              if self.warning_count == 1 {
                "warning"
              } else {
                "warnings"
              }
            )
          } else {
            String::new()
          }
        );
      } else if self.warning_count > 0 {
        println!(
          "{}: {} {} emitted",
          "warning".yellow().bold(),
          self.warning_count.to_string().yellow().bold(),
          if self.warning_count == 1 {
            "warning"
          } else {
            "warnings"
          }
        );
      }
    }
  }

  fn format_summary_plain(&self) -> String {
    if self.error_count > 0 || self.warning_count > 0 {
      if self.has_errors() {
        format!(
          "error: could not compile due to {} previous {}{}",
          self.error_count,
          if self.error_count == 1 {
            "error"
          } else {
            "errors"
          },
          if self.warning_count > 0 {
            format!(
              "; {} {} emitted",
              self.warning_count,
              if self.warning_count == 1 {
                "warning"
              } else {
                "warnings"
              }
            )
          } else {
            String::new()
          }
        )
      } else {
        format!(
          "warning: {} {} emitted",
          self.warning_count,
          if self.warning_count == 1 {
            "warning"
          } else {
            "warnings"
          }
        )
      }
    } else {
      String::new()
    }
  }

  pub fn get_diagnostics(&self) -> &[Diagnostic] {
    &self.diagnostics
  }
}
