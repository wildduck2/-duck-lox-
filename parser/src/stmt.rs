use crate::expr::Expr;
use scanner::token::Token;
use std::fmt;

#[derive(Debug, Clone)]
pub enum Stmt {
  Expr(Expr),
  VarDecl(Token, Option<Expr>),
  Block(Box<Vec<Stmt>>),
  If(Box<Expr>, Box<Stmt>, Option<Box<Stmt>>),
  While(Box<Expr>, Box<Stmt>),
  Fun(Expr, Vec<Expr>, Box<Stmt>),
  Class(Expr, Box<Vec<Stmt>>),
  Return(Token, Option<Expr>),
  Break(Token),
  Continue(Token),
}

impl fmt::Display for Stmt {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Stmt::Expr(expr) => write!(f, "ExprStmt({})", expr),
      Stmt::VarDecl(name, Some(expr)) => {
        write!(f, "VarDec({}, {})", name.lexeme, expr)
      },
      Stmt::VarDecl(name, None) => {
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
        write!(f, "Fun({}, [", name)?;
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
      Stmt::Break(token) => {
        write!(f, "Break({})", token.lexeme)
      },
      Stmt::Continue(token) => {
        write!(f, "Continue({})", token.lexeme)
      },
      Stmt::Class(name, stmts) => {
        write!(f, "Class({}, [...])", name)
      },
    }
  }
}

impl Stmt {
  /// Beautiful ASCII tree output
  pub fn print_tree(&self) {
    self.build_tree("", true);
  }

  fn build_tree(&self, prefix: &str, is_last: bool) {
    let connector = if is_last { "└── " } else { "├── " };
    let extension = if is_last { "    " } else { "│   " };

    match self {
      Stmt::Expr(expr) => {
        println!("{}{}ExprStmt", prefix, connector);
        expr.build_tree(&format!("{}{}", prefix, extension), true);
      },

      Stmt::VarDecl(name, value) => {
        println!("{}{}VarDecl({})", prefix, connector, name.lexeme);
        if let Some(expr) = value {
          expr.build_tree(&format!("{}{}", prefix, extension), true);
        } else {
          println!("{}{}└── <uninitialized>", prefix, extension);
        }
      },

      Stmt::Block(stmts) => {
        println!("{}{}Block", prefix, connector);
        let new_prefix = format!("{}{}", prefix, extension);
        for (i, stmt) in stmts.iter().enumerate() {
          stmt.build_tree(&new_prefix, i == stmts.len() - 1);
        }
      },

      Stmt::If(condition, then_branch, else_branch) => {
        println!("{}{}If", prefix, connector);
        let new_prefix = format!("{}{}", prefix, extension);

        // Condition
        println!("{}├── condition:", new_prefix);
        condition.build_tree(&format!("{}│   ", new_prefix), true);

        // Then branch
        let has_else = else_branch.is_some();
        println!(
          "{}{}then:",
          new_prefix,
          if has_else { "├── " } else { "└── " }
        );
        then_branch.build_tree(
          &format!("{}{}", new_prefix, if has_else { "│   " } else { "    " }),
          true,
        );

        // Else branch
        if let Some(else_stmt) = else_branch {
          println!("{}└── else:", new_prefix);
          else_stmt.build_tree(&format!("{}    ", new_prefix), true);
        }
      },

      Stmt::While(condition, body) => {
        println!("{}{}While", prefix, connector);
        let new_prefix = format!("{}{}", prefix, extension);

        println!("{}├── condition:", new_prefix);
        condition.build_tree(&format!("{}│   ", new_prefix), true);

        println!("{}└── body:", new_prefix);
        body.build_tree(&format!("{}    ", new_prefix), true);
      },

      Stmt::Fun(name, params, body) => {
        let params_str = params
          .iter()
          .map(|p| match p {
            Expr::Identifier(t) => t.lexeme.clone(),
            _ => format!("{}", p),
          })
          .collect::<Vec<_>>()
          .join(", ");

        println!("{}{}Fun({}, [{}])", prefix, connector, name, params_str);
        let new_prefix = format!("{}{}", prefix, extension);
        println!("{}└── body:", new_prefix);
        body.build_tree(&format!("{}    ", new_prefix), true);
      },

      Stmt::Return(_, value) => {
        println!("{}{}Return", prefix, connector);
        if let Some(expr) = value {
          expr.build_tree(&format!("{}{}", prefix, extension), true);
        } else {
          println!("{}{}└── <nil>", prefix, extension);
        }
      },

      Stmt::Break(_) => {
        println!("{}{}Break", prefix, connector);
      },

      Stmt::Continue(_) => {
        println!("{}{}Continue", prefix, connector);
      },

      Stmt::Class(name, methods) => {
        println!("{}{}Class({})", prefix, connector, name);
        let new_prefix = format!("{}{}", prefix, extension);
        for (i, method) in methods.iter().enumerate() {
          method.build_tree(&new_prefix, i == methods.len() - 1);
        }
      },
    }
  }
}
