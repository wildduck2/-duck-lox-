use colored::*;

use crate::code::DiagnosticCode;

#[derive(Debug, Clone)]
pub struct Label<'a> {
  pub span: Span,
  pub message: Option<&'a str>,
  pub style: LabelStyle,
}

#[derive(Debug, Clone, Copy)]
pub enum LabelStyle {
  Primary,   // ^^^ in red
  Secondary, // --- in blue
}

#[derive(Debug)]
pub struct Diagnostic<'a> {
  pub code: DiagnosticCode,
  pub message: String,
  pub file_path: &'a str,
  pub labels: Vec<Label<'a>>,
  pub context_lines: Vec<(usize, &'a str)>,
  pub help: Option<&'a str>,
  pub note: Option<&'a str>,
}

impl<'a> Diagnostic<'a> {
  pub fn new(code: DiagnosticCode, message: String, file_path: &'a str) -> Self {
    Self {
      code,
      message,
      file_path,
      labels: Vec::new(),
      context_lines: Vec::new(),
      help: None,
      note: None,
    }
  }

  pub fn with_label(mut self, span: Span, message: Option<&'a str>, style: LabelStyle) -> Self {
    self.labels.push(Label {
      span,
      message,
      style,
    });
    self
  }

  pub fn with_context_line(mut self, line_num: usize, content: &'a str) -> Self {
    self.context_lines.push((line_num, content));
    self
  }

  pub fn with_help(mut self, help: &'a str) -> Self {
    self.help = Some(help);
    self
  }

  pub fn with_note(mut self, note: &'a str) -> Self {
    self.note = Some(note);
    self
  }

  pub fn format(&self) -> String {
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
          self.file_path, primary.span.line, primary.span.start
        )
        .blue()
      ));

      let max_line = self
        .context_lines
        .iter()
        .map(|(ln, _)| *ln)
        .chain(self.labels.iter().map(|l| l.span.line))
        .max()
        .unwrap_or(primary.span.line);
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
      let min_line = self
        .context_lines
        .iter()
        .map(|(ln, _)| *ln)
        .min()
        .unwrap_or(1);
      let max_line_num = self
        .context_lines
        .iter()
        .map(|(ln, _)| *ln)
        .max()
        .unwrap_or(1);

      for line_num in min_line..=max_line_num {
        // Print the source line if it exists
        if let Some((_, content)) = self.context_lines.iter().find(|(ln, _)| *ln == line_num) {
          output.push_str(&format!(
            "{:width$} {} {}\n",
            line_num.to_string().blue().bold(),
            "|".blue().bold(),
            content.white(),
            width = line_width
          ));

          // Print all labels for this line
          if let Some(labels) = lines_with_labels.get(&line_num) {
            for label in labels {
              let spaces = " ".repeat(label.span.start - 1);

              // Create the marker (^ or -)
              let marker_char = match label.style {
                LabelStyle::Primary => "^",
                LabelStyle::Secondary => "-",
              };
              let marker_len = label.span.end - label.span.start;
              let markers = marker_char.repeat(marker_len);

              // Color the markers
              let colored_markers = match label.style {
                LabelStyle::Primary => markers.red(),
                LabelStyle::Secondary => markers.cyan(),
              };

              // Print marker line with optional message
              if let Some(msg) = label.message {
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
    if let Some(help) = self.help {
      output.push_str(&format!("   {} {}\n", "= help:".blue().bold(), help.blue()));
    }

    if let Some(note) = self.note {
      output.push_str(&format!("   {} {}\n", "= note:".blue().bold(), note.blue()));
    }

    output
  }

  pub fn print(&self) {
    print!("{}", self.format());
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
  pub line: usize,
  pub start: usize,
  pub end: usize,
}

impl Span {
  pub fn new(start: usize, end: usize) -> Self {
    Self {
      line: 1,
      start,
      end,
    }
  }
}
