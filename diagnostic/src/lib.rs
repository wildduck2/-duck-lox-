use colored::*;

/// Represents a source code location
#[derive(Debug, Clone, PartialEq)]
pub struct Span {
  pub file: String,
  pub line: usize,
  pub column: usize,
  pub length: usize,
}

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
  UndeclaredVariable,
  TypeMismatch,
  DuplicateDeclaration,
  InvalidAssignment,
  InvalidOperator,
  InvalidFunctionCall,
  WrongNumberOfArguments,
  CannotInferType,
  RecursiveType,
  FileNotFound,
  InvalidArguments,
  IoError,
  UnusedVariable,
  UnreachableCode,
  ImplicitConversion,
  ShadowedVariable,
}

impl DiagnosticCode {
  pub fn code(&self) -> String {
    match self {
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
      Self::UndeclaredVariable => "E0200".to_string(),
      Self::TypeMismatch => "E0201".to_string(),
      Self::DuplicateDeclaration => "E0202".to_string(),
      Self::InvalidAssignment => "E0203".to_string(),
      Self::InvalidOperator => "E0204".to_string(),
      Self::InvalidFunctionCall => "E0205".to_string(),
      Self::WrongNumberOfArguments => "E0206".to_string(),
      Self::CannotInferType => "E0300".to_string(),
      Self::RecursiveType => "E0301".to_string(),
      Self::FileNotFound => "E0400".to_string(),
      Self::InvalidArguments => "E0401".to_string(),
      Self::IoError => "E0402".to_string(),
      Self::UnusedVariable => "W0001".to_string(),
      Self::UnreachableCode => "W0002".to_string(),
      Self::ImplicitConversion => "W0003".to_string(),
      Self::ShadowedVariable => "W0004".to_string(),
    }
  }

  pub fn severity(&self) -> Severity {
    match self {
      Self::UnusedVariable | Self::UnreachableCode | Self::ImplicitConversion => Severity::Warning,
      _ => Severity::Error,
    }
  }
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

/// Formats diagnostics like rustc with colored crate
pub struct DiagnosticFormatter<'a> {
  diagnostic: &'a Diagnostic,
  source_lines: Vec<String>,
}

impl<'a> DiagnosticFormatter<'a> {
  pub fn new(diagnostic: &'a Diagnostic, source_code: &str) -> Self {
    let source_lines: Vec<String> = source_code.lines().map(|s| s.to_string()).collect();
    Self {
      diagnostic,
      source_lines,
    }
  }

