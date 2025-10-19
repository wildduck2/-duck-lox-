use parser::expression::Expr;
use scanner::token::{types::Literal, Token};

pub struct Interputer {}

impl Interputer {
  pub fn new() -> Self {
    Self {}
  }

  pub fn run(&mut self, ast: Vec<Expr>) {
    for expr in ast {
      let hi = self.run_expression(expr);
      println!("{:?}", hi);
    }
  }

  fn run_expression(&self, expr: Expr) -> LoxValue {
    match expr {
      Expr::Literal(token) => self.eval_literal(token),
      Expr::Grouping(expr) => self.eval_grouping(*expr),
      Expr::Unary { operator, rhs } => self.eval_unary(operator, *rhs),
      Expr::Binary { lhs, operator, rhs } => self.eval_binary(*lhs, operator, *rhs),
      Expr::Ternary {
        condition,
        then_branch,
        else_branch,
      } => self.eval_ternary(*condition, *then_branch, *else_branch),
      Expr::Assign { name, value } => self.eval_assign(name, *value),
      _ => LoxValue::Nil,
    }
  }

  fn eval_assign(&self, name: Token, value: Expr) -> LoxValue {
    let value = self.run_expression(value);

    value
  }

  fn eval_ternary(&self, condition: Expr, then_branch: Expr, else_branch: Expr) -> LoxValue {
    let condition = self.run_expression(condition);
    let then_branch = self.run_expression(then_branch);
    let else_branch = self.run_expression(else_branch);

    match condition {
      LoxValue::Boolean(bool) => {
        if bool {
          then_branch
        } else {
          else_branch
        }
      },
      LoxValue::Number(_) => then_branch,
      LoxValue::String(_) => then_branch,
      LoxValue::Nil => else_branch,
    }
  }

  fn eval_binary(&self, lhs: Expr, operator: Token, rhs: Expr) -> LoxValue {
    let lhs = self.run_expression(lhs);
    let rhs = self.run_expression(rhs);

    match operator.lexeme.as_str() {
      "*" => match (lhs, rhs) {
        (LoxValue::Number(a), LoxValue::Number(b)) => LoxValue::Number(a * b),
        _ => panic!("Invalid number value"),
      },

      "/" => match (lhs, rhs) {
        (LoxValue::Number(a), LoxValue::Number(b)) => LoxValue::Number(a / b),
        _ => panic!("Invalid number value"),
      },
      "+" => match (lhs, rhs) {
        (LoxValue::Number(a), LoxValue::Number(b)) => LoxValue::Number(a + b),
        _ => panic!("Invalid number value"),
      },
      "-" => match (lhs, rhs) {
        (LoxValue::Number(a), LoxValue::Number(b)) => LoxValue::Number(a - b),
        _ => panic!("Invalid number value"),
      },
      "==" => match (lhs, rhs) {
        (LoxValue::Number(a), LoxValue::Number(b)) => LoxValue::Boolean(a == b),
        _ => panic!("Invalid number value"),
      },
      "!=" => match (lhs, rhs) {
        (LoxValue::Number(a), LoxValue::Number(b)) => LoxValue::Boolean(a != b),
        _ => panic!("Invalid number value"),
      },
      ">" => match (lhs, rhs) {
        (LoxValue::Number(a), LoxValue::Number(b)) => LoxValue::Boolean(a > b),
        _ => panic!("Invalid number value"),
      },
      ">=" => match (lhs, rhs) {
        (LoxValue::Number(a), LoxValue::Number(b)) => LoxValue::Boolean(a >= b),
        _ => panic!("Invalid number value"),
      },
      "<" => match (lhs, rhs) {
        (LoxValue::Number(a), LoxValue::Number(b)) => LoxValue::Boolean(a < b),
        _ => panic!("Invalid number value"),
      },
      "<=" => match (lhs, rhs) {
        (LoxValue::Number(a), LoxValue::Number(b)) => LoxValue::Boolean(a <= b),
        _ => panic!("Invalid number value"),
      },
      _ => LoxValue::Nil,
    }
  }

  fn eval_unary(&self, operator: Token, rhs: Expr) -> LoxValue {
    let rhs = self.run_expression(rhs);

    match operator.lexeme.as_str() {
      // TODO:
      "!" => match rhs {
        LoxValue::Boolean(b) => LoxValue::Boolean(!b),
        _ => panic!("Invalid boolean value"),
      },
      "-" => match rhs {
        LoxValue::Number(n) => LoxValue::Number(-n),
        _ => panic!("Invalid number value"),
      },
      _ => panic!("Invalid unary operator"),
    }
  }

  fn eval_grouping(&self, expr: Expr) -> LoxValue {
    self.run_expression(expr)
  }

  fn eval_literal(&self, token: Token) -> LoxValue {
    match token.literal {
      Literal::Number => LoxValue::Number(token.lexeme.parse::<f64>().unwrap()),
      Literal::String => LoxValue::String(token.lexeme),
      Literal::Boolean => LoxValue::Boolean(token.lexeme == "true"),
      Literal::Nil => LoxValue::Nil,
    }
  }
}

#[derive(Debug, Clone)]
pub enum LoxValue {
  Nil,
  Number(f64),
  String(String),
  Boolean(bool),
  // Function(Function),
  // Object(Object),
}
