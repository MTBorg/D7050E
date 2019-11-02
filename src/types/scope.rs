use crate::types::{_type::Type, func_param::FuncParam, variable::Variable};
use std::collections::HashMap;

#[derive(Debug)]
pub struct Scope<'a, T> {
  pub elements: HashMap<&'a str, T>,
}

impl<'a> From<Vec<Variable<'a>>> for Scope<'a, Variable<'a>> {
  fn from(mut vars: Vec<Variable<'a>>) -> Self {
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

impl<'a> From<Vec<FuncParam<'a>>> for Scope<'a, (Type, bool)> {
  fn from(mut params: Vec<FuncParam<'a>>) -> Self {
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

impl<'a, T> Scope<'a, T> {
  pub fn new() -> Self {
    Self {
      elements: HashMap::new(),
    }
  }
}