  fn severity_text(&self) -> &'static str {
    match self.diagnostic.severity {
      Severity::Error => "error",
      Severity::Warning => "warning",
      Severity::Note => "note",
      Severity::Help => "help",
    }
  }

  fn underline_char(&self, style: LabelStyle) -> char {
    match style {
      LabelStyle::Primary => '^',
      LabelStyle::Secondary => '-',
    }
  }

  fn get_line_content(&self, line_num: usize) -> Option<&str> {
    if line_num == 0 && self.source_lines.is_empty() {
      return None;
    }
    let index = if line_num == 0 { 0 } else { line_num - 1 };
    self.source_lines.get(index).map(|s| s.as_str())
  }

  pub fn format(&self) -> String {
    let mut output = String::new();

    // Header: error[E0200]: message
    let header = match self.diagnostic.severity {
      Severity::Error => {
        format!(
          "{}: [{}]: {}",
          self.severity_text().red().bold(),
          self.diagnostic.code.code().red().bold(),
          self.diagnostic.message
        )
      },
      Severity::Warning => {
        format!(
          "{}: [{}]: {}",
          self.severity_text().yellow().bold(),
          self.diagnostic.code.code().yellow().bold(),
          self.diagnostic.message
        )
      },
      _ => {
        format!(
          "{}: [{}]: {}",
          self.severity_text().cyan().bold(),
          self.diagnostic.code.code().cyan().bold(),
          self.diagnostic.message
        )
      },
    };
    output.push_str(&header);
    output.push_str("\n");

    // Format each label with source code
    for label in &self.diagnostic.labels {
      // Location line: --> file:line:column
      output.push_str(&format!(
        "  {} {}:{}:{}\n",
        "-->".blue().bold(),
        label.span.file.white().bold(),
        label.span.line.to_string().white().bold(),
        label.span.column.to_string().white().bold()
      ));

      // Empty line with just the gutter
      output.push_str(&format!("   {}\n", "|".blue().bold()));

      // Get the source line
      if let Some(line_content) = self.get_line_content(label.span.line) {
        let line_num = label.span.line;

        // Line number and content
        output.push_str(&format!(
          " {} {} {}\n",
          format!("{}", line_num).blue().bold(),
          "|".blue().bold(),
          line_content
        ));

        // Underline with carets/dashes
        let underline_char = self.underline_char(label.style);
        let start_col = label.span.column;
        let length = label.span.length;

        let padding = " ".repeat(start_col);
        let underline = underline_char.to_string().repeat(length);

        let colored_underline = match (self.diagnostic.severity, label.style) {
          (Severity::Error, LabelStyle::Primary) => underline.red().bold(),
          (Severity::Warning, LabelStyle::Primary) => underline.yellow().bold(),
          (_, LabelStyle::Secondary) => underline.cyan().bold(),
          _ => underline.cyan().bold(),
        };

        output.push_str(&format!(
          "   {} {}{}\n",
          "|".blue().bold(),
          padding,
          colored_underline
        ));

        // Label message below the underline
        if let Some(msg) = &label.message {
          let colored_msg = match (self.diagnostic.severity, label.style) {
            (Severity::Error, LabelStyle::Primary) => msg.red().bold(),
            (Severity::Warning, LabelStyle::Primary) => msg.yellow().bold(),
            (_, LabelStyle::Secondary) => msg.cyan().bold(),
            _ => msg.cyan().bold(),
          };

          output.push_str(&format!(
            "   {} {}{}\n",
            "|".blue().bold(),
            padding,
            colored_msg
          ));
        }
      }

      // Empty line after each label
      output.push_str(&format!("   {}\n", "|".blue().bold()));
    }

    // Notes
    for note in &self.diagnostic.notes {
      output.push_str(&format!(
        "   {} {}: {}\n",
        "=".blue().bold(),
        "note".cyan().bold(),
        note
      ));
    }

    // Help
    if let Some(help) = &self.diagnostic.help {
      output.push_str(&format!(
        "   {} {}: {}\n",
        "=".blue().bold(),
        "help".cyan().bold(),
        help
      ));
    }

    output
  }

  /// Format without colors for logging to file
  pub fn format_plain(&self) -> String {
    let mut output = String::new();

    // Header
    output.push_str(&format!(
      "{}: [{}]: {}\n",
      self.severity_text(),
      self.diagnostic.code.code(),
      self.diagnostic.message
    ));

    // Format each label
    for label in &self.diagnostic.labels {
      output.push_str(&format!(
        "  --> {}:{}:{}\n",
        label.span.file, label.span.line, label.span.column
      ));

      output.push_str("   |\n");

      if let Some(line_content) = self.get_line_content(label.span.line) {
        let line_num = label.span.line;

        output.push_str(&format!(" {:>3} | {}\n", line_num, line_content));

        let underline_char = self.underline_char(label.style);
        let start_col = label.span.column.saturating_sub(1);
        let length = label.span.length.max(1);

        let padding = " ".repeat(start_col);
        let underline = underline_char.to_string().repeat(length);

        output.push_str(&format!("   | {}{}\n", padding, underline));

        if let Some(msg) = &label.message {
          output.push_str(&format!("   | {}{}\n", padding, msg));
        }
      }

      output.push_str("   |\n");
    }

    // Notes
    for note in &self.diagnostic.notes {
      output.push_str(&format!("   = note: {}\n", note));
    }

    // Help
    if let Some(help) = &self.diagnostic.help {
      output.push_str(&format!("   = help: {}\n", help));
    }

    output
  }
}

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

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_diagnostic_formatting() {
    let source = r#"var b = "asdfasdf"#;

    let mut engine = DiagnosticEngine::new();

    let error = Diagnostic::new(
      DiagnosticCode::UnterminatedString,
      "wrong string syntax".to_string(),
    )
    .with_label(Label::primary(
      Span {
        file: "input".to_string(),
        line: 0,
        column: 18,
        length: 7,
      },
      Some("newline not allowed in string".to_string()),
    ))
    .with_help("ensure strings are properly closed on the same line".to_string());

    engine.emit(error);
    println!("{}", engine.format_all_plain(source));
  }
}
