use std::{fmt, sync::Arc};

use crate::function::{normal::LoxFunction, LoxCallable};

#[derive(Debug)]
pub enum InterpreterError {
  Return(LoxValue),
  RuntimeError,
  Break,
  Continue,
}

#[derive(Clone)]
pub enum LoxValue {
  Nil,
  Number(f64),
  String(String),
  Bool(bool),
  Function(Arc<LoxFunction>),
  NativeFunction(Arc<dyn LoxCallable + Send + Sync>),
}

impl fmt::Debug for LoxValue {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      LoxValue::Nil => write!(f, "Nil"),
      LoxValue::Number(n) => write!(f, "Number({n})"),
      LoxValue::String(s) => write!(f, "String({s:?})"),
      LoxValue::Bool(b) => write!(f, "Bool({b})"),
      LoxValue::Function(_) => write!(f, "Function(<fn>)"),
      LoxValue::NativeFunction(_) => write!(f, "NativeFunction(<native>)"),
    }
  }
}

impl fmt::Display for LoxValue {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      LoxValue::String(s) => write!(f, "{s}"),
      LoxValue::Number(n) => write!(f, "{n}"),
      LoxValue::Bool(b) => write!(f, "{b}"),
      LoxValue::Nil => write!(f, "nil"),
      LoxValue::Function(_) => write!(f, "<function>"),
      LoxValue::NativeFunction(_) => write!(f, "<native function>"),
    }
  }
}
