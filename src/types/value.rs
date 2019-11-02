use crate::types::node::Node;

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

impl std::convert::TryFrom<Node> for Value {
  type Error = &'static str;

  fn try_from(node: Node) -> Result<Self, Self::Error> {
    match node {
      Node::Bool(b) => Ok(Value::Bool(b)),
      Node::Number(i) => Ok(Value::Int(i)),
      _ => Err("Cannot convert node to value"),
    }
  }
}
