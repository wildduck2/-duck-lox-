#[cfg(test)]
mod tests {
  use diagnostic::{
    code::DiagnosticCode,
    diagnostic::{Diagnostic, LabelStyle, Span},
    types::error::DiagnosticError,
  };

  #[test]
  fn test_simple_diagnostic() {
    println!("\n=== SIMPLE: Undefined Variable ===\n");

    let diagnostic = Diagnostic::new(
      DiagnosticCode::Error(DiagnosticError::UndefinedVariable),
      "cannot find value `counter` in this scope".to_string(),
      "src/main.rs".to_string(),
    )
    .with_context_line(5, r#"    println!("Count: {}", counter);"#.to_string())
    .with_label(
      Span {
        line: 5,
        start: 27,
        end: 34,
      },
      Some("not found in this scope".to_string()),
      LabelStyle::Primary,
    )
    .with_help("a local variable with a similar name exists: `count`".to_string());

    diagnostic.print();
  }

  #[test]
  fn test_medium_diagnostic() {
    println!("\n=== MEDIUM: Type Mismatch ===\n");

    let diagnostic = Diagnostic::new(
      DiagnosticCode::Error(DiagnosticError::MismatchedTypes),
      "mismatched types".to_string(),
      "src/lib.rs".to_string(),
    )
    .with_context_line(12, r#"fn process_data(value: i32) -> String {"#.to_string())
    .with_context_line(13, r#"    let result = value * 2;"#.to_string())
    .with_context_line(14, r#"    result"#.to_string())
    .with_context_line(15, r#"}"#.to_string())
    .with_label(
      Span {
        line: 14,
        start: 5,
        end: 11,
      },
      Some("expected `String`, found `i32`".to_string()),
      LabelStyle::Primary,
    )
    .with_label(
      Span {
        line: 12,
        start: 28,
        end: 34,
      },
      Some("expected `String` because of return type".to_string()),
      LabelStyle::Secondary,
    )
    .with_help("try using `.to_string()` to convert `i32` to `String`".to_string())
    .with_note("expected type `String`\n          found type `i32`".to_string());

    diagnostic.print();
  }

  #[test]
  fn test_complex_diagnostic() {
    println!("\n=== COMPLEX: Trait Bound Not Satisfied ===\n");

    let diagnostic = Diagnostic::new(
      DiagnosticCode::Error(DiagnosticError::TraitNotSatisfied),
      "the trait bound `&str: std::ops::Add<i32>` is not satisfied".to_string(),
      "src/calculator.rs".to_string(),
    )
    .with_context_line(1, r#"fn calculate_sum(a: &str, b: i32) -> i32 {"#.to_string())
    .with_context_line(2, r#"    println!("Calculating...");"#.to_string())
    .with_context_line(3, r#"    a + b"#.to_string())
    .with_context_line(4, r#"}"#.to_string())
    .with_label(
      Span {
        line: 1,
        start: 21,
        end: 25,
      },
      Some("this parameter has type `&str`".to_string()),
      LabelStyle::Secondary,
    )
    .with_label(
      Span {
        line: 3,
        start: 5,
        end: 6,
      },
      Some("no implementation for `&str + i32`".to_string()),
      LabelStyle::Primary,
    )
    .with_label(
      Span {
        line: 3,
        start: 7,
        end: 8,
      },
      Some("cannot add `i32` to `&str`".to_string()),
      LabelStyle::Secondary,
    )
    .with_help("the trait `Add<i32>` is not implemented for `&str`".to_string())
    .with_note("the following trait bounds were not satisfied:\n            `&str: Add<i32>`\n            which is required by `&str: Add<i32>`".to_string());

    diagnostic.print();
  }

  #[test]
  fn test_super_complex_diagnostic() {
    println!("\n=== SUPER COMPLEX: Borrow Checker Violation ===\n");

    let diagnostic = Diagnostic::new(
      DiagnosticCode::Error(DiagnosticError::BorrowCheckerViolation),
      "cannot borrow `data` as mutable because it is also borrowed as immutable".to_string(),
      "src/collections.rs".to_string(),
    )
    .with_context_line(
      18,
      r#"fn update_collection(data: &mut Vec<i32>) {"#.to_string(),
    )
    .with_context_line(19, r#"    let first = &data[0];"#.to_string())
    .with_context_line(20, r#"    let second = &data[1];"#.to_string())
    .with_context_line(21, r#"    "#.to_string())
    .with_context_line(22, r#"    data.push(42);"#.to_string())
    .with_context_line(23, r#"    "#.to_string())
    .with_context_line(
      24,
      r#"    println!("First: {}, Second: {}", first, second);"#.to_string(),
    )
    .with_context_line(25, r#"}"#.to_string())
    .with_label(
      Span {
        line: 19,
        start: 17,
        end: 25,
      },
      Some("immutable borrow occurs here".to_string()),
      LabelStyle::Secondary,
    )
    .with_label(
      Span {
        line: 20,
        start: 18,
        end: 26,
      },
      Some("another immutable borrow occurs here".to_string()),
      LabelStyle::Secondary,
    )
    .with_label(
      Span {
        line: 22,
        start: 5,
        end: 9,
      },
      Some("mutable borrow occurs here".to_string()),
      LabelStyle::Primary,
    )
    .with_label(
      Span {
        line: 24,
        start: 39,
        end: 44,
      },
      Some("immutable borrow later used here".to_string()),
      LabelStyle::Secondary,
    )
    .with_label(
      Span {
        line: 24,
        start: 46,
        end: 52,
      },
      Some("immutable borrow also used here".to_string()),
      LabelStyle::Secondary,
    )
    .with_help("consider cloning the values before mutating `data`".to_string())
    .with_note("cannot borrow `data` as mutable, as it is not declared as mutable".to_string());

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
