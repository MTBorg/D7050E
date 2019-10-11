use crate::{
  context::Context,
  errors::{
    type_errors::*, unknown_func_error::UnknownFuncError,
    unknown_var_error::UnknownVarError,
  },
  func::Func,
  node::Node,
  parsing::expr_parser::Opcode,
  program::Program,
  scope::Scope,
  types::Type,
};
use std::collections::HashMap;

#[allow(dead_code)]
fn type_check(
  node: &Node,
  context: &mut Context<Type>,
  funcs: &HashMap<String, Func>,
) -> Result<Option<Type>, Box<dyn std::error::Error>> {
  match node {
    Node::Number(_) => Ok(Some(Type::Int)),
    Node::Bool(_) => Ok(Some(Type::Bool)),
    Node::Var(var) => match context.get_var_type((*var).clone()) {
      Some(r#type) => Ok(Some((*r#type).clone())),
      None => Err(Box::new(UnknownVarError { name: var.clone() })),
    },
    Node::Assign(var, expr, _) => {
      // Check the type of the right hand side of assignment
      let expr_type = match type_check(expr, context, funcs) {
        Ok(res) => match res {
          Some(r#type) => r#type,
          None => return Err(Box::new(NonTypeExpressionTypeError)),
        },
        Err(e) => return Err(e),
      };

      let res = context.get_var_type(var.clone());
      match res {
        Some(r#type) => {
          if *r#type != expr_type {
            return Err(Box::new(AssignMissmatchTypeError {
              var: var.clone(),
              r#type: r#type.clone(),
              expr_type: expr_type.clone(),
            }));
          } else {
            return Ok(Some(r#type.clone()));
          }
        }
        None => {
          return Err(Box::new(UnknownVarError { name: var.clone() }));
        }
      }
    }
    Node::Op(e1, op, e2) => {
      let type1 = match type_check(e1, context, funcs) {
        Ok(r#type) => r#type,
        Err(e) => return Err(e),
      };
      let type2 = match type_check(e2, context, funcs) {
        Ok(r#type) => r#type,
        Err(e) => return Err(e),
      };
      if type1 == type2 {
        return match op {
          Opcode::Add | Opcode::Sub | Opcode::Mul | Opcode::Div => Ok(type1),
          _ => Ok(Some(Type::Bool)),
        };
      } else {
        return Err(Box::new(OpTypeError {
          op: (*op).clone(),
          type_left: type1,
          type_right: type2,
        }));
      }
    }
    Node::Let(name, r#type, expr, _) => {
      let expr_type = match type_check(expr, context, funcs) {
        Ok(res) => match res {
          Some(r#type) => r#type,
          None => return Err(Box::new(NonTypeExpressionTypeError {})),
        },
        Err(e) => {
          return Err(e);
        }
      };

      if let Some(r#type) = r#type {
        // If variable type was specified
        if expr_type == *r#type {
          context.insert_type((*name).clone(), expr_type.clone());
          return Ok(Some(expr_type));
        } else {
          return Err(Box::new(LetMissmatchTypeError {
            r#type: (*r#type).clone(),
            expr_type: expr_type,
          }));
        }
      } else {
        context.insert_type((*name).clone(), expr_type.clone());
        return Ok(Some(expr_type));
      }
    }
    Node::FuncCall(func, args, _) => {
      let func = match funcs.get(func) {
        Some(func) => func,
        None => {
          return Err(Box::new(UnknownFuncError {
            func_name: func.clone(),
          }))
        }
      };

      // Check argument types
      for (arg, param) in args.iter().zip(&func.params) {
        let arg_type = match type_check(arg, context, funcs) {
          Ok(res) => match res {
            Some(r#type) => r#type,
            None => {
              return Err(Box::new(ArgMissmatchTypeError {
                arg_type: None,
                param: (*param).clone(),
              }))
            }
          },
          Err(e) => return Err(e),
        };

        if arg_type != param._type {
          return Err(Box::new(ArgMissmatchTypeError {
            arg_type: Some(arg_type),
            param: (*param).clone(),
          }));
        }
      }

      return Ok(func.ret_type.clone());
    }
    Node::Return(expr, _) => {
      let expr_type = match type_check(expr, context, funcs) {
        Ok(res) => match res {
          Some(r#type) => r#type,
          None => return Err(Box::new(NonTypeExpressionTypeError {})),
        },
        Err(e) => return Err(e),
      };
      match &context.current_func.ret_type {
        Some(r#type) => {
          if *r#type == expr_type {
            return Ok(Some(r#type.clone()));
          } else {
            return Err(Box::new(InvalidReturnTypeError {
              func: context.current_func.clone(),
              expr_type: expr_type,
            }));
          }
        }
        None => {
          return Err(Box::new(InvalidReturnTypeError {
            func: context.current_func.clone(),
            expr_type: expr_type,
          }));
        }
      }
    }
    _ => Err(Box::new(InvalidNodeTypeError {})),
  }
}

fn type_check_tree(
  func: &Func,
  funcs: &HashMap<String, Func>,
) -> Result<(), Vec<Box<dyn std::error::Error>>> {
  let mut next_node = Some(&func.body_start);
  let mut errors: Vec<Box<dyn std::error::Error>> = vec![];
  let mut context = Context::from(func);

  context.push(Scope::from(func.params.clone()));
  while match next_node {
    Some(_) => true,
    _ => false,
  } {
    if let Err(e) = type_check(&next_node.unwrap(), &mut context, funcs) {
      errors.push(e);
    }

    if let Some(node) = next_node {
      next_node = node.get_next_instruction();
    }
  }

  return if errors.len() == 0 {
    Ok(())
  } else {
    Err(errors)
  };
}

pub fn type_check_program(
  program: &Program,
) -> Result<(), Vec<Box<dyn std::error::Error>>> {
  let mut errors: Vec<Box<dyn std::error::Error>> = vec![];

  // Iterate over the values of the hashmap (i.e. the second element)
  for func in program.funcs.iter().map(|pair| pair.1) {
    if let Err(ref mut e) = type_check_tree(func, &program.funcs) {
      errors.append(e);
    }
  }

  return if errors.len() == 0 {
    Ok(())
  } else {
    Err(errors)
  };
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::{func_param::FuncParam, parsing::expr_parser::Opcode};

  #[test]
  pub fn test_number() {
    let func_dec = Func {
      name: "foo".to_string(),
      params: vec![],
      ret_type: Some(Type::Int),
      body_start: Node::Empty,
    };
    let mut context = Context::from(&func_dec);
    assert_eq!(
      type_check(&Node::Number(2), &mut context, &HashMap::new()).unwrap(),
      Some(Type::Int)
    );
  }

  #[test]
  pub fn test_bool() {
    let func_dec = Func {
      name: "foo".to_string(),
      params: vec![],
      ret_type: Some(Type::Int),
      body_start: Node::Empty,
    };
    let mut context = Context::from(&func_dec);
    assert_eq!(
      type_check(&Node::Bool(true), &mut context, &HashMap::new()).unwrap(),
      Some(Type::Bool)
    );
  }

  #[test]
  pub fn test_operation_int_int() {
    let func_dec = Func {
      name: "foo".to_string(),
      params: vec![],
      ret_type: Some(Type::Int),
      body_start: Node::Empty,
    };
    let mut context = Context::from(&func_dec);
    assert_eq!(
      type_check(
        &Node::Op(
          Box::new(Node::Number(2)),
          Opcode::Add,
          Box::new(Node::Number(2))
        ),
        &mut context,
        &HashMap::new()
      )
      .unwrap(),
      Some(Type::Int)
    );
  }

  #[test]
  pub fn test_operation_int_bool() {
    let func_dec = Func {
      name: "foo".to_string(),
      params: vec![],
      ret_type: Some(Type::Int),
      body_start: Node::Empty,
    };
    let mut context = Context::from(&func_dec);
    assert!(!type_check(
      &Node::Op(
        Box::new(Node::Number(2)),
        Opcode::Add,
        Box::new(Node::Bool(true))
      ),
      &mut context,
      &HashMap::new()
    )
    .is_ok());
  }

  pub fn test_operation_bool_int() {
    let func_dec = Func {
      name: "foo".to_string(),
      params: vec![],
      ret_type: Some(Type::Int),
      body_start: Node::Empty,
    };
    let mut context = Context::from(&func_dec);
    assert!(!type_check(
      &Node::Op(
        Box::new(Node::Bool(true)),
        Opcode::And,
        Box::new(Node::Number(2))
      ),
      &mut context,
      &HashMap::new()
    )
    .is_ok());
  }

  #[test]
  pub fn test_operation_bool_bool() {
    let func_dec = Func {
      name: "foo".to_string(),
      params: vec![],
      ret_type: Some(Type::Int),
      body_start: Node::Empty,
    };
    let mut context = Context::from(&func_dec);
    assert_eq!(
      type_check(
        &Node::Op(
          Box::new(Node::Bool(true)),
          Opcode::And,
          Box::new(Node::Bool(true))
        ),
        &mut context,
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
    let mut context = Context::from(&func_dec);
    let mut funcs = HashMap::new();
    funcs.insert("foo".to_string(), func_dec);
    assert_eq!(
      type_check(
        &Node::FuncCall("foo".to_string(), vec!(), None),
        &mut context,
        &funcs
      )
      .unwrap(),
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
    let mut context = Context::from(&func_dec);
    let mut funcs = HashMap::new();
    funcs.insert("foo".to_string(), func_dec);
    assert_eq!(
      type_check(
        &Node::FuncCall("foo".to_string(), vec!(), None),
        &mut context,
        &funcs
      )
      .unwrap(),
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
    let mut context = Context::from(&func_dec);
    let mut funcs = HashMap::new();
    funcs.insert("foo".to_string(), func_dec);
    assert_eq!(
      type_check(
        &Node::FuncCall("foo".to_string(), vec!(Node::Number(2)), None),
        &mut context,
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
    let mut context = Context::from(&func_dec);
    let mut funcs = HashMap::new();
    funcs.insert("foo".to_string(), func_dec);
    assert_eq!(
      type_check(
        &Node::FuncCall("foo".to_string(), vec!(Node::Bool(true)), None),
        &mut context,
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
    let mut context = Context::from(&func_dec);
    let mut funcs = HashMap::new();
    funcs.insert("foo".to_string(), func_dec);
    assert!(!type_check(
      &Node::FuncCall("foo".to_string(), vec!(Node::Bool(true)), None),
      &mut context,
      &funcs
    )
    .is_ok());
  }

  #[test]
  pub fn test_func_return_int() {
    let func_dec = Func {
      name: "foo".to_string(),
      params: vec![],
      ret_type: Some(Type::Int),
      body_start: Node::Empty,
    };
    let mut context = Context::from(&func_dec);
    let mut funcs = HashMap::new();
    funcs.insert("foo".to_string(), func_dec);
    assert!(type_check(
      &Node::Return(Box::new(Node::Number(5)), None),
      &mut context,
      &funcs
    )
    .is_ok());
  }

  #[test]
  pub fn test_func_return_int_expecting_bool() {
    let func_dec = Func {
      name: "foo".to_string(),
      params: vec![],
      ret_type: Some(Type::Bool),
      body_start: Node::Empty,
    };
    let mut context = Context::from(&func_dec);
    let mut funcs = HashMap::new();
    funcs.insert("foo".to_string(), func_dec);
    assert!(!type_check(
      &Node::Return(Box::new(Node::Number(5)), None),
      &mut context,
      &funcs
    )
    .is_ok());
  }

  #[test]
  pub fn test_func_return_bool() {
    let func_dec = Func {
      name: "foo".to_string(),
      params: vec![],
      ret_type: Some(Type::Bool),
      body_start: Node::Empty,
    };
    let mut context = Context::from(&func_dec);
    let mut funcs = HashMap::new();
    funcs.insert("foo".to_string(), func_dec);
    assert!(type_check(
      &Node::Return(Box::new(Node::Bool(true)), None),
      &mut context,
      &funcs
    )
    .is_ok());
  }

  #[test]
  pub fn test_func_return_bool_expecting_int() {
    let func_dec = Func {
      name: "foo".to_string(),
      params: vec![],
      ret_type: Some(Type::Int),
      body_start: Node::Empty,
    };
    let mut context = Context::from(&func_dec);
    let mut funcs = HashMap::new();
    funcs.insert("foo".to_string(), func_dec);
    assert!(!type_check(
      &Node::Return(Box::new(Node::Bool(false)), None),
      &mut context,
      &funcs
    )
    .is_ok());
  }

  #[test]
  pub fn test_func_return_func_call() {
    let func_dec = Func {
      name: "foo".to_string(),
      params: vec![],
      ret_type: Some(Type::Int),
      body_start: Node::Empty,
    };
    let func_dec_2 = Func {
      name: "bar".to_string(),
      params: vec![],
      ret_type: Some(Type::Int),
      body_start: Node::Empty,
    };
    let mut context = Context::from(&func_dec);
    let mut funcs = HashMap::new();
    funcs.insert("foo".to_string(), func_dec);
    funcs.insert("bar".to_string(), func_dec_2);

    assert!(type_check(
      &Node::Return(
        Box::new(Node::FuncCall("bar".to_string(), vec!(), None)),
        None
      ),
      &mut context,
      &funcs
    )
    .is_ok());
  }

  #[test]
  pub fn test_return_with_missing_return_type() {
    let func_dec = Func {
      name: "foo".to_string(),
      params: vec![],
      ret_type: None,
      body_start: Node::Empty,
    };
    let mut context = Context::from(&func_dec);
    let mut funcs = HashMap::new();
    funcs.insert("foo".to_string(), func_dec);
    assert!(!type_check(
      &Node::Return(Box::new(Node::Number(2)), None),
      &mut context,
      &funcs
    )
    .is_ok());
  }
}
