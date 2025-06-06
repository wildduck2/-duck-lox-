mod file;
mod logger;
mod lox;
mod scanner;

use logger::{Log, Logger};
use lox::Lox;
use scanner::Scanner;

fn main() -> () {
  Logger::log(Log::INFO, "Starting Lox interpreter");
  let args: Vec<String> = std::env::args().collect();
  let mut lox = Lox { has_error: false };

  match args.len() {
    1 => {
      // Start an Ineractive prompt.
      Scanner::start_interactive_prompt(&mut lox);
    },
    2 => {
      // Run a file
      Scanner::run_file(&args[1], &mut lox);
    },
    _ => {
      // Multiple files
      println!("Usage: lox [script]");
    },
  }
}
