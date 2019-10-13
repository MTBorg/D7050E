use crate::types::_type::Type;

#[derive(Debug, Clone)]
pub struct FuncParam {
  pub name: String,
  pub _type: Type,
  pub mutable: bool,
}
