use std::{cell::RefCell, collections::HashMap, rc::Rc, sync::Arc};

use crate::{
  function::{normal::LoxFunction, LoxCallable},
  lox_value::{InterpreterError, LoxValue},
};

#[derive(Debug, Clone)]
pub struct LoxClass {
  pub name: String,
  pub superclass: LoxValue,
  pub methods: HashMap<String, Arc<LoxFunction>>,
  pub static_methods: HashMap<String, Arc<LoxFunction>>,
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
    if let Some(initializer) = self.methods.get("init") {
      initializer.arity()
    } else {
      0
    }
  }

  fn call(
    &self,
    interpreter: &mut crate::interpreter::Interpreter,
    arguments: Vec<(crate::lox_value::LoxValue, Option<scanner::token::Token>)>,
    engine: &mut diagnostic::DiagnosticEngine,
  ) -> Result<crate::lox_value::LoxValue, crate::lox_value::InterpreterError> {
    // STEP 1: Create the instance
    let instance = Rc::new(RefCell::new(LoxClassInstance {
      class: Arc::new(self.clone()),
      fields: HashMap::new(),
    }));

    // STEP 2: Look for init() method
    if let Some(initializer) = self.find_method("init") {
      // Bind 'this' to the instance
      let bound_init = initializer.bind(instance.clone());

      // Check arity (already checked in eval_call, but double-check here)
      if arguments.len() != bound_init.arity() {
        return Err(InterpreterError::RuntimeError);
      }

      // Call init() with arguments
      // Pass the interpreter, not self!
      bound_init.call(interpreter, arguments, engine)?;
    } else {
      // No init() - must have 0 arguments
      if !arguments.is_empty() {
        return Err(InterpreterError::RuntimeError);
      }
    }

    // STEP 3: Return the instance
    Ok(crate::lox_value::LoxValue::Instance(instance))
  }
}

impl LoxClass {
  pub fn find_method(&self, name: &str) -> Option<&Arc<LoxFunction>> {
    if let Some(method) = self.methods.get(name) {
      return Some(&method);
    }

    if let LoxValue::Class(superclass_arc) = &self.superclass {
      // Recursively call find_method on the superclass's LoxClass
      // Note: We need to check if the superclass is actually a class before calling find_method
      let superclass_loxclass: &LoxClass = &superclass_arc;
      return superclass_loxclass.find_method(name);
    }

    None
  }
}
