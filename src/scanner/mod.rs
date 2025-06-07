mod token;
mod utils;

use token::Token;

use crate::file::File;
use crate::logger::{Log, Logger};
use crate::lox::{
  types::{CompilerError, LoxError},
  Lox,
};
use std::io::{self, Write};

pub struct Scanner {
  source: String,
  tokens: Vec<Token>,
  start: usize,
  current: usize,
  line: usize,
  column: usize,
}

impl Scanner {
  /// Creates a new `Scanner`
  pub fn new() -> Scanner {
    Self {
      source: String::new(),
      tokens: Vec::new(),
      start: 0,
      current: 0,
      line: 1,
      column: 0,
    }
  }

  /// Runs a file
  pub fn run_file(&mut self, file: &str, lox: &mut Lox) -> () {
    let file_content = File::read_file(file);
    self.source = file_content;
    self.execute(lox);
  }

  /// Executes the code
  pub fn execute(&mut self, lox: &mut Lox) -> () {
    // Logger::log(Log::INFO, &format!("Executing code: {:?}", &self.source));
    self.scan_tokens(lox);

    for _token in self.tokens.iter() {
      Logger::log(Log::Hint, &format!("{:?}", _token))
      // println!("asdf {:?}", _token);
      // Lox::log_language(
      //   Log::ERROR(LoxError::CompileError(CompilerError::SyntaxError)),
      //   "you have to remove the prantheses",
      //   "3:6",
      // );
      // lox.has_error = true;
      // println!("{:?}", token);
    }
    //
    // if lox.has_error {
    //   process::exit(65);
    // }

    // Logger::log(Log::INFO, &format!("Tokenized code: {:?}", tokens));
  }

  pub fn start_interactive_prompt(&mut self, lox: &mut Lox) -> () {
    loop {
      print!("> ");
      // Flush stdout to clear the Terminal.
      io::stdout().flush().expect("Unable to flush stdout");

      // Read stdin and store it in a `String` buffer.
      let mut buf = String::new();
      io::stdin()
        .read_line(&mut buf)
        .expect("Unable to read stdin");
      let prompt = buf.trim().to_string();

      // Check if the prompt is empty.
      if prompt.len() == 0 {
        break;
      }

      Logger::log(Log::Info, &format!("Executing code: {:?}", prompt));

      // Execute the code.
      self.execute(lox);
    }
  }
}
