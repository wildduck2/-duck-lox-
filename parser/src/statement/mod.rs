use scanner::token::Token;

use crate::expression::Expr;
use std::fmt;

#[derive(Debug, Clone)]
pub enum Stmt {
  Expr(Expr),
  Print(Expr),
  VarDec(Token, Option<Expr>),
}

impl fmt::Display for Stmt {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Stmt::Expr(expr) => write!(f, "📝 ExprStmt({})", expr),
      Stmt::Print(expr) => write!(f, "🖨️  PrintStmt({})", expr),
      Stmt::VarDec(name, Some(expr)) => write!(f, "📝 VarDec({}, {})", name.lexeme, expr),
      Stmt::VarDec(name, None) => write!(f, "📝 VarDec({}, <uninitialized>)", name.lexeme),
    }
  }
}

impl Stmt {
  /// Simple version: prints the statement tree with indentation
  pub fn pretty_print(&self) {
    self.pretty_print_internal(0);
  }

  fn pretty_print_internal(&self, indent: usize) {
    let padding = " ".repeat(indent);
    match self {
      Stmt::Expr(expr) => {
        println!("{}ExpressionStatement", padding);
        expr.pretty_print_internal(indent + 2);
      },
      Stmt::Print(expr) => {
        println!("{}PrintStatement", padding);
        expr.pretty_print_internal(indent + 2);
      },
      Stmt::VarDec(name, Some(expr)) => {
        println!("{}VarDec({}, initialized)", padding, name.lexeme);
        expr.pretty_print_internal(indent + 2);
      },
      Stmt::VarDec(name, None) => {
        println!("{}VarDec({}, uninitialized)", padding, name.lexeme);
      },
    }
  }

  /// ASCII tree version
  pub fn print_tree(&self) {
    let mut lines = Vec::new();
    self.build_tree(&mut lines, "", "", true);
    for line in lines {
      println!("{}", line);
    }
  }

  fn build_tree(&self, lines: &mut Vec<String>, prefix: &str, child_prefix: &str, is_last: bool) {
    let connector = if is_last { "└── " } else { "├── " };
    let label = match self {
      Stmt::Expr(_) => "ExprStmt".to_string(),
      Stmt::Print(_) => "PrintStmt".to_string(),
      Stmt::VarDec(name, _) => format!("VarDec({})", name.lexeme),
    };

    lines.push(format!("{}{}{}", prefix, connector, label));

    let new_prefix = if is_last {
      format!("{}    ", child_prefix)
    } else {
      format!("{}│   ", child_prefix)
    };

    match self {
      Stmt::Expr(expr) => expr.build_tree(lines, &new_prefix, &new_prefix, true),
      Stmt::Print(expr) => expr.build_tree(lines, &new_prefix, &new_prefix, true),
      Stmt::VarDec(_, Some(expr)) => expr.build_tree(lines, &new_prefix, &new_prefix, true),
      Stmt::VarDec(_, None) => {
        lines.push(format!("{}└── <uninitialized>", new_prefix));
      },
    }
  }

  /// Fancy tree with emojis
  pub fn print_fancy_tree(&self) {
    println!();
    self.print_node(String::new(), String::new(), true);
    println!();
  }

  fn print_node(&self, prefix: String, child_prefix: String, is_tail: bool) {
    let label = match self {
      Stmt::Expr(_) => "📝 Expression Statement".to_string(),
      Stmt::Print(_) => "🖨️  Print Statement".to_string(),
      Stmt::VarDec(_, _) => "📝 Variable Declaration".to_string(),
    };

    println!("{}{}{}", prefix, if is_tail { "└─ " } else { "├─ " }, label);

    let new_prefix = format!("{}{}", child_prefix, if is_tail { "   " } else { "│  " });
    let new_child_prefix = new_prefix.clone();

    match self {
      Stmt::Expr(expr) => expr.print_node(new_prefix, new_child_prefix, true),
      Stmt::Print(expr) => expr.print_node(new_prefix, new_child_prefix, true),
      Stmt::VarDec(name, Some(expr)) => {
        println!("{}   ├─ Name: {}", child_prefix, name.lexeme);
        expr.print_node(format!("{}   ", child_prefix), new_child_prefix, true);
      },
      Stmt::VarDec(name, None) => {
        println!("{}   ├─ Name: {}", child_prefix, name.lexeme);
        println!("{}   └─ <uninitialized>", child_prefix);
      },
    }
  }
}
