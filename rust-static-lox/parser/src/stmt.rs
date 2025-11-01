use diagnostic::diagnostic::Span;

use crate::expr::Expr;

#[derive(Debug)]
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

#[derive(Debug, Eq, PartialEq)]
pub enum DeclKind {
  Let,
  Const,
}

#[derive(Debug)]
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

// impl fmt::Display for Stmt {
//   fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//     match self {
//       Stmt::Expr(expr) => write!(f, "ExprStmt({})", expr),
//       Stmt::LetDec { name, rhs } => write!(f, "LetDec({:?}, {:?})", name, rhs),
//     }
//   }
// }
//
// impl Stmt {
//   /// Beautiful ASCII tree output
//   pub fn print_tree(&self) {
//     self.build_tree("", true);
//   }
//
//   fn build_tree(&self, prefix: &str, is_last: bool) {
//     let connector = if is_last { "└── " } else { "├── " };
//     let extension = if is_last { "    " } else { "│   " };
//     let new_prefix = format!("{}{}", prefix, extension);
//
//     match self {
//       Stmt::Expr(expr) => {
//         println!("{}{}ExprStmt", prefix, connector);
//         expr.build_tree(&new_prefix, true);
//       },
//       Stmt::LetDec { name, rhs } => {
//         println!("{}{}LetDec", prefix, connector);
//         println!("{}{}name: {:?}", new_prefix, extension, name);
//         if let Some(rhs) = rhs {
//           rhs.build_tree(&new_prefix, false);
//         }
//       },
//     }
//   }
// }
