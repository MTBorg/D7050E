use crate::node::Node;
use crate::parsing::expr_parser::Opcode;

pub fn eval(node: Node) -> Node {
    match node{
        Node::Number(_) |
        Node::Bool(_) |
        Node::Var(_) => node,
        Node::Op(left_node, op, right_node) => {
            match op{
                Opcode::Add => eval(*left_node) + eval(*right_node),
                Opcode::Sub => eval(*left_node) - eval(*right_node),
                Opcode::Mul => eval(*left_node) * eval(*right_node),
                Opcode::Div => eval(*left_node) / eval(*right_node),
                Opcode::Eq => Node::Bool(eval(*left_node) == eval(*right_node)),
                _ => panic!("Unknown opcode: {:?}", op)
            }
        },
        Node::If(expr, then_body, else_body, next_instr) => {
            if eval(*expr) == Node::Bool(true) {
                eval(*then_body)
            } else {
                match else_body {
                    Some(body) => {
                        eval(*body)
                    },
                    None => match next_instr {
                        Some(instr) => eval(*instr),
                        None => eval(Node::Empty)
                    }
                }
            }
        },
        Node::Empty => Node::Empty,
        _ => panic!("Unknown nodetype")
    }
}
