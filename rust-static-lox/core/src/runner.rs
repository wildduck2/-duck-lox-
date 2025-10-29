use std::fs;

use diagnostic::DiagnosticEngine;
use lexer::Lexer;
use parser::Parser;

pub struct Runner {
  pub source: String,
}

impl Runner {
  pub fn new() -> Self {
    Self {
      source: String::new(),
    }
  }

  pub fn run_interactive_mode(&mut self, engine: &mut DiagnosticEngine) {}

  pub fn run_file<'a>(
    &'a mut self,
    path: String,
    engine: &mut DiagnosticEngine<'a>,
  ) -> Result<(), std::io::Error> {
    println!("\n============== READ =================\n");

    self.source = fs::read_to_string(&path)?;
    println!("{:?}", self.source);

    println!("\n============= SCANNED ===============\n");

    let mut lexer = Lexer::new(&self.source);
    lexer.scan_tokens(engine);

    if engine.has_errors() {
      engine.print_diagnostics();
      return Err(std::io::Error::new(
        std::io::ErrorKind::Other,
        "lexing error",
      ));
    }

    println!("{:?}", lexer.tokens);

    println!("\n============= PARSED ===============\n");

    let mut parser = Parser::new();
    parser.parse();

    if engine.has_errors() {
      engine.print_diagnostics();
      return Err(std::io::Error::new(
        std::io::ErrorKind::Other,
        "parsing error",
      ));
    }

    Ok(())
  }
}
