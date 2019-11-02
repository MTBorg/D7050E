use crate::types::value::Value;

#[derive(Debug)]
pub struct Variable<'a> {
  pub name: &'a str,
  pub value: Value,
}
