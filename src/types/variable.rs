use crate::types::value::Value;

#[derive(Debug)]
pub struct Variable {
  pub name: String,
  pub value: Value,
}
