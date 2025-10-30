use core::fmt;

use lexer::token::Token;

#[derive(Debug, Clone)]
pub enum Expr<'a> {
  Literal(Token<'a>),
  Identifier(Token<'a>),
  Grouping(Box<Expr<'a>>),
  Assign {
    name: Token<'a>,
    rhs: Box<Expr<'a>>,
  },
  Unary {
    operator: Token<'a>,
    rhs: Box<Expr<'a>>,
  },
  Binary {
    lhs: Box<Expr<'a>>,
    operator: Token<'a>,
    rhs: Box<Expr<'a>>,
  },
}

impl<'a> fmt::Display for Expr<'a> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Expr::Literal(token) => write!(f, "{}", token.lexeme),
      Expr::Identifier(token) => write!(f, "{}", token.lexeme),
      Expr::Unary { operator, rhs } => write!(f, "({} {})", operator.lexeme, rhs),
      Expr::Binary { lhs, operator, rhs } => write!(f, "⚙️ ({} {} {})", lhs, operator.lexeme, rhs),
      Expr::Grouping(expr) => write!(f, "({})", expr),
      Expr::Assign { name, rhs } => write!(f, "({} = {})", name.lexeme, rhs),
    }
  }
}

impl<'a> Expr<'a> {
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
    }
  }
}
