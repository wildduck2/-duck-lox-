use compiler::Compiler;
use logger::{LogType, Logger};
use lox::Lox;
use parser::Parser;
use scanner::Scanner;

mod compiler;

fn main() {
  let args: Vec<String> = std::env::args().collect();
  println!("{:?}", args);

  let mut compiler = Compiler::new(Scanner::new(), Parser::new());

  let mut lox = Lox::new();

  match args.len() {
    1 => {
      Logger::log(LogType::Info("Running the interactive mode"), 0);
      compiler.run_interactive_mode();
    },
    2 => {
      Logger::log(LogType::Info("Running the file mode"), 0);
      compiler.run_file(args[1].clone(), &mut lox);
    },
    _ => {
      Logger::log(LogType::Info("Nothing"), 0);
    },
  }

  if lox.has_error {
    std::process::exit(65);
  }
}
