use scanner::token::Token;

use crate::expr::Expr;
use std::fmt;

#[derive(Debug, Clone)]
pub enum Stmt {
  Expr(Expr),
  VarDec(Token, Option<Expr>),
  Block(Box<Vec<Stmt>>),
  If(Box<Expr>, Box<Stmt>, Option<Box<Stmt>>),
  While(Box<Expr>, Box<Stmt>),
  Fun(Expr, Vec<Expr>, Box<Stmt>),
  Return(Token, Option<Expr>),
}

impl fmt::Display for Stmt {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Stmt::Expr(expr) => write!(f, "ExprStmt({})", expr),

      Stmt::VarDec(name, Some(expr)) => {
        write!(f, "VarDec({}, {})", name.lexeme, expr)
      },
      Stmt::VarDec(name, None) => {
        write!(f, "VarDec({}, <uninitialized>)", name.lexeme)
      },

      Stmt::Block(stmts) => {
        write!(f, "BlockStmt([")?;
        for (i, stmt) in stmts.iter().enumerate() {
          if i > 0 {
            write!(f, ", ")?;
          }
          write!(f, "{}", stmt)?;
        }
        write!(f, "])")
      },

      Stmt::If(condition, then_branch, Some(else_branch)) => write!(
        f,
        "IfStmt(cond: {}, then: {}, else: {})",
        condition, then_branch, else_branch
      ),
      Stmt::If(condition, then_branch, None) => write!(
        f,
        "IfStmt(cond: {}, then: {}, else: <nil>)",
        condition, then_branch
      ),

      Stmt::While(condition, body) => {
        write!(f, "WhileStmt(cond: {}, body: {})", condition, body)
      },
      Stmt::Fun(name, params, body) => {
        write!(f, "Fun({}, ddd[", name)?;
        for (i, param) in params.iter().enumerate() {
          if i > 0 {
            write!(f, ", ")?;
          }
          write!(f, "{}", param)?;
        }
        write!(f, "], {})", body)
      },
      Stmt::Return(token, Some(value)) => {
        write!(f, "Return({}, {})", token.lexeme, value)
      },
      Stmt::Return(token, None) => {
        write!(f, "Return({})", token.lexeme)
      },
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
      Stmt::VarDec(name, Some(expr)) => {
        println!("{}VarDec({}, initialized)", padding, name.lexeme);
        expr.pretty_print_internal(indent + 2);
      },
      Stmt::VarDec(name, None) => {
        println!("{}VarDec({}, uninitialized)", padding, name.lexeme);
      },
      Stmt::Block(stmts) => {
        println!("{}BlockStatement", padding);
        for stmt in stmts.clone().into_iter() {
          stmt.pretty_print_internal(indent + 2);
        }
      },
      Stmt::If(condition, then_branch, else_branch) => {
        println!("{}IfStatement", padding);
        condition.pretty_print_internal(indent + 2);
        then_branch.pretty_print_internal(indent + 2);
        if let Some(else_branch) = else_branch {
          else_branch.pretty_print_internal(indent + 2);
        }
      },
      Stmt::While(condition, body) => {
        println!("{}WhileStatement", padding);
        condition.pretty_print_internal(indent + 2);
        body.pretty_print_internal(indent + 2);
      },
      Stmt::Fun(name, params, body) => {
        let params_str = params
          .iter()
          .map(|p| match p {
            Expr::Identifier(name) => name.lexeme.clone(),
            Expr::Literal(token) => token.lexeme.clone(),

            _ => String::new(),
          })
          .collect::<Vec<_>>()
          .join(", ");
        println!("{}Fun({}, [{}])", padding, name, params_str);
        body.pretty_print_internal(indent + 2);
      },
      Stmt::Return(token, None) => {
        println!("{}Return({})", padding, token.lexeme);
      },
      Stmt::Return(token, Some(value)) => {
        println!("{}Return({}, {})", padding, token.lexeme, value);
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
      Stmt::VarDec(name, _) => format!("VarDec({})", name.lexeme),
      Stmt::Block(_) => "BlockStmt".to_string(),
      Stmt::If(_, _, _) => "IfStmt".to_string(),
      Stmt::While(_, _) => "WhileStmt".to_string(),
      Stmt::Fun(name, params, _) => {
        let args = params
          .iter()
          .map(|a| format!("{}", a))
          .collect::<Vec<_>>()
          .join(", ");

        format!("Fun({}, [{}])", name, args)
      },
      Stmt::Return(_, _) => "ReturnStmt".to_string(),
    };

    lines.push(format!("{}{}{}", prefix, connector, label));

    let new_prefix = if is_last {
      format!("{}    ", child_prefix)
    } else {
      format!("{}│   ", child_prefix)
    };

    match self {
      Stmt::Expr(expr) => expr.build_tree(lines, &new_prefix, &new_prefix, true),
      Stmt::VarDec(_, Some(expr)) => expr.build_tree(lines, &new_prefix, &new_prefix, true),
      Stmt::VarDec(_, None) => {
        lines.push(format!("{}└── <uninitialized>", new_prefix));
      },
      Stmt::Block(stmts) => {
        for (i, stmt) in stmts.iter().enumerate() {
          stmt.build_tree(lines, &new_prefix, &new_prefix, i == stmts.len() - 1);
        }
      },
      Stmt::If(condition, then_branch, else_branch) => {
        condition.build_tree(lines, &new_prefix, &new_prefix, false);
        then_branch.build_tree(lines, &new_prefix, &new_prefix, else_branch.is_none());
        if let Some(else_branch) = else_branch {
          else_branch.build_tree(lines, &new_prefix, &new_prefix, true);
        }
      },
      Stmt::While(condition, body) => {
        condition.build_tree(lines, &new_prefix, &new_prefix, false);
        body.build_tree(lines, &new_prefix, &new_prefix, true);
      },
      Stmt::Fun(_, _, body) => {
        body.build_tree(lines, &new_prefix, &new_prefix, true);
      },
      Stmt::Return(_, None) => {
        lines.push(format!("{}└── <nil>", new_prefix));
      },
      Stmt::Return(_, Some(value)) => {
        value.build_tree(lines, &new_prefix, &new_prefix, true);
      },
    }
  }
}
