use crate::{
  errors::{
    type_error::TypeError, unknown_func_error::UnknownFuncError,
    unknown_var_error::UnknownVarError,
  },
  types::{
    _type::Type, context::Context, func::Func, node::Node, opcode::Opcode,
    program::Program, scope::Scope,
  },
};
use std::collections::HashMap;

pub fn type_check_program(
  program: &Program,
) -> Result<(), Vec<Box<dyn std::error::Error>>> {
  let mut errors: Vec<Box<dyn std::error::Error>> = vec![];

  // Iterate over the values of the hashmap (i.e. the second element)
  for func in program.funcs.iter().map(|pair| pair.1) {
    if let Err(ref mut e) = type_check_function(func, &program.funcs) {
      errors.append(e);
    }
  }

  return if errors.len() == 0 {
    Ok(())
  } else {
    Err(errors)
  };
}
fn type_check_while(
  condition: &Node,
  then_body: &Node,
  context: &mut Context<(Type, bool)>,
  funcs: &HashMap<String, Func>,
) -> Result<Option<Type>, Vec<Box<dyn std::error::Error>>> {
  let mut cond_type = type_check(condition, context, funcs);
  let mut errors: Vec<Box<dyn std::error::Error>> = vec![];

  //Make sure the condition evaluates to a boolean
  match cond_type {
    Ok(res) => match res {
      Some(r#type) => {
        if let Type::Bool = r#type {
        } else {
          errors.push(Box::new(TypeError::NonBooleanExpr {
            expr: (*condition).clone(),
            r#type: Some(r#type),
          }));
        }
      }
      None => {
        errors.push(Box::new(TypeError::NonBooleanExpr {
          expr: (*condition).clone(),
          r#type: None,
        }));
      }
    },
    Err(ref mut e) => {
      errors.append(e);
    }
  };

  // Type check the body
  let mut then_res = type_check(then_body, context, funcs);
  if let Err(ref mut e) = then_res {
    errors.append(e);
  }

  return if errors.len() == 0 {
    Ok(None)
  } else {
    Err(errors)
  };
}

