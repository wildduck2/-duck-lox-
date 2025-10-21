use std::fmt;

use diagnostic::{
  diagnostic::{Diagnostic, Label},
  diagnostic_code::DiagnosticCode,
  DiagnosticEngine,
};
use parser::{expression::Expr, statement::Stmt};
use scanner::token::{types::Literal, Token};

use crate::env::Env;

#[derive(Debug, Clone)]
pub struct Interpreter {
  pub env: Env,
}

impl Interpreter {
  pub fn new() -> Self {
    Self { env: Env::new() }
  }

  pub fn run(&mut self, ast: Vec<Stmt>, engine: &mut DiagnosticEngine) {
    let mut env = self.env.clone();
    for stmt in ast {
      let _ = self.run_statement(stmt, &mut env, engine);
    }
    self.env = env;
  }

  fn run_statement(
    &mut self,
    stmt: Stmt,
    env: &mut Env,
    engine: &mut DiagnosticEngine,
  ) -> Result<(), ()> {
    match stmt {
      Stmt::Print(expr) => {
        match self.eval_expression(expr, env, engine) {
          Ok(x) => {
            println!("{}", x.0);
            return Ok(());
          },
          Err(_) => {
            // Error occurred, stop evaluation
            return Ok(());
          },
        };
      },
      Stmt::Expr(expr) => {
        self.eval_expression(expr, env, engine)?;
        return Ok(());
      },
      Stmt::VarDec(identifier_token, Some(expr)) => {
        let (expr_value, _) = self.eval_expression(expr, env, engine)?;
        env.define(identifier_token.lexeme, expr_value);
        return Ok(());
      },
      Stmt::VarDec(token, None) => {
        env.define(token.lexeme, LoxValue::Nil);
        return Ok(());
      },
      Stmt::Block(block) => {
        self.eval_block(block, env, engine)?;
        return Ok(());
      },
    }
  }
  fn eval_block(
    &mut self,
    block: Box<Vec<Stmt>>,
    env: &mut Env,
    engine: &mut DiagnosticEngine,
  ) -> Result<(LoxValue, Option<Token>), ()> {
    let mut enclosing_env = env.with_enclosing(Env::new());

    for stmt in *block {
      match stmt {
        Stmt::VarDec(token, Some(value)) => {
          let (expr_value, _) = self.eval_expression(value, &mut enclosing_env, engine)?;
          enclosing_env.define(token.lexeme, expr_value);
        },
        Stmt::VarDec(token, None) => {
          enclosing_env.define(token.lexeme, LoxValue::Nil);
        },
        Stmt::Print(expr) => {
          let (value, _) = self.eval_expression(expr, &mut enclosing_env, engine)?;
          println!("{}", value);
        },
        _ => {
          // todo!()
        },
      }
    }

    Err(())
  }

  fn eval_expression(
    &mut self,
    expr: Expr,
    env: &mut Env,
    engine: &mut DiagnosticEngine,
  ) -> Result<(LoxValue, Option<Token>), ()> {
    match expr {
      Expr::Literal(token) => self.eval_literal(token, engine),
      Expr::Grouping(expr) => self.eval_grouping(env, *expr, engine),
      Expr::Unary { operator, rhs } => self.eval_unary(env, operator, *rhs, engine),
      Expr::Binary { lhs, operator, rhs } => self.eval_binary(env, *lhs, operator, *rhs, engine),
      Expr::Ternary {
        condition,
        then_branch,
        else_branch,
      } => self.eval_ternary(env, *condition, *then_branch, *else_branch, engine),
      Expr::Assign { name, value } => self.eval_assign(name, *value, env, engine),
      Expr::Identifier(token) => self.eval_identifier(token, env, engine), // _ => Ok((LoxValue::Nil, None)),
    }
  }

  fn eval_identifier(
    &self,
    mut token: Token,
    env: &mut Env,
    engine: &mut DiagnosticEngine,
  ) -> Result<(LoxValue, Option<Token>), ()> {
    match env.get(&token.lexeme) {
      Some(v) => Ok((v.clone(), Some(token))),
      None => {
        token.position.0 += 1;
        token.position.1 -= 1;
        let diagnostic = Diagnostic::new(
          DiagnosticCode::UndeclaredVariable,
          format!("Cannot assign to undeclared variable '{}'", token.lexeme),
        )
        .with_label(Label::primary(
          token.to_span(),
          Some("variable not declared".to_string()),
        ))
        .with_help("Use 'var' to declare variables before assigning to them".to_string());

        engine.emit(diagnostic);
        Err(())
      },
    }
  }

