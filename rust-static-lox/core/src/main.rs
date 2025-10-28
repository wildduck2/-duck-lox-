use diagnostic::{
  code::DiagnosticCode,
  diagnostic::{Diagnostic, LabelStyle, Span},
  types::error::DiagnosticError,
  DiagnosticEngine,
};

use crate::runner::Runner;
use colored::*;

mod runner;

fn main() {
  let args = std::env::args().collect::<Vec<_>>();
  let mut diagnostic_engine = DiagnosticEngine::new();
  let mut runner = Runner::new();

  match args.len() {
    1 => {
      println!("{}", "Running the interactive mode");
      runner.run_interactive_mode(&mut diagnostic_engine);
    },
    2 => {
      println!("{}", format!("Running file: {}", args[1]).cyan().bold());
      runner.run_file(args[1].clone(), &mut diagnostic_engine);
    },
    _ => {
      // Error: Invalid arguments
      let diagnostic = Diagnostic::new(
        DiagnosticCode::Error(DiagnosticError::InvalidArguments),
        "invalid number of arguments".to_string(),
        "demo.lox",
      )
      .with_help("Usage: lox [script]");

      diagnostic_engine.add(diagnostic);
      diagnostic_engine.print_diagnostics();
      std::process::exit(64);
    },
  }

  // Check if compilation had errors
  if diagnostic_engine.has_errors() {
    std::process::exit(65);
  }
}
