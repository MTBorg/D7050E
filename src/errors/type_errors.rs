use crate::{
  func::Func, func_param::FuncParam, parsing::expr_parser::Opcode, types::Type,
};
use std::error;

#[derive(Debug)]
pub struct OpTypeError {
  pub op: Opcode,
  pub type_left: Option<Type>,
  pub type_right: Option<Type>,
}

impl std::fmt::Display for OpTypeError {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(
      f,
      "Invalid types for operand {} (left: {}, right: {})",
      self.op.to_str(),
      match &self.type_left {
        Some(r#type) => r#type.to_str(),
        None => "void",
      },
      match &self.type_right {
        Some(r#type) => r#type.to_str(),
        None => "void",
      }
    )
  }
}

impl error::Error for OpTypeError {
  fn source(&self) -> Option<&(dyn error::Error + 'static)> {
    None
  }
}

#[derive(Debug)]
pub struct ArgMissmatchTypeError {
  pub arg_type: Option<Type>,
  pub param: FuncParam,
}

impl std::fmt::Display for ArgMissmatchTypeError {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(
      f,
      "Argument type ({}) does not match parameter {}'s type ({})",
      match &self.arg_type {
        Some(r#type) => r#type.to_str(),
        None => "void",
      },
      self.param.name,
      self.param._type.to_str()
    )
  }
}

impl error::Error for ArgMissmatchTypeError {
  fn source(&self) -> Option<&(dyn error::Error + 'static)> {
    None
  }
}

#[derive(Debug)]
pub struct NonTypeExpressionTypeError;

impl std::fmt::Display for NonTypeExpressionTypeError {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "NonTypeExpressionTypeError")
  }
}

impl error::Error for NonTypeExpressionTypeError {
  fn source(&self) -> Option<&(dyn error::Error + 'static)> {
    None
  }
}

#[derive(Debug)]
pub struct InvalidReturnTypeError {
  pub func: Func,
  pub expr_type: Type,
}

impl std::fmt::Display for InvalidReturnTypeError {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(
      f,
      "Type of returned expression ({}) does not match function {}'s signature ({})",
      self.expr_type.to_str(),
      self.func.name,
      match &self.func.ret_type {
        Some(r#type) => r#type.to_str(),
        None => "void",
      }
    )
  }
}

impl error::Error for InvalidReturnTypeError {
  fn source(&self) -> Option<&(dyn error::Error + 'static)> {
    None
  }
}

#[derive(Debug)]
pub struct InvalidNodeTypeError;

impl std::fmt::Display for InvalidNodeTypeError {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "This node does not evaluate to a type")
  }
}

impl error::Error for InvalidNodeTypeError {
  fn source(&self) -> Option<&(dyn error::Error + 'static)> {
    None
  }
}

#[derive(Debug)]
pub struct LetMissmatchTypeError {
  pub r#type: Type,
  pub expr_type: Type,
}

impl std::fmt::Display for LetMissmatchTypeError {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(
      f,
      "Let statement expected type {} because of declaration but received {}",
      self.r#type.to_str(),
      self.expr_type.to_str()
    )
  }
}

impl error::Error for LetMissmatchTypeError {
  fn source(&self) -> Option<&(dyn error::Error + 'static)> {
    None
  }
}

#[derive(Debug)]
pub struct AssignMissmatchTypeError {
  pub var: String,
  pub r#type: Type,
  pub expr_type: Type,
}

impl std::fmt::Display for AssignMissmatchTypeError {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(
      f,
      "Cannot assign type {} to variable {} of type {}",
      self.expr_type.to_str(),
      self.var,
      self.r#type.to_str()
    )
  }
}

impl error::Error for AssignMissmatchTypeError {
  fn source(&self) -> Option<&(dyn error::Error + 'static)> {
    None
  }
}
