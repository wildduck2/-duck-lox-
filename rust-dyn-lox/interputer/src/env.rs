use crate::lox_value::LoxValue;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

#[derive(Debug, Clone)]
pub struct Env {
  pub values: HashMap<String, LoxValue>,
  pub enclosing: Option<Rc<RefCell<Env>>>,
}

impl Env {
  pub fn new() -> Self {
    Self {
      values: HashMap::new(),
      enclosing: None,
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

  /// Get value at a specific depth (0 = current scope)
  pub fn get_at(&self, distance: usize, name: &str) -> Option<LoxValue> {
    if distance == 0 {
      // Distance 0 means THIS environment
      return self.values.get(name).cloned();
    }

    // Otherwise, walk up the chain
    self
      .ancestor(distance)
      .and_then(|env| env.borrow().values.get(name).cloned())
  }

  /// Assign at a specific depth (0 = current scope)
  pub fn assign_at(&mut self, distance: usize, name: &str, value: LoxValue) -> bool {
    if distance == 0 {
      if self.values.contains_key(name) {
        self.values.insert(name.to_string(), value);
        return true;
      }
      return false;
    }

    if let Some(env) = self.ancestor(distance) {
      let mut env = env.borrow_mut();
      if env.values.contains_key(name) {
        env.values.insert(name.to_string(), value);
        return true;
      }
    }
    false
  }

  /// Walk up the environment chain by 'distance' steps
  /// distance=1 means parent, distance=2 means grandparent, etc.
  fn ancestor(&self, distance: usize) -> Option<Rc<RefCell<Env>>> {
    if distance == 0 {
      return None; // Should not be called with 0, handled in get_at/assign_at
    }

    let mut env = self.enclosing.clone()?;
    for _ in 1..distance {
      let next = env.borrow().enclosing.clone()?;
      env = next;
    }
    Some(env)
  }
}
