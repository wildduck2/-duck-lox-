use logger::{LogType, Logger};
use scanner::Scanner;

fn main() {
  let args: Vec<String> = std::env::args().collect();
  println!("{:?}", args);

  match args.len() {
    1 => {
      Scanner::run_interactive_mode();
      Logger::log(LogType::Info("Running the interactive mode"), 0);
    },
    2 => {
      Scanner::run_file(args[1].clone());
      Logger::log(LogType::Info("Running the file mode"), 0);
    },
    _ => {
      Logger::log(LogType::Info("Nothing"), 0);
    },
  }
}
