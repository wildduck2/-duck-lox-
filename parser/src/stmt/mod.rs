/*
*
* program      → declaration* EOF ;
* declaration  → expression_statements
*               | print_statement;
*
* expression_statements → expression ";" ;
* print_statement → "print" expression ";" ;
*
* expression   → comma ;
* comma        → ternary ( "," ternary )* ;
* ternary      → assignment ( "?" expression ":" ternary )? ;
* assignment   → IDENTIFIER "=" assignment
*               | equality ;
* equality     → comparison ( ( "!=" | "==" ) comparison )* ;
* comparison   → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
* term         → factor ( ( "-" | "+" ) factor )* ;
* factor       → unary ( ( "/" | "*" ) unary )* ;
* unary        → ( "!" | "-" ) unary
*               | primary ;
* primary      → NUMBER | STRING | IDENTIFIER
*               | "true" | "false" | "nil"
*               | "(" expression ")" ;
*/

use scanner::token::Token;

use crate::expr::Expr;
use std::fmt;

#[derive(Debug, Clone)]
pub enum Stmt {
  Expr(Expr),
  Print(Expr),
  VarDec(Token, Option<Expr>),
  Block(Box<Vec<Stmt>>),
  If(Box<Expr>, Box<Stmt>, Option<Box<Stmt>>),
  While(Box<Expr>, Box<Stmt>),
  For(Box<Stmt>, Box<Expr>, Box<Expr>, Box<Stmt>),
}

impl fmt::Display for Stmt {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Stmt::Expr(expr) => write!(f, "📝 ExprStmt({})", expr),
      Stmt::Print(expr) => write!(f, "🖨️  PrintStmt({})", expr),
      Stmt::VarDec(name, Some(expr)) => write!(f, "📝 VarDec({}, {})", name.lexeme, expr),
      Stmt::VarDec(name, None) => write!(f, "📝 VarDec({}, <uninitialized>)", name.lexeme),
      Stmt::Block(stmts) => write!(f, "📝 BlockStmt({:?})", stmts),
      Stmt::If(condition, then_branch, Some(else_branch)) => write!(
        f,
        "📝 IfStmt({}, {}, {})",
        condition, then_branch, else_branch
      ),
      Stmt::If(condition, then_branch, None) => {
        write!(f, "📝 IfStmt({}, {}, <nil>)", condition, then_branch)
      },
      Stmt::While(condition, body) => write!(f, "📝 WhileStmt({}, {})", condition, body),
      Stmt::For(initializer, condition, increment, body) => write!(
        f,
        "📝 ForStmt({}, {}, {}, {})",
        initializer, condition, increment, body
      ),
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
      Stmt::For(initializer, condition, increment, body) => {
        println!("{}ForStatement", padding);
        initializer.pretty_print_internal(indent + 2);
        condition.pretty_print_internal(indent + 2);
        increment.pretty_print_internal(indent + 2);
        body.pretty_print_internal(indent + 2);
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
      Stmt::Block(_) => "BlockStmt".to_string(),
      Stmt::If(_, _, _) => "IfStmt".to_string(),
      Stmt::While(_, _) => "WhileStmt".to_string(),
      Stmt::For(_, _, _, _) => "ForStmt".to_string(),
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
      Stmt::Block(stmts) => {
        for (i, stmt) in stmts.clone().into_iter().enumerate() {
          if i == stmts.len() - 1 {
            stmt.build_tree(lines, &new_prefix, &new_prefix, true);
          } else {
            stmt.build_tree(lines, &new_prefix, &new_prefix, false);
          }
        }
      },
      Stmt::If(condition, then_branch, else_branch) => {
        condition.build_tree(lines, &new_prefix, &new_prefix, true);
        then_branch.build_tree(lines, &new_prefix, &new_prefix, true);
        if let Some(else_branch) = else_branch {
          else_branch.build_tree(lines, &new_prefix, &new_prefix, true);
        }
      },
      Stmt::While(condition, body) => {
        condition.build_tree(lines, &new_prefix, &new_prefix, true);
        body.build_tree(lines, &new_prefix, &new_prefix, true);
      },
      Stmt::For(initializer, condition, increment, body) => {
        initializer.build_tree(lines, &new_prefix, &new_prefix, true);
        condition.build_tree(lines, &new_prefix, &new_prefix, true);
        increment.build_tree(lines, &new_prefix, &new_prefix, true);
        body.build_tree(lines, &new_prefix, &new_prefix, true);
      },
    }
  }
}
