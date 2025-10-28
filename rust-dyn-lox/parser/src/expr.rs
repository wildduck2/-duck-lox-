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
  Assign {
    name: Token, // must be IDENTIFIER
    value: Box<Expr>,
  },
  Ternary {
    condition: Box<Expr>,
    then_branch: Box<Expr>,
    else_branch: Box<Expr>,
  },
  Call {
    callee: Box<Expr>,
    paren: Token,
    arguments: Vec<Expr>,
  },
  Grouping(Box<Expr>),
  Get {
    object: Box<Expr>,
    name: Token,
  },
  Set {
    object: Box<Expr>,
    name: Token,
    value: Box<Expr>,
  },
  This(Token),
  Super(Token, Token),
}

impl fmt::Display for Expr {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Expr::Literal(token) => write!(f, "{}", token.lexeme),
      Expr::Identifier(token) => write!(f, "{}", token.lexeme),
      Expr::Unary { operator, rhs } => write!(f, "({} {})", operator.lexeme, rhs),
      Expr::Binary { lhs, operator, rhs } => write!(f, "⚙️ ({} {} {})", lhs, operator.lexeme, rhs),
      Expr::Grouping(expr) => write!(f, "({})", expr),
      Expr::Assign { name, value } => write!(f, "({} = {})", name.lexeme, value),
      Expr::Ternary {
        condition,
        then_branch,
        else_branch,
      } => write!(f, "({} ? {} : {})", condition, then_branch, else_branch),
      Expr::Call {
        callee, arguments, ..
      } => {
        let args = arguments
          .iter()
          .map(|a| format!("{}", a))
          .collect::<Vec<_>>()
          .join(", ");
        write!(f, "{}({})", callee, args)
      },
      Expr::Get { object, name } => {
        write!(f, "({}.{})", object, name.lexeme)
      },

      Expr::Set {
        object,
        name,
        value,
      } => {
        write!(f, "({}.{} = {})", object, name.lexeme, value)
      },
      Expr::This(token) => {
        write!(f, "this")
      },
      Expr::Super(token, name) => {
        write!(f, "super.{}", name.lexeme)
      },
    }
  }
}

// In expr.rs
impl Expr {
  pub(crate) fn build_tree(&self, prefix: &str, is_last: bool) {
    let connector = if is_last { "└── " } else { "├── " };
    let extension = if is_last { "    " } else { "│   " };

    match self {
      Expr::Literal(token) => {
        println!("{}{}Literal({})", prefix, connector, token.lexeme);
      },

      Expr::Identifier(token) => {
        println!("{}{}Identifier({})", prefix, connector, token.lexeme);
      },

      Expr::Binary { lhs, operator, rhs } => {
        println!("{}{}Binary({})", prefix, connector, operator.lexeme);
        let new_prefix = format!("{}{}", prefix, extension);
        lhs.build_tree(&new_prefix, false);
        rhs.build_tree(&new_prefix, true);
      },

      Expr::Unary { operator, rhs } => {
        println!("{}{}Unary({})", prefix, connector, operator.lexeme);
        rhs.build_tree(&format!("{}{}", prefix, extension), true);
      },

      Expr::Grouping(expr) => {
        println!("{}{}Grouping", prefix, connector);
        expr.build_tree(&format!("{}{}", prefix, extension), true);
      },

      Expr::Assign { name, value } => {
        println!("{}{}Assign({})", prefix, connector, name.lexeme);
        value.build_tree(&format!("{}{}", prefix, extension), true);
      },

      Expr::Call {
        callee,
        paren: _,
        arguments,
      } => {
        println!("{}{}Call", prefix, connector);
        let new_prefix = format!("{}{}", prefix, extension);

        println!("{}├── callee:", new_prefix);
        callee.build_tree(&format!("{}│   ", new_prefix), true);

        if !arguments.is_empty() {
          println!("{}└── arguments:", new_prefix);
          let arg_prefix = format!("{}    ", new_prefix);
          for (i, arg) in arguments.iter().enumerate() {
            arg.build_tree(&arg_prefix, i == arguments.len() - 1);
          }
        }
      },

      Expr::Ternary {
        condition,
        then_branch,
        else_branch,
      } => {
        println!("{}{}Ternary", prefix, connector);
        let new_prefix = format!("{}{}", prefix, extension);

        println!("{}├── condition:", new_prefix);
        condition.build_tree(&format!("{}│   ", new_prefix), true);

        println!("{}├── then:", new_prefix);
        then_branch.build_tree(&format!("{}│   ", new_prefix), true);

        println!("{}└── else:", new_prefix);
        else_branch.build_tree(&format!("{}    ", new_prefix), true);
      },
      Expr::Get { object, name } => {
        println!("{}{}Get({})", prefix, connector, name.lexeme);
        let new_prefix = format!("{}{}", prefix, extension);
        println!("{}└── object:", new_prefix);
        object.build_tree(&format!("{}    ", new_prefix), true);
      },

      Expr::Set {
        object,
        name,
        value,
      } => {
        println!("{}{}Set({})", prefix, connector, name.lexeme);
        let new_prefix = format!("{}{}", prefix, extension);

        println!("{}├── object:", new_prefix);
        object.build_tree(&format!("{}│   ", new_prefix), true);

        println!("{}└── value:", new_prefix);
        value.build_tree(&format!("{}    ", new_prefix), true);
      },
      Expr::This(token) => {
        println!("{}{}This", prefix, connector);
      },
      Expr::Super(token, name) => {
        println!("{}{}Super", prefix, connector);
      },
    }
  }
}
