use crate::{func::Func, scope::Scope, value::Value, variable::Variable};

#[derive(Debug)]
pub struct Context {
  scopes: Vec<Scope>,
  pub current_func: Func,
}

impl From<&Func> for Context {
  fn from(func: &Func) -> Self {
    Context {
      scopes: vec![],
      current_func: (*func).clone(),
    }
  }
}

impl Context {
  pub fn push(&mut self, scope: Scope) {
    self.scopes.push(scope);
  }

  pub fn pop(&mut self) {
    self.scopes.pop();
  }

  pub fn insert_variable(&mut self, id: String, val: Value) {
    match (*self).scopes.iter_mut().last() {
      Some(scope) => (*scope).vars.insert(
        id.clone(),
        Variable {
          name: id,
          value: val,
        },
      ),
      None => panic!("Inserting into empty scope"),
    };
  }

  pub fn get_variable(&self, var: String) -> Option<&Variable> {
    for scope in self.scopes.iter().rev() {
      match scope.vars.get(&var) {
        Some(ref mut var) => {
          return Some(&var);
        }
        None => (),
      };
    }
    None
  }

  pub fn get_variable_mut(&mut self, var: String) -> Option<&mut Variable> {
    for scope in self.scopes.iter_mut().rev() {
      match scope.vars.get_mut(&var) {
        Some(var) => {
          return Some(var);
        }
        None => (),
      };
    }
    None
  }
}
