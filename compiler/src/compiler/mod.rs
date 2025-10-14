use diagnostic::{Diagnostic, DiagnosticCode, DiagnosticEngine};
use parser::Parser;
use scanner::Scanner;
use std::fs;

pub struct Compiler {
  scanner: Scanner,
  parser: Parser,
}

impl Compiler {
  pub fn new(scanner: Scanner, parser: Parser) -> Self {
    Self { scanner, parser }
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

    // Set the source for the scanner
    self.scanner.source = source.clone();

    // Scan the tokens
    self.scanner.get_tokens(engine);

    // Check if there were scanning errors
    if engine.has_errors() {
      engine.print_all(&source);
      return;
    }

    // Parse the tokens
    // self.parser.parse(engine, &self.scanner.tokens);

    // Check if there were parsing errors
    if engine.has_errors() {
      engine.print_all(&source);
      return;
    }

    // If no errors, compilation succeeded
    println!("Compilation successful!");
  }
}
