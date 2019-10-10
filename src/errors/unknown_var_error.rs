use std::error;

#[derive(Debug)]
pub struct UnknownVarError {
    pub name: String
}

impl std::fmt::Display for UnknownVarError {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "Variable {} not found in current scope", self.name)
  }
}

impl error::Error for UnknownVarError {
  fn source(&self) -> Option<&(dyn error::Error + 'static)> {
    None
  }
}
