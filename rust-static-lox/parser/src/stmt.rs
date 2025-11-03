use diagnostic::diagnostic::Span;
use std::fmt;

use crate::expr::Expr;

#[derive(Debug, Clone)]
pub enum Stmt {
  Expr(Expr),

  Decl {
    name: String,
    kind: DeclKind, // Let | Const
    type_annotation: Option<Type>,
    initializer: Option<Expr>,
    is_mutable: bool,
    span: Span,
  },

  If {
    condition: Box<Expr>,
    then_branch: Vec<Stmt>,
    else_branch: Option<Vec<Stmt>>,
    span: Span,
  },

  While {
    condition: Box<Expr>,
    body: Vec<Stmt>,
    span: Span,
  },

  For {
    initializer: Box<Expr>,
    collection: Box<Expr>,
    body: Vec<Stmt>,
    span: Span,
  },

  Block(Vec<Stmt>),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum DeclKind {
  Let,
  Const,
}

#[derive(Debug, Clone)]
pub enum Type {
  Int,
  Float,
  String,
  Bool,
  Void,
  Array(Box<Type>),
  Tuple(Vec<Type>),
  Function {
    params: Vec<Type>,
    return_type: Box<Type>,
  },
  Named(String),
  Generic {
    name: String,
    type_params: Vec<Type>,
  },
  TypeVar(String), // For inference
}

impl fmt::Display for Type {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Type::Int => write!(f, "int"),
      Type::Float => write!(f, "float"),
      Type::String => write!(f, "string"),
      Type::Bool => write!(f, "bool"),
      Type::Void => write!(f, "void"),
      Type::Array(ty) => write!(f, "[{}]", ty),
      Type::Tuple(tys) => write!(
        f,
        "({})",
        tys
          .iter()
          .map(|t| t.to_string())
          .collect::<Vec<_>>()
          .join(", ")
      ),
      Type::Function {
        params,
        return_type,
      } => write!(
        f,
        "fn({}) -> {}",
        params
          .iter()
          .map(|t| t.to_string())
          .collect::<Vec<_>>()
          .join(", "),
        return_type
      ),
      Type::Named(name) => write!(f, "{}", name),
      Type::Generic { name, type_params } => {
        write!(
          f,
          "{}<{}>",
          name,
          type_params
            .iter()
            .map(|t| t.to_string())
            .collect::<Vec<_>>()
            .join(", ")
        )
      },
      Type::TypeVar(name) => write!(f, "{}", name),
    }
  }
}

impl fmt::Display for Stmt {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Stmt::Expr(expr) => write!(f, "ExprStmt({})", expr),
      Stmt::Decl {
        name,
        kind,
        type_annotation,
        initializer,
        is_mutable,
        ..
      } => {
        write!(
          f,
          "Decl(kind: {:?}, name: {}, mutable: {}, type: {}, init: {})",
          kind,
          name,
          is_mutable,
          type_annotation
            .as_ref()
            .map(|t| t.to_string())
            .unwrap_or_else(|| "none".to_string()),
          initializer
            .as_ref()
            .map(|e| e.to_string())
            .unwrap_or_else(|| "none".to_string())
        )
      },
      Stmt::If {
        condition,
        then_branch,
        else_branch,
        ..
      } => {
        write!(
          f,
          "If(condition: {}, then_branch: {:?}, else_branch: {:?})",
          condition, then_branch, else_branch
        )
      },
      Stmt::While {
        condition, body, ..
      } => {
        write!(f, "While(condition: {}, body: {:?})", condition, body)
      },
      Stmt::Block(stmts) => {
        write!(f, "Block({:?})", stmts)
      },
      Stmt::For {
        initializer,
        collection,
        body,
        ..
      } => {
        write!(
          f,
          "For(initializer: {}, collection: {}, body: {:?})",
          initializer, collection, body
        )
      },
    }
  }
}

impl Stmt {
  pub fn print_tree(&self) {
    self.build_tree("", true);
  }

