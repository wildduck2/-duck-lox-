use crate::logger::{LOG, LOGGER};
use std::{
  fs,
  io::{self, Read, Write},
};

pub struct Scanner;

impl Scanner {
  pub fn run_file(file: &str) -> () {
    let file_content = read_file(file);
    Scanner::execute(&file_content);
  }

  // REPL mode
  pub fn execute(_content: &str) -> () {
    LOGGER::log(LOG::INFO, "Executing code");
  }

  pub fn start_interactive_prompt() -> () {
    loop {
      print!("> ");
      io::stdout().flush().expect("Unable to flush stdout");
      let mut buf = String::new();
      let prompt = io::Stdin::read_line(&mut io::stdin(), &mut buf)
        .expect("Unable to read stdin")
        .to_string();
      let prompt = prompt.trim().to_string();

      if prompt.trim().len() == 0 {
        break;
      }
      Scanner::execute(&prompt);
      break;
    }
  }
}

fn read_file(file: &str) -> String {
  dbg!(fs::read_to_string(file).expect("Unable to read file"))
}
