use std::{fs, process};

use logger::Logger;
use lox::Lox;
use parser::Parser;
use scanner::Scanner;

pub struct Compiler {
  scanner: Scanner,
  parser: Parser,
}

impl Compiler {
  pub fn new(scanner: Scanner, parser: Parser) -> Self {
    Self { scanner, parser }
  }

  /// Function that starts the runtime env for the language takes stdin and puts stdout or strerr.
  pub fn run_interactive_mode(&self) {}

  /// Function that runs the process of compiling file.
  pub fn run_file(&mut self, path: String, lox: &mut Lox) {
    self.scanner.source = match fs::read_to_string(path) {
      Ok(buff) => buff,
      Err(err) => {
        Logger::log(logger::LogType::Error(&err.to_string()), 0);
        process::exit(1);
      },
    };

    self.scanner.scan(lox);
  }
}