  fn eval_assign(
    &mut self,
    mut name: Token,
    value: Expr,
    env: &mut Env,
    engine: &mut DiagnosticEngine,
  ) -> Result<(LoxValue, Option<Token>), ()> {
    let (value, token) = self.eval_expression(value, env, engine)?;
    if !env.assign(&name.lexeme, value.clone()) {
      name.position.0 += 1;
      name.position.1 -= 1;
      let diagnostic = Diagnostic::new(
        DiagnosticCode::UndeclaredVariable,
        format!("Cannot assign to undeclared variable '{}'", name.lexeme),
      )
      .with_label(Label::primary(
        name.to_span(),
        Some("variable not declared".to_string()),
      ))
      .with_help("Use 'var' to declare variables before assigning to them".to_string());

      engine.emit(diagnostic);
      return Err(());
    }

    Ok((value, token))
  }

  fn eval_ternary(
    &mut self,
    env: &mut Env,
    condition: Expr,
    then_branch: Expr,
    else_branch: Expr,
    engine: &mut DiagnosticEngine,
  ) -> Result<(LoxValue, Option<Token>), ()> {
    let (condition_val, condition_token) = self.eval_expression(condition, env, engine)?;

    let is_truthy = match &condition_val {
      LoxValue::Bool(b) => *b,
      LoxValue::Nil => false,
      LoxValue::Number(n) => *n != 0.0,
      LoxValue::String(s) => !s.is_empty(),
    };

    if is_truthy {
      self.eval_expression(then_branch, env, engine)
    } else {
      self.eval_expression(else_branch, env, engine)
    }
  }

  fn eval_binary(
    &mut self,
    env: &mut Env,
    lhs: Expr,
    operator: Token,
    rhs: Expr,
    engine: &mut DiagnosticEngine,
  ) -> Result<(LoxValue, Option<Token>), ()> {
    let (lhs_val, lhs_token) = self.eval_expression(lhs, env, engine)?;
    let (rhs_val, rhs_token) = self.eval_expression(rhs, env, engine)?;

    match operator.lexeme.as_str() {
      "*" | "/" | "-" => {
        self.eval_arithmetic(lhs_val, operator, rhs_val, lhs_token, rhs_token, engine)
      },
      "+" => self.eval_addition(lhs_val, operator, rhs_val, lhs_token, rhs_token, engine),
      "==" | "!=" => self.eval_equality(lhs_val, operator, rhs_val, engine),
      ">" | ">=" | "<" | "<=" => {
        self.eval_comparison(lhs_val, operator, rhs_val, lhs_token, rhs_token, engine)
      },
      _ => self.emit_error(
        engine,
        DiagnosticCode::InvalidOperator,
        &format!("Unknown binary operator '{}'", operator.lexeme),
        &operator,
        "This operator is not supported",
        Some("Valid operators are: +, -, *, /, ==, !=, <, <=, >, >="),
      ),
    }
  }

  fn eval_arithmetic(
    &self,
    lhs: LoxValue,
    operator: Token,
    rhs: LoxValue,
    lhs_token: Option<Token>,
    rhs_token: Option<Token>,
    engine: &mut DiagnosticEngine,
  ) -> Result<(LoxValue, Option<Token>), ()> {
    match (lhs.clone(), rhs) {
      (LoxValue::Number(a), LoxValue::Number(b)) => {
        let result = match operator.lexeme.as_str() {
          "*" => a * b,
          "/" => {
            if b == 0.0 {
              return self.emit_error_with_note(
                engine,
                DiagnosticCode::DivisionByZero,
                "Division by zero",
                &operator,
                "Cannot divide by zero",
                "Consider checking if the divisor is zero before performing division",
                rhs_token.as_ref(),
                "This evaluates to zero",
              );
            }
            a / b
          },
          "-" => a - b,
          _ => unreachable!(),
        };
        Ok((LoxValue::Number(result), Some(operator)))
      },
      (LoxValue::Number(_), non_number) | (non_number, LoxValue::Number(_)) => {
        let (bad_token, bad_value) = if matches!(lhs, LoxValue::Number(_)) {
          (rhs_token, non_number)
        } else {
          (lhs_token, non_number)
        };

        self.emit_type_error(
          engine,
          &operator,
          bad_token.as_ref(),
          &format!("Arithmetic operations require numeric operands"),
          &format!("Expected number, found {}", Self::type_name(&bad_value)),
        )
      },
      (lhs, rhs) => self.emit_error(
        engine,
        DiagnosticCode::InvalidOperator,
        &format!(
          "Cannot perform arithmetic on {} and {}",
          Self::type_name(&lhs),
          Self::type_name(&rhs)
        ),
        &operator,
        "Both operands must be numbers",
        Some(&format!(
          "Left operand is {}, right operand is {}",
          Self::type_name(&lhs),
          Self::type_name(&rhs)
        )),
      ),
    }
  }

