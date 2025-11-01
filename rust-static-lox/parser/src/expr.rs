use core::fmt;

use diagnostic::diagnostic::Span;

#[derive(Debug, Clone)]
pub enum Expr {
  Integer {
    value: i64,
    span: Span,
  },
  Float {
    value: f64,
    span: Span,
  },
  String {
    value: String,
    span: Span,
  },
  Bool {
    value: bool,
    span: Span,
  },
  Nil {
    span: Span,
  },
  Identifier {
    name: String,
    span: Span,
  },

  Binary {
    left: Box<Expr>,
    op: BinaryOp,
    right: Box<Expr>,
    span: Span,
  },

  Unary {
    op: UnaryOp,
    expr: Box<Expr>,
    span: Span,
  },

  Call {
    callee: Box<Expr>,
    args: Vec<Expr>,
    span: Span,
  },

  Member {
    object: Box<Expr>,
    field: String,
    span: Span,
  },

  Index {
    object: Box<Expr>,
    index: Box<Expr>,
    span: Span,
  },

  Assign {
    target: Box<Expr>,
    value: Box<Expr>,
    span: Span,
  },

  // Array {
  //   elements: Vec<Expr>,
  //   span: Span,
  // },

  // Object {
  //   type_name: String,
  //   fields: Vec<(String, Expr)>,
  //   span: Span,
  // },

  // Lambda {
  //   params: Vec<Param>,
  //   return_type: Type,
  //   body: Vec<Stmt>,
  //   span: Span,
  // },

  // Match {
  //   expr: Box<Expr>,
  //   arms: Vec<MatchArm>,
  //   span: Span,
  // },
  Grouping {
    expr: Box<Expr>,
    span: Span,
  },

  Ternary {
    condition: Box<Expr>,
    then_branch: Box<Expr>,
    else_branch: Box<Expr>,
    span: Span,
  },
}

#[derive(Debug, Clone)]
pub enum BinaryOp {
  Add,
  Sub,
  Mul,
  Div,
  Mod,
  Power,
  Eq,
  NotEq,
  Less,
  LessEq,
  Greater,
  GreaterEq,
  And,
  Or,
}

#[derive(Debug, Clone)]
pub enum UnaryOp {
  Neg, // -x
  Not, // !x
}

// impl fmt::Display for Expr {
//   fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//     match self {
//       Expr::Literal(token) => write!(f, "{}", token.lexeme),
//       Expr::Identifier(token) => write!(f, "{}", token.lexeme),
//       Expr::Unary { operator, rhs } => write!(f, "({} {})", operator.lexeme, rhs),
//       Expr::Binary { lhs, operator, rhs } => write!(f, "⚙️ ({} {} {})", lhs, operator.lexeme, rhs),
//       Expr::Grouping(expr) => write!(f, "({})", expr),
//       Expr::Assign { name, rhs } => write!(f, "({} = {})", name.lexeme, rhs),
//       Expr::Ternary {
//         condition,
//         then_branch,
//         else_branch,
//       } => write!(f, "({} ? {} : {})", condition, then_branch, else_branch),
//       Expr::Call {
//         callee,
//         paren: _,
//         arguments,
//       } => write!(
//         f,
//         "{}({})",
//         callee,
//         arguments
//           .iter()
//           .map(|a| a.to_string())
//           .collect::<Vec<String>>()
//           .join(", ")
//       ),
//     }
//   }
// }
//
// impl Expr {
//   pub(crate) fn build_tree(&self, prefix: &str, is_last: bool) {
//     let (connector, extension) = if is_last {
//       ("└── ", "    ")
//     } else {
//       ("├── ", "│   ")
//     };
//     let new_prefix = format!("{}{}", prefix, extension);
//
//     macro_rules! print_node {
//       ($label:expr) => {
//         println!("{}{}{}", prefix, connector, $label)
//       };
//       ($label:expr, $lexeme:expr) => {
//         println!("{}{}{}({})", prefix, connector, $label, $lexeme)
//       };
//     }
//
//     match self {
//       Expr::Literal(token) => print_node!("Literal", token.lexeme),
//       Expr::Identifier(token) => print_node!("Identifier", token.lexeme),
//       Expr::Binary { lhs, operator, rhs } => {
//         print_node!("Binary", operator.lexeme);
//         lhs.build_tree(&new_prefix, false);
//         rhs.build_tree(&new_prefix, true);
//       },
//       Expr::Unary { operator, rhs } => {
//         print_node!("Unary", operator.lexeme);
//         rhs.build_tree(&new_prefix, true);
//       },
//       Expr::Grouping(expr) => {
//         print_node!("Grouping");
//         expr.build_tree(&new_prefix, true);
//       },
//       Expr::Assign { name, rhs } => {
//         print_node!("Assign", name.lexeme);
//         rhs.build_tree(&new_prefix, true);
//       },
//       Expr::Ternary {
//         condition,
//         then_branch,
//         else_branch,
//       } => {
//         print_node!("Ternary");
//         condition.build_tree(&new_prefix, false);
//         then_branch.build_tree(&new_prefix, false);
//         else_branch.build_tree(&new_prefix, true);
//       },
//       Expr::Call {
//         callee,
//         paren: _,
//         arguments,
//       } => {
//         print_node!("Call");
//         let total_children = 1 + arguments.len();
//         callee.build_tree(&new_prefix, arguments.is_empty());
//         for (i, arg) in arguments.iter().enumerate() {
//           let is_last_arg = i == arguments.len() - 1;
//           arg.build_tree(&new_prefix, is_last_arg);
//         }
//       },
//     }
//   }
// }
