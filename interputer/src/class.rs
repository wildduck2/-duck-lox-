use std::{cell::RefCell, collections::HashMap, rc::Rc, sync::Arc};

use crate::function::{normal::LoxFunction, LoxCallable};

#[derive(Debug, Clone)]
pub struct LoxClass {
  pub name: String,
  pub methods: HashMap<String, Arc<LoxFunction>>,
}

pub struct LoxClassInstance {
  pub class: Arc<LoxClass>,
  pub fields: HashMap<String, crate::lox_value::LoxValue>,
}

// Implement Debug manually to avoid infinite recursion
impl std::fmt::Debug for LoxClassInstance {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("LoxClassInstance")
      .field("class", &self.class.name) // Just print the class name
      .field("fields", &self.fields.keys().collect::<Vec<_>>()) // Just print field names
      .finish()
  }
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
    let instance = Rc::new(RefCell::new(LoxClassInstance {
      class: Arc::new(self.clone()),
      fields: HashMap::new(),
    }));

    Ok(crate::lox_value::LoxValue::Instance(instance))
  }
}
