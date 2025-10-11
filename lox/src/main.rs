use logger::{LogType, Logger};
use scanner::{lox::Lox, Scanner};

fn main() {
  let args: Vec<String> = std::env::args().collect();
  println!("{:?}", args);

  let mut scanner = Scanner::new();
  let mut lox = Lox::new();

  match args.len() {
    1 => {
      Logger::log(LogType::Info("Running the interactive mode"), 0);
      scanner.run_interactive_mode();
    },
    2 => {
      Logger::log(LogType::Info("Running the file mode"), 0);
      scanner.run_file(args[1].clone(), &mut lox);
    },
    _ => {
      Logger::log(LogType::Info("Nothing"), 0);
    },
  }

  if lox.has_error {
    std::process::exit(65);
  }
}
