use crate::types::_type::Type;

#[derive(Debug, Clone)]
pub struct FuncParam<'a> {
  pub name: &'a str,
  pub _type: Type,
  pub mutable: bool,
}
