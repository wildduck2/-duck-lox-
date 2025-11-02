use core::fmt;
use diagnostic::diagnostic::Span;

use crate::stmt::{Stmt, Type};

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

  Array {
    elements: Vec<Expr>,
    span: Span,
  },

  Object {
    type_name: String,
    fields: Vec<(String, Expr)>,
    span: Span,
  },

  Lambda {
    params: Vec<Param>,
    return_type: Option<Type>,
    body: Vec<Stmt>,
    span: Span,
  },

  Match {
    expr: Box<Expr>,
    arms: Vec<MatchArm>,
    span: Span,
  },

  Tuple {
    elements: Vec<Expr>,
    span: Span,
  },

  Grouping {
    expr: Box<Expr>,
    span: Span,
  },

  Comma {
    expressions: Vec<Expr>,
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
pub struct Param {
  pub name: String,
  pub type_annotation: Option<Type>,
  pub default_value: Option<Expr>,
}

#[derive(Debug, Clone)]
pub struct Field {
  pub name: String,
  pub type_annotation: Type,
  pub default_value: Option<Expr>,
}

#[derive(Debug, Clone)]
pub struct FnSignature {
  pub name: String,
  pub params: Vec<Param>,
  pub return_type: Type,
}

#[derive(Debug, Clone)]
pub struct MatchArm {
  pub pattern: Pattern,
  pub guard: Option<Expr>,
  pub body: Vec<Stmt>,
}

#[derive(Debug, Clone)]
pub enum Pattern {
  Wildcard,           // _
  Literal(Expr),      // 42, "hello", true
  Identifier(String), // x, name

  Tuple(Vec<Pattern>), // (x, y, _)

  Struct {
    name: String,                   // Person
    fields: Vec<(String, Pattern)>, // { name: x, age: _ }
  },

  // For tuple-style patterns: Some(x), Point(1, 2)
  TupleStruct {
    name: String,
    patterns: Vec<Pattern>,
  },

  // Path with tuple: Option::Some(x), Result::Ok(val)
  PathTuple {
    path: Vec<String>, // ["Option", "Some"]
    patterns: Vec<Pattern>,
  },

  // Path with struct: User::Name { first, last }
  PathStruct {
    path: Vec<String>, // ["User", "Name"]
    fields: Vec<(String, Pattern)>,
  },

  Array(Vec<Pattern>), // [first, second, rest @ ..]

  // Rest,                                      // ... or @ rest
  Or(Vec<Pattern>), // 1 | 2 | 3

  Range {
    start: Expr,
    end: Expr,
  }, // 1..=10
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

impl fmt::Display for Pattern {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Pattern::Wildcard => write!(f, "_"),
      Pattern::Literal(expr) => write!(f, "{}", expr),
      Pattern::Identifier(name) => write!(f, "{}", name),

      Pattern::Tuple(patterns) => {
        let pattern_str = patterns
          .iter()
          .map(|p| p.to_string())
          .collect::<Vec<_>>()
          .join(", ");
        write!(f, "({})", pattern_str)
      },

      Pattern::Struct { name, fields } => {
        let field_str = fields
          .iter()
          .map(|(k, v)| format!("{}: {}", k, v))
          .collect::<Vec<_>>()
          .join(", ");
        write!(f, "{} {{ {} }}", name, field_str)
      },

      // TupleStruct: Some(x), Point(1, 2)
      Pattern::TupleStruct { name, patterns } => {
        if patterns.is_empty() {
          write!(f, "{}", name)
        } else {
          let pattern_str = patterns
            .iter()
            .map(|p| p.to_string())
            .collect::<Vec<_>>()
            .join(", ");
          write!(f, "{}({})", name, pattern_str)
        }
      },

      // Path: Option::Some(x)
      Pattern::PathTuple { path, patterns } => {
        let path_str = path.join("::");
        if patterns.is_empty() {
          write!(f, "{}", path_str)
        } else {
          let pattern_str = patterns
            .iter()
            .map(|p| p.to_string())
            .collect::<Vec<_>>()
            .join(", ");
          write!(f, "{}({})", path_str, pattern_str)
        }
      },

      Pattern::PathStruct { path, fields } => {
        let path_str = path.join("::");
        let field_str = fields
          .iter()
          .map(|(k, v)| format!("{}: {}", k, v))
          .collect::<Vec<_>>()
          .join(", ");
        write!(f, "{}::{{ {} }}", path_str, field_str)
      },

      Pattern::Array(patterns) => {
        let pattern_str = patterns
          .iter()
          .map(|p| p.to_string())
          .collect::<Vec<_>>()
          .join(", ");
        write!(f, "[{}]", pattern_str)
      },

      Pattern::Or(patterns) => {
        let pattern_str = patterns
          .iter()
          .map(|p| p.to_string())
          .collect::<Vec<_>>()
          .join(" | ");
        write!(f, "{}", pattern_str) // Remove extra parens
      },

      Pattern::Range { start, end } => write!(f, "{}..{}", start, end),
    }
  }
}

impl fmt::Display for Expr {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Expr::Integer { value, .. } => write!(f, "{}", value),
      Expr::Float { value, .. } => write!(f, "{}", value),
      Expr::String { value, .. } => write!(f, "{}", value),
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

      Expr::Comma { expressions, .. } => {
        let element_str = expressions
          .iter()
          .map(|e| e.to_string())
          .collect::<Vec<_>>()
          .join(", ");
        write!(f, "({})", element_str)
      },

