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
  ConstDecl {
    name: String,
    type_annotation: Option<Type>,
    initializer: Option<Expr>,
    span: Span,
  },
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
      Stmt::ConstDecl {
        name,
        type_annotation,
        initializer,
        ..
      } => {
        write!(
          f,
          "ConstDecl(name: {}, type: {}, init: {})",
          name,
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
    }
  }
}

impl Stmt {
  pub fn print_tree(&self) {
    self.build_tree("", true);
  }

  pub fn build_tree(&self, prefix: &str, is_last: bool) {
    let connector = if is_last { "└── " } else { "├── " };
    let extension = if is_last { "    " } else { "│   " };
    let new_prefix = format!("{}{}", prefix, extension);

    match self {
      Stmt::Expr(expr) => {
        println!("{}{}ExprStmt", prefix, connector);
        expr.build_tree(&new_prefix, true);
      },
      Stmt::Decl {
        name,
        kind,
        type_annotation,
        initializer,
        is_mutable,
        ..
      } => {
        println!("{}{}Decl({:?})", prefix, connector, kind);

        // Count how many children we have
        let has_type = type_annotation.is_some();
        let has_init = initializer.is_some();
        let total_fields = 2 + (if has_type { 1 } else { 0 }) + (if has_init { 1 } else { 0 });
        let mut field_count = 0;

        field_count += 1;
        let is_last_field = field_count == total_fields && !has_init;
        println!(
          "{}{}name: {}",
          new_prefix,
          if is_last_field {
            "└── "
          } else {
            "├── "
          },
          name
        );

        field_count += 1;
        let is_last_field = field_count == total_fields && !has_init;
        println!(
          "{}{}mutable: {}",
          new_prefix,
          if is_last_field {
            "└── "
          } else {
            "├── "
          },
          is_mutable
        );

        if let Some(ty) = type_annotation {
          field_count += 1;
          let is_last_field = field_count == total_fields && !has_init;
          println!(
            "{}{}type: {}",
            new_prefix,
            if is_last_field {
              "└── "
            } else {
              "├── "
            },
            ty
          );
        }

        if let Some(init) = initializer {
          println!("{}{}initializer:", new_prefix, "└── ");
          let init_prefix = format!("{}    ", new_prefix);
          init.build_tree(&init_prefix, true);
        }
      },
      Stmt::ConstDecl {
        name,
        type_annotation,
        initializer,
        ..
      } => {
        println!("{}{}ConstDecl", prefix, connector);

        let has_type = type_annotation.is_some();
        let has_init = initializer.is_some();
        let total_fields = 1 + (if has_type { 1 } else { 0 }) + (if has_init { 1 } else { 0 });
        let mut field_count = 0;

        field_count += 1;
        let is_last_field = field_count == total_fields && !has_init;
        println!(
          "{}{}name: {}",
          new_prefix,
          if is_last_field {
            "└── "
          } else {
            "├── "
          },
          name
        );

        if let Some(ty) = type_annotation {
          field_count += 1;
          let is_last_field = field_count == total_fields && !has_init;
          println!(
            "{}{}type: {}",
            new_prefix,
            if is_last_field {
              "└── "
            } else {
              "├── "
            },
            ty
          );
        }

        if let Some(init) = initializer {
          println!("{}{}initializer:", new_prefix, "└── ");
          let init_prefix = format!("{}    ", new_prefix);
          init.build_tree(&init_prefix, true);
        }
      },
    }
  }
}
