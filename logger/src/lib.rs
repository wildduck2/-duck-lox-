use colored::*;
use std::{
  fmt,
  fs::{self},
  io::{self, Write},
};

mod colors;

#[derive(Debug)]
pub enum LogType<'a> {
  Error(&'a str),
  Warn(&'a str),
  Info(&'a str),
  Debug(&'a str),
}

impl<'a> fmt::Display for LogType<'a> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      LogType::Error(error) => write!(f, "{} {}", "[Error]".red().bold(), error.red().bold()),
      LogType::Warn(warn) => write!(f, "{} {}", "[Warn]".yellow().bold(), warn.yellow().bold()),
      LogType::Info(info) => write!(f, "{} {}", "[Info]".cyan().bold(), info.cyan().bold()),
      LogType::Debug(plain) => write!(f, "{} {}", "[Debug]", plain),
    }
  }
}

pub struct Logger;

impl Logger {
  pub fn log(log: LogType, option: u8) -> String {
    let now = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

    let log_str = format!("[{}] {}", now, log);
    match option {
      1 => {
        Logger::log_to_file(&log_str);
      },
      2 => {
        Logger::log_to_file(&log_str);
        println!("{}", log_str);
      },
      _ => {
        println!("{}", log_str);
      },
    };
    log_str
  }

  fn log_to_file(log_str: &String) {
    let file_handler = fs::OpenOptions::new()
      .create(true)
      .append(true)
      .open("./tmp/log.txt");

    match file_handler {
      Ok(file) => {
        let mut writer = io::BufWriter::new(file);
        if let Err(e) = writer.write_all(log_str.as_bytes()) {
          eprintln!("Failed to write to log file: {}", e);
        }
      },
      Err(e) => {
        eprintln!("Failed to open or create log file: {}", e);
      },
    }
  }
}
