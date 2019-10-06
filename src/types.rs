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
