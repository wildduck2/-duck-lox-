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

  pub fn execute(content: &str) -> () {
    println!("\n this is the code will be executed {}", content);
  }

  pub fn start_interactive_prompt() -> () {
    loop {
      print!("> ");
      io::stdout().flush().expect("Unable to flush stdout");
      let mut buf = String::new();
      let _ = io::Stdin::read_to_string(&mut io::stdin(), &mut buf);
    }
  }
}

fn read_file(file: &str) -> String {
  dbg!(fs::read_to_string(file).expect("Unable to read file"))
}
