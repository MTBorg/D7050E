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
                _ => panic!("Unknown opcode: {:?}", op)
            }
        },
        Node::Empty => panic!("Empty node"),
        _ => panic!("Unknown nodetype")
    }
}
