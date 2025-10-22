use std::{
  cell::RefCell,
  fmt::{self, write},
  rc::Rc,
};

use diagnostic::{
  diagnostic::{Diagnostic, Label},
  diagnostic_code::DiagnosticCode,
  DiagnosticEngine,
};
use parser::{expr::Expr, stmt::Stmt};
use scanner::token::{types::Literal, Token};

use crate::env::Env;

#[derive(Debug, Clone)]
pub struct Interpreter {
  pub env: Rc<RefCell<Env>>,
}

impl Interpreter {
  pub fn new() -> Self {
    Self {
      env: Rc::new(RefCell::new(Env::new())),
    }
  }

  pub fn run(&mut self, ast: Vec<Stmt>, engine: &mut DiagnosticEngine) {
    let mut env = self.env.clone();
    for stmt in ast {
      let _ = self.eval_stmt(stmt, &mut env, engine);
    }
    self.env = env;
  }

  fn eval_stmt(
    &mut self,
    stmt: Stmt,
    env: &mut Rc<RefCell<Env>>,
    engine: &mut DiagnosticEngine,
  ) -> Result<(), ()> {
    match stmt {
      Stmt::Print(expr) => {
        match self.eval_expr(expr, env, engine) {
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
        self.eval_expr(expr, env, engine)?;
        return Ok(());
      },
      Stmt::VarDec(identifier_token, expr) => match expr {
        Some(expr) => {
          let (expr_value, _) = self.eval_expr(expr, env, engine)?;
          env.borrow_mut().define(identifier_token.lexeme, expr_value);
          return Ok(());
        },
        None => {
          env
            .borrow_mut()
            .define(identifier_token.lexeme, LoxValue::Nil);
          return Ok(());
        },
      },
      Stmt::Block(block) => {
        self.eval_block(block, env, engine)?;
        return Ok(());
      },
      Stmt::If(condition, then_branch, else_branch) => {
        self.eval_if(env, *condition, *then_branch, else_branch, engine)?;
        return Ok(());
      },
      Stmt::While(condition, stmt) => {
        self.eval_while(env, *condition, *stmt, engine)?;
        return Ok(());
      },
    }
  }

  fn eval_while(
    &mut self,
    env: &mut Rc<RefCell<Env>>,
    condition: Expr,
    stmt: Stmt,
    engine: &mut DiagnosticEngine,
  ) -> Result<(LoxValue, Option<Token>), ()> {
    loop {
      let (condition_val, _) = self.eval_expr(condition.clone(), env, engine)?;

      if !self.is_truthy(&condition_val) {
        break;
      }

      self.eval_stmt(stmt.clone(), env, engine)?;
    }

    Ok((LoxValue::Nil, None))
  }

  fn eval_if(
    &mut self,
    env: &mut Rc<RefCell<Env>>,
    condition: Expr,
    then_branch: Stmt,
    else_branch: Option<Box<Stmt>>,
    engine: &mut DiagnosticEngine,
  ) -> Result<(), ()> {
    let (expr_val, token) = self.eval_expr(condition, env, engine)?;

    match expr_val {
      LoxValue::Bool(v) => {
        if v {
          self.eval_stmt(then_branch, env, engine)
        } else {
          match else_branch {
            Some(else_branch) => self.eval_stmt(*else_branch, env, engine),
            None => Ok(()),
          }
        }
      },
      _ => {
        self.emit_type_error(
          engine,
          &token.unwrap(),
          None,
          "If condition must be a boolean",
          &format!("Expected boolean, found {}", Self::type_name(&expr_val)),
        )?;
        Err(())
      },
    }
  }

  pub fn eval_block(
    &mut self,
    block: Box<Vec<Stmt>>,
    env: &mut Rc<RefCell<Env>>,
    engine: &mut DiagnosticEngine,
  ) -> Result<(LoxValue, Option<Token>), ()> {
    let mut enclosing_env = Rc::new(RefCell::new(
      env.borrow_mut().with_enclosing(Rc::clone(env)),
    ));

    for stmt in *block {
      match stmt {
        Stmt::VarDec(identifier_token, expr) => match expr {
          Some(expr) => {
            let (expr_value, _) = self.eval_expr(expr, &mut enclosing_env, engine)?;
            enclosing_env
              .borrow_mut()
              .define(identifier_token.lexeme, expr_value);
          },
          None => {
            enclosing_env
              .borrow_mut()
              .define(identifier_token.lexeme, LoxValue::Nil);
          },
        },
        Stmt::Expr(expr) => {
          self.eval_expr(expr, &mut enclosing_env, engine)?;
        },
        Stmt::Print(expr) => {
          let (value, _) = self.eval_expr(expr, &mut enclosing_env, engine)?;
          println!("{}", value);
        },
        Stmt::Block(block) => {
          self.eval_block(block, &mut enclosing_env, engine)?;
        },
        Stmt::If(condition, then_branch, else_branch) => {
          self.eval_if(
            &mut enclosing_env,
            *condition,
            *then_branch,
            else_branch,
            engine,
          )?;
        },
        Stmt::While(condition, stmt) => {
          self.eval_while(&mut enclosing_env, *condition, *stmt, engine)?;
        },
      }
    }

    Ok((LoxValue::Nil, None))
  }

  fn eval_expr(
    &mut self,
    expr: Expr,
    env: &mut Rc<RefCell<Env>>,
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
      Expr::Identifier(token) => self.eval_identifier(token, env, engine),
      Expr::Call {
        callee,
        paren,
        arguments,
      } => self.eval_call(env, *callee, paren, arguments, engine),
    }
  }

