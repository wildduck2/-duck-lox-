#[cfg(test)]
mod tests {
  use diagnostic::{
    code::DiagnosticCode,
    diagnostic::{Diagnostic, LabelStyle},
    source_map::{SourceMap, Span},
    types::error::DiagnosticError,
  };
  use std::fs;
  use std::path::Path;

  // Setup function to create test files
  fn setup_test_files() {
    // Create test directory if it doesn't exist
    let _ = fs::create_dir_all("test_sources");

    // Create main.rs
    fs::write(
      "test_sources/main.rs",
      r#"fn main() {
    let count = 5;
    let result = calculate(count);
    
    println!("Count: {}", counter);
    println!("Result: {}", result);
}

fn calculate(n: i32) -> i32 {
    n * 2
}
"#,
    )
    .unwrap();

    // Create lib.rs
    fs::write(
      "test_sources/lib.rs",
      r#"pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub fn subtract(a: i32, b: i32) -> i32 {
    a - b
}

pub fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

fn process_data(value: i32) -> String {
    let result = value * 2;
    result
}

pub fn divide(a: i32, b: i32) -> Option<i32> {
    if b == 0 {
        None
    } else {
        Some(a / b)
    }
}
"#,
    )
    .unwrap();

    // Create calculator.rs
    fs::write(
      "test_sources/calculator.rs",
      r#"fn calculate_sum(a: &str, b: i32) -> i32 {
    println!("Calculating...");
    a + b
}

pub fn add_numbers(x: i32, y: i32) -> i32 {
    x + y
}

pub fn multiply_numbers(x: i32, y: i32) -> i32 {
    x * y
}
"#,
    )
    .unwrap();

    // Create collections.rs
    fs::write(
      "test_sources/collections.rs",
      r#"use std::collections::HashMap;

pub fn create_map() -> HashMap<String, i32> {
    let mut map = HashMap::new();
    map.insert("one".to_string(), 1);
    map.insert("two".to_string(), 2);
    map
}

pub fn process_vector(data: Vec<i32>) -> i32 {
    data.iter().sum()
}

pub fn filter_even(data: Vec<i32>) -> Vec<i32> {
    data.into_iter().filter(|x| x % 2 == 0).collect()
}

fn update_collection(data: &mut Vec<i32>) {
    let first = &data[0];
    let second = &data[1];
    
    data.push(42);
    
    println!("First: {}, Second: {}", first, second);
}

pub fn sort_vector(data: &mut Vec<i32>) {
    data.sort();
}
"#,
    )
    .unwrap();

    // Create example.rs
    fs::write(
      "test_sources/example.rs",
      r#"pub fn example_function() {
    let x = 10;
    let y = 20;
    let z = 30;
    
    println!("x = {}", x);
    println!("y = {}", y);
    println!("z = {}", z);
    
    let unknown = mystery_var;
    
    println!("Done");
}

