use crate::types::{func::Func, scope::Scope, _type::Type, variable::Variable};

#[derive(Debug)]
pub struct Context<T> {
  scopes: Vec<Scope<T>>,
  pub current_func: Func,
}

impl<T> From<&Func> for Context<T> {
  fn from(func: &Func) -> Self {
    Context {
      scopes: vec![],
      current_func: (*func).clone(),
    }
  }
}

impl Context<Variable> {
  pub fn insert_variable(&mut self, var: Variable) {
    match (*self).scopes.iter_mut().last() {
      Some(scope) => (*scope).elements.insert(
        var.name.clone(),
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
  pub fn get_variable_mut(&mut self, var: &str) -> Option<&mut Variable> {
    self.get_element_mut(var)
  }
}

impl Context<(Type, bool)> {
  pub fn insert_type(&mut self, id: &str, r#type: Type, mutable: bool) {
    match (*self).scopes.iter_mut().last() {
      Some(scope) => (*scope).elements.insert(id.to_string(), (r#type, mutable)),
      None => unreachable!("Inserting into context without scopes"),
    };
  }

  // Wrapper for more readable code
  pub fn get_var_type(&self, var: &str) -> Option<&(Type, bool)> {
    self.get_element(var)
  }
}

impl<T> Context<T> {
  pub fn push(&mut self, scope: Scope<T>) {
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