  fn eval_addition(
    &self,
    lhs: LoxValue,
    operator: Token,
    rhs: LoxValue,
    lhs_token: Option<Token>,
    rhs_token: Option<Token>,
    engine: &mut DiagnosticEngine,
  ) -> Result<(LoxValue, Option<Token>), ()> {
    match (lhs, rhs) {
      (LoxValue::Number(a), LoxValue::Number(b)) => Ok((LoxValue::Number(a + b), Some(operator))),
      (LoxValue::String(a), LoxValue::String(b)) => {
        Ok((LoxValue::String(format!("{}{}", a, b)), Some(operator)))
      },
      (LoxValue::String(a), LoxValue::Number(b)) => {
        Ok((LoxValue::String(format!("{}{}", a, b)), Some(operator)))
      },
      (LoxValue::Number(a), LoxValue::String(b)) => {
        Ok((LoxValue::String(format!("{}{}", a, b)), Some(operator)))
      },
      (lhs, rhs) => self.emit_error(
        engine,
        DiagnosticCode::InvalidOperator,
        &format!(
          "Cannot add {} and {}",
          Self::type_name(&lhs),
          Self::type_name(&rhs)
        ),
        &operator,
        "Operands must be two numbers or at least one string",
        Some(&format!("Try converting both operands to the same type")),
      ),
    }
  }

  fn eval_equality(
    &self,
    lhs: LoxValue,
    operator: Token,
    rhs: LoxValue,
    engine: &mut DiagnosticEngine,
  ) -> Result<(LoxValue, Option<Token>), ()> {
    let result = match operator.lexeme.as_str() {
      "==" => Self::is_equal(&lhs, &rhs),
      "!=" => !Self::is_equal(&lhs, &rhs),
      _ => unreachable!(),
    };
    Ok((LoxValue::Bool(result), Some(operator)))
  }

  fn eval_comparison(
    &self,
    lhs: LoxValue,
    operator: Token,
    rhs: LoxValue,
    lhs_token: Option<Token>,
    rhs_token: Option<Token>,
    engine: &mut DiagnosticEngine,
  ) -> Result<(LoxValue, Option<Token>), ()> {
    match (lhs, rhs) {
      (LoxValue::Number(a), LoxValue::Number(b)) => {
        let result = match operator.lexeme.as_str() {
          ">" => a > b,
          ">=" => a >= b,
          "<" => a < b,
          "<=" => a <= b,
          _ => unreachable!(),
        };
        Ok((LoxValue::Bool(result), Some(operator)))
      },
      (lhs, rhs) => self.emit_error(
        engine,
        DiagnosticCode::InvalidOperator,
        &format!(
          "Cannot compare {} and {}",
          Self::type_name(&lhs),
          Self::type_name(&rhs)
        ),
        &operator,
        "Comparison operators require numeric operands",
        Some(&format!("Both operands must be numbers for comparison")),
      ),
    }
  }

  fn eval_unary(
    &mut self,
    env: &mut Env,
    operator: Token,
    rhs: Expr,
    engine: &mut DiagnosticEngine,
  ) -> Result<(LoxValue, Option<Token>), ()> {
    let (rhs_val, rhs_token) = self.eval_expression(rhs, env, engine)?;

    match operator.lexeme.as_str() {
      "!" => {
        let is_truthy = match &rhs_val {
          LoxValue::Bool(b) => *b,
          LoxValue::Nil => false,
          LoxValue::Number(n) => *n != 0.0,
          LoxValue::String(s) => !s.is_empty(),
        };
        Ok((LoxValue::Bool(!is_truthy), Some(operator)))
      },
      "-" => match rhs_val {
        LoxValue::Number(n) => Ok((LoxValue::Number(-n), Some(operator))),
        _ => self.emit_type_error(
          engine,
          &operator,
          rhs_token.as_ref(),
          "Unary minus requires a numeric operand",
          &format!("Expected number, found {}", Self::type_name(&rhs_val)),
        ),
      },
      _ => self.emit_error(
        engine,
        DiagnosticCode::InvalidUnaryOperator,
        &format!("Unknown unary operator '{}'", operator.lexeme),
        &operator,
        "This operator is not supported as a unary operator",
        Some("Valid unary operators are: !, -"),
      ),
    }
  }