  fn eval_call(
    &mut self,
    env: &mut Rc<RefCell<Env>>,
    callee: Expr,
    paren: Token,
    arguments: Vec<Expr>,
    engine: &mut DiagnosticEngine,
  ) -> Result<(LoxValue, Option<Token>), ()> {
    let args_val = self.eval_args(env, arguments, engine)?;
    let (callee_val, _) = self.eval_expr(callee, env, engine)?;

    match callee_val {
      // LoxValue::Function(fnc) => {
      //   if args_val.len() != fnc.arity() {
      //     return Err(());
      //   }
      //
      //   // let result = fnc.call(self, args_val, engine)?;
      //   // return Ok((result, Some(paren)));
      // },
      // LoxValue::NativeFunction(fnc) => {
      //   // let result = fnc.call(self, args_val, engine)?;
      //   // return Ok((result, Some(paren)));
      // },
      _ => Err(()),
    }
  }

  fn eval_args(
    &mut self,
    env: &mut Rc<RefCell<Env>>,
    arguments: Vec<Expr>,
    engine: &mut DiagnosticEngine,
  ) -> Result<Vec<(LoxValue, Option<Token>)>, ()> {
    let mut args_val = vec![];
    for arg in arguments {
      let arg_val = self.eval_expr(arg, env, engine)?;
      args_val.push(arg_val);
    }

    Ok(args_val)
  }

