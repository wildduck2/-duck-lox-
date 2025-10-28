#[cfg(test)]
mod tests {
  use diagnostic::{
    code::DiagnosticCode,
    diagnostic::{Diagnostic, LabelStyle, Span},
    types::error::DiagnosticError,
    DiagnosticEngine,
  };

  #[test]
  fn test_simple_diagnostic() {
    println!("\n=== SIMPLE: Undefined Variable ===\n");

    let diagnostic = Diagnostic::new(
      DiagnosticCode::Error(DiagnosticError::UndefinedVariable),
      "cannot find value `counter` in this scope".to_string(),
      "src/main.rs",
    )
    .with_context_line(5, r#"    println!("Count: {}", counter);"#)
    .with_label(
      Span {
        line: 5,
        start: 27,
        end: 34,
      },
      Some("not found in this scope"),
      LabelStyle::Primary,
    )
    .with_help("a local variable with a similar name exists: `count`");

    diagnostic.print();
  }

  #[test]
  fn test_medium_diagnostic() {
    println!("\n=== MEDIUM: Type Mismatch ===\n");

    let diagnostic = Diagnostic::new(
      DiagnosticCode::Error(DiagnosticError::MismatchedTypes),
      "mismatched types".to_string(),
      "src/lib.rs",
    )
    .with_context_line(12, r#"fn process_data(value: i32) -> String {"#)
    .with_context_line(13, r#"    let result = value * 2;"#)
    .with_context_line(14, r#"    result"#)
    .with_context_line(15, r#"}"#)
    .with_label(
      Span {
        line: 14,
        start: 5,
        end: 11,
      },
      Some("expected `String`, found `i32`"),
      LabelStyle::Primary,
    )
    .with_label(
      Span {
        line: 12,
        start: 28,
        end: 34,
      },
      Some("expected `String` because of return type"),
      LabelStyle::Secondary,
    )
    .with_help("try using `.to_string()` to convert `i32` to `String`")
    .with_note("expected type `String`\n          found type `i32`");

    diagnostic.print();
  }

  #[test]
  fn test_complex_diagnostic() {
    println!("\n=== COMPLEX: Trait Bound Not Satisfied ===\n");

    let diagnostic = Diagnostic::new(
      DiagnosticCode::Error(DiagnosticError::TraitNotSatisfied),
      "the trait bound `&str: std::ops::Add<i32>` is not satisfied".to_string(),
      "src/calculator.rs",
    )
    .with_context_line(1, r#"fn calculate_sum(a: &str, b: i32) -> i32 {"#)
    .with_context_line(2, r#"    println!("Calculating...");"#)
    .with_context_line(3, r#"    a + b"#)
    .with_context_line(4, r#"}"#)
    .with_label(
      Span {
        line: 1,
        start: 21,
        end: 25,
      },
      Some("this parameter has type `&str`"),
      LabelStyle::Secondary,
    )
    .with_label(
      Span {
        line: 3,
        start: 5,
        end: 6,
      },
      Some("no implementation for `&str + i32`"),
      LabelStyle::Primary,
    )
    .with_label(
      Span {
        line: 3,
        start: 7,
        end: 8,
      },
      Some("cannot add `i32` to `&str`"),
      LabelStyle::Secondary,
    )
    .with_help("the trait `Add<i32>` is not implemented for `&str`")
    .with_note("the following trait bounds were not satisfied:\n            `&str: Add<i32>`\n            which is required by `&str: Add<i32>`");

    diagnostic.print();
  }

  #[test]
  fn test_super_complex_diagnostic() {
    println!("\n=== SUPER COMPLEX: Borrow Checker Violation ===\n");

    let diagnostic = Diagnostic::new(
      DiagnosticCode::Error(DiagnosticError::BorrowCheckerViolation),
      "cannot borrow `data` as mutable because it is also borrowed as immutable".to_string(),
      "src/collections.rs",
    )
    .with_context_line(18, r#"fn update_collection(data: &mut Vec<i32>) {"#)
    .with_context_line(19, r#"    let first = &data[0];"#)
    .with_context_line(20, r#"    let second = &data[1];"#)
    .with_context_line(21, r#"    "#)
    .with_context_line(22, r#"    data.push(42);"#)
    .with_context_line(23, r#"    "#)
    .with_context_line(
      24,
      r#"    println!("First: {}, Second: {}", first, second);"#,
    )
    .with_context_line(25, r#"}"#)
    .with_label(
      Span {
        line: 19,
        start: 17,
        end: 25,
      },
      Some("immutable borrow occurs here"),
      LabelStyle::Secondary,
    )
    .with_label(
      Span {
        line: 20,
        start: 18,
        end: 26,
      },
      Some("another immutable borrow occurs here"),
      LabelStyle::Secondary,
    )
    .with_label(
      Span {
        line: 22,
        start: 5,
        end: 9,
      },
      Some("mutable borrow occurs here"),
      LabelStyle::Primary,
    )
    .with_label(
      Span {
        line: 24,
        start: 39,
        end: 44,
      },
      Some("immutable borrow later used here"),
      LabelStyle::Secondary,
    )
    .with_label(
      Span {
        line: 24,
        start: 46,
        end: 52,
      },
      Some("immutable borrow also used here"),
      LabelStyle::Secondary,
    )
    .with_help("consider cloning the values before mutating `data`")
    .with_note("cannot borrow `data` as mutable, as it is not declared as mutable");

    diagnostic.print();
  }

  #[test]
  fn test_all_diagnostics() {
    println!("\n\n\n DIAGNOSTIC SYSTEM TEST SUITE \n\n\n");

    test_simple_diagnostic();
    test_medium_diagnostic();
    test_complex_diagnostic();
    test_super_complex_diagnostic();

    println!("\n\n\n ALL TESTS COMPLETED \n\n\n");
  }
}
