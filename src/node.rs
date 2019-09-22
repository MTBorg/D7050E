use crate::parsing::expr_parser::Opcode;

#[derive(Debug, PartialEq, Clone)]
pub enum Node{
    Number(i32),
    Bool(bool),
    Var(String),
    Let(Box<Node>, Box<Node>, Option<Box<Node>>),
    FuncCall(String, Vec<String>, Option<Box<Node>>),
    Op(Box<Node>, Opcode, Box<Node>),
    If(Box<Node>, Box<Node>, Option<Box<Node>>, Option<Box<Node>>),
    DebugContext(Option<Box<Node>>),
    Empty
}

impl Node{
    /// Attach a node to the right most child of a node.
    ///
    /// # Arguments
    /// * `child` - The child node to attach.
    pub fn attach_right_most_child(&mut self, child: Node){
        match *self{
            Node::Let(_,_, ref mut right_most) |
            Node::FuncCall(_,_, ref mut right_most) |
            Node::If(_,_,_, ref mut right_most) |
            Node::DebugContext(ref mut right_most) => *right_most = Some(Box::new(child)),
            _ => panic!("Failed to attach right most child (unknown nodetype)!")
        };
    }
}

impl std::ops::Add<Node> for Node{
    type Output = Node;

    fn add(self, other: Node) -> Node {
        match (self, other){
            (Node::Number(n1), Node::Number(n2)) => Node::Number(n1 + n2),
            _ => panic!("Type error")
        }
    }
}

impl std::ops::Sub<Node> for Node{
    type Output = Node;

    fn sub(self, other: Node) -> Node {
        match (self, other){
            (Node::Number(n1), Node::Number(n2)) => Node::Number(n1 - n2),
            _ => panic!("Type error")
        }
    }
}

impl std::ops::Mul<Node> for Node{
    type Output = Node;

    fn mul(self, other: Node) -> Node {
        match (self, other){
            (Node::Number(n1), Node::Number(n2)) => Node::Number(n1 * n2),
            _ => panic!("Type error")
        }
    }
}

impl std::ops::Div<Node> for Node{
    type Output = Node;

    fn div(self, other: Node) -> Node {
        match (self, other){
            (Node::Number(n1), Node::Number(n2)) => Node::Number(n1 / n2),
            _ => panic!("Type error")
        }
    }
}
