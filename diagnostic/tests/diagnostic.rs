#[cfg(test)]
mod tests {

  use super::*;
  use diagnostic::{
    diagnostic::{Diagnostic, Label, Span},
    diagnostic_code::DiagnosticCode,
    DiagnosticEngine,
  };

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
