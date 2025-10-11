use logger::{self, LogType, Logger};
use std::{fs, process};

mod token;

pub struct Scanner;

impl Scanner {
  pub fn run_interactive_mode() {}

  pub fn run_file(path: String) {
    let file_content = match fs::read_to_string(path) {
      Ok(buff) => buff,
      Err(err) => {
        Logger::log(logger::LogType::Error(&err.to_string()), 0);
        process::exit(1);
      },
    };

    Logger::log(LogType::Debug(&file_content), 0);
    Scanner::execute(file_content);
  }

  ///
  ///
  ///

  fn execute(content: String) {}
}
