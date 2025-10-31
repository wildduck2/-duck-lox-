use core::fmt;

use lexer::token::Token;

#[derive(Debug, Clone)]
pub enum Expr {
  Literal(Token),
  Identifier(Token),
  Grouping(Box<Expr>),
  Assign {
    name: Token,
    rhs: Box<Expr>,
  },
  Unary {
    operator: Token,
    rhs: Box<Expr>,
  },
  Binary {
    lhs: Box<Expr>,
    operator: Token,
    rhs: Box<Expr>,
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
      Expr::Literal(token) => write!(f, "{}", token.lexeme),
      Expr::Identifier(token) => write!(f, "{}", token.lexeme),
      Expr::Unary { operator, rhs } => write!(f, "({} {})", operator.lexeme, rhs),
      Expr::Binary { lhs, operator, rhs } => write!(f, "⚙️ ({} {} {})", lhs, operator.lexeme, rhs),
      Expr::Grouping(expr) => write!(f, "({})", expr),
      Expr::Assign { name, rhs } => write!(f, "({} = {})", name.lexeme, rhs),
      Expr::Ternary {
        condition,
        then_branch,
        else_branch,
      } => write!(f, "({} ? {} : {})", condition, then_branch, else_branch),
    }
  }
}

impl Expr {
  pub(crate) fn build_tree(&self, prefix: &str, is_last: bool) {
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
      ($label:expr, $lexeme:expr) => {
        println!("{}{}{}({})", prefix, connector, $label, $lexeme)
      };
    }

    match self {
      Expr::Literal(token) => print_node!("Literal", token.lexeme),
      Expr::Identifier(token) => print_node!("Identifier", token.lexeme),
      Expr::Binary { lhs, operator, rhs } => {
        print_node!("Binary", operator.lexeme);
        lhs.build_tree(&new_prefix, false);
        rhs.build_tree(&new_prefix, true);
      },
      Expr::Unary { operator, rhs } => {
        print_node!("Unary", operator.lexeme);
        rhs.build_tree(&new_prefix, true);
      },
      Expr::Grouping(expr) => {
        print_node!("Grouping");
        expr.build_tree(&new_prefix, true);
      },
      Expr::Assign { name, rhs } => {
        print_node!("Assign", name.lexeme);
        rhs.build_tree(&new_prefix, true);
      },
      Expr::Ternary {
        condition,
        then_branch,
        else_branch,
      } => {
        print_node!("Ternary");
        condition.build_tree(&new_prefix, false);
        then_branch.build_tree(&new_prefix, false);
        else_branch.build_tree(&new_prefix, true);
      },
    }
  }
}
