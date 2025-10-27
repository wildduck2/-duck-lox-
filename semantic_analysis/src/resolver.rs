use diagnostic::{
  diagnostic::{Diagnostic, Label, Span},
  diagnostic_code::DiagnosticCode,
  DiagnosticEngine,
};
use parser::{expr::Expr, stmt::Stmt};
use scanner::token::Token;
use std::collections::HashMap;

pub struct Resolver {
  scopes: Vec<HashMap<String, VariableState>>,
  locals: HashMap<String, usize>,
  current_class: ClassType,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum ClassType {
  None,
  Class,
  Instance,
  StaticMethod, // ADD THIS
}

#[derive(Debug, Clone)]
struct VariableState {
  defined: bool,
  used: bool,
  line: usize,
}

impl Resolver {
  pub fn new() -> Self {
    Self {
      scopes: vec![],
      locals: HashMap::new(),
      current_class: ClassType::None,
    }
  }

  pub fn run(&mut self, ast: &Vec<Stmt>, engine: &mut DiagnosticEngine) {
    self.resolve_statements(ast, engine);
  }

  /// Entry points
  pub fn resolve_statements(&mut self, stmts: &Vec<Stmt>, engine: &mut DiagnosticEngine) {
    for s in stmts {
      self.resolve_stmt(&s, engine);
    }
  }

  fn resolve_stmt(&mut self, stmt: &Stmt, engine: &mut DiagnosticEngine) {
    match stmt {
      Stmt::Block(block) => {
        self.begin_scope();
        self.resolve_statements(block, engine);
        self.end_scope(engine);
      },
      Stmt::VarDecl(token, value) => {
        if self.scopes.is_empty() {
          if let Some(value) = value {
            self.resolve_expr(value, engine);
          }
        } else {
          self.declare(token, engine);
          if let Some(value) = value {
            self.resolve_expr(value, engine);
          }
          self.define(token);
        }
      },
      Stmt::Expr(expr) => self.resolve_expr(expr, engine),
      Stmt::If(condition, then_branch, else_branch) => {
        self.resolve_expr(condition, engine);
        self.resolve_stmt(then_branch, engine);
        if let Some(else_branch) = else_branch {
          self.resolve_stmt(else_branch, engine);
        }
      },
      Stmt::While(condition, body) => {
        self.resolve_expr(condition, engine);
        self.resolve_stmt(body, engine);
      },
      Stmt::Fun(name, params, body) => {
        if let Expr::Identifier(name) = name {
          if !self.scopes.is_empty() {
            self.declare(name, engine);
            self.define(name);
          }
        }

        self.resolve_function(params, body, engine);
      },
      Stmt::Return(_, value) => {
        if let Some(value) = value {
          self.resolve_expr(value, engine);
        }
      },
      Stmt::Class(name, methods, static_methods) => {
        let enclosing_class = self.current_class;
        self.current_class = ClassType::Class;

        match &name {
          Expr::Identifier(token) => {
            self.declare(token, engine);
            self.define(token);
          },
          _ => {
            eprintln!("Class name must be an identifier got {:?}", name);
            return;
          },
        };

        // Resolve INSTANCE methods with 'this' in scope
        self.begin_scope();
        self.scopes.last_mut().unwrap().insert(
          "this".to_string(),
          VariableState {
            defined: true,
            used: false,
            line: if let Expr::Identifier(token) = &name {
              token.position.0
            } else {
              0
            },
          },
        );

        for method in methods.iter() {
          self.resolve_stmt(method, engine);
        }

        self.end_scope(engine);

        // Resolve STATIC methods WITHOUT 'this' in scope
        let prev_class = self.current_class;
        self.current_class = ClassType::StaticMethod; // Mark as static context

        for method in static_methods.iter() {
          self.resolve_stmt(method, engine);
        }

        self.current_class = prev_class; // Restore context

        self.current_class = enclosing_class;
      },

      Stmt::Break(_) | Stmt::Continue(_) => {},
    }
  }

