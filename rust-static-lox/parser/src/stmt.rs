use crate::expr::Expr;
use core::fmt;

#[derive(Debug, Clone)]
pub enum Stmt<'a> {
  Expr(Expr<'a>),
}

impl<'a> fmt::Display for Stmt<'a> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Stmt::Expr(expr) => write!(f, "ExprStmt({})", expr),
    }
  }
}

impl<'a> Stmt<'a> {
  /// Beautiful ASCII tree output
  pub fn print_tree(&self) {
    self.build_tree("", true);
  }

  fn build_tree(&self, prefix: &str, is_last: bool) {
    let connector = if is_last { "└── " } else { "├── " };
    let extension = if is_last { "    " } else { "│   " };
    let new_prefix = format!("{}{}", prefix, extension);

    match self {
      Stmt::Expr(expr) => {
        println!("{}{}ExprStmt", prefix, connector);
        expr.build_tree(&new_prefix, true);
      },
    }
  }
}
