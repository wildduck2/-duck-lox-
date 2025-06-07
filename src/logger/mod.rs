mod color;
use chrono::Local;
use color::{Color, ColorType};
use colored::*;
use std::{fs::OpenOptions, io::Write};

use crate::lox::types::LoxError;

#[derive(Debug)]
pub enum Log {
  Error(LoxError),
  Warning,
  Info,
  Hint,
}

impl Log {
  fn to_plain_str(&self) -> String {
    match self {
      Log::Error(error) => match error {
        LoxError::CompileError(error) => match error {
          _ => format!("{:?}", error),
        },
        _ => format!("{:?}", error),
      },
      Log::Warning => "WARN".to_string(),
      Log::Info => "INFO".to_string(),
      Log::Hint => "HINT".to_string(),
    }
  }

  fn to_colored_str(&self) -> colored::ColoredString {
    match self {
      Log::Error(error) => match error {
        LoxError::CompileError(error) => match error {
          _ => format!("{:?}", error).red().bold(),
        },
        _ => format!("{:?}", error).red().bold(),
      },
      Log::Warning => "WARN".yellow().bold(),
      Log::Info => "INFO".cyan().bold(),
      Log::Hint => "HINT".green(),
    }
  }
}

pub struct Logger;

impl Logger {
  /// Logs a message with a timestamp, level, and message, also writes to a log file.
  pub fn log(level: Log, message: &str) -> () {
    let now = Local::now().format("%Y-%m-%d %H:%M:%S");

    // For file (no colors).
    Logger::log_to_file(&format!(
      "[{}] [{}] {}\n",
      now,
      level.to_plain_str(),
      message
    ));

    // For terminal.
    match level {
      Log::Error(_) => {
        eprintln!(
          "[{}] [{}] {}",
          now.to_string().to_colored_str(ColorType::ERROR),
          level.to_colored_str(),
          message.red()
        );
      },
      _ => {
        println!(
          "[{}] [{}] {}",
          now.to_string().to_colored_str(ColorType::INFO),
          level.to_colored_str(),
          message
        );
      },
    }
  }

  /// Returns a message with a timestamp, level, and message.
  pub fn message(level: Log, message: &str) -> String {
    let now = Local::now().format("%Y-%m-%d %H:%M:%S");
    format!(
      "[{}] [{}] {}",
      now.to_string().to_colored_str(ColorType::INFO),
      level.to_colored_str(),
      message
    )
  }

  /// Writes a message to a log file.
  pub fn log_to_file(message: &str) {
    let file_handler = OpenOptions::new().create(true).append(true).open("log.txt");

    match file_handler {
      Ok(file) => {
        let mut writer = std::io::BufWriter::new(file);
        if let Err(e) = writer.write_all(message.as_bytes()) {
          eprintln!("Failed to write to log file: {}", e);
        }
      },
      Err(e) => {
        eprintln!("Failed to open or create log file: {}", e);
      },
    }
  }
}
