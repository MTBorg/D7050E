use std::collections::HashMap;

use crate::{
    node::Node,
    parsing::expr_parser::Opcode,
    context::Context,
    func::FuncDec,
    value::Value
};

pub fn eval(node: &Node, context: &mut Context, funcs: &HashMap<String, FuncDec>) -> Node {
    match node{
        Node::Var(var_name) => match context.get_variable(var_name.to_string()){
            Some(var) => match var.value{
                Value::Bool(b) => Node::Bool(b),
                Value::Int(n) => Node::Number(n)
            },
            None => panic!("Undefined variable {}", (*var_name))
        },
        Node::Number(_) |
        Node::Bool(_) => node.clone(),
        Node::Op(left_node, op, right_node) => {
            match op{
                Opcode::Add => eval(left_node, context, funcs) + eval(right_node, context, funcs),
                Opcode::Sub => eval(left_node, context, funcs) - eval(right_node, context, funcs),
                Opcode::Mul => eval(left_node, context, funcs) * eval(right_node, context, funcs),
                Opcode::Div => eval(left_node, context, funcs) / eval(right_node, context, funcs),
                Opcode::Eq => Node::Bool(eval(left_node, context, funcs) == eval(right_node, context, funcs)),
                _ => panic!("Unknown opcode: {:?}", op)
            }
        },
        Node::If(expr, then_body, else_body, next_instr) => {
            if eval(expr, context, funcs) == Node::Bool(true) {
                eval(then_body, context, funcs)
            } else {
                match else_body {
                    Some(body) => {
                        eval(body, context, funcs)
                    },
                    None => match next_instr {
                        Some(instr) => eval(instr, context, funcs),
                        None => eval(&Node::Empty, context, funcs)
                    }
                }
            }
        },
        Node::DebugContext(next_instr) => { 
            debug_print!(context);
            match next_instr{
                Some(instr) => eval(instr, context, funcs),
                None => Node::Empty
            }
        },
        Node::FuncCall(func, args, next_instr) => {
            match funcs.get(func){
                Some(func) => { func.execute(args, funcs, context); },
                None => panic!("No function {}", func)
            }
            match next_instr {
                Some(instr) => eval(instr, context, funcs),
                None => Node::Empty
            }
        },
        Node::Let(id, expr, next_instr) => {
            let val = eval(expr, context, funcs).to_value().unwrap();
            context.insert_variable(id.to_string(), val);
            match next_instr {
                Some(instr) => eval(instr, context, funcs),
                None => Node::Empty
            }
        },
        Node::Empty => Node::Empty,
        _ => panic!("Unknown nodetype")
    }
}


