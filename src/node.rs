use crate::parsing::expr_parser::Opcode;

#[derive(Debug)]
pub enum Node{
    Number(i32),
    Var(String),
    Op(Box<Node>, Opcode, Box<Node>)
}
