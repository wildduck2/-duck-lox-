use std::{cell::RefCell, collections::HashMap, rc::Rc, sync::Arc};

use diagnostic::{
  diagnostic::{Diagnostic, Label, Span},
  diagnostic_code::DiagnosticCode,
  DiagnosticEngine,
};
use parser::{expr::Expr, stmt::Stmt};
use scanner::token::{types::Literal, Token};

use crate::{
  class::{LoxClass, LoxClassInstance},
  env::Env,
  function::{
    native::{clock::ClockFunction, print::PrintFunction},
    normal::LoxFunction,
    LoxCallable,
  },
  lox_value::{InterpreterError, LoxValue},
};

#[derive(Debug, Clone)]
pub struct Interpreter {
  pub env: Rc<RefCell<Env>>,
  pub locals: HashMap<String, usize>,
}

impl Interpreter {
  pub fn new() -> Self {
    Self {
      env: Rc::new(RefCell::new(Env::new())),
      locals: HashMap::new(),
    }
  }

  pub fn run(
    &mut self,
    ast: Vec<Stmt>,
    locals: HashMap<String, usize>,
    engine: &mut DiagnosticEngine,
  ) {
    PrintFunction::add(self);
    ClockFunction::add(self);
    self.locals = locals;

    let mut env = self.env.clone();
    for stmt in ast {
      let _ = self.eval_stmt(stmt, &mut env, engine);
    }
    self.env = env;
  }

