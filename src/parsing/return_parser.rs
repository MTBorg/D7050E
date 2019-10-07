use super::ParseError;
use crate::node::Node;

#[allow(dead_code)]
pub fn parse(s: &str) -> Result<Box<Node>, ParseError> {
  let res = crate::parsing::grammar::ReturnParser::new().parse(s);
  return match res {
    Ok(s) => Ok(s),
    Err(e) => Err(ParseError {
      message: e.to_string(),
    }),
  };
}

#[cfg(test)]
mod tests {
  use super::{parse, Node};
  use crate::parsing::expr_parser::Opcode;

  #[test]
  pub fn test_parse_return_constant() {
    assert_eq!(
      *parse("return 3;").unwrap(),
      Node::Return(Box::new(Node::Number(3)), None)
    );
  }

  #[test]
  pub fn test_parse_return_var() {
    assert_eq!(
      *parse("return a;").unwrap(),
      Node::Return(Box::new(Node::Var("a".to_string())), None)
    );
  }

  #[test]
  pub fn test_parse_return_calculation() {
    assert_eq!(
      *parse("return a + 4;").unwrap(),
      Node::Return(
        Box::new(Node::Op(
          Box::new(Node::Var("a".to_string())),
          Opcode::Add,
          Box::new(Node::Number(4))
        )),
        None
      )
    );
  }

  #[test]
  pub fn test_parse_return_func_call_no_args() {
    assert_eq!(
      *parse("return foo();").unwrap(),
      Node::Return(
        Box::new(Node::FuncCall("foo".to_string(), vec!(), None)),
        None
      )
    );
  }

  #[test]
  pub fn test_parse_return_func_call_with_args() {
    assert_eq!(
      *parse("return foo(a, 2, b - 3);").unwrap(),
      Node::Return(
        Box::new(Node::FuncCall(
          "foo".to_string(),
          vec![
            Node::Var("a".to_string()),
            Node::Number(2),
            Node::Op(
              Box::new(Node::Var("b".to_string())),
              Opcode::Sub,
              Box::new(Node::Number(3))
            )
          ],
          None
        )),
        None
      )
    );
  }
}