      Expr::Array { elements, .. } => {
        let element_str = elements
          .iter()
          .map(|e| e.to_string())
          .collect::<Vec<_>>()
          .join(", ");
        write!(f, "[{}]", element_str)
      },

      Expr::Object {
        type_name, fields, ..
      } => {
        let field_str = fields
          .iter()
          .map(|(k, v)| format!("{}: {}", k, v))
          .collect::<Vec<_>>()
          .join(", ");
        write!(f, "{} {{ {} }}", type_name, field_str)
      },

      Expr::Lambda {
        params,
        return_type,
        body,
        ..
      } => {
        let param_str = params
          .iter()
          .map(|p| {
            let type_annotation = if let Some(ref type_annotation) = p.type_annotation {
              format!(": {}", type_annotation)
            } else {
              String::from("")
            };
            format!("{}: {}", p.name, type_annotation)
          })
          .collect::<Vec<_>>()
          .join(", ");

        let return_type = if let Some(ref return_type) = return_type {
          format!("-> {}", return_type)
        } else {
          String::from("")
        };

        write!(
          f,
          "fn({}) -> {} {{ {} stmts }}",
          param_str,
          return_type,
          body.len()
        )
      },

      Expr::Match { expr, arms, .. } => {
        write!(f, "match {} {{ {} arms }}", expr, arms.len())
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
      Expr::Float { value, .. } => print_node!("Float", format!("{}", value)),
      Expr::String { value, .. } => print_node!("String", format!("{}", value)),
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
        if args.is_empty() {
          print_node!("Call()");
        } else {
          print_node!("Call");
        }
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

      Expr::Object {
        type_name, fields, ..
      } => {
        print_node!("Object", type_name);
        for (i, (field_name, field_value)) in fields.iter().enumerate() {
          let is_last_field = i == fields.len() - 1;
          println!(
            "{}{}{}:",
            new_prefix,
            if is_last_field {
              "└── "
            } else {
              "├── "
            },
            field_name
          );
          let field_prefix = format!(
            "{}{}",
            new_prefix,
            if is_last_field { "    " } else { "│   " }
          );
          field_value.build_tree(&field_prefix, true);
        }
      },

      Expr::Comma { expressions, .. } => {
        let element_str = expressions
          .iter()
          .map(|e| e.to_string())
          .collect::<Vec<_>>()
          .join(", ");
        print_node!("Comma", element_str);
        for (i, element) in expressions.iter().enumerate() {
          element.build_tree(&new_prefix, i == expressions.len() - 1);
        }
      },

      Expr::Lambda {
        params,
        return_type,
        body,
        ..
      } => {
        let return_type_str = if let Some(ref return_type) = return_type {
          format!("-> {}", return_type)
        } else {
          String::from("-> ()")
        };
        print_node!(format!("Lambda() {}", return_type_str));

        // Print params
        let has_body = !body.is_empty();
        for (i, param) in params.iter().enumerate() {
          let is_last_param = i == params.len() - 1 && !has_body;
          let type_annotation = if let Some(ref type_annotation) = param.type_annotation {
            format!(": {}", type_annotation)
          } else {
            String::from("")
          };

          let param_str = if let Some(ref default) = param.default_value {
            format!("{}{} = {}", param.name, type_annotation, default)
          } else {
            format!("{}{}", param.name, type_annotation)
          };
          println!(
            "{}{}param: {}",
            new_prefix,
            if is_last_param {
              "└── "
            } else {
              "├── "
            },
            param_str
          );
        }

        // Print body statements
        if !body.is_empty() {
          println!("{}└── body:", new_prefix);
          let body_prefix = format!("{}    ", new_prefix);
          for (i, stmt) in body.iter().enumerate() {
            stmt.build_tree(&body_prefix, i == body.len() - 1);
          }
        }
      },

      Expr::Match { expr, arms, .. } => {
        print_node!("Match");

        // Print the expression being matched
        println!("{}├── expr:", new_prefix);
        let expr_prefix = format!("{}│   ", new_prefix);
        expr.build_tree(&expr_prefix, true);

        // Print arms
        for (i, arm) in arms.iter().enumerate() {
          let is_last_arm = i == arms.len() - 1;
          println!(
            "{}{}arm {}:",
            new_prefix,
            if is_last_arm {
              "└── "
            } else {
              "├── "
            },
            i + 1
          );
          let arm_prefix = format!(
            "{}{}",
            new_prefix,
            if is_last_arm { "    " } else { "│   " }
          );

          // Count children in this arm
          let has_guard = arm.guard.is_some();
          let has_body = !arm.body.is_empty();

          // Print pattern
          let pattern_connector = if !has_guard && !has_body {
            "└── "
          } else {
            "├── "
          };
          println!(
            "{}{}pattern: {}",
            arm_prefix, pattern_connector, arm.pattern
          );

          // Print guard if exists
          if let Some(ref guard) = arm.guard {
            let guard_connector = if !has_body {
              "└── "
            } else {
              "├── "
            };
            println!("{}{}guard:", arm_prefix, guard_connector);
            let guard_prefix = format!("{}{}", arm_prefix, if !has_body { "    " } else { "│   " });
            guard.build_tree(&guard_prefix, true);
          }

          // Print body
          if has_body {
            println!("{}└── body:", arm_prefix);
            let body_prefix = format!("{}    ", arm_prefix);
            for (j, stmt) in arm.body.iter().enumerate() {
              stmt.build_tree(&body_prefix, j == arm.body.len() - 1);
            }
          }
        }
      },
    }
  }
}
