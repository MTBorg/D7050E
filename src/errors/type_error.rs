use crate::types::{
  _type::Type, func::Func, func_param::FuncParam, node::Node, opcode::Opcode,
};
use std::error;

#[derive(Debug)]
pub enum TypeError {
  OperatorMissmatch {
    expr: Node,
    op: Opcode,
    type_left: Option<Type>,
    type_right: Option<Type>,
  },
  ArgMissmatch {
    arg_type: Option<Type>,
    param: FuncParam,
  },
  TooManyArgs {
    func: String,
    expected: usize,
    received: usize,
  },
  MissingArgs {
    func: String,
    missing: Vec<FuncParam>,
  },
  NonTypeExpression,
  InvalidReturnType {
    func: Func,
    expr_type: Type,
  },
  InvalidNode,
  LetMissmatch {
    r#type: Type,
    expr_type: Type,
  },
  AssignMissmatch {
    var: String,
    r#type: Type,
    expr_type: Type,
  },
  ImmutableAssignment {
    var: String,
  },
  MissingReturn {
    func_name: String,
    ret_type: Type,
  },
  NonBooleanExpr {
    expr: Node,
    r#type: Option<Type>,
  },
}

impl std::fmt::Display for TypeError {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    let message = match self {
      TypeError::OperatorMissmatch {
        expr,
        op,
        type_left,
        type_right,
      } => format!(
        "Invalid types for operand {} (left: {}, right: {}) in expression: \n\t{}",
        op.to_str(),
        match &type_left {
          Some(r#type) => r#type.to_str(),
          None => "void",
        },
        match &type_right {
          Some(r#type) => r#type.to_str(),
          None => "void",
        },
        expr.expr_into_string()
      ),
      TypeError::ArgMissmatch { arg_type, param } => format!(
        "Argument type ({}) does not match parameter {}'s type ({})",
        match arg_type {
          Some(r#type) => r#type.to_str(),
          None => "void",
        },
        param.name,
        param._type.to_str()
      ),
      TypeError::NonTypeExpression => "NonTypeExpression".to_string(),
      TypeError::InvalidReturnType { func, expr_type } => format!(
        "Type of returned expression ({}) does not match function {}'s signature ({})",
        expr_type.to_str(),
        func.name,
        match &func.ret_type {
          Some(r#type) => r#type.to_str(),
          None => "void",
        }
      ),
      TypeError::InvalidNode => "This node does not evaluate to a type".to_string(),
      TypeError::LetMissmatch { r#type, expr_type } => format!(
        "Let statement expected type {} because of declaration but received {}",
        r#type.to_str(),
        expr_type.to_str()
      ),
      TypeError::AssignMissmatch {
        var,
        r#type,
        expr_type,
      } => format!(
        "Cannot assign type {} to variable {} of type {}",
        expr_type.to_str(),
        var,
        r#type.to_str()
      ),
      TypeError::ImmutableAssignment { var } => format!("Variable {} is immutable", var),
      TypeError::MissingReturn {
        func_name,
        ret_type,
      } => format!(
        "Missing return statement in function {}, expected to return type {}",
        func_name,
        ret_type.to_str()
      ),
      TypeError::TooManyArgs {
        func,
        expected,
        received,
      } => format!(
        "Function {} received too many arguments: expected {} arguments but received {}",
        func, expected, received
      ),
      TypeError::MissingArgs { func, missing } => {
        let mut missing_string: String = "".to_string();
        let missing_length = missing.len();
        for (i, param) in missing.iter().enumerate() {
          missing_string += &(param.name.clone()
            + ": "
            + param._type.to_str()
            + if i != missing_length - 1 { "," } else { "" });
        }
        format!(
          "Function {} missing arguments to parameters {}",
          func, missing_string
        )
      }
      TypeError::NonBooleanExpr{ expr, r#type} => {
          format!("Expression \n\t{}\nin conditional does not evaluate to a boolean (evaluated to {}", expr.expr_into_string(), if let Some(r#type) = r#type { r#type.to_str() } else{"void"})}
    };
    write!(f, "{}", message)
  }
}

impl error::Error for TypeError {
  fn source(&self) -> Option<&(dyn error::Error + 'static)> {
    None
  }
}
