use colored::*;

pub enum ColorType {
  ERROR,
  WARNING,
  INFO,
  HINT,
}

pub trait Color {
  fn to_colored_str(&self, color_type: ColorType) -> colored::ColoredString;
  fn to_plain_str<'a>(&'a self) -> &'a str;
}

impl Color for String {
  /// Converts a String to a ColoredString
  fn to_colored_str(&self, color_type: ColorType) -> colored::ColoredString {
    match color_type {
      ColorType::ERROR => self.red().bold(),
      ColorType::WARNING => self.yellow().bold(),
      ColorType::INFO => self.cyan().bold(),
      ColorType::HINT => self.green(),
    }
  }

  /// Converts a String to a &str
  fn to_plain_str<'a>(&'a self) -> &'a str {
    self
  }
}
