use logger::{LogType, Logger};
use scanner::Scanner;

fn main() {
  let args: Vec<String> = std::env::args().collect();
  println!("{:?}", args);

  let mut scanner = Scanner::new();

  match args.len() {
    1 => {
      scanner.run_interactive_mode();
      Logger::log(LogType::Info("Running the interactive mode"), 0);
    },
    2 => {
      scanner.run_file(args[1].clone());
      Logger::log(LogType::Info("Running the file mode"), 0);
    },
    _ => {
      Logger::log(LogType::Info("Nothing"), 0);
    },
  }
}
