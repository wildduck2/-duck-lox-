use diagnostic::{
  code::DiagnosticCode,
  diagnostic::{Diagnostic, LabelStyle, Span},
  types::error::DiagnosticError,
};

fn main() {
  println!("\n=== SUPER COMPLEX: Borrow Checker Violation ===\n");

  let diagnostic = Diagnostic::new(
    DiagnosticCode::Error(DiagnosticError::BorrowCheckerViolation),
    "cannot borrow `data` as mutable because it is also borrowed as immutable",
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
