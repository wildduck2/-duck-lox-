pub mod types;

use crate::logger::{Log, Logger};

pub struct Lox {
  /// Checks if there are any errors in the code if there are then exit the program with 65.
  pub has_error: bool,
}

impl Lox {
  /// Creates a new instance of the Lox struct.
  pub fn new() -> Lox {
    Lox { has_error: false }
  }

  /// Logs a message with a timestamp, level, and message, also writes to a log file but for
  /// the language.
  pub fn log_language(&mut self, level: Log, message: &str, position: &str) -> () {
    Logger::log(level, format!("[line: {}] {}", position, message).as_str());
  }
}
