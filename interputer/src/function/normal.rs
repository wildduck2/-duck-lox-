use std::{cell::RefCell, rc::Rc, sync::Arc};

use diagnostic::DiagnosticEngine;
use parser::stmt::Stmt;
use scanner::token::Token;

use crate::{
  class::LoxClassInstance,
  env::Env,
  function::LoxCallable,
  interpreter::Interpreter,
  lox_value::{InterpreterError, LoxValue},
};

#[derive(Debug, Clone)]
pub struct LoxFunction {
  pub params: Vec<Token>,
  pub body: Vec<Stmt>,
  pub closure: Rc<RefCell<Env>>,
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
  ) -> Result<LoxValue, InterpreterError> {
    let mut enclosing_env = Rc::new(RefCell::new(
      self
        .closure
        .borrow_mut()
        .with_enclosing(Rc::clone(&self.closure)),
    ));

    // Defining the args in the function scope to be used
    for (i, (arg_val, _)) in arguments.iter().enumerate() {
      enclosing_env
        .borrow_mut()
        .define(self.params[i].lexeme.to_string(), arg_val.clone());
    }

    match interpreter.eval_block(Box::new(self.body.clone()), &mut enclosing_env, engine) {
      Ok((v, _)) => Ok(v),
      Err(e) => match e {
        InterpreterError::Return(v) => Ok(v),
        _ => Ok(LoxValue::Nil),
      },
    }
  }
}

impl LoxFunction {
  pub fn bind(&self, instance: Rc<RefCell<LoxClassInstance>>) -> Arc<LoxFunction> {
    // Create a new environment with "this" bound to the instance
    let mut environment = Env::new();
    environment.enclosing = Some(self.closure.clone());
    environment.define("this".to_string(), LoxValue::Instance(instance));

    Arc::new(LoxFunction {
      params: self.params.clone(),
      body: self.body.clone(),
      closure: Rc::new(RefCell::new(environment)),
    })
  }
}
