use std::fmt;

use scanner::token::Token;

#[derive(Debug, Clone)]
pub enum Expr {
  Literal(Token),
  Identifier(Token),
  Unary {
    operator: Token,
    rhs: Box<Expr>,
  },
  Binary {
    lhs: Box<Expr>,
    operator: Token,
    rhs: Box<Expr>,
  },
  Grouping(Box<Expr>),
  Assign {
    name: Token, // must be IDENTIFIER
    value: Box<Expr>,
  },
  Ternary {
    condition: Box<Expr>,
    then_branch: Box<Expr>,
    else_branch: Box<Expr>,
  },
}

impl fmt::Display for Expr {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Expr::Literal(token) => write!(f, "📍 {}", token.lexeme),
      Expr::Identifier(token) => write!(f, "📍 {}", token.lexeme),
      Expr::Unary { operator, rhs } => write!(f, "🔧 ({} {})", operator.lexeme, rhs),
      Expr::Binary { lhs, operator, rhs } => write!(f, "⚙️ ({} {} {})", lhs, operator.lexeme, rhs),
      Expr::Grouping(expr) => write!(f, "📦 ({})", expr),
      Expr::Assign { name, value } => write!(f, "🔧 ({} = {})", name.lexeme, value),
      Expr::Ternary {
        condition,
        then_branch,
        else_branch,
      } => write!(f, "🔀 ({} ? {} : {})", condition, then_branch, else_branch),
    }
  }
}

impl Expr {
  /// Simple version: prints the tree with indentation
  /// Each level gets indented by 4 spaces
  pub fn pretty_print(&self) {
    self.pretty_print_internal(0);
  }

  pub fn pretty_print_internal(&self, indent: usize) {
    let padding = " ".repeat(indent);
    match self {
      Expr::Literal(token) => {
        println!("{}Literal({})", padding, token.lexeme);
      },
      Expr::Identifier(token) => {
        println!("{}Identifier({})", padding, token.lexeme);
      },
      Expr::Unary {
        operator,
        rhs: right,
      } => {
        println!("{}Unary({})", padding, operator.lexeme);
        right.pretty_print_internal(indent + 2);
      },
      Expr::Binary {
        lhs: left,
        operator,
        rhs: right,
      } => {
        println!("{}Binary({})", padding, operator.lexeme);
        left.pretty_print_internal(indent + 2);
        right.pretty_print_internal(indent + 2);
      },
      Expr::Grouping(expr) => {
        println!("{}Grouping", padding);
        expr.pretty_print_internal(indent + 2);
      },
      Expr::Assign {
        name: operator,
        value: rhs,
      } => {
        println!("{}Assign({})", padding, operator.lexeme);
        rhs.pretty_print_internal(indent + 2);
      },
      Expr::Ternary {
        condition,
        then_branch,
        else_branch,
      } => {
        println!("{}Ternary", padding);
        condition.pretty_print_internal(indent + 2);
        then_branch.pretty_print_internal(indent + 2);
        else_branch.pretty_print_internal(indent + 2);
      },
    }
  }

  /// Advanced version: prints a beautiful ASCII tree like this:
  ///          (-)
  ///         /   \
  ///      (+)     (2)
  ///     /   \
  ///   (8)   (*)
  ///         / \
  ///       (5) (6)
  pub fn print_tree(&self) {
    let mut lines = Vec::new();
    self.build_tree(&mut lines, "", "", true);
    for line in lines {
      println!("{}", line);
    }
  }

