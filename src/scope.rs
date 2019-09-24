use crate::variable::Variable;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Scope {
  pub vars: HashMap<String, Variable>,
}

impl From<Vec<Variable>> for Scope {
  fn from(mut vars: Vec<Variable>) -> Self {
    let mut map = HashMap::new();
    map.reserve(vars.len());
    for var in vars.drain(..) {
      if map.contains_key(&var.name) {
        panic!("Duplicate argument");
      }
      map.insert(var.name.clone(), var);
    }
    Scope { vars: map }
  }
}

impl Scope {
  pub fn new() -> Scope {
    Scope {
      vars: HashMap::new(),
    }
  }
}
