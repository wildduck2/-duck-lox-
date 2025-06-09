mod file;
mod logger;
mod lox;
mod parser;
mod scanner;

use std::fs;

use logger::{Log, Logger};
use lox::Lox;
use scanner::Scanner;

fn main() -> () {
  fs::remove_file("log.txt").expect("Failed to flush the file");

  Logger::log(Log::Info, "Starting Lox interpreter");
  let args: Vec<String> = std::env::args().collect();
  let mut lox = Lox::new();
  let mut scanner = Scanner::new();

  match args.len() {
    1 => {
      // Start an Ineractive prompt.
      scanner.start_interactive_prompt(&mut lox);
    },
    2 => {
      // Run a file
      scanner.run_file(&args[1], &mut lox);
    },
    _ => {
      // Multiple files
      println!("Usage: lox [script]");
    },
  }

  if lox.has_error {
    std::process::exit(65);
  }
}

#[derive(Debug)]
pub enum GH {
  Value(String),
}

impl GH {
  pub fn get_value(&self) -> String {
    match self {
      GH::Value(value) => value.to_string(),
    }
  }
}