fn type_check_let(
  name: &str,
  r#type: &Option<Type>,
  mutable: bool,
  expr: &Node,
  context: &mut Context<(Type, bool)>,
  funcs: &HashMap<String, Func>,
) -> Result<Option<Type>, Vec<Box<dyn std::error::Error>>> {
  let expr_type = match type_check(expr, context, funcs) {
    Ok(res) => match res {
      Some(r#type) => r#type,
      None => return Err(vec![Box::new(TypeError::NonTypeExpression {})]),
    },
    Err(e) => {
      return Err(e);
    }
  };

  return if let Some(r#type) = r#type {
    // If variable type was specified
    if expr_type == *r#type {
      context.insert_type(name, expr_type.clone(), mutable);
      Ok(None)
    } else {
      Err(vec![Box::new(TypeError::LetMissmatch {
        r#type: (*r#type).clone(),
        expr_type: expr_type,
      })])
    }
  } else {
    context.insert_type(name, expr_type.clone(), mutable);
    Ok(None)
  };
}
fn type_check_func_call(
  func: &str,
  args: &Vec<Node>,
  context: &mut Context<(Type, bool)>,
  funcs: &HashMap<String, Func>,
) -> Result<Option<Type>, Vec<Box<dyn std::error::Error>>> {
  let func = match funcs.get(func) {
    Some(func) => func,
    None => {
      return Err(vec![Box::new(UnknownFuncError {
        func_name: func.to_string(),
      })])
    }
  };

  // Check argument/parameter length
  if args.len() != func.params.len() {
    if args.len() > func.params.len() {
      return Err(vec![Box::new(TypeError::TooManyArgs {
        func: func.name.clone(),
        expected: func.params.len(),
        received: args.len(),
      })]);
    } else {
      let missing = func.params[args.len()..func.params.len()].to_vec();
      return Err(vec![Box::new(TypeError::MissingArgs {
        func: func.name.clone(),
        missing: missing,
      })]);
    }
  }

  // Check argument types
  for (arg, param) in args.iter().zip(&func.params) {
    let arg_type = match type_check(arg, context, funcs) {
      Ok(res) => match res {
        Some(r#type) => r#type,
        None => {
          return Err(vec![Box::new(TypeError::ArgMissmatch {
            arg_type: None,
            param: (*param).clone(),
          })])
        }
      },
      Err(e) => return Err(e),
    };

    if arg_type != param._type {
      return Err(vec![Box::new(TypeError::ArgMissmatch {
        arg_type: Some(arg_type),
        param: (*param).clone(),
      })]);
    }
  }

  return if let Some(ret_type) = func.ret_type.clone() {
    Ok(Some(ret_type.clone()))
  } else {
    Ok(None)
  };
}

fn type_check_assign(
  expr: &Node,
  var: &str,
  context: &mut Context<(Type, bool)>,
  funcs: &HashMap<String, Func>,
) -> Result<Option<Type>, Vec<Box<dyn std::error::Error>>> {
  // Check the type of the right hand side of assignment
  let expr_type = match type_check(expr, context, funcs) {
    Ok(res) => match res {
      Some(r#type) => r#type,
      None => return Err(vec![Box::new(TypeError::NonTypeExpression)]),
    },
    Err(e) => return Err(e),
  };

  match context.get_var_type(&var) {
    Some((r#type, mutable)) => {
      if !mutable {
        return Err(vec![Box::new(TypeError::ImmutableAssignment {
          var: var.to_string(),
        })]);
      }
      return if *r#type != expr_type {
        Err(vec![Box::new(TypeError::AssignMissmatch {
          var: var.to_string(),
          r#type: r#type.clone(),
          expr_type: expr_type.clone(),
        })])
      } else {
        Ok(None)
      };
    }
    None => {
      return Err(vec![Box::new(UnknownVarError {
        name: var.to_string(),
      })]);
    }
  }
}

fn type_check_function(
  func: &Func,
  funcs: &HashMap<String, Func>,
) -> Result<(), Vec<Box<dyn std::error::Error>>> {
  let mut context: Context<(Type, bool)> = Context::from(func);

  context.push(Scope::from(func.params.clone()));

  // Corner case: If the function is empty
  if let Node::Empty = func.body_start {
    return if let Some(_) = func.ret_type {
      Err(vec![Box::new(TypeError::MissingReturn {
        func_name: func.name.clone(),
        ret_type: func.ret_type.clone().unwrap(),
      })])
    } else {
      Ok(())
    };
  }

  match type_check(&func.body_start, &mut context, &funcs) {
    Ok(res) => {
      if let None = res {
        if let Some(_) = func.ret_type {
          return Err(vec![Box::new(TypeError::MissingReturn {
            func_name: func.name.clone(),
            ret_type: func.ret_type.clone().unwrap(),
          })]);
        }
      } else {
        println!("{:#?}", res);
      }
      Ok(())
    }
    Err(errors) => Err(errors),
  }
}

fn type_check_op(
  left: &Node,
  op: &Opcode,
  right: &Node,
  context: &mut Context<(Type, bool)>,
  funcs: &HashMap<String, Func>,
) -> Result<Option<Type>, Vec<Box<dyn std::error::Error>>> {
  // Extract the types out of the operands.
  // We don't care about the second value of the tuple (mutable)
  // as an expression is always immutable.
  let type1 = match type_check(left, context, funcs) {
    Ok(res) => match res {
      Some(r#type) => Some(r#type),
      None => None,
    },
    Err(e) => return Err(e),
  };
  let type2 = match type_check(right, context, funcs) {
    Ok(res) => match res {
      Some(r#type) => Some(r#type),
      None => None,
    },
    Err(e) => return Err(e),
  };

  return if type1 == type2 {
    match op {
      // This match is pretty ugly but is needed since arithmetic operations
      // evaluate to the type of their operands where as logical operations always
      // evaluate to booleans.
      Opcode::Add | Opcode::Sub | Opcode::Mul | Opcode::Div => Ok(match type1 {
        Some(r#type) => Some(r#type),
        None => None,
      }),
      _ => Ok(Some(Type::Bool)),
    }
  } else {
    Err(vec![Box::new(TypeError::OperatorMissmatch {
      expr: Node::Op(
        Box::new((*left).clone()),
        (*op).clone(),
        Box::new((*right).clone()),
      ),
      op: (*op).clone(),
      type_left: if let Some(r#type) = type1 {
        Some(r#type)
      } else {
        None
      },
      type_right: if let Some(r#type) = type2 {
        Some(r#type)
      } else {
        None
      },
    })])
  };
}

fn type_check_return(
  expr: &Node,
  context: &mut Context<(Type, bool)>,
  funcs: &HashMap<String, Func>,
) -> Result<Option<Type>, Vec<Box<dyn std::error::Error>>> {
  let expr_type = match type_check(expr, context, funcs) {
    Ok(res) => match res {
      Some(r#type) => r#type,
      None => return Err(vec![Box::new(TypeError::NonTypeExpression {})]),
    },
    Err(e) => return Err(e),
  };
  match &context.current_func.ret_type {
    Some(r#type) => {
      if *r#type == expr_type {
        return Ok(Some(r#type.clone()));
      } else {
        return Err(vec![Box::new(TypeError::InvalidReturnType {
          func: context.current_func.clone(),
          expr_type: expr_type,
        })]);
      }
    }
    None => {
      return Err(vec![Box::new(TypeError::InvalidReturnType {
        func: context.current_func.clone(),
        expr_type: expr_type,
      })]);
    }
  }
}

fn type_check(
  node: &Node,
  context: &mut Context<(Type, bool)>,
  funcs: &HashMap<String, Func>,
) -> Result<Option<Type>, Vec<Box<dyn std::error::Error>>> {
  let mut next_instr: &Option<Box<Node>> = &None;
  let res: Result<Option<Type>, Vec<Box<dyn std::error::Error>>> = match node {
    Node::Number(_) => Ok(Some(Type::Int)),
    Node::Bool(_) => Ok(Some(Type::Bool)),
    Node::Var(var) => match context.get_var_type(&var) {
      Some((r#type, _)) => Ok(Some((*r#type).clone())),
      None => Err(vec![Box::new(UnknownVarError { name: var.clone() })]),
    },
    Node::Op(left, op, right) => type_check_op(left, op, right, context, funcs),
    Node::Assign(var, expr, next_node) => {
      next_instr = next_node;
      type_check_assign(expr, var, context, funcs)
    }
    Node::Let(name, r#type, mutable, expr, next_node) => {
      next_instr = next_node;
      type_check_let(name, r#type, *mutable, expr, context, funcs)
    }
    Node::FuncCall(func_name, args, next_node) => {
      next_instr = next_node;
      type_check_func_call(func_name, args, context, funcs)
    }
    Node::If(condition, then_body, else_body, next_node) => {
      next_instr = next_node;
      let mut errors = vec![];

      //Continue: Implement if statement branch checks

      // Type check condition
      let res = type_check(condition, context, funcs);
      if let Err(mut e) = res {
        errors.append(&mut e);
      }

      // Type check then body
      if let Err(mut e) = type_check(then_body, context, funcs) {
        errors.append(&mut e);
      }

      // Type check else body
      if let Some(else_body) = else_body {
        if let Err(mut e) = type_check(else_body, context, funcs) {
          errors.append(&mut e);
        }
      }

      if errors.len() != 0 {
        Err(errors)
      } else {
        Ok(None) //TODO: This should not return none
      }
    }
    Node::While(condition, then_body, next_node) => {
      next_instr = next_node;
      type_check_while(condition, then_body, context, funcs)
    }
    Node::Return(expr, _) => type_check_return(expr, context, funcs),
    Node::Empty => Ok(None),
    _ => unimplemented!(),
  };

  //If there is a next node
  if let Some(next_instr) = next_instr {
    let mut res_next = type_check(next_instr, context, funcs);
    //If both this node and and the right subtree resulted in type errors
    //combine the errors.
    if let Err(ref mut type_errors) = res_next {
      if let Err(mut errors) = res {
        errors.append(type_errors);
        return Err(errors);
      }
      return Err(type_errors.drain(..).collect());
    }

    return res_next;
  } else {
    return res;
  }
  return res;
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::types::{func_param::FuncParam, opcode::Opcode};

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

  #[test]
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
    let mut funcs: HashMap<String, Func> = HashMap::new();
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
  pub fn test_func_missing_arg() {
    let func_dec = Func {
      name: "foo".to_string(),
      params: vec![FuncParam {
        name: "a".to_string(),
        _type: Type::Int,
        mutable: true,
      }],
      ret_type: Some(Type::Int),
      body_start: Node::Empty,
    };
    let mut context = Context::from(&func_dec);
    let mut funcs = HashMap::new();
    funcs.insert("foo".to_string(), func_dec);
    assert!(!type_check(
      &Box::new(Node::FuncCall("foo".to_string(), vec!(), None)),
      &mut context,
      &funcs
    )
    .is_ok());
  }

  #[test]
  pub fn test_func_too_many_args() {
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
      &Box::new(Node::FuncCall(
        "foo".to_string(),
        vec!(Node::Bool(false)),
        None
      )),
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

  #[test]
  pub fn can_assign_to_mutable() {
    let func_dec = Func {
      name: "foo".to_string(),
      params: vec![],
      ret_type: None,
      body_start: Node::Empty,
    };
    let mut context = Context::from(&func_dec);
    let mut funcs = HashMap::new();
    funcs.insert("foo".to_string(), func_dec.clone());
    context.push(Scope::from(func_dec.params));
    context.insert_type("a", Type::Int, true);
    assert!(type_check(
      &Node::Assign("a".to_string(), Box::new(Node::Number(3)), None),
      &mut context,
      &funcs
    )
    .is_ok());
  }

  #[test]
  pub fn can_not_assign_to_mutable() {
    let func_dec = Func {
      name: "foo".to_string(),
      params: vec![],
      ret_type: None,
      body_start: Node::Empty,
    };
    let mut context = Context::from(&func_dec);
    let mut funcs = HashMap::new();
    funcs.insert("foo".to_string(), func_dec.clone());
    context.push(Scope::from(func_dec.params));
    context.insert_type("a", Type::Int, false);
    assert!(!type_check(
      &Node::Assign("a".to_string(), Box::new(Node::Number(3)), None),
      &mut context,
      &funcs
    )
    .is_ok());
  }

  #[test]
  pub fn no_return_in_returning_function() {
    let func_dec = Func {
      name: "foo".to_string(),
      params: vec![],
      ret_type: Some(Type::Int),
      body_start: Node::Empty,
    };
    let mut context = Context::from(&func_dec);
    let mut funcs: HashMap<String, Func> = HashMap::new();
    funcs.insert("foo".to_string(), func_dec.clone());
    context.push(Scope::from(func_dec.params.clone()));
    context.insert_type("a", Type::Int, false);
    assert!(!type_check_function(&func_dec, &funcs).is_ok());
  }

  #[test]
  pub fn empty_function_no_ret_type() {
    let func_dec = Func {
      name: "foo".to_string(),
      params: vec![],
      ret_type: None,
      body_start: Node::Empty,
    };
    let mut context = Context::from(&func_dec);
    let mut funcs: HashMap<String, Func> = HashMap::new();
    funcs.insert("foo".to_string(), func_dec.clone());
    context.push(Scope::from(func_dec.params.clone()));
    context.insert_type("a", Type::Int, false);
    assert!(type_check_function(&func_dec, &funcs).is_ok());
  }

  #[test]
  pub fn while_non_boolean_condition() {
    let func_dec = Func {
      name: "foo".to_string(),
      params: vec![],
      ret_type: None,
      body_start: Node::While(Box::new(Node::Number(4)), Box::new(Node::Empty), None),
    };
    let mut funcs: HashMap<String, Func> = HashMap::new();
    funcs.insert("foo".to_string(), func_dec.clone());
    assert!(!type_check_function(&func_dec, &funcs).is_ok());
  }

  #[test]
  pub fn test_non_mutability_in_if_statement() {
    let assign = Box::new(Node::Assign(
      "a".to_string(),
      Box::new(Node::Number(3)),
      None,
    ));
    let if_statement = Box::new(Node::If(Box::new(Node::Bool(true)), assign, None, None));
    let let_statement = Node::Let(
      "a".to_string(),
      None,
      false,
      Box::new(Node::Number(2)),
      Some(if_statement),
    );
    let func_dec = Func {
      name: "foo".to_string(),
      params: vec![],
      ret_type: None,
      body_start: let_statement,
    };
    let mut funcs: HashMap<String, Func> = HashMap::new();
    funcs.insert("foo".to_string(), func_dec.clone());
    assert!(!type_check_function(&func_dec, &funcs).is_ok());
  }

  #[test]
  pub fn test_non_mutability_in_while_statement() {
    let assign = Box::new(Node::Assign(
      "a".to_string(),
      Box::new(Node::Number(3)),
      None,
    ));
    let while_statement = Box::new(Node::While(Box::new(Node::Bool(true)), assign, None));
    let let_statement = Node::Let(
      "a".to_string(),
      None,
      false,
      Box::new(Node::Number(2)),
      Some(while_statement),
    );
    let func_dec = Func {
      name: "foo".to_string(),
      params: vec![],
      ret_type: None,
      body_start: let_statement,
    };
    let mut funcs: HashMap<String, Func> = HashMap::new();
    funcs.insert("foo".to_string(), func_dec.clone());
    assert!(!type_check_function(&func_dec, &funcs).is_ok());
  }
}
