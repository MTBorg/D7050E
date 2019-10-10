use std::error;

#[derive(Debug)]
pub struct UnknownFuncError {
    pub func_name: String
}

impl std::fmt::Display for UnknownFuncError {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "Function {} not found in current scope", self.func_name)
  }
}

impl error::Error for UnknownFuncError {
  fn source(&self) -> Option<&(dyn error::Error + 'static)> {
    None
  }
}
