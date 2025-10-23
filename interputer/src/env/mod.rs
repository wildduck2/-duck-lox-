use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::lox_value::LoxValue;

#[derive(Debug, Clone)]
pub struct Env {
  values: HashMap<String, LoxValue>,
  enclosing: Option<Rc<RefCell<Env>>>,
}

impl Env {
  pub fn new() -> Self {
    Self {
      values: HashMap::new(),
      enclosing: None,
    }
  }

  pub fn new_with_enclosing(enclosing: Rc<RefCell<Env>>) -> Self {
    Self {
      values: HashMap::new(),
      enclosing: Some(enclosing),
    }
  }

  pub fn with_enclosing(&mut self, env: Rc<RefCell<Env>>) -> Self {
    Self {
      values: HashMap::new(),
      enclosing: Some(env),
    }
  }

  pub fn define(&mut self, name: String, value: LoxValue) {
    self.values.insert(name, value);
  }

  pub fn get(&self, name: &str) -> Option<LoxValue> {
    if let Some(v) = self.values.get(name) {
      return Some(v.clone());
    }
    if let Some(enclosing) = &self.enclosing {
      return enclosing.borrow().get(name);
    }
    None
  }

  pub fn assign(&mut self, name: &str, value: LoxValue) -> bool {
    if self.values.contains_key(name) {
      self.values.insert(name.to_string(), value);
      return true;
    }
    if let Some(enclosing) = &mut self.enclosing {
      return enclosing.borrow_mut().assign(name, value);
    }
    false
  }
}

pub fn global_functions() {}
