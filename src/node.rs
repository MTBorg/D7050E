use crate::parsing::expr_parser::Opcode;

#[derive(Debug)]
pub enum Node{
    Number(i32),
    Var(String),
    Let(Box<Node>, Box<Node>, Option<Box<Node>>),
    FuncCall(String, Vec<String>, Option<Box<Node>>),
    Op(Box<Node>, Opcode, Box<Node>),
    If(Box<Node>, Box<Node>, Option<Box<Node>>, Option<Box<Node>>),
    Empty
}

impl Node{
    /// Attach a node to the right most child of a node.
    ///
    /// # Arguments
    /// * `child` - The child node to attach.
    pub fn attach_right_most_child(&mut self, child: Node){
        match *self{
            Node::Let(_,_, ref mut next_op) |
            Node::FuncCall(_,_, ref mut next_op) => *next_op = Some(Box::new(child)),
            Node::If(_,_,_, ref mut next_op) => *next_op = Some(Box::new(child)),
            _ => panic!("Failed to attach right most child (unknown nodetype)!")
        };
    }
}
