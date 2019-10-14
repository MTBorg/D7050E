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

type TypePair = (Type, bool);
type BoxErr = Box<dyn std::error::Error>;

pub fn type_check_program(
  program: &Program,
) -> Result<(), Vec<BoxErr>> {
  let mut errors: Vec<BoxErr> = vec![];

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

#[allow(dead_code)]
fn type_check_node(
  node: &Node,
  context: &mut Context<TypePair>,
  funcs: &HashMap<String, Func>,
) -> Result<Option<TypePair>, BoxErr> {
  match node {
    Node::Number(_) => Ok(Some((Type::Int, false))),
    Node::Bool(_) => Ok(Some((Type::Bool, false))),
    Node::Var(var) => match context.get_var_type(&var) {
      Some((r#type, mutable)) => Ok(Some(((*r#type).clone(), *mutable))),
      None => Err(Box::new(UnknownVarError { name: var.clone() })),
    },
    Node::Assign(var, expr, _) => type_check_assign(var, expr, context, funcs),
    Node::Op(e1, op, e2) => type_check_expr(e1, e2, op, context, funcs),
    Node::Let(name, r#type, mutable, expr, _) => {
      type_check_let(name, r#type, *mutable, expr, context, funcs)
    }
    Node::FuncCall(func, args, _) => type_check_func_call(func, args, context, funcs),
    Node::If(condition, _, _, _) => type_check_node(condition, context, funcs),
    Node::Return(expr, _) => {type_check_return(expr, context, funcs)}
    _ => Err(Box::new(TypeError::InvalidNode {})),
  }
}

fn type_check_tree(
  func: &Func,
  funcs: &HashMap<String, Func>,
) -> Result<(), Vec<BoxErr>> {
  let mut next_node = Some(&func.body_start);
  let mut errors: Vec<BoxErr> = vec![];
  let mut context: Context<TypePair> = Context::from(func);

  context.push(Scope::from(func.params.clone()));
  let mut returned = false;
  while match next_node {
    Some(_) => true,
    _ => false,
  } {
    //TODO: This should not be needed
    // If the next instruction is an empty node we should be at an empty body
    if let Node::Empty = next_node.unwrap() {
      if let Some(r#type) = func.ret_type.clone() {
        errors.push(Box::new(TypeError::MissingReturn {
          func_name: func.name.clone(),
          ret_type: r#type.clone(),
        }));
        return Err(errors);
      } else {
        return Ok(());
      }
    }

    if let Node::Return(_, _) = next_node.unwrap() {
      returned = true;
    }

    if let Err(e) = type_check_node(&next_node.unwrap(), &mut context, funcs) {
      errors.push(e);
    }

    if let Some(node) = next_node {
      next_node = node.get_next_instruction();
    }
  }

  if let Some(r#type) = &func.ret_type {
    if !returned {
      errors.push(Box::new(TypeError::MissingReturn {
        func_name: func.name.clone(),
        ret_type: (*r#type).clone(),
      }));
    }
  }

  return if errors.len() == 0 {
    Ok(())
  } else {
    Err(errors)
  };
}

pub fn type_check_expr(
  e1: &Node,
  e2: &Node,
  op: &Opcode,
  context: &mut Context<TypePair>,
  funcs: &HashMap<String, Func>,
) -> Result<Option<TypePair>, BoxErr> {
  // Extract the types out of the operands.
  // We don't care about the second value of the tuple (mutable)
  // as an expression is always immutable.
  let type1 = match type_check_node(e1, context, funcs) {
    Ok(res) => match res {
      Some((r#type, _)) => Some(r#type),
      None => None,
    },
    Err(e) => return Err(e),
  };
  let type2 = match type_check_node(e2, context, funcs) {
    Ok(res) => match res {
      Some((r#type, _)) => Some(r#type),
      None => None,
    },
    Err(e) => return Err(e),
  };

  return if type1 == type2 {
    // Extract type and make sure it is not none (void)
    let expr_type = match type1.clone() {
      Some(r#type) => r#type,
      None => {
        return Err(Box::new(TypeError::InvalidOperator {
          op: (*op).clone(),
          type_left: type1,
          type_right: type2,
        }))
      }
    };
    match op {
      // This match is pretty ugly but is needed since arithmetic and
      // most relational operations evaluate to the type of their operands
      // where as logical operations always evaluate to booleans.
      Opcode::Add
      | Opcode::Sub
      | Opcode::Mul
      | Opcode::Div
      | Opcode::Geq
      | Opcode::Gneq
      | Opcode::Lneq
      | Opcode::Leq => match expr_type {
        Type::Int => Ok(Some((Type::Int, false))),
        _ => {
          return Err(Box::new(TypeError::InvalidOperator {
            op: (*op).clone(),
            type_left: type1,
            type_right: type2,
          }))
        }
      },
      Opcode::And | Opcode::Or => match expr_type {
        Type::Bool => Ok(Some((Type::Bool, false))),
        _ => {
          return Err(Box::new(TypeError::InvalidOperator {
            op: (*op).clone(),
            type_left: type1,
            type_right: type2,
          }))
        }
      },
      _ => Ok(Some((expr_type, false))),
    }
  } else {
    Err(Box::new(TypeError::OperatorMissmatch {
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
    }))
  };
}

fn type_check_assign(
  var: &String,
  expr: &Node,
  context: &mut Context<TypePair>,
  funcs: &HashMap<String, Func>,
) -> Result<Option<TypePair>, BoxErr> {
  // Check the type of the right hand side of assignment
  let (expr_type, _) = match type_check_node(expr, context, funcs) {
    Ok(res) => match res {
      Some(r#type) => r#type,
      None => return Err(Box::new(TypeError::NonTypeExpression)),
    },
    Err(e) => return Err(e),
  };

  let res = context.get_var_type(&var);
  match res {
    Some((r#type, mutable)) => {
      if !mutable {
        return Err(Box::new(TypeError::ImmutableAssignment {
          var: var.clone(),
        }));
      }
      return if *r#type != expr_type {
        Err(Box::new(TypeError::AssignMissmatch {
          var: var.clone(),
          r#type: r#type.clone(),
          expr_type: expr_type.clone(),
        }))
      } else {
        Ok(Some((r#type.clone(), true)))
      };
    }
    None => {
      return Err(Box::new(UnknownVarError { name: var.clone() }));
    }
  }
}

fn type_check_let(
  name: &String,
  r#type: &Option<Type>,
  mutable: bool,
  expr: &Node,
  context: &mut Context<TypePair>,
  funcs: &HashMap<String, Func>,
) -> Result<Option<TypePair>, BoxErr> {
  let (expr_type, _) = match type_check_node(expr, context, funcs) {
    Ok(res) => match res {
      Some(r#type) => r#type,
      None => return Err(Box::new(TypeError::NonTypeExpression {})),
    },
    Err(e) => {
      return Err(e);
    }
  };

  return if let Some(r#type) = r#type {
    // If variable type was specified
    if expr_type == *r#type {
      context.insert_type(name, expr_type.clone(), mutable);
      Ok(Some((expr_type, mutable)))
    } else {
      Err(Box::new(TypeError::LetMissmatch {
        r#type: (*r#type).clone(),
        expr_type: expr_type,
      }))
    }
  } else {
    context.insert_type(name, expr_type.clone(), mutable);
    Ok(Some((expr_type, mutable)))
  };
}

fn type_check_func_call(
  func: &String,
  args: &Vec<Node>,
  context: &mut Context<TypePair>,
  funcs: &HashMap<String, Func>,
) -> Result<Option<TypePair>, BoxErr> {
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
    let (arg_type, _) = match type_check_node(arg, context, funcs) {
      Ok(res) => match res {
        Some(r#type) => r#type,
        None => {
          return Err(Box::new(TypeError::ArgMissmatch {
            arg_type: None,
            param: (*param).clone(),
          }))
        }
      },
      Err(e) => return Err(e),
    };

    if arg_type != param._type {
      return Err(Box::new(TypeError::ArgMissmatch {
        arg_type: Some(arg_type),
        param: (*param).clone(),
      }));
    }
  }

  return if let Some(ret_type) = func.ret_type.clone() {
    Ok(Some((ret_type.clone(), false)))
  } else {
    Ok(None)
  };
}

fn type_check_return(
  expr: &Node,
  context: &mut Context<TypePair>,
  funcs: &HashMap<String, Func>,
) -> Result<Option<TypePair>, BoxErr> {
  let (expr_type, _) = match type_check_node(expr, context, funcs) {
    Ok(res) => match res {
      Some(r#type) => r#type,
      None => return Err(Box::new(TypeError::NonTypeExpression {})),
    },
    Err(e) => return Err(e),
  };
  match &context.current_func.ret_type {
    Some(r#type) => {
      if *r#type == expr_type {
        return Ok(Some((r#type.clone(), false)));
      } else {
        return Err(Box::new(TypeError::InvalidReturnType {
          func: context.current_func.clone(),
          expr_type: expr_type,
        }));
      }
    }
    None => {
      return Err(Box::new(TypeError::InvalidReturnType {
        func: context.current_func.clone(),
        expr_type: expr_type,
      }));
    }
  }
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
      type_check_node(&Node::Number(2), &mut context, &HashMap::new()).unwrap(),
      Some((Type::Int, false))
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
      type_check_node(&Node::Bool(true), &mut context, &HashMap::new()).unwrap(),
      Some((Type::Bool, false))
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
      type_check_node(
        &Node::Op(
          Box::new(Node::Number(2)),
          Opcode::Add,
          Box::new(Node::Number(2))
        ),
        &mut context,
        &HashMap::new()
      )
      .unwrap(),
      Some((Type::Int, false))
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
    assert!(!type_check_node(
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
    assert!(!type_check_node(
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
      type_check_node(
        &Node::Op(
          Box::new(Node::Bool(true)),
          Opcode::And,
          Box::new(Node::Bool(true))
        ),
        &mut context,
        &HashMap::new()
      )
      .unwrap(),
      Some((Type::Bool, false))
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
      type_check_node(
        &Node::FuncCall("foo".to_string(), vec!(), None),
        &mut context,
        &funcs
      )
      .unwrap(),
      Some((Type::Int, false))
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
      type_check_node(
        &Node::FuncCall("foo".to_string(), vec!(), None),
        &mut context,
        &funcs
      )
      .unwrap(),
      Some((Type::Bool, false))
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
      type_check_node(
        &Node::FuncCall("foo".to_string(), vec!(Node::Number(2)), None),
        &mut context,
        &funcs
      )
      .unwrap(),
      Some((Type::Int, false))
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
      type_check_node(
        &Node::FuncCall("foo".to_string(), vec!(Node::Bool(true)), None),
        &mut context,
        &funcs
      )
      .unwrap(),
      Some((Type::Bool, false))
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
    assert!(!type_check_node(
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
    assert!(type_check_node(
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
    assert!(!type_check_node(
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
    assert!(type_check_node(
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
    assert!(!type_check_node(
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

    assert!(type_check_node(
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
    assert!(!type_check_node(
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
    assert!(type_check_node(
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
    assert!(!type_check_node(
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
    assert!(!type_check_tree(&func_dec, &funcs).is_ok());
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
    assert!(type_check_tree(&func_dec, &funcs).is_ok());
  }

  #[test]
  pub fn arithmetic_expr_with_two_booleans() {
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
    assert!(!type_check_node(
      &Node::Op(
        Box::new(Node::Bool(true)),
        Opcode::Add,
        Box::new(Node::Bool(false))
      ),
      &mut context,
      &funcs
    )
    .is_ok());
  }

  #[test]
  pub fn arithmetic_expr_with_two_voids() {
    let func_dec = Func {
      name: "foo".to_string(),
      params: vec![],
      ret_type: None,
      body_start: Node::Empty,
    };
    let func_dec_2 = Func {
      name: "bar".to_string(),
      params: vec![],
      ret_type: None,
      body_start: Node::Empty,
    };
    let mut context = Context::from(&func_dec);
    let mut funcs: HashMap<String, Func> = HashMap::new();
    funcs.insert("foo".to_string(), func_dec.clone());
    funcs.insert("bar".to_string(), func_dec_2.clone());
    context.push(Scope::from(func_dec.params.clone()));
    assert!(!type_check_node(
      &Node::Op(
        Box::new(Node::FuncCall("bar".to_string(), vec!(), None)),
        Opcode::Add,
        Box::new(Node::FuncCall("bar".to_string(), vec!(), None))
      ),
      &mut context,
      &funcs
    )
    .is_ok());
  }
}