  fn eval_grouping(
    &mut self,
    env: &mut Env,
    expr: Expr,
    engine: &mut DiagnosticEngine,
  ) -> Result<(LoxValue, Option<Token>), ()> {
    self.eval_expression(expr, env, engine)
  }

  fn eval_literal(
    &self,
    token: Token,
    engine: &mut DiagnosticEngine,
  ) -> Result<(LoxValue, Option<Token>), ()> {
    match token.literal {
      Literal::Number => match token.lexeme.parse::<f64>() {
        Ok(num) => Ok((LoxValue::Number(num), Some(token))),
        Err(_) => self.emit_error(
          engine,
          DiagnosticCode::InvalidNumber,
          &format!("Invalid number literal '{}'", token.lexeme),
          &token,
          "Failed to parse as a number",
          Some("Check that the number is formatted correctly"),
        ),
      },
      Literal::String => Ok((LoxValue::String(token.lexeme.clone()), Some(token))),
      Literal::Boolean => Ok((LoxValue::Bool(token.lexeme == "true"), Some(token))),
      Literal::Nil => Ok((LoxValue::Nil, Some(token))),
    }
  }

  // Helper methods

  fn type_name(value: &LoxValue) -> &'static str {
    match value {
      LoxValue::Nil => "nil",
      LoxValue::Number(_) => "number",
      LoxValue::String(_) => "string",
      LoxValue::Bool(_) => "boolean",
    }
  }

  fn is_equal(a: &LoxValue, b: &LoxValue) -> bool {
    match (a, b) {
      (LoxValue::Nil, LoxValue::Nil) => true,
      (LoxValue::Number(a), LoxValue::Number(b)) => a == b,
      (LoxValue::String(a), LoxValue::String(b)) => a == b,
      (LoxValue::Bool(a), LoxValue::Bool(b)) => a == b,
      _ => false,
    }
  }

  fn emit_error(
    &self,
    engine: &mut DiagnosticEngine,
    code: DiagnosticCode,
    message: &str,
    token: &Token,
    label_msg: &str,
    help: Option<&str>,
  ) -> Result<(LoxValue, Option<Token>), ()> {
    let mut diagnostic = Diagnostic::new(code, message.to_string())
      .with_label(Label::primary(token.to_span(), Some(label_msg.to_string())));

    if let Some(help_msg) = help {
      diagnostic = diagnostic.with_help(help_msg.to_string());
    }

    engine.emit(diagnostic);
    Err(())
  }

  fn emit_type_error(
    &self,
    engine: &mut DiagnosticEngine,
    operator: &Token,
    operand_token: Option<&Token>,
    message: &str,
    label_msg: &str,
  ) -> Result<(LoxValue, Option<Token>), ()> {
    let mut diagnostic = Diagnostic::new(DiagnosticCode::TypeError, message.to_string())
      .with_label(Label::primary(
        operator.to_span(),
        Some("operation here".to_string()),
      ));

    if let Some(token) = operand_token {
      diagnostic = diagnostic.with_label(Label::secondary(
        token.to_span(),
        Some(label_msg.to_string()),
      ));
    }

    engine.emit(diagnostic);
    Err(())
  }

  fn emit_error_with_note(
    &self,
    engine: &mut DiagnosticEngine,
    code: DiagnosticCode,
    message: &str,
    primary_token: &Token,
    primary_label: &str,
    help: &str,
    note_token: Option<&Token>,
    note_label: &str,
  ) -> Result<(LoxValue, Option<Token>), ()> {
    let mut diagnostic = Diagnostic::new(code, message.to_string())
      .with_label(Label::primary(
        primary_token.to_span(),
        Some(primary_label.to_string()),
      ))
      .with_help(help.to_string());

    if let Some(token) = note_token {
      diagnostic = diagnostic.with_label(Label::secondary(
        token.to_span(),
        Some(note_label.to_string()),
      ));
    }

    engine.emit(diagnostic);
    Err(())
  }
}

#[derive(Debug, Clone)]
pub enum LoxValue {
  Nil,
  Number(f64),
  String(String),
  Bool(bool),
}

impl fmt::Display for LoxValue {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      LoxValue::String(s) => write!(f, "{s}"),
      LoxValue::Number(n) => write!(f, "{n}"),
      LoxValue::Bool(b) => write!(f, "{b}"),
      LoxValue::Nil => write!(f, "nil"),
    }
  }
}
