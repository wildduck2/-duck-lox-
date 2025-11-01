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

  Tuple {
    elements: Vec<Expr>,
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
  Array {
    elements: Vec<Expr>,
    span: Span,
  },
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

impl fmt::Display for BinaryOp {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let op = match self {
      BinaryOp::Add => "+",
      BinaryOp::Sub => "-",
      BinaryOp::Mul => "*",
      BinaryOp::Div => "/",
      BinaryOp::Mod => "%",
      BinaryOp::Power => "^",
      BinaryOp::Eq => "==",
      BinaryOp::NotEq => "!=",
      BinaryOp::Less => "<",
      BinaryOp::LessEq => "<=",
      BinaryOp::Greater => ">",
      BinaryOp::GreaterEq => ">=",
      BinaryOp::And => "and",
      BinaryOp::Or => "or",
    };
    write!(f, "{}", op)
  }
}

impl fmt::Display for UnaryOp {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let op = match self {
      UnaryOp::Neg => "-",
      UnaryOp::Not => "!",
    };
    write!(f, "{}", op)
  }
}

impl fmt::Display for Expr {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Expr::Integer { value, .. } => write!(f, "{}", value),
      Expr::Float { value, .. } => write!(f, "{}", value),
      Expr::String { value, .. } => write!(f, "\"{}\"", value),
      Expr::Bool { value, .. } => write!(f, "{}", value),
      Expr::Nil { .. } => write!(f, "nil"),
      Expr::Identifier { name, .. } => write!(f, "{}", name),
      Expr::Tuple { elements, .. } => {
        let element_str = elements
          .iter()
          .map(|e| e.to_string())
          .collect::<Vec<_>>()
          .join(", ");
        write!(f, "({})", element_str)
      },

      Expr::Unary { op, expr, .. } => write!(f, "({}{})", op, expr),
      Expr::Binary {
        left, op, right, ..
      } => write!(f, "({} {} {})", left, op, right),
      Expr::Assign { target, value, .. } => write!(f, "({} = {})", target, value),

      Expr::Call { callee, args, .. } => {
        let arg_str = args
          .iter()
          .map(|a| a.to_string())
          .collect::<Vec<_>>()
          .join(", ");
        write!(f, "{}({})", callee, arg_str)
      },

      Expr::Member { object, field, .. } => write!(f, "{}.{}", object, field),
      Expr::Index { object, index, .. } => write!(f, "{}[{}]", object, index),
      Expr::Grouping { expr, .. } => write!(f, "({})", expr),

      Expr::Ternary {
        condition,
        then_branch,
        else_branch,
        ..
      } => write!(f, "({} ? {} : {})", condition, then_branch, else_branch),

      Expr::Array { elements, .. } => {
        let element_str = elements
          .iter()
          .map(|e| e.to_string())
          .collect::<Vec<_>>()
          .join(", ");
        write!(f, "[{}]", element_str)
      },
    }
  }
}

impl Expr {
  pub fn build_tree(&self, prefix: &str, is_last: bool) {
    let (connector, extension) = if is_last {
      ("└── ", "    ")
    } else {
      ("├── ", "│   ")
    };
    let new_prefix = format!("{}{}", prefix, extension);

    macro_rules! print_node {
      ($label:expr) => {
        println!("{}{}{}", prefix, connector, $label)
      };
      ($label:expr, $value:expr) => {
        println!("{}{}{}({})", prefix, connector, $label, $value)
      };
    }

    match self {
      Expr::Integer { value, .. } => print_node!("Integer", value),
      Expr::Float { value, .. } => print_node!("Float", value),
      Expr::String { value, .. } => print_node!("String", value),
      Expr::Bool { value, .. } => print_node!("Bool", value),
      Expr::Nil { .. } => print_node!("Nil"),
      Expr::Identifier { name, .. } => print_node!("Identifier", name),

      Expr::Unary { op, expr, .. } => {
        print_node!("Unary", op);
        expr.build_tree(&new_prefix, true);
      },

      Expr::Binary {
        left, op, right, ..
      } => {
        print_node!("Binary", op);
        left.build_tree(&new_prefix, false);
        right.build_tree(&new_prefix, true);
      },

      Expr::Assign { target, value, .. } => {
        print_node!("Assign");
        target.build_tree(&new_prefix, false);
        value.build_tree(&new_prefix, true);
      },

      Expr::Call { callee, args, .. } => {
        print_node!("Call");
        callee.build_tree(&new_prefix, args.is_empty());
        for (i, arg) in args.iter().enumerate() {
          arg.build_tree(&new_prefix, i == args.len() - 1);
        }
      },

      Expr::Member { object, field, .. } => {
        print_node!("Member", field);
        object.build_tree(&new_prefix, true);
      },

      Expr::Index { object, index, .. } => {
        print_node!("Index");
        object.build_tree(&new_prefix, false);
        index.build_tree(&new_prefix, true);
      },

      Expr::Grouping { expr, .. } => {
        print_node!("Grouping");
        expr.build_tree(&new_prefix, true);
      },

      Expr::Ternary {
        condition,
        then_branch,
        else_branch,
        ..
      } => {
        print_node!("Ternary");
        condition.build_tree(&new_prefix, false);
        then_branch.build_tree(&new_prefix, false);
        else_branch.build_tree(&new_prefix, true);
      },
      Expr::Tuple { elements, .. } => {
        print_node!("Tuple");
        for (i, element) in elements.iter().enumerate() {
          element.build_tree(&new_prefix, i == elements.len() - 1);
        }
      },

      Expr::Array { elements, .. } => {
        print_node!("Array");
        for (i, element) in elements.iter().enumerate() {
          element.build_tree(&new_prefix, i == elements.len() - 1);
        }
      },
    }
  }
}
