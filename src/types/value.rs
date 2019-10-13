#[derive(Debug, PartialEq)]
pub enum Value {
  Bool(bool),
  Int(i32),
}

impl Value {
  pub fn to_string(&self) -> String {
    match self {
      Value::Bool(b) => b.to_string(),
      Value::Int(i) => i.to_string(),
    }
  }
}