  fn resolve_expr(&mut self, expr: &Expr, engine: &mut DiagnosticEngine) {
    match expr {
      Expr::Identifier(token) => {
        if let Some(scope) = self.scopes.last() {
          if let Some(is_defined) = scope.get(&token.lexeme) {
            if !is_defined.defined {
              eprintln!(
                "Can't read local variable '{}' in its own initializer",
                token.lexeme
              )
            }
          }
        }
        self.resolve_local(&token.lexeme);
      },
      Expr::Call {
        callee,
        paren,
        arguments,
      } => {
        self.resolve_expr(callee, engine);
        for argument in arguments {
          self.resolve_expr(argument, engine);
        }
      },
      Expr::Unary { operator, rhs } => {
        self.resolve_expr(rhs, engine);
      },
      Expr::Binary { lhs, operator, rhs } => {
        self.resolve_expr(lhs, engine);
        self.resolve_expr(rhs, engine);
      },
      Expr::Grouping(expr) => {
        self.resolve_expr(expr, engine);
      },
      Expr::Ternary {
        condition,
        then_branch,
        else_branch,
      } => {
        self.resolve_expr(condition, engine);
        self.resolve_expr(then_branch, engine);
        self.resolve_expr(else_branch, engine);
      },
      Expr::Assign { name, value } => {
        self.resolve_expr(value, engine);
        self.resolve_local(&name.lexeme);
      },
      Expr::Literal(_) => {},

      Expr::Get { object, name: _ } => {
        // Only resolve the object, not the property name
        // (property names are resolved at runtime)
        self.resolve_expr(object, engine);
      },

      Expr::Set {
        object,
        name: _,
        value,
      } => {
        self.resolve_expr(value, engine);
        self.resolve_expr(object, engine);
      },
      Expr::This(keyword) => {
        // Check if we're in a static method
        if self.current_class == ClassType::StaticMethod {
          let diagnostic = Diagnostic::new(
      DiagnosticCode::InvalidThis,
      "Can't use 'this' in static methods".to_string(),
    )
    .with_label(Label::primary(
      keyword.to_span(),
      Some("'this' not allowed in static context".to_string()),
    ))
    .with_help("Static methods don't have access to instance data. Remove 'static' or use instance methods instead.".to_string());
          engine.emit(diagnostic);
          return;
        }

        // Check if we're outside any class
        if self.current_class == ClassType::None {
          let diagnostic = Diagnostic::new(
            DiagnosticCode::InvalidThis,
            "Can't use 'this' outside of a class".to_string(),
          )
          .with_label(Label::primary(
            keyword.to_span(),
            Some("'this' not allowed here".to_string()),
          ));
          engine.emit(diagnostic);
          return;
        }

        self.resolve_local(&keyword.lexeme);
      },
    }
  }

  fn resolve_function(&mut self, params: &[Expr], body: &Stmt, engine: &mut DiagnosticEngine) {
    self.begin_scope();

    for param in params {
      if let Expr::Identifier(param) = param {
        if !self.scopes.is_empty() {
          self.declare(param, engine);
          self.define(param);
        }
      }
    }

    self.resolve_stmt(body, engine);
    self.end_scope(engine);
  }

  fn resolve_local(&mut self, name: &str) {
    // Iterate from INNERMOST (last) to OUTERMOST (first)
    for (i, scope) in self.scopes.iter_mut().rev().enumerate() {
      if let Some(local) = scope.get_mut(name) {
        local.used = true;
        self.locals.insert(name.to_string(), i);
        return;
      }
    }
    // Not found in any local scope = global variable
  }

  // Helpers
  fn begin_scope(&mut self) {
    self.scopes.push(HashMap::new());
  }

  fn end_scope(&mut self, engine: &mut DiagnosticEngine) {
    if let Some(scope) = self.scopes.pop() {
      for (name, state) in scope {
        if state.defined && !state.used {
          let diagnostic = Diagnostic::new(
            DiagnosticCode::UnusedVariable,
            format!("Variable '{}' is never used", name),
          )
          .with_label(Label::primary(
            Span {
              line: state.line + 1,
              column: 0,
              length: 25,
              file: "".to_string(),
            },
            Some("never used".to_string()),
          ))
          .with_help("Did you forget to use it?".to_string())
          .with_note("Unused variables are a common source of bugs.".to_string());

          engine.emit(diagnostic);
        }
      }
    }
  }

  /// Returns true if successful, false if already declared
  fn declare(&mut self, name: &Token, engine: &mut DiagnosticEngine) -> bool {
    if self.scopes.is_empty() {
      return true; // global scope, always allow
    }

    let scope = self.scopes.last_mut().unwrap();

    // Check for duplicate declaration in same scope
    if scope.contains_key(&name.lexeme) {
      let diagnostic = Diagnostic::new(
        DiagnosticCode::VariableAlreadyDeclared,
        format!(
          "Variable '{}' is already declared in this scope",
          name.lexeme
        ),
      )
      .with_label(Label::primary(
        name.to_span(),
        Some("already declared here".to_string()),
      ))
      .with_help(
        "Did you mean to assign to the existing variable? Remove 'var' to assign.".to_string(),
      );

      engine.emit(diagnostic);
      return false;
    }

    // Mark as declared but not yet defined
    scope.insert(
      name.lexeme.clone(),
      VariableState {
        defined: false,
        used: false,
        line: name.position.0,
      },
    );
    true
  }

  /// Mark variable as defined / ready to use.
  fn define(&mut self, name: &Token) {
    if self.scopes.is_empty() {
      return; // global, we do not track in local scope
    }
    let scope = self.scopes.last_mut().unwrap();
    if let Some(local) = scope.get_mut(&name.lexeme) {
      local.defined = true;
    }
  }

  pub fn get_locals(&self) -> &HashMap<String, usize> {
    &self.locals
  }
}
