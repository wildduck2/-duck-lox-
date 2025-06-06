use chrono::Local;
use colored::*;
use std::{fmt, fs::OpenOptions, io::Write, path::Path};

pub enum LOG {
  ERROR,
  WARNING,
  INFO,
  HINT,
}

impl LOG {
  fn to_plain_str(&self) -> &'static str {
    match self {
      LOG::ERROR => "ERROR",
      LOG::WARNING => "WARN",
      LOG::INFO => "INFO",
      LOG::HINT => "HINT",
    }
  }

  fn to_colored_str(&self) -> colored::ColoredString {
    match self {
      LOG::ERROR => "ERROR".red().bold(),
      LOG::WARNING => "WARN".yellow().bold(),
      LOG::INFO => "INFO".cyan().bold(),
      LOG::HINT => "HINT".green(),
    }
  }
}

pub struct LOGGER;

impl LOGGER {
  pub fn log(level: LOG, message: &str) {
    let now = Local::now().format("%Y-%m-%d %H:%M:%S");
    // For terminal
    println!("[{}] [{}] {}", now, level.to_colored_str(), message);
    // For file (no colors)
    LOGGER::log_to_file(&format!(
      "[{}] [{}] {}\n",
      now,
      level.to_plain_str(),
      message
    ));
  }

  pub fn message(level: LOG, message: &str) -> String {
    let now = Local::now().format("%Y-%m-%d %H:%M:%S");
    // Also logs to file
    LOGGER::log_to_file(&format!(
      "[{}] [{}] {}\n",
      now,
      level.to_plain_str(),
      message
    ));
    format!("[{}] [{}] {}", now, level.to_colored_str(), message)
  }

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