  /// Recursively builds the tree representation
  /// This is the core algorithm that constructs the ASCII art
  pub fn build_tree(
    &self,
    lines: &mut Vec<String>,
    prefix: &str,
    child_prefix: &str,
    is_last: bool,
  ) {
    let (node_label, children) = match self {
      Expr::Literal(token) => {
        // Literals are leaf nodes - they have no children
        (format!("({})", token.lexeme), vec![])
      },
      Expr::Identifier(token) => {
        // Literals are leaf nodes - they have no children
        (format!("({})", token.lexeme), vec![])
      },
      Expr::Unary {
        operator,
        rhs: right,
      } => {
        // Unary has one child on the right
        (format!("({})", operator.lexeme), vec![right.as_ref()])
      },
      Expr::Binary {
        lhs: left,
        operator,
        rhs: right,
      } => {
        // Binary has two children: left and right
        (
          format!("({})", operator.lexeme),
          vec![left.as_ref(), right.as_ref()],
        )
      },
      Expr::Grouping(expr) => {
        // Grouping has one child
        ("(group)".to_string(), vec![expr.as_ref()])
      },
      Expr::Assign {
        name: operator,
        value: rhs,
      } => {
        // Grouping has one child
        (format!("({})", operator.lexeme), vec![rhs.as_ref()])
      },
      Expr::Ternary {
        condition,
        then_branch,
        else_branch,
      } => {
        // Grouping has one child
        (
          "(?:)".to_string(),
          vec![
            condition.as_ref(),
            then_branch.as_ref(),
            else_branch.as_ref(),
          ],
        )
      },
    };

    // Add the current node to the output
    // The connector is either "└── " (last child) or "├── " (not last child)
    let connector = if is_last { "└── " } else { "├── " };
    lines.push(format!("{}{}{}", prefix, connector, node_label));

    // Process children
    for (index, child) in children.iter().enumerate() {
      let is_last_child = index == children.len() - 1;

      // Calculate the new prefix for the child
      // If current node is last, use spaces; otherwise use vertical bar
      let new_prefix = if is_last {
        format!("{}    ", child_prefix)
      } else {
        format!("{}│   ", child_prefix)
      };

      // Recursively print the child
      child.build_tree(lines, &new_prefix, &new_prefix, is_last_child);
    }
  }

  /// Even fancier version with branch lines connecting parent to children
  /// This shows the actual "tree" structure more clearly
  pub fn print_fancy_tree(&self) {
    println!();
    self.print_node(String::new(), String::new(), true);
    println!();
  }

  pub fn print_node(&self, prefix: String, child_prefix: String, is_tail: bool) {
    let label = match self {
      Expr::Literal(token) => format!("📍 {}", token.lexeme),
      Expr::Identifier(token) => format!("📍 {}", token.lexeme),
      Expr::Unary { operator, .. } => format!("🔧 {}", operator.lexeme),
      Expr::Binary { operator, .. } => format!("⚙️  {}", operator.lexeme),
      Expr::Grouping(_) => "📦 group".to_string(),
      Expr::Assign { name: operator, .. } => format!("🔧 {}", operator.lexeme),
      Expr::Ternary {
        condition,
        then_branch,
        else_branch,
      } => format!("🔀 ({} ? {} : {})", condition, then_branch, else_branch),
    };

    println!("{}{}{}", prefix, if is_tail { "└─ " } else { "├─ " }, label);

    let children = match self {
      Expr::Literal(_) => vec![],
      Expr::Identifier(_) => vec![],
      Expr::Unary { rhs: right, .. } => vec![right.as_ref()],
      Expr::Binary {
        lhs: left,
        rhs: right,
        ..
      } => vec![left.as_ref(), right.as_ref()],
      Expr::Grouping(expr) => vec![expr.as_ref()],
      Expr::Assign { value: right, .. } => vec![right.as_ref()],
      Expr::Ternary {
        condition,
        then_branch,
        else_branch,
      } => vec![
        condition.as_ref(),
        then_branch.as_ref(),
        else_branch.as_ref(),
      ],
    };

    for (i, child) in children.iter().enumerate() {
      let is_last = i == children.len() - 1;
      let new_prefix = format!("{}{}", child_prefix, if is_tail { "   " } else { "│  " });
      let new_child_prefix = format!("{}{}", child_prefix, if is_tail { "   " } else { "│  " });
      child.print_node(new_prefix, new_child_prefix, is_last);
    }
  }
}
