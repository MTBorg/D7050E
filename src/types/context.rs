use crate::types::{_type::Type, func::Func, scope::Scope, variable::Variable};

#[derive(Debug)]
pub struct Context<'a, T> {
  scopes: Vec<Scope<'a, T>>,
  pub current_func: &'a Func<'a>,
}

impl<'a, T> From<&'a Func<'a>> for Context<'a, T> {
  fn from(func: &'a Func<'a>) -> Self {
    Context {
      scopes: vec![],
      current_func: func,
    }
  }
}

impl<'a> Context<'a, Variable<'a>> {
  pub fn insert_variable(&mut self, var: Variable<'a>) {
    match (*self).scopes.iter_mut().last() {
      Some(scope) => (*scope).elements.insert(
        var.name,
        Variable {
          name: var.name,
          value: var.value,
        },
      ),
      None => unreachable!("Inserting into context without scopes"),
    };
  }

  // Wrapper for more readable code
  pub fn get_variable(&self, var: &str) -> Option<&Variable> {
    self.get_element(var)
  }

  // Wrapper for more readable code
  pub fn get_variable_mut(&mut self, var: &str) -> Option<&'a mut Variable> {
    self.get_element_mut(var)
  }
}

impl<'a> Context<'a, (Type, bool)> {
  pub fn insert_type(&mut self, id: &'a str, r#type: Type, mutable: bool) {
    match (*self).scopes.iter_mut().last() {
      Some(scope) => (*scope).elements.insert(id, (r#type, mutable)),
      None => unreachable!("Inserting into context without scopes"),
    };
  }

  // Wrapper for more readable code
  pub fn get_var_type(&self, var: &str) -> Option<&(Type, bool)> {
    self.get_element(var)
  }
}

impl<'a, T> Context<'a, T> {
  pub fn push(&mut self, scope: Scope<'a, T>) {
    self.scopes.push(scope);
  }

  pub fn pop(&mut self) {
    self.scopes.pop();
  }

  fn get_element(&self, var: &str) -> Option<&T> {
    for scope in self.scopes.iter().rev() {
      match scope.elements.get(var) {
        Some(ref mut var) => {
          return Some(&var);
        }
        None => (),
      };
    }
    None
  }

  fn get_element_mut(&mut self, var: &str) -> Option<&mut T> {
    for scope in self.scopes.iter_mut().rev() {
      match scope.elements.get_mut(var) {
        Some(var) => {
          return Some(var);
        }
        None => (),
      };
    }
    None
  }
}