  fn eval_identifier(
    &self,
    mut token: Token,
    env: &mut Rc<RefCell<Env>>,
    engine: &mut DiagnosticEngine,
  ) -> Result<(LoxValue, Option<Token>), ()> {
    match env.borrow().get(&token.lexeme) {
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
    env: &mut Rc<RefCell<Env>>,
    engine: &mut DiagnosticEngine,
  ) -> Result<(LoxValue, Option<Token>), ()> {
    let (value, token) = self.eval_expr(value, env, engine)?;
    if !env.borrow_mut().assign(&name.lexeme, value.clone()) {
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
    env: &mut Rc<RefCell<Env>>,
    condition: Expr,
    then_branch: Expr,
    else_branch: Expr,
    engine: &mut DiagnosticEngine,
  ) -> Result<(LoxValue, Option<Token>), ()> {
    let (condition_val, _) = self.eval_expr(condition, env, engine)?;

    if self.is_truthy(&condition_val) {
      self.eval_expr(then_branch, env, engine)
    } else {
      self.eval_expr(else_branch, env, engine)
    }
  }

  fn eval_binary(
    &mut self,
    env: &mut Rc<RefCell<Env>>,
    lhs: Expr,
    operator: Token,
    rhs: Expr,
    engine: &mut DiagnosticEngine,
  ) -> Result<(LoxValue, Option<Token>), ()> {
    match operator.lexeme.as_str() {
      "%" | "*" | "/" | "-" => self.eval_arithmetic(env, operator, lhs, rhs, engine),
      "+" => self.eval_addition(env, operator, lhs, rhs, engine),
      "==" | "!=" => self.eval_equality(env, operator, lhs, rhs, engine),
      ">" | ">=" | "<" | "<=" => self.eval_comparison(env, operator, lhs, rhs, engine),
      "||" | "&&" => self.eval_logical(env, operator, lhs, rhs, engine),
      "," => Err(()),
      _ => self.emit_error(
        engine,
        DiagnosticCode::InvalidOperator,
        &format!("Unknown binary operator '{}'", operator.lexeme),
        &operator,
        "This operator is not supported",
        Some("Valid operators are: +, -, %, *, /, ==, !=, <, <=, >, >="),
      ),
    }
  }

  fn eval_logical(
    &mut self,
    env: &mut Rc<RefCell<Env>>,
    operator: Token,
    lhs: Expr,
    rhs: Expr,
    engine: &mut DiagnosticEngine,
  ) -> Result<(LoxValue, Option<Token>), ()> {
    let (lhs_val, lhs_token) = self.eval_expr(lhs, env, engine)?;

    let is_truthy = self.is_truthy(&lhs_val);

    match operator.lexeme.as_str() {
      "||" => {
        // short-circuit: if lhs is truthy, return it
        if is_truthy {
          Ok((lhs_val, lhs_token))
        } else {
          self.eval_expr(rhs, env, engine)
        }
      },
      "&&" => {
        // short-circuit: if lhs is falsy, return it
        if !is_truthy {
          Ok((lhs_val, lhs_token))
        } else {
          self.eval_expr(rhs, env, engine)
        }
      },
      _ => Err(()),
    }
  }

  fn eval_arithmetic(
    &mut self,
    env: &mut Rc<RefCell<Env>>,
    operator: Token,
    lhs: Expr,
    rhs: Expr,
    engine: &mut DiagnosticEngine,
  ) -> Result<(LoxValue, Option<Token>), ()> {
    let (lhs_val, lhs_token) = self.eval_expr(lhs, env, engine)?;
    let (rhs_val, rhs_token) = self.eval_expr(rhs, env, engine)?;

    match (&lhs_val, &rhs_val) {
      (LoxValue::Number(a), LoxValue::Number(b)) => {
        let result = match operator.lexeme.as_str() {
          "%" => a % b,
          "*" => a * b,
          "/" => {
            if *b == 0.0 {
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
        let (bad_token, bad_value) = if matches!(lhs_val, LoxValue::Number(_)) {
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
    &mut self,
    env: &mut Rc<RefCell<Env>>,
    operator: Token,
    lhs: Expr,
    rhs: Expr,
    engine: &mut DiagnosticEngine,
  ) -> Result<(LoxValue, Option<Token>), ()> {
    let (lhs_val, _) = self.eval_expr(lhs, env, engine)?;
    let (rhs_val, _) = self.eval_expr(rhs, env, engine)?;

    match (lhs_val, rhs_val) {
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
    &mut self,
    env: &mut Rc<RefCell<Env>>,
    operator: Token,
    lhs: Expr,
    rhs: Expr,
    engine: &mut DiagnosticEngine,
  ) -> Result<(LoxValue, Option<Token>), ()> {
    let (lhs_val, _) = self.eval_expr(lhs, env, engine)?;
    let (rhs_val, _) = self.eval_expr(rhs, env, engine)?;

    let result = match operator.lexeme.as_str() {
      "==" => Self::is_equal(&lhs_val, &rhs_val),
      "!=" => !Self::is_equal(&lhs_val, &rhs_val),
      _ => unreachable!(),
    };
    Ok((LoxValue::Bool(result), Some(operator)))
  }

  fn eval_comparison(
    &mut self,
    env: &mut Rc<RefCell<Env>>,
    operator: Token,
    lhs: Expr,
    rhs: Expr,
    engine: &mut DiagnosticEngine,
  ) -> Result<(LoxValue, Option<Token>), ()> {
    let (lhs_val, _) = self.eval_expr(lhs, env, engine)?;
    let (rhs_val, _) = self.eval_expr(rhs, env, engine)?;

    match (lhs_val, rhs_val) {
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
    env: &mut Rc<RefCell<Env>>,
    operator: Token,
    rhs: Expr,
    engine: &mut DiagnosticEngine,
  ) -> Result<(LoxValue, Option<Token>), ()> {
    let (rhs_val, rhs_token) = self.eval_expr(rhs, env, engine)?;

    match operator.lexeme.as_str() {
      "!" => {
        let is_truthy = self.is_truthy(&rhs_val);
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
    env: &mut Rc<RefCell<Env>>,
    expr: Expr,
    engine: &mut DiagnosticEngine,
  ) -> Result<(LoxValue, Option<Token>), ()> {
    self.eval_expr(expr, env, engine)
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
      LoxValue::Function(_) => "function",
      LoxValue::NativeFunction(_) => "native function",
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

  fn is_truthy(&self, val: &LoxValue) -> bool {
    return match &val {
      LoxValue::Bool(b) => *b,
      LoxValue::Nil => false,
      LoxValue::Number(n) => *n != 0.0,
      LoxValue::String(s) => !s.is_empty(),
      LoxValue::Function(_) => false,
      LoxValue::NativeFunction(_) => false,
    };
  }
}

#[derive(Clone)]
pub enum LoxValue {
  Nil,
  Number(f64),
  String(String),
  Bool(bool),
  Function(Rc<LoxFunction>),
  NativeFunction(Rc<dyn LoxCallable + Send + Sync>),
}

impl fmt::Debug for LoxValue {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      LoxValue::Nil => write!(f, "Nil"),
      LoxValue::Number(n) => write!(f, "Number({n})"),
      LoxValue::String(s) => write!(f, "String({s:?})"),
      LoxValue::Bool(b) => write!(f, "Bool({b})"),
      LoxValue::Function(_) => write!(f, "Function(<fn>)"),
      LoxValue::NativeFunction(_) => write!(f, "NativeFunction(<native>)"),
    }
  }
}

impl fmt::Display for LoxValue {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      LoxValue::String(s) => write!(f, "{s}"),
      LoxValue::Number(n) => write!(f, "{n}"),
      LoxValue::Bool(b) => write!(f, "{b}"),
      LoxValue::Nil => write!(f, "nil"),
      LoxValue::Function(_) => write!(f, "<function>"),
      LoxValue::NativeFunction(_) => write!(f, "<native function>"),
    }
  }
}

pub trait LoxCallable {
  fn arity(&self) -> usize;
  fn call(
    &self,
    interpreter: &mut Interpreter,
    arguments: Vec<(LoxValue, Option<Token>)>,
    engine: &mut DiagnosticEngine,
  ) -> Result<LoxValue, ()>;
}

#[derive(Debug, Clone)]
pub struct LoxFunction {
  params: Vec<(LoxValue, Option<Token>)>,
  body: Vec<Stmt>,
}

impl LoxCallable for LoxFunction {
  fn arity(&self) -> usize {
    self.params.len()
  }
  fn call(
    &self,
    interpreter: &mut Interpreter,
    arguments: Vec<(LoxValue, Option<Token>)>,
    engine: &mut DiagnosticEngine,
  ) -> Result<LoxValue, ()> {
    let mut enclosing_env = Rc::new(RefCell::new(
      interpreter
        .env
        .borrow_mut()
        .with_enclosing(Rc::clone(&interpreter.env)),
    ));

    // Defining the args in the function scope to be used
    for (arg_val, token) in arguments {
      enclosing_env
        .borrow_mut()
        .define(token.unwrap().lexeme, arg_val.clone());
    }

    match interpreter.eval_block(Box::new(self.body.clone()), &mut enclosing_env, engine) {
      Ok((v, _)) => Ok(v),
      Err(e) => Ok(LoxValue::Nil),
    }
  }
}
