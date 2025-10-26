use std::{cell::RefCell, collections::HashMap, rc::Rc, sync::Arc};

use crate::function::{normal::LoxFunction, LoxCallable};

#[derive(Debug, Clone)]
pub struct LoxClass {
  pub name: String,
  pub methods: HashMap<String, Arc<LoxFunction>>,
  pub instance: Option<Arc<LoxClassInstance>>,
}

#[derive(Debug)]
pub struct LoxClassInstance {
  pub class: Arc<LoxClass>,
  pub fields: HashMap<String, crate::lox_value::LoxValue>,
}

impl LoxCallable for LoxClass {
  fn arity(&self) -> usize {
    0
  }

  fn call(
    &self,
    interpreter: &mut crate::interpreter::Interpreter,
    arguments: Vec<(crate::lox_value::LoxValue, Option<scanner::token::Token>)>,
    engine: &mut diagnostic::DiagnosticEngine,
  ) -> Result<crate::lox_value::LoxValue, crate::lox_value::InterpreterError> {
    // let mut enclosing_env = Rc::new(RefCell::new(
    //   self
    //     .closure
    //     .borrow_mut()
    //     .with_enclosing(Rc::clone(&self.closure)),
    // ));
    println!("Class: {:#?}", self);

    let instance = Rc::new(RefCell::new(LoxClassInstance {
      class: Arc::new(self.clone()),
      fields: HashMap::new(),
    }));

    Ok(crate::lox_value::LoxValue::Instance(instance))
  }
}