  pub fn eval_stmt(
    &mut self,
    stmt: Stmt,
    env: &mut Rc<RefCell<Env>>,
    engine: &mut DiagnosticEngine,
  ) -> Result<(), InterpreterError> {
    match stmt {
      Stmt::Expr(expr) => {
        self.eval_expr(expr, env, engine)?;
        return Ok(());
      },
      Stmt::VarDecl(identifier_token, expr) => match expr {
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
      Stmt::Fun(name, params, body) => {
        self.eval_fun(env, name, params, *body, engine)?;
        return Ok(());
      },
      Stmt::Return(name, _) => {
        let diagnostic = Diagnostic::new(
          DiagnosticCode::ReturnNotInFunction,
          "Return statement is not allowed in top-level code".to_string(),
        )
        .with_label(Label::primary(
          name.to_span(),
          Some("return statement here".to_string()),
        ));

        engine.emit(diagnostic);
        return Ok(());
      },
      Stmt::Break(token) => {
        let mut token = token;
        token.position.0 -= 1;
        token.position.1 += 7;
        // At top level, this is an error
        let diagnostic = Diagnostic::new(
          DiagnosticCode::BreakOutsideLoop,
          "Break statement outside of loop".to_string(),
        )
        .with_label(Label::primary(
          token.to_span(),
          Some("break not allowed here".to_string()),
        ))
        .with_help("Break statements can only be used inside while loops".to_string());

        engine.emit(diagnostic);
        Ok(())
      },

      Stmt::Continue(token) => {
        // At top level, this is an error
        let diagnostic = Diagnostic::new(
          DiagnosticCode::ContinueOutsideLoop,
          "Continue statement outside of loop".to_string(),
        )
        .with_label(Label::primary(
          token.to_span(),
          Some("continue not allowed here".to_string()),
        ))
        .with_help("Continue statements can only be used inside while loops".to_string());

        engine.emit(diagnostic);
        Ok(())
      },
      Stmt::Class(name, superclass, methods, static_methods) => {
        self.eval_class(env, name, superclass, *methods, *static_methods, engine)?;
        Ok(())
      },
    }
  }

  fn eval_class(
    &mut self,
    env: &mut Rc<RefCell<Env>>,
    name: Expr,
    superclass: Option<Expr>,
    methods: Vec<Stmt>,
    static_methods: Vec<Stmt>,
    engine: &mut DiagnosticEngine,
  ) -> Result<(LoxValue, Option<Token>), InterpreterError> {
    let class_name = match name {
      Expr::Identifier(token) => token.lexeme.clone(),
      _ => {
        eprintln!("Class name must be an identifier");
        return Err(InterpreterError::RuntimeError);
      },
    };

    // Define the class name in the environment first (allows recursion/self-reference)
    env.borrow_mut().define(class_name.clone(), LoxValue::Nil);

    let mut super_class_val = LoxValue::Nil;
    let mut class_env = env.clone();

    if let Some(superclass_expr) = superclass {
      let (superclass_val, token) = self.eval_expr(superclass_expr, env, engine)?;
      if let LoxValue::Class(_) = superclass_val {
        super_class_val = superclass_val.clone();

        // **INHERITANCE ENVIRONMENT STEP:**
        // Create a new environment nested *inside* the current environment
        // to hold the 'super' binding.
        let super_env = Rc::new(RefCell::new(
          class_env.borrow_mut().with_enclosing(Rc::clone(&class_env)),
        ));
        // Define 'super' in this new environment.
        super_env
          .borrow_mut()
          .define("super".to_string(), super_class_val.clone());

        // All methods will now capture this 'super_env' as their closure.
        class_env = super_env;
      } else {
        // ... (Error handling for invalid superclass, kept as is)
        let diagnostic = Diagnostic::new(
          DiagnosticCode::InvalidSuperclass,
          "Superclass must be a class".to_string(),
        )
        .with_label(Label::primary(
          token.unwrap().to_span(),
          Some("superclass name here".to_string()),
        ))
        .with_help("Superclass must be a class".to_string());

        engine.emit(diagnostic);
        return Err(InterpreterError::RuntimeError);
      }
    };

    let mut methods_map = HashMap::new();
    let mut static_methods_map = HashMap::new();

    // Pass the potentially new `class_env` (which contains 'super' if a superclass exists)
    self.eval_method_map(&mut class_env, methods, &mut methods_map, engine);
    // Static methods are resolved outside the super environment (use the original `env` or its enclosing)
    self.eval_method_map(env, static_methods, &mut static_methods_map, engine);

    let class = Arc::new(LoxClass {
      name: class_name.clone(),
      superclass: super_class_val,
      methods: methods_map,
      static_methods: static_methods_map,
    });

    // Assign the actual class object to the name we defined earlier (overwriting LoxValue::Nil)
    env.borrow_mut().assign(&class_name, LoxValue::Class(class));

    Ok((LoxValue::Nil, None))
  }

  fn eval_return(
    &mut self,
    env: &mut Rc<RefCell<Env>>,
    _name: Token,
    value: Option<Expr>,
    engine: &mut DiagnosticEngine,
  ) -> Result<(LoxValue, Option<Token>), InterpreterError> {
    match value {
      Some(expr) => match self.eval_expr(expr, env, engine) {
        Ok((expr_value, _)) => Err(InterpreterError::Return(expr_value)),
        Err(_) => Err(InterpreterError::Return(LoxValue::Nil)),
      },

      None => Err(InterpreterError::Return(LoxValue::Nil)),
    }
  }

  fn eval_method_map(
    &mut self,
    env: &mut Rc<RefCell<Env>>,
    methods: Vec<Stmt>,
    methods_map: &mut HashMap<String, Arc<LoxFunction>>,
    engine: &mut DiagnosticEngine,
  ) {
    for method in methods {
      match method {
        Stmt::Fun(name, params, body) => {
          // Extract method name
          let method_name = match name {
            Expr::Identifier(token) => token.lexeme.clone(),
            _ => continue,
          };

          // Extract parameters
          let params_names: Vec<Token> = params
            .into_iter()
            .filter_map(|expr| match expr {
              Expr::Identifier(token) => Some(token),
              _ => None,
            })
            .collect();
          let is_initializer = method_name == "init";

          // Create LoxFunction for this method
          let function = Arc::new(LoxFunction {
            params: params_names,
            body: match *body {
              Stmt::Block(stmts) => *stmts,
              _ => vec![],
            },
            closure: env.clone(), // Capture current environment
            is_initializer,
          });

          methods_map.insert(method_name, function);
        },
        _ => {
          println!("not handled");
        },
      }
    }
  }

  fn eval_fun(
    &mut self,
    env: &mut Rc<RefCell<Env>>,
    name: Expr,
    params: Vec<Expr>,
    body: Stmt,
    _engine: &mut DiagnosticEngine,
  ) -> Result<(LoxValue, Option<Token>), InterpreterError> {
    let name = match name {
      Expr::Identifier(token) => token.lexeme.clone(),
      _ => {
        eprintln!("Function name must be an identifier");
        return Err(InterpreterError::RuntimeError);
      },
    };

    let params_names = params
      .into_iter()
      .map(|expr| match expr {
        Expr::Identifier(token) => Ok(token),
        _ => Err(InterpreterError::RuntimeError),
      })
      .collect::<Result<Vec<_>, _>>()?;

    match body {
      Stmt::Block(body) => {
        let function = Arc::new(LoxFunction {
          params: params_names,
          body: *body,
          closure: env.borrow().enclosing.clone().unwrap_or(env.clone()),
          is_initializer: false,
        });

        env.borrow_mut().define(name, LoxValue::Function(function));
      },
      _ => {},
    };

    Ok((LoxValue::Nil, None))
  }

  fn eval_while(
    &mut self,
    env: &mut Rc<RefCell<Env>>,
    condition: Expr,
    stmt: Stmt,
    engine: &mut DiagnosticEngine,
  ) -> Result<(LoxValue, Option<Token>), InterpreterError> {
    loop {
      let (condition_val, _) = self.eval_expr(condition.clone(), env, engine)?;

      if !self.is_truthy(&condition_val) {
        break;
      }

      // Execute the body and handle break/continue
      match self.eval_stmt(stmt.clone(), env, engine) {
        Ok(_) => continue,                           // Normal execution, continue loop
        Err(InterpreterError::Break) => break,       // Break out of loop
        Err(InterpreterError::Continue) => continue, // Continue to next iteration
        Err(e) => return Err(e),                     // Propagate other errors (like Return)
      }
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
  ) -> Result<(), InterpreterError> {
    let (expr_val, token) = self.eval_expr(condition, env, engine)?;

    match expr_val {
      LoxValue::Bool(v) => {
        if v {
          self.eval_stmt(then_branch, env, engine)?;
        } else {
          if let Some(else_branch) = else_branch {
            self.eval_stmt(*else_branch, env, engine)?;
          }
        }
        Ok(())
      },
      _ => {
        self.emit_type_error(
          engine,
          &token.unwrap(),
          None,
          "If condition must be a boolean",
          &format!("Expected boolean, found {}", &expr_val.to_string()),
        )?;
        Err(InterpreterError::RuntimeError)
      },
    }
  }

  pub fn eval_block(
    &mut self,
    block: Box<Vec<Stmt>>,
    env: &mut Rc<RefCell<Env>>,
    engine: &mut DiagnosticEngine,
  ) -> Result<(LoxValue, Option<Token>), InterpreterError> {
    let mut enclosing_env = Rc::new(RefCell::new(
      env.borrow_mut().with_enclosing(Rc::clone(env)),
    ));

    for stmt in *block {
      match stmt {
        Stmt::VarDecl(identifier_token, expr) => match expr {
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
        Stmt::Fun(name, params, body) => {
          self.eval_fun(&mut enclosing_env, name, params, *body, engine)?;
        },
        Stmt::Return(name, value) => {
          self.eval_return(&mut enclosing_env, name, value, engine)?;
        },
        Stmt::Break(token) => {
          return Err(InterpreterError::Break);
        },
        Stmt::Continue(token) => {
          return Err(InterpreterError::Continue);
        },
        Stmt::Class(name, superclass, methods, static_methods) => {
          self.eval_class(env, name, superclass, *methods, *static_methods, engine)?;
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
  ) -> Result<(LoxValue, Option<Token>), InterpreterError> {
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
      } => match self.eval_call(env, *callee, paren, arguments, engine) {
        Ok(v) => Ok(v),
        Err(InterpreterError::Return(v)) => Ok((v, None)),
        _ => Err(InterpreterError::RuntimeError),
      },
      Expr::Get { object, name } => self.eval_get(env, *object, name, engine),
      Expr::Set {
        object,
        name,
        value,
      } => self.eval_set(env, *object, name, *value, engine),
      Expr::This(token) => self.eval_identifier(token, env, engine),
      Expr::Super(token, name) => self.eval_super_expr(token, name, env),
    }
  }

  fn eval_super_expr(
    &mut self,
    keyword: Token,
    name: Token,
    env: &mut Rc<RefCell<Env>>,
  ) -> Result<(LoxValue, Option<Token>), InterpreterError> {
    // The Resolver guaranteed this is in `self.locals`.
    let &distance = self
      .locals
      .get(&keyword.lexeme)
      .ok_or(InterpreterError::RuntimeError)?; // Should not fail if resolved

    // 1. Look up "super" (the superclass object) at the resolved distance.
    let superclass_val = env
      .borrow_mut()
      .get_at(distance, "super")
      .ok_or(InterpreterError::RuntimeError)?
      .clone();

    let superclass = match superclass_val {
      LoxValue::Class(c) => c,
      _ => return Err(InterpreterError::RuntimeError), // Should be a class
    };

    // 2. Look up "this" (the instance object) one environment closer.
    // 'this' is always defined one scope inside 'super'.
    let instance_val = env
      .borrow_mut()
      .get_at(distance - 1, "this")
      .ok_or(InterpreterError::RuntimeError)?
      .clone();

    let instance = match instance_val {
      LoxValue::Instance(i) => i,
      _ => return Err(InterpreterError::RuntimeError), // Should be an instance
    };

    // 3. Find the method starting from the superclass.
    // Use the LoxClass::find_method which recursively searches superclasses.
    let method = superclass.find_method(&name.lexeme).ok_or_else(|| {
      eprintln!("Undefined property '{}'", name.lexeme);
      InterpreterError::RuntimeError
    })?;

    // 4. Bind the method to the current instance (`this`).
    let bound_method = method.bind(instance.clone());

    Ok((LoxValue::Function(bound_method), Some(name)))
  }

  fn eval_get(
    &mut self,
    env: &mut Rc<RefCell<Env>>,
    object: Expr,
    name: Token,
    engine: &mut DiagnosticEngine,
  ) -> Result<(LoxValue, Option<Token>), InterpreterError> {
    let (object_val, _) = self.eval_expr(object, env, engine)?;

    // ADD THIS HERE - Check if accessing a class (for static methods)
    if let LoxValue::Class(class) = object_val {
      // Accessing static method: MyClass.staticMethod()
      if let Some(static_method) = class.static_methods.get(&name.lexeme) {
        // Don't bind 'this' - static methods have no instance context
        return Ok((LoxValue::Function(static_method.clone()), Some(name)));
      }

      eprintln!("Undefined static method '{}'", name.lexeme);
      return Err(InterpreterError::RuntimeError);
    }

    if let LoxValue::Instance(instance) = object_val {
      if let Some(field) = instance.borrow().fields.get(&name.lexeme) {
        return Ok((field.clone(), Some(name)));
      }

      if let Some(method) = instance.borrow().class.find_method(&name.lexeme) {
        // Bind 'this' to the instance, regardless of which class defined the method
        let bound_method = method.bind(instance.clone());
        return Ok((LoxValue::Function(bound_method), Some(name)));
      }

      // Check methods and bind 'this'
      if let Some(method) = instance.borrow().class.methods.get(&name.lexeme) {
        let bound_method = method.bind(instance.clone());
        return Ok((LoxValue::Function(bound_method), Some(name)));
      }

      eprintln!("Undefined property '{}'", name.lexeme);
      return Err(InterpreterError::RuntimeError);
    }

    eprintln!("Cannot read property '{}' of non-instance", name.lexeme);
    return Err(InterpreterError::RuntimeError);
  }

  fn eval_set(
    &mut self,
    env: &mut Rc<RefCell<Env>>,
    object: Expr,
    name: Token,
    value: Expr,
    engine: &mut DiagnosticEngine,
  ) -> Result<(LoxValue, Option<Token>), InterpreterError> {
    let (object_val, _) = self.eval_expr(object, env, engine)?;

    if let LoxValue::Instance(instance) = object_val {
      let (value_result, _) = self.eval_expr(value, env, engine)?;

      // Set the field
      instance
        .borrow_mut()
        .fields
        .insert(name.lexeme.clone(), value_result.clone());

      return Ok((value_result, Some(name)));
    }

    eprintln!("Only instances have fields");
    Err(InterpreterError::RuntimeError)
  }

  fn eval_call(
    &mut self,
    env: &mut Rc<RefCell<Env>>,
    callee: Expr,
    paren: Token,
    arguments: Vec<Expr>,
    engine: &mut DiagnosticEngine,
  ) -> Result<(LoxValue, Option<Token>), InterpreterError> {
    let args_val = self.eval_args(env, arguments, engine)?;
    let (callee_val, token) = self.eval_expr(callee, env, engine)?;

    match callee_val {
      LoxValue::Function(fnc) => {
        let mut token = token.unwrap();
        token.position.0 += 1;

        if args_val.len() != fnc.arity() {
          let args_space: usize = args_val
            .clone()
            .into_iter()
            .map(|(_, v)| v.unwrap().lexeme.len())
            .sum();

          let diagnostic = Diagnostic::new(
            DiagnosticCode::WrongNumberOfArguments,
            "Wrong number of arguments".to_string(),
          )
          .with_label(Label::primary(
            token.to_span(),
            Some(format!(
              "wrong number of arguments, expected {} arguments but you passed {} arguments",
              fnc.arity(),
              args_val.len()
            )),
          ))
          .with_label(Label::secondary(
            Span {
              length: (args_space + 2 as usize),
              column: token.position.1 + 1,
              ..token.to_span()
            },
            Some(format!("expected {} arguments here", fnc.arity())),
          ));
          engine.emit(diagnostic);

          return Err(InterpreterError::RuntimeError);
        }

        let result = fnc.call(self, args_val, engine)?;
        return Ok((result, Some(paren)));
      },
      LoxValue::NativeFunction(fnc) => {
        if fnc.arity() != usize::MAX && args_val.len() != fnc.arity() {
          return Err(InterpreterError::RuntimeError);
        }

        let result = fnc.call(self, args_val, engine)?;
        return Ok((result, Some(paren)));
      },
      LoxValue::Class(class) => {
        // Check arity
        if args_val.len() != class.arity() {
          let mut token_copy = paren.clone();
          token_copy.position.0 += 1;

          let diagnostic = Diagnostic::new(
            DiagnosticCode::WrongNumberOfArguments,
            "Wrong number of arguments".to_string(),
          )
          .with_label(Label::primary(
            token_copy.to_span(),
            Some(format!(
              "Expected {} arguments but got {}",
              class.arity(),
              args_val.len()
            )),
          ));
          engine.emit(diagnostic);

          return Err(InterpreterError::RuntimeError);
        }

        // Call the class (which handles init() internally)
        let result = class.call(self, args_val, engine)?;

        return Ok((result, Some(paren)));
      },
      _ => Err(InterpreterError::RuntimeError),
    }
  }

  fn eval_args(
    &mut self,
    env: &mut Rc<RefCell<Env>>,
    arguments: Vec<Expr>,
    engine: &mut DiagnosticEngine,
  ) -> Result<Vec<(LoxValue, Option<Token>)>, InterpreterError> {
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
  ) -> Result<(LoxValue, Option<Token>), InterpreterError> {
    if let Some(&depth) = self.locals.get(&token.lexeme) {
      match env.borrow_mut().get_at(depth, &token.lexeme.as_str()) {
        Some(v) => return Ok((v.clone(), Some(token))),
        None => {
          eprintln!(
            "INTERNAL ERROR: Resolved variable '{}' not found at depth {}",
            token.lexeme, depth
          );
          return Err(InterpreterError::RuntimeError);
        },
      }
    }

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
        Err(InterpreterError::RuntimeError)
      },
    }
  }

  fn eval_assign(
    &mut self,
    mut name: Token,
    value: Expr,
    env: &mut Rc<RefCell<Env>>,
    engine: &mut DiagnosticEngine,
  ) -> Result<(LoxValue, Option<Token>), InterpreterError> {
    let (value, token) = self.eval_expr(value, env, engine)?;

    // Check if we have a resolved depth
    if let Some(&depth) = self.locals.get(&name.lexeme) {
      if env
        .borrow_mut()
        .assign_at(depth, &name.lexeme, value.clone())
      {
        return Ok((value, token));
      }
    }

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
      return Err(InterpreterError::RuntimeError);
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
  ) -> Result<(LoxValue, Option<Token>), InterpreterError> {
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
  ) -> Result<(LoxValue, Option<Token>), InterpreterError> {
    match operator.lexeme.as_str() {
      "%" | "*" | "/" | "-" => self.eval_arithmetic(env, operator, lhs, rhs, engine),
      "+" => self.eval_addition(env, operator, lhs, rhs, engine),
      "==" | "!=" => self.eval_equality(env, operator, lhs, rhs, engine),
      ">" | ">=" | "<" | "<=" => self.eval_comparison(env, operator, lhs, rhs, engine),
      "||" | "&&" => self.eval_logical(env, operator, lhs, rhs, engine),
      "," => Err(InterpreterError::RuntimeError),
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
  ) -> Result<(LoxValue, Option<Token>), InterpreterError> {
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
      _ => Err(InterpreterError::RuntimeError),
    }
  }

  fn eval_arithmetic(
    &mut self,
    env: &mut Rc<RefCell<Env>>,
    operator: Token,
    lhs: Expr,
    rhs: Expr,
    engine: &mut DiagnosticEngine,
  ) -> Result<(LoxValue, Option<Token>), InterpreterError> {
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
          &format!("Expected number, found {}", &bad_value.to_string()),
        )
      },
      (lhs, rhs) => self.emit_error(
        engine,
        DiagnosticCode::InvalidOperator,
        &format!(
          "Cannot perform arithmetic on {} and {}",
          &lhs.to_string(),
          &rhs.to_string()
        ),
        &operator,
        "Both operands must be numbers",
        Some(&format!(
          "Left operand is {}, right operand is {}",
          &lhs.to_string(),
          &rhs.to_string()
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
  ) -> Result<(LoxValue, Option<Token>), InterpreterError> {
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
        &format!("Cannot add {} and {}", &lhs.to_string(), &rhs.to_string()),
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
  ) -> Result<(LoxValue, Option<Token>), InterpreterError> {
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
  ) -> Result<(LoxValue, Option<Token>), InterpreterError> {
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
          &lhs.to_string(),
          &rhs.to_string()
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
  ) -> Result<(LoxValue, Option<Token>), InterpreterError> {
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
          &format!("Expected number, found {}", &rhs_val.to_string()),
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
  ) -> Result<(LoxValue, Option<Token>), InterpreterError> {
    self.eval_expr(expr, env, engine)
  }

  fn eval_literal(
    &self,
    token: Token,
    engine: &mut DiagnosticEngine,
  ) -> Result<(LoxValue, Option<Token>), InterpreterError> {
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
  ) -> Result<(LoxValue, Option<Token>), InterpreterError> {
    let mut diagnostic = Diagnostic::new(code, message.to_string())
      .with_label(Label::primary(token.to_span(), Some(label_msg.to_string())));

    if let Some(help_msg) = help {
      diagnostic = diagnostic.with_help(help_msg.to_string());
    }

    engine.emit(diagnostic);
    Err(InterpreterError::RuntimeError)
  }

  fn emit_type_error(
    &self,
    engine: &mut DiagnosticEngine,
    operator: &Token,
    operand_token: Option<&Token>,
    message: &str,
    label_msg: &str,
  ) -> Result<(LoxValue, Option<Token>), InterpreterError> {
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
    Err(InterpreterError::RuntimeError)
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
  ) -> Result<(LoxValue, Option<Token>), InterpreterError> {
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
    Err(InterpreterError::RuntimeError)
  }

  fn is_truthy(&self, val: &LoxValue) -> bool {
    return match &val {
      LoxValue::Bool(b) => *b,
      LoxValue::Nil => false,
      LoxValue::Number(n) => *n != 0.0,
      LoxValue::String(s) => !s.is_empty(),
      LoxValue::Function(_) => false,
      LoxValue::NativeFunction(_) => false,
      LoxValue::Class(_) => false,
      LoxValue::Instance(_) => false,
    };
  }
}
