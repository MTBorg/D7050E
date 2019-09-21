use std::collections::HashMap;

use crate::node::Node;
use crate::parsing::expr_parser::Opcode;
use crate::types::{ Context, Scope };
use crate::func::FuncDec;

pub fn eval(node: &Node, context: &mut Context, funcs: &HashMap<String, FuncDec>) -> Node {
    match node{
        Node::Number(_) |
        Node::Bool(_) |
        Node::Var(_) => node.clone(),
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
        Node::FuncCall(func, args, next_instr) => {
            context.push(create_scope((*args).clone()));
            match funcs.get(func){
                Some(func) => { 
                    func.execute(funcs);
                    validate_arguments(&args, func);
                } ,
                None => panic!("No function {}", func)
            }
            match next_instr {
                Some(instr) => eval(instr, context, funcs),
                None => Node::Empty
            }
        },
        Node::Empty => Node::Empty,
        _ => panic!("Unknown nodetype")
    }
}

fn validate_arguments(args: &Vec<String>, func: &FuncDec){
    if args.len() < func.params.len(){
        let mut error_msg = "Missing parameter ".to_string() +
            &func.params[args.len()].name;
        for param in args.len()+1..func.params.len(){
            error_msg.push_str(", ");
            error_msg += &func.params[param].name;
        }

        error_msg.push_str(" to function ");
        error_msg += &func.name;
        panic!(error_msg);
    }
}

fn create_scope(mut args: Vec<String>) -> Scope{
    let mut scope = HashMap::new();
    scope.reserve(args.len());
    for arg in args.drain(..){
        if scope.contains_key(&arg){
            panic!("Duplicate argument");
        }
        //TODO: Don't use arg arg
        scope.insert(arg.clone(), arg);
    }
    scope
}
