#[derive(Debug, PartialEq)]
pub enum Value {
  Bool(bool),
  Int(i32),
}

impl std::convert::From<&Value> for String {
  fn from(val: &Value) -> Self {
    match val {
      Value::Bool(b) => b.to_string(),
      Value::Int(i) => i.to_string(),
    }
  }
}