  pub fn build_tree(&self, prefix: &str, is_last: bool) {
    let connector = if is_last { "└── " } else { "├── " };
    println!("{}{}{}", prefix, connector, self.label());

    let new_prefix = format!("{}{}", prefix, if is_last { "    " } else { "│   " });

    match self {
      // ───────────────────────────────
      Stmt::Expr(expr) => {
        expr.build_tree(&new_prefix, true);
      },

      // ───────────────────────────────
      Stmt::Decl {
        name,
        kind,
        type_annotation,
        initializer,
        is_mutable,
        ..
      } => {
        let mut fields = Vec::new();
        fields.push(format!("name: {}", name));
        fields.push(format!("mutable: {}", is_mutable));
        if let Some(ty) = type_annotation {
          fields.push(format!("type: {}", ty));
        }

        // Print all fields except initializer
        for (i, field) in fields.iter().enumerate() {
          let last = i == fields.len() - 1 && initializer.is_none();
          let conn = if last { "└── " } else { "├── " };
          println!("{}{}{}", new_prefix, conn, field);
        }

        // Handle initializer if present
        if let Some(init) = initializer {
          println!("{}└── initializer:", new_prefix);
          init.build_tree(&format!("{}    ", new_prefix), true);
        }
      },

      // ───────────────────────────────
      Stmt::If {
        condition,
        then_branch,
        else_branch,
        ..
      } => {
        // Condition
        println!("{}├── condition:", new_prefix);
        condition.build_tree(&format!("{}│   ", new_prefix), true);

        // Then branch
        println!("{}├── then_branch:", new_prefix);
        let then_prefix = format!("{}│   ", new_prefix);
        for (i, stmt) in then_branch.iter().enumerate() {
          stmt.build_tree(
            &then_prefix,
            i == then_branch.len() - 1 && else_branch.is_none(),
          );
        }

        // Else branch (optional)
        if let Some(else_branch) = else_branch {
          println!("{}└── else_branch:", new_prefix);
          let else_prefix = format!("{}    ", new_prefix);
          for (i, stmt) in else_branch.iter().enumerate() {
            stmt.build_tree(&else_prefix, i == else_branch.len() - 1);
          }
        }
      },

      // ───────────────────────────────
      Stmt::While {
        condition, body, ..
      } => {
        println!("{}├── condition:", new_prefix);
        condition.build_tree(&format!("{}│   ", new_prefix), true);

        println!("{}└── body:", new_prefix);
        let body_prefix = format!("{}    ", new_prefix);
        for (i, stmt) in body.iter().enumerate() {
          stmt.build_tree(&body_prefix, i == body.len() - 1);
        }
      },

      // ───────────────────────────────
      Stmt::Block(stmts) => {
        for (i, stmt) in stmts.iter().enumerate() {
          stmt.build_tree(&new_prefix, i == stmts.len() - 1);
        }
      },
      Stmt::For {
        initializer,
        collection,
        body,
        ..
      } => {
        println!("{}├── initializer:", new_prefix);
        initializer.build_tree(&format!("{}│   ", new_prefix), true);
        println!("{}├── collection:", new_prefix);
        collection.build_tree(&format!("{}│   ", new_prefix), true);
        println!("{}└── body:", new_prefix);
        let body_prefix = format!("{}    ", new_prefix);
        for (i, stmt) in body.iter().enumerate() {
          stmt.build_tree(&body_prefix, i == body.len() - 1);
        }
      },
    }
  }

  fn label(&self) -> String {
    match self {
      Stmt::Expr(_) => "ExprStmt".to_string(),
      Stmt::Decl { kind, .. } => format!("Decl({:?})", kind),
      Stmt::If { .. } => "If".to_string(),
      Stmt::While { .. } => "While".to_string(),
      Stmt::Block(_) => "Block".to_string(),
      Stmt::For { .. } => "For".to_string(),
    }
  }
}
