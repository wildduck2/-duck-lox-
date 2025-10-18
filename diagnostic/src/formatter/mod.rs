use colored::*;

use crate::{
  diagnostic::{Diagnostic, LabelStyle},
  diagnostic_code::Severity,
};

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

    // Get the primary label (first one, usually)
    if let Some(primary_label) = self.diagnostic.labels.first() {
      // Location line: --> file:line:column
      output.push_str(&format!(
        "  {} {}:{}:{}\n",
        "-->".blue().bold(),
        primary_label.span.file.white().bold(),
        primary_label.span.line.to_string().white().bold(),
        primary_label.span.column.to_string().white().bold()
      ));

      // Empty line with just the gutter
      output.push_str(&format!("   {}\n", "|".blue().bold()));

      // Get the source line
      if let Some(line_content) = self.get_line_content(primary_label.span.line) {
        let line_num = primary_label.span.line;

        // Line number and content
        output.push_str(&format!(
          " {} {} {}\n",
          format!("{}", line_num).blue().bold(),
          "|".blue().bold(),
          line_content
        ));

        // Now render ALL labels for this line
        for label in &self.diagnostic.labels {
          // Only show labels that are on the same line
          if label.span.line == line_num {
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

            // Combine underline and message on the same line
            if let Some(msg) = &label.message {
              let colored_msg = match (self.diagnostic.severity, label.style) {
                (Severity::Error, LabelStyle::Primary) => msg.red().bold(),
                (Severity::Warning, LabelStyle::Primary) => msg.yellow().bold(),
                (_, LabelStyle::Secondary) => msg.cyan().bold(),
                _ => msg.cyan().bold(),
              };

              output.push_str(&format!(
                "   {} {}{} {}\n",
                "|".blue().bold(),
                padding,
                colored_underline,
                colored_msg
              ));
            } else {
              output.push_str(&format!(
                "   {} {}{}\n",
                "|".blue().bold(),
                padding,
                colored_underline
              ));
            }
          }
        }
      }

      // Empty line after all labels
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

    // Get the primary label
    if let Some(primary_label) = self.diagnostic.labels.first() {
      output.push_str(&format!(
        "  --> {}:{}:{}\n",
        primary_label.span.file, primary_label.span.line, primary_label.span.column
      ));

      output.push_str("   |\n");

      if let Some(line_content) = self.get_line_content(primary_label.span.line) {
        let line_num = primary_label.span.line;

        output.push_str(&format!(" {:>3} | {}\n", line_num, line_content));

        // Render all labels for this line
        for label in &self.diagnostic.labels {
          if label.span.line == line_num {
            let underline_char = self.underline_char(label.style);
            let start_col = label.span.column.saturating_sub(1);
            let length = label.span.length.max(1);

            let padding = " ".repeat(start_col);
            let underline = underline_char.to_string().repeat(length);

            // Combine underline and message on the same line
            if let Some(msg) = &label.message {
              output.push_str(&format!("   | {}{} {}\n", padding, underline, msg));
            } else {
              output.push_str(&format!("   | {}{}\n", padding, underline));
            }
          }
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
