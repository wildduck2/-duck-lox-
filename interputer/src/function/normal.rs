use std::{cell::RefCell, rc::Rc};

use diagnostic::DiagnosticEngine;
use parser::stmt::Stmt;
use scanner::token::Token;

use crate::{function::LoxCallable, interpreter::Interpreter, lox_value::LoxValue};

#[derive(Debug, Clone)]
pub struct LoxFunction {
  pub params: Vec<Token>,
  pub body: Vec<Stmt>,
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
    for (i, (arg_val, _)) in arguments.iter().enumerate() {
      enclosing_env
        .borrow_mut()
        .define(self.params[i].lexeme.to_string(), arg_val.clone());
    }

    match interpreter.eval_block(Box::new(self.body.clone()), &mut enclosing_env, engine) {
      Ok((v, _)) => Ok(v),
      Err(e) => Ok(LoxValue::Nil),
    }
  }
}
