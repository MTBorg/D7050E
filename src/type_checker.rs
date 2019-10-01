use crate::{func::Func, node::Node, types::Type};
use std::collections::HashMap;

#[allow(dead_code)]
pub fn type_check(
  node: &Node,
  funcs: &HashMap<String, Func>,
) -> Result<Option<Type>, &'static str> {
  match node {
    Node::Number(_) => Ok(Some(Type::Int)),
    Node::Bool(_) => Ok(Some(Type::Bool)),
    Node::Op(e1, _, e2) => {
      let type1 = match type_check(e1, funcs) {
        Ok(r#type) => r#type,
        Err(e) => return Err(e),
      };
      let type2 = match type_check(e2, funcs) {
        Ok(r#type) => r#type,
        Err(e) => return Err(e),
      };
      if type1 == type2 {
        return Ok(type1);
      } else {
        return Err("Invalid type for operands");
      }
    }
    Node::FuncCall(func, args, _) => {
      let func = match funcs.get(func) {
        Some(func) => func,
        None => panic!("Could not find function {} while checking types", func),
      };

      // Check argument types
      for (arg, param) in args.iter().zip(&func.params) {
        let arg_type = match type_check(arg, funcs) {
          Ok(res) => match res {
            Some(r#type) => r#type,
            None => return Err("Cannot pass void type as argument"),
          },
          Err(e) => return Err(e),
        };

        if arg_type != param._type {
          return Err("Invalid type for function parameter");
        }
      }

      return Ok(func.ret_type.clone());
    }
    _ => Err("This type of node does not evaluate to a type"),
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::{func_param::FuncParam, parsing::expr_parser::Opcode};

  #[test]
  pub fn test_number() {
    assert_eq!(
      type_check(&Node::Number(2), &HashMap::new()).unwrap(),
      Some(Type::Int)
    );
  }

  #[test]
  pub fn test_bool() {
    assert_eq!(
      type_check(&Node::Bool(true), &HashMap::new()).unwrap(),
      Some(Type::Bool)
    );
  }

  #[test]
  pub fn test_operation_int_int() {
    assert_eq!(
      type_check(
        &Node::Op(
          Box::new(Node::Number(2)),
          Opcode::Add,
          Box::new(Node::Number(2))
        ),
        &HashMap::new()
      )
      .unwrap(),
      Some(Type::Int)
    );
  }

  #[test]
  pub fn test_operation_int_bool() {
    assert!(!type_check(
      &Node::Op(
        Box::new(Node::Number(2)),
        Opcode::Add,
        Box::new(Node::Bool(true))
      ),
      &HashMap::new()
    )
    .is_ok());
  }

  pub fn test_operation_bool_int() {
    assert!(!type_check(
      &Node::Op(
        Box::new(Node::Bool(true)),
        Opcode::And,
        Box::new(Node::Number(2))
      ),
      &HashMap::new()
    )
    .is_ok());
  }

  #[test]
  pub fn test_operation_bool_bool() {
    assert_eq!(
      type_check(
        &Node::Op(
          Box::new(Node::Bool(true)),
          Opcode::And,
          Box::new(Node::Bool(true))
        ),
        &HashMap::new()
      )
      .unwrap(),
      Some(Type::Bool)
    );
  }

  #[test]
  pub fn test_func_call_int() {
    let func_dec = Func {
      name: "foo".to_string(),
      params: vec![],
      ret_type: Some(Type::Int),
      body_start: Node::Empty,
    };
    let mut funcs = HashMap::new();
    funcs.insert("foo".to_string(), func_dec);
    assert_eq!(
      type_check(&Node::FuncCall("foo".to_string(), vec!(), None), &funcs).unwrap(),
      Some(Type::Int)
    );
  }

  #[test]
  pub fn test_func_call_bool() {
    let func_dec = Func {
      name: "foo".to_string(),
      params: vec![],
      ret_type: Some(Type::Bool),
      body_start: Node::Empty,
    };
    let mut funcs = HashMap::new();
    funcs.insert("foo".to_string(), func_dec);
    assert_eq!(
      type_check(&Node::FuncCall("foo".to_string(), vec!(), None), &funcs).unwrap(),
      Some(Type::Bool)
    );
  }

  #[test]
  pub fn test_func_call_args_int() {
    let func_dec = Func {
      name: "foo".to_string(),
      params: vec![FuncParam {
        name: "a".to_string(),
        _type: Type::Int,
        mutable: false,
      }],
      ret_type: Some(Type::Int),
      body_start: Node::Empty,
    };
    let mut funcs = HashMap::new();
    funcs.insert("foo".to_string(), func_dec);
    assert_eq!(
      type_check(
        &Node::FuncCall("foo".to_string(), vec!(Node::Number(2)), None),
        &funcs
      )
      .unwrap(),
      Some(Type::Int)
    );
  }

  #[test]
  pub fn test_func_call_args_bool() {
    let func_dec = Func {
      name: "foo".to_string(),
      params: vec![FuncParam {
        name: "a".to_string(),
        _type: Type::Bool,
        mutable: false,
      }],
      ret_type: Some(Type::Bool),
      body_start: Node::Empty,
    };
    let mut funcs = HashMap::new();
    funcs.insert("foo".to_string(), func_dec);
    assert_eq!(
      type_check(
        &Node::FuncCall("foo".to_string(), vec!(Node::Bool(true)), None),
        &funcs
      )
      .unwrap(),
      Some(Type::Bool)
    );
  }

  #[test]
  pub fn test_func_call_args_invalid_type() {
    let func_dec = Func {
      name: "foo".to_string(),
      params: vec![FuncParam {
        name: "a".to_string(),
        _type: Type::Int,
        mutable: false,
      }],
      ret_type: Some(Type::Int),
      body_start: Node::Empty,
    };
    let mut funcs = HashMap::new();
    funcs.insert("foo".to_string(), func_dec);
    assert!(!type_check(
      &Node::FuncCall("foo".to_string(), vec!(Node::Bool(true)), None),
      &funcs
    )
    .is_ok());
  }
}
