use std::collections::HashMap;

use crate::interpreter::LoxValue;

#[derive(Debug, Clone)]
pub struct Env {
  values: HashMap<String, LoxValue>,
  enclosing: Option<Box<Env>>,
}

impl Env {
  pub fn new() -> Self {
    Self {
      values: HashMap::new(),
      enclosing: None,
    }
  }

  pub fn with_enclosing(&mut self, env: Env) -> Self {
    Self {
      values: HashMap::new(),
      enclosing: Some(Box::new(env)),
    }
  }

  pub fn get(&self, name: &str) -> Option<LoxValue> {
    if let Some(v) = self.values.get(name) {
      return Some(v.clone());
    }
    if let Some(enclosing) = &self.enclosing {
      return enclosing.get(name);
    }
    None
  }

  pub fn define(&mut self, name: String, value: LoxValue) {
    self.values.insert(name, value);
  }

  pub fn assign(&mut self, name: &str, value: LoxValue) -> bool {
    if self.values.contains_key(name) {
      self.values.insert(name.to_string(), value);
      return true;
    }
    if let Some(enclosing) = &mut self.enclosing {
      return enclosing.assign(name, value);
    }
    false
  }
}
