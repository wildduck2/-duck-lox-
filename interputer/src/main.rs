use colored::*;
use diagnostic::{diagnostic::Diagnostic, diagnostic_code::DiagnosticCode, DiagnosticEngine};
use runner::Runner;

mod env;
mod error;
mod function;
mod interpreter;
mod lox_value;
mod runner;

fn main() {
  let args: Vec<String> = std::env::args().collect();

  let mut diagnostic = DiagnosticEngine::new();
  let mut compiler = Runner::new();

  match args.len() {
    1 => {
      // Info message for interactive mode
      println!("{}", "Running the interactive mode".cyan().bold());
      compiler.run_interactive_mode(&mut diagnostic);
    },
    2 => {
      // Info message for file mode
      println!("{}", format!("Running file: {}", args[1]).cyan().bold());
      compiler.run_file(args[1].clone(), &mut diagnostic);

      // Check if compilation had errors
      if diagnostic.has_errors() {
        std::process::exit(65);
      }
    },
    _ => {
      // Error: Invalid arguments
      let error = Diagnostic::new(
        DiagnosticCode::InvalidArguments,
        "invalid number of arguments".to_string(),
      )
      .with_help("Usage: lox [script]".to_string());

      diagnostic.emit(error);
      diagnostic.print_all("");
      std::process::exit(64);
    },
  }
}
