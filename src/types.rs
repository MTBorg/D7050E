use crate::value::Value;

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
  Bool,
  Int,
}

impl Type {
  pub fn to_str(&self) -> &'static str {
    match self {
      Type::Bool => "bool",
      Type::Int => "i32",
    }
  }
}

impl std::convert::From<&Value> for Type {
  fn from(val: &Value) -> Self {
    match *val {
      Value::Int(_) => Type::Int,
      Value::Bool(_) => Type::Bool,
    }
  }
}
