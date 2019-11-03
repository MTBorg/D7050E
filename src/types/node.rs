use crate::types::{_type::Type, opcode::Opcode};

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
  // Attach the next instruction node to a node.
  //
  // # Arguments
  // * `next_instr` - The child node to attach.
  pub fn attach_next_instruction(&mut self, next_instr: Node) {
    match *self {
      Node::Let(.., ref mut right_most)
      | Node::FuncCall(.., ref mut right_most)
      | Node::Assign(.., ref mut right_most)
      | Node::If(.., ref mut right_most)
      | Node::Return(.., ref mut right_most)
      | Node::Print(.., ref mut right_most)
      | Node::DebugContext(ref mut right_most) => {
        *right_most = Some(Box::new(next_instr))
      }
      _ => panic!("Failed to attach right most child (unknown nodetype)!"),
    };
  }

  pub fn get_next_instruction(&self) -> Option<&Node> {
    match self {
      Node::Let(.., ref right_most)
      | Node::FuncCall(.., ref right_most)
      | Node::Assign(.., ref right_most)
      | Node::If(.., ref right_most)
      | Node::Return(.., ref right_most)
      | Node::Print(.., ref right_most)
      | Node::DebugContext(ref right_most) => match right_most {
        Some(node) => Some(&*node),
        _ => None,
      },
      _ => unreachable!("Cannot get next instruction from unknown node type"),
    }
  }

  pub fn expr_into_string(&self) -> String {
    match self {
      Node::Number(i) => i.to_string(),
      Node::Bool(b) => b.to_string(),
      Node::Var(name) => name.clone(),
      Node::Op(left, op, right) => format!(
        "{} {} {}",
        //If the left side is an operation add parenthesis
        if let Node::Op(..) = **left {
          format!("({})", left.expr_into_string())
        } else {
          format!("{}", left.expr_into_string())
        },
        op.to_str(),
        //If the right side is an operation add parenthesis
        if let Node::Op(..) = **right {
          format!("({})", right.expr_into_string())
        } else {
          format!("{}", right.expr_into_string())
        },
      ),
      _ => panic!("Cannot convert node to expression: {:#?}", self),
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
