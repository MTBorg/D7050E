use std::{collections::HashMap, convert::TryInto};

use crate::{
  interpreter::eval,
  types::{
    _type::Type, context::Context, func_param::FuncParam, node::Node, scope::Scope,
    value::Value, variable::Variable,
  },
};

#[derive(Debug, Clone)]
pub struct Func {
  pub name: String,
  pub params: Vec<FuncParam>,
  pub ret_type: Option<Type>,
  pub body_start: Node,
}

impl Func {
  pub fn execute(
    &self,
    args: &Vec<Node>,
    funcs: &HashMap<String, Func>,
    context: &mut Context<Variable>,
  ) -> Option<Value> {
    self.validate_arguments(args);

    // Evaluate argument nodes and push the result to the functions scope
    let mut _args: Vec<Variable> = vec![];
    for (node, param) in (*args).iter().zip(self.params.iter()) {
      //Convert the node to a value
      let val: Value = eval(node, context, funcs).try_into().unwrap();

      _args.push(Variable {
        name: param.name.clone(),
        value: val,
      });
    }
    let mut context: Context<Variable> = Context::from(self);
    context.push(Scope::from(_args));

    // Extract return value (if any)
    match eval(&self.body_start, &mut context, &funcs) {
      Node::Number(n) => Some(Value::Int(n)),
      Node::Bool(b) => Some(Value::Bool(b)),
      Node::Empty => None,
      _ => panic!("Unknown return type from function {}", self.name),
    }
  }

  fn validate_arguments(&self, args: &Vec<Node>) {
    if args.len() < self.params.len() {
      let mut error_msg =
        "Missing parameter ".to_string() + &self.params[args.len()].name;
      for param in args.len() + 1..self.params.len() {
        error_msg.push_str(", ");
        error_msg += &self.params[param].name;
      }

      error_msg.push_str(" to function ");
      error_msg += &self.name;
      panic!(error_msg);
    } else if args.len() > self.params.len() {
      panic!("Unexpected argument");
    }
  }
}
