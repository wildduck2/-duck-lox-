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
  current_superclass: ClassType,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum ClassType {
  None,
  Class,
  // Instance,
  Subclass,
  StaticMethod,
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
      current_superclass: ClassType::None,
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
      Stmt::Class(name, superclass_expr, methods, static_methods) => {
        let enclosing_class = self.current_class;
        let enclosing_superclass = self.current_superclass; // Store previous state

        let name_token = match &name {
          Expr::Identifier(token) => {
            self.declare(token, engine);
            self.define(token);
            token
          },
          _ => {
            eprintln!("Class name must be an identifier got {:?}", name);
            return;
          },
        };

        // 1. Resolve Superclass Expression (if present)
        if let Some(superclass) = superclass_expr {
          // Check for illegal inheritance (Class A inherits A)
          if let Expr::Identifier(token) = superclass {
            if token.lexeme == name_token.lexeme {
              // ... (Keep your existing self-inheritance diagnostic code here) ...
              let diagnostic = Diagnostic::new(
                DiagnosticCode::InvalidSuperclass,
                "A class cannot inherit from itself.".to_string(),
              )
              .with_label(Label::primary(
                token.to_span(),
                Some("self-inheritance here".to_string()),
              ))
              .with_help("Change the superclass name to a different class.".to_string());
              engine.emit(diagnostic);
              return;
            }
          }
          self.resolve_expr(superclass, engine);
          self.current_superclass = ClassType::Subclass; // Set superclass flag
        };

        // Determine current class type
        self.current_class = if superclass_expr.is_some() {
          ClassType::Subclass
        } else {
          ClassType::Class
        };

        // 2. Begin Scope for 'super' (if superclass exists)
        if superclass_expr.is_some() {
          self.begin_scope();
          // Define 'super' in the scope so it can be resolved.
          // This scope will be one level "outside" the 'this' scope.
          self.scopes.last_mut().unwrap().insert(
            "super".to_string(),
            VariableState {
              defined: true,
              used: false,
              line: name_token.position.0,
            },
          );
        }

        // 3. Resolve INSTANCE methods (with 'this' in scope)
        self.begin_scope(); // Scope for 'this'
        self.scopes.last_mut().unwrap().insert(
          "this".to_string(),
          VariableState {
            defined: true,
            used: false,
            line: name_token.position.0,
          },
        );

        for method in methods.iter() {
          // NOTE: A more complete implementation would check if the method is 'init'
          // and disallow 'super' access within it, as per the Lox language design.
          self.resolve_stmt(method, engine);
        }

        self.end_scope(engine); // End 'this' scope

        // 4. End 'super' scope (if one was created)
        if superclass_expr.is_some() {
          self.end_scope(engine);
        }

        // 5. Resolve STATIC methods (without 'this' in scope)
        let prev_class = self.current_class;
        self.current_class = ClassType::StaticMethod;

        for method in static_methods.iter() {
          self.resolve_stmt(method, engine);
        }

        // 6. Restore context
        self.current_class = enclosing_class;
        self.current_superclass = enclosing_superclass;
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

      Expr::Super(keyword, method_name) => {
        // Check 1: Must be inside a class
        if self.current_class == ClassType::None {
          let diagnostic = Diagnostic::new(
            DiagnosticCode::InvalidThis,
            "Can't use 'super' outside of a class method".to_string(),
          )
          .with_label(Label::primary(
            keyword.to_span(),
            Some("'super' not allowed here".to_string()),
          ));
          engine.emit(diagnostic);
          return;
        }

        // heck 2: Must be in a subclass (i.e., class with a superclass)
        if self.current_superclass != ClassType::Subclass {
          let diagnostic = Diagnostic::new(
            DiagnosticCode::InvalidThis,
            "Can't use 'super' in a class that doesn't inherit from another class".to_string(),
          )
          .with_label(Label::primary(
            keyword.to_span(),
            Some("'super' not allowed without a superclass".to_string()),
          ));
          engine.emit(diagnostic);
          return;
        }

        // Check 3: Disallow 'super' in static methods
        if self.current_class == ClassType::StaticMethod {
          let diagnostic = Diagnostic::new(
            DiagnosticCode::InvalidThis,
                "Can't use 'super' in static methods".to_string(),
            )
            .with_label(Label::primary(
                keyword.to_span(),
                Some("'super' not allowed in static context".to_string()),
            ))
            .with_help("Superclass methods are instance methods. Remove 'static' or use instance methods instead.".to_string());
          engine.emit(diagnostic);
          return;
        }

        // Resolve 'super' keyword. This finds the environment where the superclass
        // reference is stored, and records the depth in `self.locals`.
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
