mod scanner;

use scanner::Scanner;

fn main() -> () {
  let args: Vec<String> = std::env::args().collect();

  match args.len() {
    1 => {
      // Start an Ineractive prompt.
      println!("Usage: lox [script]");
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
