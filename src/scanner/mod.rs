use crate::file::File;
use crate::logger::{Log, Logger};
use crate::lox::{CompilerError, Lox, LoxError};
use std::collections::HashMap;
use std::process;
use std::{
  fs,
  io::{self, Read, Write},
};

pub struct Scanner;

impl Scanner {
  pub fn run_file(file: &str, lox: &mut Lox) -> () {
    let file_content = File::read_file(file);
    Scanner::execute(&file_content, lox);
  }

  // REPL mode
  pub fn execute(content: &str, lox: &mut Lox) -> () {
    Logger::log(Log::INFO, &format!("Executing code: {:?}", content));
    let tokens = Scanner::get_file_tokens(&content);

    for token in tokens.iter() {
      Lox::log_language(
        Log::ERROR(LoxError::CompileError(CompilerError::SyntaxError)),
        "you have to remove the prantheses",
        "3:6",
      );
      lox.has_error = true;
      // println!("{:?}", token);
    }

    if lox.has_error {
      process::exit(65);
    }

    // Logger::log(Log::INFO, &format!("Tokenized code: {:?}", tokens));
  }

  pub fn start_interactive_prompt(lox: &mut Lox) -> () {
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

      Logger::log(Log::INFO, &format!("Executing code: {:?}", prompt));

      // Execute the code.
      Scanner::execute(&prompt, lox);
    }
  }

  pub fn get_file_tokens(file_content: &str) -> HashMap<String, String> {
    let mut tokens: HashMap<String, String> = HashMap::new();
    tokens.insert("a".to_string(), file_content.to_string());
    tokens
  }
}
