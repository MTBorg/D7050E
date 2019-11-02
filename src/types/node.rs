use crate::types::{_type::Type, opcode::Opcode, value::Value};

#[derive(Debug, PartialEq, Clone)]
pub enum Node<'a> {
  Number(i32),
  Bool(bool),
  //Name
  Var(&'a str),
  // Variable, type, mutable, expression, next instruction
  Let(
    &'a str,
    Option<Type>,
    bool,
    Box<Node<'a>>,
    Option<Box<Node<'a>>>,
  ),
  // Variable, expression, next instruction
  Assign(&'a str, Box<Node<'a>>, Option<Box<Node<'a>>>),
  // Function, arguments, next instruction
  FuncCall(&'a str, Vec<Node<'a>>, Option<Box<Node<'a>>>),
  // Expr, operation, Expr
  Op(Box<Node<'a>>, Opcode, Box<Node<'a>>),
  // Condition, then body, else_body, next instruction
  If(
    Box<Node<'a>>,
    Box<Node<'a>>,
    Option<Box<Node<'a>>>,
    Option<Box<Node<'a>>>,
  ),
  // Expression, next instruction
  Return(Box<Node<'a>>, Option<Box<Node<'a>>>),
  // Expression, next instruction
  Print(Box<Node<'a>>, Option<Box<Node<'a>>>),
  // Next instruction
  DebugContext(Option<Box<Node<'a>>>),
  Empty,
}

impl<'a> Node<'a> {
  /// Attach a node to the right most child of a node.
  ///
  /// # Arguments
  /// * `child` - The child node to attach.
  pub fn attach_right_most_child(&mut self, child: Node<'a>) {
    match *self {
      Node::Let(_, _, _, _, ref mut right_most)
      | Node::FuncCall(_, _, ref mut right_most)
      | Node::Assign(_, _, ref mut right_most)
      | Node::If(_, _, _, ref mut right_most)
      | Node::Return(_, ref mut right_most)
      | Node::Print(_, ref mut right_most)
      | Node::DebugContext(ref mut right_most) => *right_most = Some(Box::new(child)),
      _ => panic!("Failed to attach right most child (unknown nodetype)!"),
    };
  }

  pub fn to_value(&self) -> Result<Value, &'static str> {
    match self {
      Node::Bool(b) => Ok(Value::Bool(*b)),
      Node::Number(n) => Ok(Value::Int(*n)),
      _ => Err("Cannot convert node to value"),
    }
  }

  pub fn get_next_instruction(&self) -> Option<&Node> {
    match self {
      Node::Let(_, _, _, _, ref right_most)
      | Node::FuncCall(_, _, ref right_most)
      | Node::Assign(_, _, ref right_most)
      | Node::If(_, _, _, ref right_most)
      | Node::Return(_, ref right_most)
      | Node::Print(_, ref right_most)
      | Node::DebugContext(ref right_most) => match right_most {
        Some(node) => Some(&*node),
        _ => None,
      },
      _ => unreachable!("Cannot get next instruction from unknown node type"),
    }
  }
}

impl<'a> std::ops::Add<Node<'a>> for Node<'_> {
  type Output = Node<'a>;

  fn add(self, other: Node) -> Node {
    match (self, other) {
      (Node::Number(n1), Node::Number(n2)) => Node::Number(n1 + n2),
      _ => panic!("Type error"),
    }
  }
}

impl<'a> std::ops::Sub<Node<'a>> for Node<'a> {
  type Output = Node<'a>;

  fn sub(self, other: Node) -> Node {
    match (self, other) {
      (Node::Number(n1), Node::Number(n2)) => Node::Number(n1 - n2),
      _ => panic!("Type error"),
    }
  }
}

impl<'a> std::ops::Mul<Node<'a>> for Node<'a> {
  type Output = Node<'a>;

  fn mul(self, other: Node) -> Node {
    match (self, other) {
      (Node::Number(n1), Node::Number(n2)) => Node::Number(n1 * n2),
      _ => panic!("Type error"),
    }
  }
}

impl<'a> std::ops::Div<Node<'a>> for Node<'a> {
  type Output = Node<'a>;

  fn div(self, other: Node) -> Node {
    match (self, other) {
      (Node::Number(n1), Node::Number(n2)) => Node::Number(n1 / n2),
      _ => panic!("Type error"),
    }
  }
}

impl<'a> std::cmp::PartialOrd<Node<'a>> for Node<'a> {
  fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
    match (self, other) {
      (Node::Number(n1), Node::Number(n2)) => Some(n1.cmp(n2)),
      // Unreachable because the type checker should catch the type missmatch
      _ => unreachable!("Invalid node comparison"),
    }
  }
}
