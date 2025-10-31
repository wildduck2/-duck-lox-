use colored::*;
use std::fs;
use std::io::{self, BufRead};

use crate::code::DiagnosticCode;

#[derive(Debug, Clone)]
pub struct Label {
  pub span: Span,
  pub message: Option<String>,
  pub style: LabelStyle,
}

#[derive(Debug, Clone, Copy)]
pub enum LabelStyle {
  Primary,   // ^^^ in red
  Secondary, // --- in blue
}

#[derive(Debug)]
pub struct Diagnostic {
  pub code: DiagnosticCode,
  pub message: String,
  pub file_path: String,
  pub labels: Vec<Label>,
  pub help: Option<String>,
  pub note: Option<String>,
  context_padding: usize, // Number of lines to show above/below the error
}

impl Diagnostic {
  pub fn new(code: DiagnosticCode, message: String, file_path: String) -> Self {
    Self {
      code,
      message,
      file_path,
      labels: Vec::new(),
      help: None,
      note: None,
      context_padding: 2, // Default: show 2 lines above and below
    }
  }

  pub fn with_label(mut self, span: Span, message: Option<String>, style: LabelStyle) -> Self {
    self.labels.push(Label {
      span,
      message,
      style,
    });
    self
  }

  pub fn with_help(mut self, help: String) -> Self {
    self.help = Some(help);
    self
  }

  pub fn with_note(mut self, note: String) -> Self {
    self.note = Some(note);
    self
  }

  pub fn with_context_padding(mut self, padding: usize) -> Self {
    self.context_padding = padding;
    self
  }

  /// Loads the source file and extracts relevant context lines based on label spans
  fn load_context(&self) -> Result<Vec<(usize, String)>, io::Error> {
    let file = match fs::File::open(&self.file_path) {
      Ok(file) => file,
      Err(_) => {
        // When real source is missing (e.g. synthetic demo paths), skip context lines gracefully.
        return Ok(Vec::new());
      },
    };
    let reader = io::BufReader::new(file);
    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;

    if self.labels.is_empty() {
      return Ok(Vec::new());
    }

    // Find the range of lines we need to display
    let min_label_line = self.labels.iter().map(|l| l.span.line).min().unwrap();
    let max_label_line = self.labels.iter().map(|l| l.span.line).max().unwrap();

    let start_line = min_label_line.saturating_sub(self.context_padding).max(1);
    let end_line = (max_label_line + self.context_padding).min(lines.len());

    let mut context_lines = Vec::new();
    for line_num in start_line..=end_line {
      if let Some(content) = lines.get(line_num - 1) {
        context_lines.push((line_num, content.clone()));
      }
    }

    Ok(context_lines)
  }

  pub fn format(&self) -> Result<String, io::Error> {
    let mut output = String::new();

    // Error header
    output.push_str(&format!(
      "{} {}\n",
      format!("error[{}]:", self.code.code()).red().bold(),
      self.message.red().bold()
    ));

    if let Some(primary) = self.labels.first() {
      output.push_str(&format!(
        "  {} {}\n",
        "-->".blue().bold(),
        format!(
          "{}:{}:{}",
          self.file_path, primary.span.line, primary.span.col
        )
        .blue()
      ));

      // Load context dynamically
      let context_lines = self.load_context()?;

      if !context_lines.is_empty() {
        let max_line = context_lines.iter().map(|(ln, _)| *ln).max().unwrap_or(1);
        let line_width = max_line.to_string().len();

        output.push_str(&format!(
          "{} {}\n",
          " ".repeat(line_width),
          "|".blue().bold()
        ));

        // Group labels by line for proper rendering
        let mut lines_with_labels: std::collections::HashMap<usize, Vec<&Label>> =
          std::collections::HashMap::new();

        for label in &self.labels {
          lines_with_labels
            .entry(label.span.line)
            .or_insert_with(Vec::new)
            .push(label);
        }

        // Render all context lines in order
        for (line_num, content) in &context_lines {
          output.push_str(&format!(
            "{:width$} {} {}\n",
            line_num.to_string().blue().bold(),
            "|".blue().bold(),
            content.white(),
            width = line_width
          ));

          // Print all labels for this line
          if let Some(labels) = lines_with_labels.get(line_num) {
            for label in labels {
              let spaces = " ".repeat(label.span.col.saturating_sub(1));

              // Create the marker (^ or -)
              let marker_char = match label.style {
                LabelStyle::Primary => "^",
                LabelStyle::Secondary => "-",
              };
              let marker_len = label.span.len.max(1);
              let markers = marker_char.repeat(marker_len);

              // Color the markers
              let colored_markers = match label.style {
                LabelStyle::Primary => markers.red(),
                LabelStyle::Secondary => markers.cyan(),
              };

              // Print marker line with optional message
              if let Some(msg) = &label.message {
                let colored_msg = match label.style {
                  LabelStyle::Primary => msg.red(),
                  LabelStyle::Secondary => msg.cyan(),
                };

                output.push_str(&format!(
                  "{} {} {}{} {}\n",
                  " ".repeat(line_width),
                  "|".blue().bold(),
                  spaces,
                  colored_markers,
                  colored_msg
                ));
              } else {
                output.push_str(&format!(
                  "{} {} {}{}\n",
                  " ".repeat(line_width),
                  "|".blue().bold(),
                  spaces,
                  colored_markers
                ));
              }
            }
          }
        }
      }
    }

    // Help and note
    if let Some(help) = &self.help {
      output.push_str(&format!("   {} {}\n", "= help:".blue().bold(), help.blue()));
    }

    if let Some(note) = &self.note {
      output.push_str(&format!("   {} {}\n", "= note:".blue().bold(), note.blue()));
    }

    Ok(output)
  }

  pub fn print(&self) -> Result<(), io::Error> {
    print!("{}", self.format()?);
    Ok(())
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
  pub line: usize,
  pub col: usize, // Column where the span starts (1-indexed)
  pub len: usize, // Length of the span in characters
}

impl Span {
  pub fn new(line: usize, col: usize, len: usize) -> Self {
    Self { line, col, len }
  }

  /// Create a span from start and end columns
  pub fn from_range(line: usize, start: usize, end: usize) -> Self {
    Self {
      line,
      col: start,
      len: end.saturating_sub(start),
    }
  }
}
