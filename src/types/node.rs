use crate::types::{opcode::Opcode, _type::Type, value::Value};

#[derive(Debug, PartialEq, Clone)]
pub enum Node {
  Number(i32),
  Bool(bool),
  //Name
  Var(String),
  // Variable, type, mutable, expression, next instruction
  Let(String, Option<Type>, bool, Box<Node>, Option<Box<Node>>),
  // Variable, expression, next instruction
  Assign(String, Box<Node>, Option<Box<Node>>),
  // Function, arguments, next instruction
  FuncCall(String, Vec<Node>, Option<Box<Node>>),
  // Expr, operation, Expr
  Op(Box<Node>, Opcode, Box<Node>),
  // Condition, then body, else_body, next instruction
  If(Box<Node>, Box<Node>, Option<Box<Node>>, Option<Box<Node>>),
  // Expression, next instruction
  Return(Box<Node>, Option<Box<Node>>),
  // Expression, next instruction
  Print(Box<Node>, Option<Box<Node>>),
  // Next instruction
  DebugContext(Option<Box<Node>>),
  Empty,
}

impl Node {
  /// Attach a node to the right most child of a node.
  ///
  /// # Arguments
  /// * `child` - The child node to attach.
  pub fn attach_right_most_child(&mut self, child: Node) {
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

impl std::ops::Add<Node> for Node {
  type Output = Node;

  fn add(self, other: Node) -> Node {
    match (self, other) {
      (Node::Number(n1), Node::Number(n2)) => Node::Number(n1 + n2),
      _ => panic!("Type error"),
    }
  }
}

impl std::ops::Sub<Node> for Node {
  type Output = Node;

  fn sub(self, other: Node) -> Node {
    match (self, other) {
      (Node::Number(n1), Node::Number(n2)) => Node::Number(n1 - n2),
      _ => panic!("Type error"),
    }
  }
}

impl std::ops::Mul<Node> for Node {
  type Output = Node;

  fn mul(self, other: Node) -> Node {
    match (self, other) {
      (Node::Number(n1), Node::Number(n2)) => Node::Number(n1 * n2),
      _ => panic!("Type error"),
    }
  }
}

impl std::ops::Div<Node> for Node {
  type Output = Node;

  fn div(self, other: Node) -> Node {
    match (self, other) {
      (Node::Number(n1), Node::Number(n2)) => Node::Number(n1 / n2),
      _ => panic!("Type error"),
    }
  }
}

impl std::cmp::PartialOrd<Node> for Node {
  fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
    match (self, other) {
      (Node::Number(n1), Node::Number(n2)) => Some(n1.cmp(n2)),
      // Unreachable because the type checker should catch the type missmatch
      _ => unreachable!("Invalid node comparison"),
    }
  }
}
