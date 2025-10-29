use std::fs;

use diagnostic::DiagnosticEngine;
use lexer::Lexer;

pub struct Runner {}

impl Runner {
  pub fn new() -> Self {
    Self {}
  }

  pub fn run_interactive_mode(&mut self, engine: &mut DiagnosticEngine) {}

  pub fn run_file(
    &mut self,
    path: String,
    engine: &mut DiagnosticEngine,
  ) -> Result<(), std::io::Error> {
    println!("\n============== READ =================\n");

    let source = fs::read_to_string(&path)?;
    println!("{}", source);

    println!("\n============= SCANNED ===============\n");
    let mut lexer = Lexer::new(source.as_str());
    lexer.scan_tokens(engine);
    println!("{:?}", lexer.tokens);

    // Check if there were scanning errors
    if engine.has_errors() {
      engine.print_diagnostics();
      return Err(std::io::Error::new(
        std::io::ErrorKind::Other,
        "lexing error",
      ));
    }

    Ok(())
  }
}
