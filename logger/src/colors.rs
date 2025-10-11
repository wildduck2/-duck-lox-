use colored::*;

enum ColorType {
  Error,
  Warn,
  Info,
}

trait Color {
  fn to_colred_str(&self, color_type: ColorType) -> colored::ColoredString;
}

impl Color for String {
  fn to_colred_str(&self, color_type: ColorType) -> colored::ColoredString {
    match color_type {
      ColorType::Error => self.red().bold(),
      ColorType::Warn => self.yellow().bold(),
      ColorType::Info => self.cyan().bold(),
    }
  }
}
