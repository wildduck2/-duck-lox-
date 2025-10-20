use crate::interpreter::Interpreter;
use diagnostic::{diagnostic::Diagnostic, diagnostic_code::DiagnosticCode, DiagnosticEngine};
use parser::Parser;
use scanner::Scanner;
use std::fs;

pub struct Runner {}

impl Runner {
  pub fn new() -> Self {
    Self {}
  }

  /// Function that starts the runtime env for the language takes stdin and puts stdout or stderr.
  pub fn run_interactive_mode(&mut self, engine: &mut DiagnosticEngine) {
    // TODO: Implement interactive mode
    // This would typically involve:
    // 1. Reading lines from stdin
    // 2. Scanning and parsing each line
    // 3. Executing the parsed code
    // 4. Emitting diagnostics to engine if errors occur
  }

  /// Function that runs the process of compiling file.
  pub fn run_file(&mut self, path: String, engine: &mut DiagnosticEngine) {
    // Reading files to get the string buff
    let source = match fs::read_to_string(&path) {
      Ok(content) => content,
      Err(err) => {
        let diagnostic = Diagnostic::new(
          DiagnosticCode::FileNotFound,
          format!("could not read file: {}", path),
        )
        .with_help(format!("reason: {}", err));

        engine.emit(diagnostic);
        engine.print_all("");
        std::process::exit(66);
      },
    };

    println!("\n============== READ =================\n");

    println!("{}", source);

    // Scanning the buffer of string
    let mut scanner = Scanner::new(source.clone());

    println!("\n============= INITIALIZED ===========\n");

    // Scan the tokens
    scanner.scan(engine);

    // Check if there were scanning errors
    if engine.has_errors() {
      engine.print_all(&source);
      return;
    }
    println!("ToLongVector(value...)");
    println!("\n============= SCANNED ===============\n");

    // Parse the tokens
    let mut parser = Parser::new(scanner.tokens);
    parser.parse(engine);

    // Check if there were parsing errors
    if engine.has_errors() {
      engine.print_all(&source);
      return;
    }
    println!("\n============== PARSED ===============\n");

    let mut interputer = Interpreter::new();
    interputer.run(parser.ast, engine);

    if engine.has_errors() {
      engine.print_all(&source);
      return;
    }
    println!("\n============ INTERPRETED ============\n");

    // If no errors, compilation succeeded
    println!("Compilation successful!");
  }
}
