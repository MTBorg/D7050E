use std::{collections::HashMap, convert::TryInto};

use crate::types::{
  context::Context, func::Func, node::Node, opcode::Opcode, scope::Scope, value::Value,
  variable::Variable,
};

macro_rules! eval_next_instr {
  ($next_instr:expr, $context:expr, $funcs:expr) => {
    match $next_instr {
      Some(instr) => eval(instr, $context, $funcs),
      None => Node::Empty,
    }
  };
}

pub fn eval(
  node: &Node,
  context: &mut Context<Variable>,
  funcs: &HashMap<String, Func>,
) -> Node {
  match node {
    Node::Var(var_name) => match context.get_variable(&var_name) {
      Some(var) => match var.value {
        Value::Bool(b) => Node::Bool(b),
        Value::Int(n) => Node::Number(n),
      },
      None => panic!("Undefined variable {}", (*var_name)),
    },
    Node::Number(_) | Node::Bool(_) => node.clone(),
    Node::Op(left_node, op, right_node) => {
      let left = eval(left_node, context, funcs);
      let right = eval(right_node, context, funcs);
      match op {
        Opcode::Add => left + right,
        Opcode::Sub => left - right,
        Opcode::Mul => left * right,
        Opcode::Div => left / right,
        Opcode::Geq => Node::Bool(left >= right),
        Opcode::Leq => Node::Bool(left <= right),
        Opcode::Gneq => Node::Bool(left > right),
        Opcode::Lneq => Node::Bool(left < right),
        Opcode::Eq => Node::Bool(left == right),
        Opcode::Neq => Node::Bool(left != right),
        Opcode::And => {
          let b1 = match left {
            Node::Bool(b) => b,
            _ => panic!("Left side of logical operator && does not evaluate to boolean"),
          };
          let b2 = match right {
            Node::Bool(b) => b,
            _ => panic!("Right side of logical operator && does not evaluate to boolean"),
          };
          Node::Bool(b1 && b2)
        }
        Opcode::Or => {
          let b1 = match left {
            Node::Bool(b) => b,
            _ => panic!("Left side of logical operator || does not evaluate to boolean"),
          };
          let b2 = match right {
            Node::Bool(b) => b,
            _ => panic!("Right side of logical operator || does not evaluate to boolean"),
          };
          Node::Bool(b1 || b2)
        }
      }
    }
    Node::If(expr, then_body, else_body, next_instr) => {
      context.push(Scope::new());
      let res = if eval(expr, context, funcs) == Node::Bool(true) {
        eval(then_body, context, funcs)
      } else {
        match else_body {
          Some(body) => eval(body, context, funcs),
          None => Node::Empty,
        }
      };
      context.pop();

      // If res is empty then there can not have been a return statement in any of the
      // if/else-bodies and thus the next instruction should be executed, otherwise
      // the value from the bodies should be returned.
      return if let Node::Empty = res {
        eval_next_instr!(next_instr, context, funcs)
      } else {
        res
      };
    }
    Node::DebugContext(next_instr) => {
      debug_print!(context);
      eval_next_instr!(next_instr, context, funcs)
    }
    Node::Print(expr, next_instr) => {
      debug_print!(eval(expr, context, funcs));
      eval_next_instr!(next_instr, context, funcs)
    }
    Node::FuncCall(func, args, next_instr) => {
      let ret_val = match funcs.get(func) {
        Some(func) => func.execute(args, funcs, context),
        None => panic!("No function {}", func),
      };
      match next_instr {
        Some(instr) => eval(instr, context, funcs),
        None => match ret_val {
          Some(val) => match val {
            Value::Int(n) => Node::Number(n),
            Value::Bool(b) => Node::Bool(b),
          },
          None => Node::Empty,
        },
      }
    }
    Node::Let(id, r#type, _, expr, next_instr) => {
      let val: Value = eval(expr, context, funcs).try_into().unwrap();

      if let Some(r#type) = r#type {
        let expr_type = (&val).into();
        if *r#type != expr_type {
          panic!(
            "Specified type {} for variable {} does not match type {} of expression",
            r#type.to_str(),
            id,
            expr_type.to_str()
          );
        }
      }
      context.insert_variable(Variable {
        name: id.to_string(),
        value: val,
      });
      eval_next_instr!(next_instr, context, funcs)
    }
    Node::Assign(id, expr, next_instr) => {
      let val: Value = match eval(expr, context, funcs).try_into() {
        Ok(val) => val,
        Err(e) => panic!("Invalid expression in assign statement: {}", e),
      };
      match context.get_variable_mut(id) {
        Some(var) => {
          *var = Variable {
            name: id.to_string(),
            value: val,
          }
        }
        None => panic!("No variable {} found in context", id),
      };
      eval_next_instr!(next_instr, context, funcs)
    }
    Node::Return(expr, _) => eval(expr, context, funcs),
    Node::Empty => Node::Empty,
  }
}
