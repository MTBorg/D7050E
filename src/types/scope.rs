use crate::types::{func_param::FuncParam, _type::Type, variable::Variable};
use std::collections::HashMap;

#[derive(Debug)]
pub struct Scope<T> {
  pub elements: HashMap<String, T>,
}

impl From<Vec<Variable>> for Scope<Variable> {
  fn from(mut vars: Vec<Variable>) -> Self {
    let mut map = HashMap::new();
    map.reserve(vars.len());
    for var in vars.drain(..) {
      if map.contains_key(&var.name) {
        panic!("Duplicate argument");
      }
      map.insert(var.name.clone(), var);
    }
    Scope { elements: map }
  }
}

impl From<Vec<FuncParam>> for Scope<(Type, bool)> {
  fn from(mut params: Vec<FuncParam>) -> Self {
    let mut map = HashMap::new();
    map.reserve(params.len());
    for param in params.drain(..) {
      if map.contains_key(&param.name) {
        panic!("Duplicate argument");
      }
      map.insert(param.name.clone(), (param._type, param.mutable));
    }
    Scope { elements: map }
  }
}

impl<T> Scope<T> {
  pub fn new() -> Self {
    Self {
      elements: HashMap::new(),
    }
  }
}
