use logger::{self, Logger};
use std::{fs, process};

use crate::{lox::Lox, token::Token};

mod lox;
mod token;
mod utils;

pub struct Scanner {
  pub tokens: Vec<Token>,
  pub source: String,
  pub line: u32,
  pub column: u32,
  pub current: u32,
  pub start: u32,
}

impl Scanner {
  /// Function that created a new scanner
  pub fn new() -> Self {
    Self {
      source: String::from(""),
      column: 0,
      line: 0,
      start: 0,
      current: 0,
      tokens: vec![],
    }
  }

  /// Function that starts the runtime env for the language takes stdin and puts stdout or strerr.
  pub fn run_interactive_mode(&self) {}

  /// Function that runs the process of compiling file.
  pub fn run_file(&mut self, path: String) {
    let lox = Lox::new();

    self.source = match fs::read_to_string(path) {
      Ok(buff) => buff,
      Err(err) => {
        Logger::log(logger::LogType::Error(&err.to_string()), 0);
        process::exit(1);
      },
    };

    self.execute(lox);
  }

  /// Function that executes the scanning operation on a lox content.
  fn execute(&mut self, lox: Lox) {
    self.get_tokens(lox);
    println!("{:#?}", self.tokens);
  }
}