pub fn another_function() {
    println!("Another function");
}
"#,
    )
    .unwrap();
  }

  // Cleanup function to remove test files
  fn cleanup_test_files() {
    let _ = fs::remove_dir_all("test_sources");
  }

  #[test]
  fn test_all_diagnostics() {
    println!("\n\n\n DIAGNOSTIC SYSTEM TEST SUITE \n\n\n");

    // Setup once at the beginning
    setup_test_files();

    // Run all tests without cleanup
    println!("\n=== SIMPLE: Undefined Variable ===\n");
    let mut source_map = SourceMap::new();
    // Load sources into source map
    let main_rs = fs::read_to_string("test_sources/main.rs").unwrap();
    let lib_rs = fs::read_to_string("test_sources/lib.rs").unwrap();
    let calculator_rs = fs::read_to_string("test_sources/calculator.rs").unwrap();
    let collections_rs = fs::read_to_string("test_sources/collections.rs").unwrap();
    let example_rs = fs::read_to_string("test_sources/example.rs").unwrap();
    source_map.add_file("test_sources/main.rs", &main_rs);
    source_map.add_file("test_sources/lib.rs", &lib_rs);
    source_map.add_file("test_sources/calculator.rs", &calculator_rs);
    source_map.add_file("test_sources/collections.rs", &collections_rs);
    source_map.add_file("test_sources/example.rs", &example_rs);

    let diagnostic1 = Diagnostic::new(
      DiagnosticCode::Error(DiagnosticError::UndefinedVariable),
      "cannot find value `counter` in this scope".to_string(),
      "test_sources/main.rs".to_string(),
    )
    .with_label(
      Span::from_line_col(5, 28, 7, source_map.get("test_sources/main.rs").unwrap()),
      Some("not found in this scope".to_string()),
      LabelStyle::Primary,
    )
    .with_help("a local variable with a similar name exists: `count`".to_string());
    let _ = diagnostic1.print(&source_map);

    println!("\n=== MEDIUM: Type Mismatch ===\n");
    let diagnostic2 = Diagnostic::new(
      DiagnosticCode::Error(DiagnosticError::MismatchedTypes),
      "mismatched types".to_string(),
      "test_sources/lib.rs".to_string(),
    )
    .with_label(
      Span::from_line_col(15, 5, 6, source_map.get("test_sources/lib.rs").unwrap()),
      Some("expected `String`, found `i32`".to_string()),
      LabelStyle::Primary,
    )
    .with_label(
      Span::from_line_col(13, 32, 6, source_map.get("test_sources/lib.rs").unwrap()),
      Some("expected `String` because of return type".to_string()),
      LabelStyle::Secondary,
    )
    .with_help("try using `.to_string()` to convert `i32` to `String`".to_string())
    .with_note("expected type `String`\n          found type `i32`".to_string());
    let _ = diagnostic2.print(&source_map);

    println!("\n=== COMPLEX: Trait Bound Not Satisfied ===\n");
    let diagnostic3 = Diagnostic::new(
      DiagnosticCode::Error(DiagnosticError::TraitNotSatisfied),
      "the trait bound `&str: std::ops::Add<i32>` is not satisfied".to_string(),
      "test_sources/calculator.rs".to_string(),
    )
    .with_label(
      Span::from_line_col(1, 21, 4, source_map.get("test_sources/calculator.rs").unwrap()),
      Some("this parameter has type `&str`".to_string()),
      LabelStyle::Secondary,
    )
    .with_label(
      Span::from_line_col(3, 5, 1, source_map.get("test_sources/calculator.rs").unwrap()),
      Some("no implementation for `&str + i32`".to_string()),
      LabelStyle::Primary,
    )
    .with_label(
      Span::from_line_col(3, 9, 1, source_map.get("test_sources/calculator.rs").unwrap()),
      Some("cannot add `i32` to `&str`".to_string()),
      LabelStyle::Secondary,
    )
    .with_help("the trait `Add<i32>` is not implemented for `&str`".to_string())
    .with_note("the following trait bounds were not satisfied:\n            `&str: Add<i32>`\n            which is required by `&str: Add<i32>`".to_string());
    let _ = diagnostic3.print(&source_map);

    println!("\n=== SUPER COMPLEX: Borrow Checker Violation ===\n");
    let diagnostic4 = Diagnostic::new(
      DiagnosticCode::Error(DiagnosticError::BorrowCheckerViolation),
      "cannot borrow `data` as mutable because it is also borrowed as immutable".to_string(),
      "test_sources/collections.rs".to_string(),
    )
    .with_context_padding(1)
    .with_label(
      Span::from_line_col(
        19,
        17,
        8,
        source_map.get("test_sources/collections.rs").unwrap(),
      ),
      Some("immutable borrow occurs here".to_string()),
      LabelStyle::Secondary,
    )
    .with_label(
      Span::from_line_col(
        20,
        18,
        8,
        source_map.get("test_sources/collections.rs").unwrap(),
      ),
      Some("another immutable borrow occurs here".to_string()),
      LabelStyle::Secondary,
    )
    .with_label(
      Span::from_line_col(
        22,
        5,
        4,
        source_map.get("test_sources/collections.rs").unwrap(),
      ),
      Some("mutable borrow occurs here".to_string()),
      LabelStyle::Primary,
    )
    .with_label(
      Span::from_line_col(
        24,
        39,
        5,
        source_map.get("test_sources/collections.rs").unwrap(),
      ),
      Some("immutable borrow later used here".to_string()),
      LabelStyle::Secondary,
    )
    .with_label(
      Span::from_line_col(
        24,
        46,
        6,
        source_map.get("test_sources/collections.rs").unwrap(),
      ),
      Some("immutable borrow also used here".to_string()),
      LabelStyle::Secondary,
    )
    .with_help("consider cloning the values before mutating `data`".to_string())
    .with_note("cannot borrow `data` as mutable, as it is not declared as mutable".to_string());
    let _ = diagnostic4.print(&source_map);

    println!("\n=== CUSTOM PADDING: Wide Context ===\n");
    let diagnostic5 = Diagnostic::new(
      DiagnosticCode::Error(DiagnosticError::UndefinedVariable),
      "variable not found".to_string(),
      "test_sources/example.rs".to_string(),
    )
    .with_context_padding(3)
    .with_label(
      Span::from_line_col(
        10,
        19,
        11,
        source_map.get("test_sources/example.rs").unwrap(),
      ),
      Some("undefined variable".to_string()),
      LabelStyle::Primary,
    );
    let _ = diagnostic5.print(&source_map);

    println!("\n=== USING from_range ===\n");
    let diagnostic6 = Diagnostic::new(
      DiagnosticCode::Error(DiagnosticError::MismatchedTypes),
      "type error".to_string(),
      "test_sources/main.rs".to_string(),
    )
    .with_label(
      Span::from_line_col(3, 18, 27, source_map.get("test_sources/main.rs").unwrap()),
      Some("type mismatch here".to_string()),
      LabelStyle::Primary,
    );
    let _ = diagnostic6.print(&source_map);

    // Cleanup once at the end
    cleanup_test_files();

    println!("\n\n\n ALL TESTS COMPLETED \n\n\n");
  }
}
