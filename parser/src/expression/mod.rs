/*
*
*  The expressions
*
*  expression -> literal
*              | unary
*              | binary
*              | grouping;
*
*  literal    -> NUMBER
*              | STRING
*              | "true"
*              | "false"
*              | "nil";
*
*  grouping   -> "(" expression ")" ;
*
*  unary      -> ("-" | "!") expression
*
*  binary     -> expression operator expression
*
*  operator   -> "+" | "-" | "*" | "/"
*                "!=" | "==" | "<=" | ">=" | "<" | ">"
*
*/

use scanner::token::Token;

#[derive(Debug)]
pub enum Expr {
  Literal(Token),
  Unary {
    operator: Token,
    right: Box<Expr>,
  },
  Binary {
    left: Box<Expr>,
    operator: Token,
    right: Box<Expr>,
  },
  Grouping(Box<Expr>),
}

impl Expr {
  /// Simple version: prints the tree with indentation
  /// Each level gets indented by 4 spaces
  pub fn pretty_print(&self) {
    self.pretty_print_internal(0);
  }

  fn pretty_print_internal(&self, indent: usize) {
    let padding = " ".repeat(indent);
    match self {
      Expr::Literal(token) => {
        println!("{}Literal({})", padding, token.lexeme);
      },
      Expr::Unary { operator, right } => {
        println!("{}Unary({})", padding, operator.lexeme);
        right.pretty_print_internal(indent + 2);
      },
      Expr::Binary {
        left,
        operator,
        right,
      } => {
        println!("{}Binary({})", padding, operator.lexeme);
        left.pretty_print_internal(indent + 2);
        right.pretty_print_internal(indent + 2);
      },
      Expr::Grouping(expr) => {
        println!("{}Grouping", padding);
        expr.pretty_print_internal(indent + 2);
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
  fn build_tree(&self, lines: &mut Vec<String>, prefix: &str, child_prefix: &str, is_last: bool) {
    let (node_label, children) = match self {
      Expr::Literal(token) => {
        // Literals are leaf nodes - they have no children
        (format!("({})", token.lexeme), vec![])
      },
      Expr::Unary { operator, right } => {
        // Unary has one child on the right
        (format!("({})", operator.lexeme), vec![right.as_ref()])
      },
      Expr::Binary {
        left,
        operator,
        right,
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

  fn print_node(&self, prefix: String, child_prefix: String, is_tail: bool) {
    let label = match self {
      Expr::Literal(token) => format!("📍 {}", token.lexeme),
      Expr::Unary { operator, .. } => format!("🔧 {}", operator.lexeme),
      Expr::Binary { operator, .. } => format!("⚙️  {}", operator.lexeme),
      Expr::Grouping(_) => "📦 group".to_string(),
    };

    println!("{}{}{}", prefix, if is_tail { "└─ " } else { "├─ " }, label);

    let children = match self {
      Expr::Literal(_) => vec![],
      Expr::Unary { right, .. } => vec![right.as_ref()],
      Expr::Binary { left, right, .. } => vec![left.as_ref(), right.as_ref()],
      Expr::Grouping(expr) => vec![expr.as_ref()],
    };

    for (i, child) in children.iter().enumerate() {
      let is_last = i == children.len() - 1;
      let new_prefix = format!("{}{}", child_prefix, if is_tail { "   " } else { "│  " });
      let new_child_prefix = format!("{}{}", child_prefix, if is_tail { "   " } else { "│  " });
      child.print_node(new_prefix, new_child_prefix, is_last);
    }
  }
}
