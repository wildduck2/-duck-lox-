mod logger;
mod scanner;

use logger::{LOG, LOGGER};
use scanner::Scanner;

fn main() -> () {
  let args: Vec<String> = std::env::args().collect();

  LOGGER::log(LOG::INFO, "Starting Lox interpreter");
  match args.len() {
    1 => {
      // Start an Ineractive prompt.
      Scanner::start_interactive_prompt();
    },
    2 => {
      // Run a file
      Scanner::run_file(&args[1]);
    },
    _ => {
      // Multiple files
      println!("Usage: lox [script]");
    },
  }
}
