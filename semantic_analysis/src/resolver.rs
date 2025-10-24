use diagnostic::DiagnosticEngine;
use parser::stmt::Stmt;

pub struct Resolver {}

impl Resolver {
  pub fn new() -> Self {
    Self {}
  }

  pub fn resolve(&mut self, ast: Vec<Stmt>, engine: &mut DiagnosticEngine) {
    // let mut env = self.env.clone();
    for stmt in ast {
      println!("{:?}", stmt);
      // let _ = self.eval_stmt(stmt, &mut env, engine);
    }
    // self.env = env;
  }
}
