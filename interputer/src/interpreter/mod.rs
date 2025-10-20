use std::{collections::HashMap, hash::Hash, panic};

use diagnostic::{
  diagnostic::{Diagnostic, Label},
  diagnostic_code::DiagnosticCode,
  DiagnosticEngine,
};
use parser::{expression::Expr, statement::Stmt};
use scanner::token::{types::Literal, Token};

pub struct Interpreter {
  /// Environment
  pub env: HashMap<String, LoxValue>,
}

impl Interpreter {
  pub fn new() -> Self {
    Self {
      env: HashMap::new(),
    }
  }

  pub fn run(&mut self, ast: Vec<Stmt>, engine: &mut DiagnosticEngine) {
    for stmt in ast {
      match self.run_statement(stmt, engine) {
        Ok(x) => {},
        Err(_) => return,
      }
    }
  }

  fn run_statement(&mut self, stmt: Stmt, engine: &mut DiagnosticEngine) -> Result<(), ()> {
    match stmt {
      Stmt::Print(expr) => {
        match self.eval_expression(expr, engine) {
          Ok(x) => {
            println!("{:?}", x.0);
            return Ok(());
          },
          Err(_) => {
            // Error occurred, stop evaluation
            return Ok(());
          },
        };
      },
      Stmt::Expr(expr) => {
        self.eval_expression(expr, engine)?;
        return Ok(());
      },
      Stmt::VarDec(identifier_token, Some(expr)) => {
        let (expr_value, _) = self.eval_expression(expr, engine)?;
        self.env.insert(identifier_token.lexeme, expr_value);
        return Ok(());
      },
      Stmt::VarDec(token, None) => {
        self.env.insert(token.lexeme, LoxValue::Nil);
        return Ok(());
      },
    }
  }

  fn eval_expression(
    &mut self,
    expr: Expr,
    engine: &mut DiagnosticEngine,
  ) -> Result<(LoxValue, Option<Token>), ()> {
    match expr {
      Expr::Literal(token) => self.eval_literal(token, engine),
      Expr::Grouping(expr) => self.eval_grouping(*expr, engine),
      Expr::Unary { operator, rhs } => self.eval_unary(operator, *rhs, engine),
      Expr::Binary { lhs, operator, rhs } => self.eval_binary(*lhs, operator, *rhs, engine),
      Expr::Ternary {
        condition,
        then_branch,
        else_branch,
      } => self.eval_ternary(*condition, *then_branch, *else_branch, engine),
      Expr::Assign { name, value } => self.eval_assign(name, *value, engine),
      Expr::Identifier(token) => self.eval_identifier(token, engine), // _ => Ok((LoxValue::Nil, None)),
      _ => {
        todo!()
      },                                   //
    }
  }

  fn eval_identifier(
    &self,
    token: Token,
    engine: &mut DiagnosticEngine,
  ) -> Result<(LoxValue, Option<Token>), ()> {
    match self.env.get(&token.lexeme) {
      Some(v) => Ok((v.clone(), Some(token))),
      None => Ok((LoxValue::Nil, Some(token))),
    }
  }

  fn eval_assign(
    &mut self,
    name: Token,
    value: Expr,
    engine: &mut DiagnosticEngine,
  ) -> Result<(LoxValue, Option<Token>), ()> {
    if !self.env.contains_key(&name.lexeme) {
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

    let (value, token) = self.eval_expression(value, engine)?;
    self.env.insert(name.lexeme, value.clone());
    Ok((value, token))
  }

  fn eval_ternary(
    &mut self,
    condition: Expr,
    then_branch: Expr,
    else_branch: Expr,
    engine: &mut DiagnosticEngine,
  ) -> Result<(LoxValue, Option<Token>), ()> {
    let (condition_val, condition_token) = self.eval_expression(condition, engine)?;

    let is_truthy = match &condition_val {
      LoxValue::Boolean(b) => *b,
      LoxValue::Nil => false,
      LoxValue::Number(n) => *n != 0.0,
      LoxValue::String(s) => !s.is_empty(),
    };

    if is_truthy {
      self.eval_expression(then_branch, engine)
    } else {
      self.eval_expression(else_branch, engine)
    }
  }

  fn eval_binary(
    &mut self,
    lhs: Expr,
    operator: Token,
    rhs: Expr,
    engine: &mut DiagnosticEngine,
  ) -> Result<(LoxValue, Option<Token>), ()> {
    let (lhs_val, lhs_token) = self.eval_expression(lhs, engine)?;
    let (rhs_val, rhs_token) = self.eval_expression(rhs, engine)?;

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
    Ok((LoxValue::Boolean(result), Some(operator)))
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
        Ok((LoxValue::Boolean(result), Some(operator)))
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
    operator: Token,
    rhs: Expr,
    engine: &mut DiagnosticEngine,
  ) -> Result<(LoxValue, Option<Token>), ()> {
    let (rhs_val, rhs_token) = self.eval_expression(rhs, engine)?;

    match operator.lexeme.as_str() {
      "!" => {
        let is_truthy = match &rhs_val {
          LoxValue::Boolean(b) => *b,
          LoxValue::Nil => false,
          LoxValue::Number(n) => *n != 0.0,
          LoxValue::String(s) => !s.is_empty(),
        };
        Ok((LoxValue::Boolean(!is_truthy), Some(operator)))
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
    expr: Expr,
    engine: &mut DiagnosticEngine,
  ) -> Result<(LoxValue, Option<Token>), ()> {
    self.eval_expression(expr, engine)
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
      Literal::Boolean => Ok((LoxValue::Boolean(token.lexeme == "true"), Some(token))),
      Literal::Nil => Ok((LoxValue::Nil, Some(token))),
    }
  }

  // Helper methods

  fn type_name(value: &LoxValue) -> &'static str {
    match value {
      LoxValue::Nil => "nil",
      LoxValue::Number(_) => "number",
      LoxValue::String(_) => "string",
      LoxValue::Boolean(_) => "boolean",
    }
  }

  fn is_equal(a: &LoxValue, b: &LoxValue) -> bool {
    match (a, b) {
      (LoxValue::Nil, LoxValue::Nil) => true,
      (LoxValue::Number(a), LoxValue::Number(b)) => a == b,
      (LoxValue::String(a), LoxValue::String(b)) => a == b,
      (LoxValue::Boolean(a), LoxValue::Boolean(b)) => a == b,
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
  Boolean(bool),
}
